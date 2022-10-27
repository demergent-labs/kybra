use proc_macro2::Ident;
use quote::quote;
use syn::{DataEnum, Fields};

pub fn derive_try_from_vm_value_enum(
    enum_name: &Ident,
    data_enum: &DataEnum,
) -> proc_macro2::TokenStream {
    let item_initializers = derive_item_initializers(enum_name, data_enum);

    quote! {
        impl CdkActTryFromVmValue<#enum_name, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<#enum_name, CdkActTryFromVmValueError> {
                #(#item_initializers)*

                return Err(CdkActTryFromVmValueError("Enum variant does not exist".to_string()));
            }
        }

        impl CdkActTryFromVmValue<Vec<#enum_name>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<#enum_name>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }
    }
}

fn derive_item_initializers(
    enum_name: &Ident,
    data_enum: &DataEnum,
) -> Vec<proc_macro2::TokenStream> {
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
) -> proc_macro2::TokenStream {
    quote! {
        let get_item_result = self.get_item(stringify!(#variant_name), vm);

        if let Ok(item) = get_item_result {
            return Ok(#enum_name::#variant_name(item.try_from_vm_value(vm).unwrap()));
        }
    }
}

fn derive_item_initializers_for_unit(
    enum_name: &Ident,
    variant_name: &Ident,
) -> proc_macro2::TokenStream {
    quote! {
        let get_item_result = self.get_item(stringify!(#variant_name), vm);

        if let Ok(_) = get_item_result {
            return Ok(#enum_name::#variant_name);
        }
    }
}
