use cdk_framework::{act::node::DataType, ToTokenStream};
use quote::{format_ident, quote};
use syn::Ident;

pub fn generate(
    act_data_type: &DataType,
    memory_id: u8,
    key_or_value: &str,
) -> (Ident, proc_macro2::TokenStream) {
    let key_type = &act_data_type.to_token_stream(&vec![]); // TODO do we need the keyword lists?
    let wrapper_struct_name_ident =
        format_ident!("StableBTreeMap{}{}Type", memory_id, key_or_value);

    (
        wrapper_struct_name_ident.clone(),
        quote! {
            #[derive(CandidType, Deserialize, CdkActTryFromVmValue)]
            struct #wrapper_struct_name_ident(#key_type);
        },
    )
}
