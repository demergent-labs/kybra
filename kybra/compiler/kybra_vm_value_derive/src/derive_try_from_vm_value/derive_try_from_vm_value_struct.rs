use cdk_framework::traits::ToIdent;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataStruct, Fields, Index};

pub fn derive_try_from_vm_value_struct(
    struct_name: &Ident,
    data_struct: &DataStruct,
) -> TokenStream {
    let field_variable_definitions = generate_field_variable_definitions(data_struct);
    let field_variable_names = generate_field_initializers(data_struct);

    quote! {
        impl CdkActTryFromVmValue<#struct_name, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<#struct_name, rustpython_vm::builtins::PyBaseExceptionRef> {
                #(#field_variable_definitions)*

                Ok(#struct_name {
                    #(#field_variable_names),*
                })
            }
        }

        impl CdkActTryFromVmValue<Vec<#struct_name>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<#struct_name>, rustpython_vm::builtins::PyBaseExceptionRef> {
                try_from_vm_value_generic_array(self, vm)
            }
        }
    }
}

fn generate_field_variable_definitions(data_struct: &DataStruct) -> Vec<TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                let variable_name = format_ident!(
                    "user_defined_{}",
                    field.ident.as_ref().expect("Named field must have a name")
                );
                let restored_field_name = match field_name {
                    Some(field_name) => Some(
                        cdk_framework::keyword::restore_for_vm(
                            &field_name.to_string(),
                            &crate::get_python_keywords(),
                        )
                        .to_ident(),
                    ),
                    None => field_name.clone(),
                };

                quote! {
                    let #variable_name = self.get_item(stringify!(#restored_field_name), vm)?;
                }
            })
            .collect(),
        Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let variable_name = format_ident!("field_{}", index);
                let syn_index = Index::from(index);

                quote! {
                    // TODO tuple_self is being repeated more times than necessary
                    let tuple_self: rustpython_vm::builtins::PyTupleRef = self
                        .clone()
                        .try_into_value(vm)?;
                    let #variable_name = match tuple_self.get(#syn_index) {
                        Some(element) => element,
                        None => return Err(vm.new_index_error(format!("tuple index out of range"))),
                    };
                }
            })
            .collect(),
        _ => panic!("Only named and unnamed fields supported for Structs"),
    }
}

fn generate_field_initializers(data_struct: &DataStruct) -> Vec<TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                let variable_name = format_ident!(
                    "user_defined_{}",
                    field.ident.as_ref().expect("Named field must have a name")
                );

                quote! {
                    #field_name: #variable_name.try_from_vm_value(vm)?
                }
            })
            .collect(),
        Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let variable_name = format_ident!("field_{}", index);
                let syn_index = Index::from(index);

                quote! {
                    #syn_index: #variable_name.clone().try_from_vm_value(vm)?
                }
            })
            .collect(),
        _ => panic!("Only named and unnamed fields supported for Structs"),
    }
}
