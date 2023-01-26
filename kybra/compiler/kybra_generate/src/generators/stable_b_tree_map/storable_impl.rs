use proc_macro2::{Ident, TokenStream};

pub fn generate(wrapper_type_name: &Ident) -> TokenStream {
    // match act_data_type {
    //     ActDataType::Array(_) => quote! {},
    //     ActDataType::Func(_) => quote! {},
    //     ActDataType::Option(_) => quote! {},
    //     ActDataType::Primitive(primitive) => match &primitive.act_type {
    //         cdk_framework::nodes::data_type_nodes::LiteralOrTypeAlias::Literal(literal) => {
    //             let wrapper_type_name_ident = format_ident!("{}", wrapper_type_name);

    //             quote! {
    //                 impl Storable for #wrapper_type_name_ident {
    //                     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
    //                         Cow::Owned(candid::Encode!(self).unwrap())
    //                     }

    //                     fn from_bytes(bytes: Vec<u8>) -> Self {
    //                         candid::Decode!(&bytes, Self).unwrap()
    //                     }
    //                 }
    //             }
    //         }
    //         cdk_framework::nodes::data_type_nodes::LiteralOrTypeAlias::TypeAlias(_) => todo!(),
    //     },
    //     ActDataType::Record(_) => quote! {},
    //     ActDataType::Tuple(_) => quote! {},
    //     ActDataType::TypeRef(_) => quote! {},
    //     ActDataType::Variant(_) => quote! {},
    // }

    // let wrapper_type_name_ident = format_ident!("{}", wrapper_type_name);

    quote::quote! {
        impl Storable for #wrapper_type_name {
            fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                Cow::Owned(candid::Encode!(self).unwrap())
            }

            fn from_bytes(bytes: Vec<u8>) -> Self {
                candid::Decode!(&bytes, Self).unwrap()
            }
        }
    }
}
