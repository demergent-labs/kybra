use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        use rustpython_vm::{AsObject, builtins::{PyDict, PyBaseException, PyGenerator, PyListRef, PyTupleRef, PyIntRef, PyStr, PyList, PyTuple, PyBytes}, class::PyClassImpl, convert::ToPyObject, function::IntoFuncArgs, PyObjectRef, PyObject, PyRef, VirtualMachine, protocol::{PyIter, PyIterReturn}, py_serde::{deserialize, serialize}};
        use rustpython_derive::{pyclass, PyPayload};
        use kybra_vm_value_derive::{CdkActTryIntoVmValue, CdkActTryFromVmValue};
        use std::str::FromStr;
        use ic_cdk::api::call::CallResult;
        use serde::de::{DeserializeSeed, Visitor};
        use serde::ser::{Serialize, SerializeMap, SerializeSeq, SerializeTuple};
        use slotmap::Key;
        use rand::{Rng, SeedableRng, rngs::StdRng};
        use std::convert::TryInto;
    }
}
