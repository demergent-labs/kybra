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

        impl CdkActTryFromVmValue<Vec<#struct_name>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<#struct_name>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }
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
