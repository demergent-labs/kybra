use cdk_framework::{
    act::{node::CandidType, ToTypeAnnotation},
    traits::ToIdent,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate(
    act_data_type: &CandidType,
    memory_id: u8,
    key_or_value: &str,
) -> (Ident, TokenStream) {
    let wrapper_struct_name = format!("StableBTreeMap{}{}Type", memory_id, key_or_value);
    let key_type = &act_data_type
        .to_type_annotation(&crate::get_python_keywords(), wrapper_struct_name.clone());
    let wrapper_struct_name_ident = wrapper_struct_name.to_ident();

    (
        wrapper_struct_name_ident.clone(),
        quote! {
            #[derive(CandidType, Deserialize, CdkActTryFromVmValue)]
            struct #wrapper_struct_name_ident(#key_type);
        },
    )
}
