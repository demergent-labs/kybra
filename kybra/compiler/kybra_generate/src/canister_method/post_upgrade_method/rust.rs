use cdk_framework::{act::node::Param, traits::ToIdent};
use proc_macro2::TokenStream;
use quote::quote;

use crate::errors::KybraResult;

pub fn generate(params: &Vec<Param>) -> KybraResult<TokenStream> {
    let params_initializations = params.iter().map(|param| {
        let param_name = format!("_cdk_user_defined_{}", param.name).to_ident();
        let init_param_name = format!("INIT_PARAM_{}", param.name).to_ident();

        quote! {
            #init_param_name.with(|x| *x.borrow_mut() = Some(#param_name));
        }
    });

    // TODO I think we will want to put some permissions on this, like the original installer
    Ok(quote! {
        #(#params_initializations)*
    })
}
