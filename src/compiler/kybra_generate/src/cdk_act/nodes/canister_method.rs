use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::cdk_act::{act_node::ActNode, ToTokenStream};

#[derive(Clone)]
pub enum CanisterMethodActNode {
    QueryMethod(CanisterMethod),
    UpdateMethod(CanisterMethod),
}

/// Describes a Rust canister method function body
#[derive(Clone)]
pub struct CanisterMethod {
    pub body: TokenStream,
    pub param_names: Vec<Ident>,
    pub param_types: Vec<ActNode>,
    pub inline_types: Box<Vec<ActNode>>,
    pub is_manual: bool,
    pub name: String,
    pub return_type: ActNode,
    // pub rust_return_type: TokenStream,
}

impl ToTokenStream for CanisterMethodActNode {
    fn to_token_stream(&self) -> TokenStream {
        match self {
            CanisterMethodActNode::QueryMethod(query_method) => {
                let function_signature = generate_function(query_method);

                let manual_reply_arg = if query_method.is_manual {
                    quote! {(manual_reply = true)}
                } else {
                    quote! {}
                };

                quote! {
                    #[ic_cdk_macros::query#manual_reply_arg]
                    #[candid::candid_method(query)]
                    #function_signature
                }
            }
            CanisterMethodActNode::UpdateMethod(update_method) => {
                let function_signature = generate_function(update_method);

                let manual_reply_arg = if update_method.is_manual {
                    quote! {(manual_reply = true)}
                } else {
                    quote! {}
                };

                quote! {
                    #[ic_cdk_macros::update#manual_reply_arg]
                    #[candid::candid_method(update)]
                    #function_signature
                }
            }
        }
    }
}

impl CanisterMethodActNode {
    pub fn get_inline_types(&self) -> Box<Vec<ActNode>> {
        match self {
            CanisterMethodActNode::QueryMethod(canister_method) => {
                canister_method.inline_types.clone()
            }
            CanisterMethodActNode::UpdateMethod(canister_method) => {
                canister_method.inline_types.clone()
            }
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            CanisterMethodActNode::QueryMethod(canister_method) => canister_method.name.clone(),
            CanisterMethodActNode::UpdateMethod(canister_method) => canister_method.name.clone(),
        }
    }

    pub fn get_rust_return_type(&self) -> TokenStream {
        // match self {
        //     CanisterMethodActNode::QueryMethod(canister_method) => {
        //         canister_method.rust_return_type.clone()
        //     }
        //     CanisterMethodActNode::UpdateMethod(canister_method) => {
        //         canister_method.rust_return_type.clone()
        //     }
        // }
        quote! {}
    }

    pub fn is_manual(&self) -> bool {
        match self {
            CanisterMethodActNode::QueryMethod(canister_method) => {
                canister_method.is_manual.clone()
            }
            CanisterMethodActNode::UpdateMethod(canister_method) => {
                canister_method.is_manual.clone()
            }
        }
    }
}

fn generate_params(names: &Vec<Ident>, types: &Vec<ActNode>) -> Vec<TokenStream> {
    names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let param_type_token_stream = types[i].get_type_ident();
            quote! {
                #name: #param_type_token_stream
            }
        })
        .collect()
}

fn generate_function(canister_method: &CanisterMethod) -> TokenStream {
    let function_name = format_ident!("{}", canister_method.name);
    let params = generate_params(&canister_method.param_names, &canister_method.param_types);

    let function_body = &canister_method.body;

    let return_type_token = canister_method.return_type.get_type_ident();
    let wrapped_return_type = if canister_method.is_manual {
        quote! {
            ic_cdk::api::call::ManualReply<#return_type_token>
        }
    } else {
        return_type_token
    };

    quote! {
        async fn #function_name(#(#params),*) -> #wrapped_return_type {
            #function_body
        }
    }
}
