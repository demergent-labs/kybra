use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{DataStruct, Fields, Index};

pub fn derive_try_from_vm_value_struct(
    struct_name: &Ident,
    data_struct: &DataStruct,
) -> proc_macro2::TokenStream {
    let field_variable_definitions = generate_field_variable_definitions(data_struct);
    let field_variable_names = generate_field_initializers(data_struct);

    quote! {
        impl CdkActTryFromVmValue<#struct_name, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<#struct_name, CdkActTryFromVmValueError> {
                #(#field_variable_definitions)*

                Ok(#struct_name {
                    #(#field_variable_names),*
                })
            }
        }

        // TODO the body of this function is repeated in try_from_vm_value_trait.ts
        // impl CdkActTryFromVmValue<Vec<#struct_name>, &mut boa_engine::Context> for boa_engine::JsValue {
        //     fn try_from_vm_value(self, context: &mut boa_engine::Context) -> Result<Vec<#struct_name>, CdkActTryFromVmValueError> {
        //         match self.as_object() {
        //             Some(js_object) => {
        //                 if js_object.is_array() {
        //                     let mut processing: bool = true;
        //                     let mut index: usize = 0;

        //                     let mut result = vec![];

        //                     while processing == true {
        //                         match js_object.get(index, context) {
        //                             Ok(js_value) => {
        //                                 if js_value.is_undefined() {
        //                                     processing = false;
        //                                 }
        //                                 else {
        //                                     match js_value.try_from_vm_value(&mut *context) {
        //                                         Ok(value) => {
        //                                             result.push(value);
        //                                             index += 1;
        //                                         }
        //                                         Err(err) => {
        //                                             return Err(err);
        //                                         }
        //                                     }
        //                                 }
        //                             },
        //                             Err(_) => {
        //                                 return Err(CdkActTryFromVmValueError("Item at array index does not exist".to_string()))
        //                             }
        //                         }
        //                     }

        //                     Ok(result)
        //                 }
        //                 else {
        //                     Err(CdkActTryFromVmValueError("JsObject is not an array".to_string()))
        //                 }
        //             },
        //             None => Err(CdkActTryFromVmValueError("JsValue is not an object".to_string()))
        //         }
        //     }
        // }
    }
}

fn generate_field_variable_definitions(data_struct: &DataStruct) -> Vec<proc_macro2::TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;

                quote! {
                    let #field_name = self.get_item(stringify!(#field_name), vm).unwrap();
                }
            })
            .collect(),
        Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let field_name = format_ident!("field_{}", index);
                let syn_index = Index::from(index);

                quote! {
                    // TODO tuple_self is being repeated more times than necessary
                    let tuple_self: PyTupleRef = self.clone().try_into_value(vm).unwrap();
                    let #field_name = tuple_self.get(#syn_index).unwrap();
                }
            })
            .collect(),
        _ => panic!("Only named and unnamed fields supported for Structs"),
    }
}

fn generate_field_initializers(data_struct: &DataStruct) -> Vec<proc_macro2::TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;

                quote! {
                    #field_name: #field_name.try_from_vm_value(vm).unwrap()
                }
            })
            .collect(),
        Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let field_name = format_ident!("field_{}", index);
                let syn_index = Index::from(index);

                quote! {
                    #syn_index: #field_name.clone().try_from_vm_value(vm).unwrap()
                }
            })
            .collect(),
        _ => panic!("Only named and unnamed fields supported for Structs"),
    }
}
