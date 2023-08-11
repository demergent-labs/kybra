use cdk_framework::traits::ToIdent;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Fields};

pub fn derive_try_from_vm_value_enum(enum_name: &Ident, data_enum: &DataEnum) -> TokenStream {
    let item_initializers = derive_item_initializers(enum_name, data_enum);

    quote! {
        impl CdkActTryFromVmValue<#enum_name, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<#enum_name, rustpython_vm::builtins::PyBaseExceptionRef> {
                #(#item_initializers)*

                return Err(vm.new_type_error("Enum variant does not exist".to_string()));
            }
        }

        impl CdkActTryFromVmValue<Vec<#enum_name>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<#enum_name>, rustpython_vm::builtins::PyBaseExceptionRef> {
                try_from_vm_value_generic_array(self, vm)
            }
        }
    }
}

fn derive_item_initializers(enum_name: &Ident, data_enum: &DataEnum) -> Vec<TokenStream> {
    data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.fields {
                Fields::Named(_) => panic!("Named fields not currently supported"),
                Fields::Unnamed(_) => {
                    derive_item_initializers_for_unnamed_fields(enum_name, variant_name)
                }
                Fields::Unit => derive_item_initializers_for_unit(enum_name, variant_name),
            }
        })
        .collect()
}

fn derive_item_initializers_for_unnamed_fields(
    enum_name: &Ident,
    variant_name: &Ident,
) -> TokenStream {
    let restored_variant_name = cdk_framework::keyword::restore_for_vm(
        &variant_name.to_string(),
        &crate::get_python_keywords(),
    )
    .to_ident();
    quote! {
        let get_item_result = self.get_item(stringify!(#restored_variant_name), vm);

        if let Ok(item) = get_item_result {
            return Ok(#enum_name::#variant_name(item.try_from_vm_value(vm)?));
        }
    }
}

fn derive_item_initializers_for_unit(enum_name: &Ident, variant_name: &Ident) -> TokenStream {
    let restored_variant_name = cdk_framework::keyword::restore_for_vm(
        &variant_name.to_string(),
        &crate::get_python_keywords(),
    )
    .to_ident();
    quote! {
        let get_item_result = self.get_item(stringify!(#restored_variant_name), vm);

        if let Ok(_) = get_item_result {
            return Ok(#enum_name::#variant_name);
        }
    }
}
