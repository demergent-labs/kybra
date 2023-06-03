use cdk_framework::traits::ToIdent;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Field, Fields};

pub fn derive_try_into_vm_value_enum(enum_name: &Ident, data_enum: &DataEnum) -> TokenStream {
    let variant_branches = derive_variant_branches(&enum_name, &data_enum);

    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for #enum_name {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                match self {
                    #(#variant_branches),*
                }
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for Vec<#enum_name> {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                try_into_vm_value_generic_array(self, vm)
            }
        }
    }
}

fn derive_variant_branches(enum_name: &Ident, data_enum: &DataEnum) -> Vec<TokenStream> {
    data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.fields {
                Fields::Named(_) => panic!("Named fields not currently supported"),
                Fields::Unnamed(fields_unnamed) => derive_variant_branches_unnamed_fields(
                    enum_name,
                    variant_name,
                    fields_unnamed.unnamed.iter().collect(),
                ),
                Fields::Unit => {
                    derive_variant_branches_unnamed_fields(enum_name, variant_name, vec![])
                }
            }
        })
        .collect()
}

fn derive_variant_branches_unnamed_fields(
    enum_name: &Ident,
    variant_name: &Ident,
    unnamed_fields: Vec<&Field>,
) -> TokenStream {
    let restored_variant_name = cdk_framework::keyword::restore_for_vm(
        &variant_name.to_string(),
        &crate::get_python_keywords(),
    )
    .to_ident();
    if unnamed_fields.len() == 0 {
        quote! {
            #enum_name::#variant_name => {
                let dict = vm.ctx.new_dict();

                dict.set_item(stringify!(#restored_variant_name), vm.ctx.none(), vm);

                Ok(dict.into())
            }
        }
    } else {
        quote! {
            #enum_name::#variant_name(value) => {
                let dict = vm.ctx.new_dict();

                dict.set_item(stringify!(#restored_variant_name), value.try_into_vm_value(vm)?, vm);

                Ok(dict.into())
            }
        }
    }
}
