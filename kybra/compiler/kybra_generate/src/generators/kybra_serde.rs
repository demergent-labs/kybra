// This code is a derivative work of https://github.com/RustPython/RustPython/blob/main/vm/src/py_serde.rs and is licensed partially as follows:

// MIT License

// Copyright (c) 2020 RustPython Team

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use quote::quote;

pub fn generate_kybra_serde() -> proc_macro2::TokenStream {
    quote! {
        #[inline]
        pub fn kybra_serialize<S>(
            vm: &VirtualMachine,
            pyobject: &PyObject,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            KybraPyObjectSerializer { pyobject, vm }.serialize(serializer)
        }

        #[inline]
        pub fn kybra_deserialize<'de, D>(
            vm: &'de VirtualMachine,
            deserializer: D,
        ) -> Result<<KybraPyObjectDeserializer as DeserializeSeed>::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            KybraPyObjectDeserializer { vm }.deserialize(deserializer)
        }

        // We need to have a VM available to serialise a PyObject based on its subclass, so we implement
        // PyObject serialisation via a proxy object which holds a reference to a VM
        pub struct KybraPyObjectSerializer<'s> {
            pyobject: &'s PyObject,
            vm: &'s VirtualMachine,
        }

        impl<'s> KybraPyObjectSerializer<'s> {
            pub fn new(vm: &'s VirtualMachine, pyobject: &'s PyObjectRef) -> Self {
                KybraPyObjectSerializer { pyobject, vm }
            }

            fn clone_with_object(&self, pyobject: &'s PyObjectRef) -> KybraPyObjectSerializer {
                KybraPyObjectSerializer {
                    pyobject,
                    vm: self.vm,
                }
            }
        }

        impl<'s> serde::Serialize for KybraPyObjectSerializer<'s> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let serialize_seq_elements =
                    |serializer: S, elements: &[PyObjectRef]| -> Result<S::Ok, S::Error> {
                        let mut seq = serializer.serialize_seq(Some(elements.len()))?;

                        seq.serialize_element(&"LIST")?;

                        for e in elements {
                            seq.serialize_element(&self.clone_with_object(e))?;
                        }
                        seq.end()
                    };
                let serialize_tuple_elements =
                    |serializer: S, elements: &[PyObjectRef]| -> Result<S::Ok, S::Error> {
                        let mut tup = serializer.serialize_tuple(elements.len())?;

                        tup.serialize_element(&"TUPLE")?;

                        for e in elements {
                            tup.serialize_element(&self.clone_with_object(e))?;
                        }
                        tup.end()
                    };
                let serialize_bytes_elements =
                    |serializer: S, elements: &[u8]| -> Result<S::Ok, S::Error> {
                        let mut seq = serializer.serialize_seq(Some(elements.len()))?;

                        seq.serialize_element(&"BYTES")?;

                        for e in elements {
                            seq.serialize_element(e.into())?;
                        }
                        seq.end()
                    };
                if let Some(s) = self.pyobject.payload::<PyStr>() {
                    serialize(self.vm, self.pyobject, serializer)
                } else if self.pyobject.fast_isinstance(self.vm.ctx.types.float_type) {
                    serialize(self.vm, self.pyobject, serializer)
                } else if self.pyobject.fast_isinstance(self.vm.ctx.types.bool_type) {
                    serialize(self.vm, self.pyobject, serializer)
                } else if self.pyobject.fast_isinstance(self.vm.ctx.types.int_type) {
                    serialize(self.vm, self.pyobject, serializer)
                } else if let Some(list) = self.pyobject.payload_if_subclass::<PyList>(self.vm) {
                    serialize_seq_elements(serializer, &list.borrow_vec())
                } else if let Some(tuple) = self.pyobject.payload_if_subclass::<PyTuple>(self.vm) {
                    serialize_tuple_elements(serializer, tuple)
                } else if let Some(bytes) = self.pyobject.payload_if_subclass::<PyBytes>(self.vm) {
                    serialize_bytes_elements(serializer, bytes.as_bytes())
                } else if self.pyobject.fast_isinstance(self.vm.ctx.types.dict_type) {
                    let dict: PyRef<PyDict> = self.pyobject.to_owned().downcast().unwrap();
                    let pairs: Vec<_> = dict.into_iter().collect();
                    let mut map = serializer.serialize_map(Some(pairs.len()))?;
                    for (key, e) in &pairs {
                        map.serialize_entry(&self.clone_with_object(key), &self.clone_with_object(e))?;
                    }
                    map.end()
                } else if self.vm.is_none(self.pyobject) {
                    serialize(self.vm, self.pyobject, serializer)
                } else {
                    let class = self.pyobject.class();
                    let class_name = class.name();

                    if class_name.to_string() == "Principal" {
                        let to_str = _kybra_unwrap_rust_python_result(self.pyobject.get_attr("to_str", self.vm), self.vm);
                        let to_str_invoke_result = _kybra_unwrap_rust_python_result(self.vm.invoke(&to_str, ()), self.vm);
                        let to_str_invoke_string: String = _kybra_unwrap_rust_python_result(to_str_invoke_result.try_into_value(self.vm), self.vm);

                        return serializer.serialize_str(&format!("KYBRA::Principal::{}", to_str_invoke_string));
                    }

                    Err(serde::ser::Error::custom(format!(
                        "Object of type '{}' is not serializable",
                        self.pyobject.class()
                    )))
                }
            }
        }

        // This object is used as the seed for deserialization so we have access to the Context for type
        // creation
        #[derive(Clone)]
        pub struct KybraPyObjectDeserializer<'c> {
            vm: &'c VirtualMachine,
        }

        impl<'c> KybraPyObjectDeserializer<'c> {
            pub fn new(vm: &'c VirtualMachine) -> Self {
                KybraPyObjectDeserializer { vm }
            }
        }

        impl<'de> DeserializeSeed<'de> for KybraPyObjectDeserializer<'de> {
            type Value = PyObjectRef;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(self.clone())
            }
        }

        impl<'de> Visitor<'de> for KybraPyObjectDeserializer<'de> {
            type Value = PyObjectRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a type that can deserialise in Python")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(self.vm.ctx.new_bool(value).into())
            }

            // Other signed integers delegate to this method by default, it’s the only one needed
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(self.vm.ctx.new_int(value).into())
            }

            // Other unsigned integers delegate to this method by default, it’s the only one needed
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(self.vm.ctx.new_int(value).into())
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(self.vm.ctx.new_float(value).into())
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value.starts_with("KYBRA::Principal::") {
                    let principal_class = _kybra_unwrap_rust_python_result(self.vm.run_block_expr(
                        self.vm.new_scope_with_builtins(),
                        r#"
from kybra import Principal

Principal
                        "#
                    ), self.vm);

                    let from_str = _kybra_unwrap_rust_python_result(principal_class.get_attr("from_str", self.vm), self.vm);
                    let principal_string = value.to_string().replace("KYBRA::Principal::", "");
                    let principal_instance = _kybra_unwrap_rust_python_result(self.vm.invoke(&from_str, (principal_string,)), self.vm);

                    Ok(principal_instance)
                }
                else {
                    // Owned value needed anyway, delegate to visit_string
                    self.visit_string(value.to_owned())
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(self.vm.ctx.new_str(value).into())
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(self.vm.ctx.none())
            }

            fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut seq_type = "".to_string();

                if let Some(first_value) = access.next_element_seed(self.clone())? {
                    let first_value_string: String = _kybra_unwrap_rust_python_result(first_value.try_into_value(self.vm), self.vm);
                    seq_type = first_value_string;
                }

                if seq_type == "BYTES" {
                    let mut seq = Vec::with_capacity(access.size_hint().unwrap_or(0));

                    while let Some(value) = access.next_element_seed(self.clone())? {
                        let value_u8: u8 = value.try_from_vm_value(self.vm).unwrap();
                        seq.push(value_u8);
                    }

                    Ok(self.vm.ctx.new_bytes(seq).into())
                }
                else {
                    let mut seq = Vec::with_capacity(access.size_hint().unwrap_or(0));

                    while let Some(value) = access.next_element_seed(self.clone())? {
                        seq.push(value);
                    }

                    if seq_type == "TUPLE" {
                        Ok(self.vm.ctx.new_tuple(seq).into())
                    }
                    else {
                        Ok(self.vm.ctx.new_list(seq).into())
                    }
                }
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let dict = self.vm.ctx.new_dict();
                // Although JSON keys must be strings, implementation accepts any keys
                // and can be reused by other deserializers without such limit
                while let Some((key_obj, value)) = access.next_entry_seed(self.clone(), self.clone())? {
                    _kybra_unwrap_rust_python_result(dict.set_item(&*key_obj, value, self.vm), self.vm);
                }
                Ok(dict.into())
            }
        }
    }
}
