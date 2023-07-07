use cdk_framework::{
    act::{
        node::{CandidType, Context},
        ToTypeAnnotation,
    },
    traits::ToIdent,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::keywords;

pub fn generate(
    candid_type: &CandidType,
    memory_id: u8,
    key_or_value: &str,
) -> (Ident, TokenStream) {
    let wrapper_struct_name = format!("StableBTreeMap{}{}Type", memory_id, key_or_value);
    let context = Context {
        keyword_list: keywords::get_python_keywords(),
        cdk_name: "kybra".to_string(),
    };
    let key_type = &candid_type.to_type_annotation(&context, wrapper_struct_name.clone());
    let wrapper_struct_name_ident = wrapper_struct_name.to_ident();

    (
        wrapper_struct_name_ident.clone(),
        quote! {
            #[derive(candid::CandidType, candid::Deserialize, CdkActTryFromVmValue, Ord, PartialOrd, Eq, PartialEq, Clone)]
            struct #wrapper_struct_name_ident((#key_type));
        },
    )
}
