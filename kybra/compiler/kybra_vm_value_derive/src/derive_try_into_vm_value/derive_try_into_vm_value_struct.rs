use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{DataStruct, Fields, Index};

pub fn derive_try_into_vm_value_struct(
    struct_name: &Ident,
    data_struct: &DataStruct,
) -> proc_macro2::TokenStream {
    let data_structure_definition = derive_data_structure_definition(data_struct);
    let variable_definitions = derive_struct_fields_variable_definitions(data_struct);
    let property_definitions = derive_struct_fields_property_definitions(data_struct);

    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for #struct_name {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                #(#variable_definitions)*

                #data_structure_definition

                #(#property_definitions)*

                Ok(py_data_structure.into())
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for Vec<#struct_name> {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                try_into_vm_value_generic_array(self, vm)
            }
        }
    }
}

fn derive_data_structure_definition(data_struct: &DataStruct) -> proc_macro2::TokenStream {
    match &data_struct.fields {
        Fields::Named(_) => quote!(let py_data_structure = vm.ctx.new_dict();),
        Fields::Unnamed(fields_unnamed) => {
            let field_names: Vec<proc_macro2::TokenStream> = fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    let field_name = format_ident!("field_{}", index);

                    quote!(#field_name)
                })
                .collect();

            quote!(let py_data_structure = vm.ctx.new_tuple(vec![#(#field_names),*]);)
        }
        _ => panic!("Only named and unnamed fields supported for Structs"),
    }
}

fn derive_struct_fields_variable_definitions(
    data_struct: &DataStruct,
) -> Vec<proc_macro2::TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;

                quote! {
                    let #field_name = self.#field_name.try_into_vm_value(vm).unwrap();
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
                    let #field_name = self.#syn_index.try_into_vm_value(vm).unwrap();
                }
            })
            .collect(),
        _ => panic!("Only named and unnamed fields supported for Structs"),
    }
}

fn derive_struct_fields_property_definitions(
    data_struct: &DataStruct,
) -> Vec<proc_macro2::TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;

                quote! {
                    py_data_structure.set_item(
                        stringify!(#field_name),
                        #field_name,
                        vm
                    );
                }
            })
            .collect(),
        Fields::Unnamed(_) => vec![],
        _ => panic!("Only named and unnamed fields supported for Structs"),
    }
}
