use cdk_framework::traits::CollectResults;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustpython_parser::ast::Located;
use rustpython_parser::ast::StmtKind;

use crate::{
    kybra_unreachable, method_utils::params::InternalOrExternal, source_map::SourceMapped, tuple,
    Error,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn generate_call_to_py_function(&self) -> Result<TokenStream, Vec<Error>> {
        match self.node {
            StmtKind::FunctionDef { .. } => {
                let (function_name, params) = (
                    self.get_name_or_err().map_err(Error::into),
                    self.build_params(InternalOrExternal::Internal),
                )
                    .collect_results()?;

                let param_conversions = params
                    .iter()
                    .map(|param| {
                        let name = format_ident!("{}", param.get_prefixed_name());
                        quote! {
                            #name.try_into_vm_value(vm).unwrap_or_trap()
                        }
                    })
                    .collect();
                let params = tuple::generate_tuple(&param_conversions);

                Ok(quote! {
                    let params = #params;

                    call_global_python_function_sync(#function_name, params)
                        .unwrap_or_trap()
                })
            }
            _ => kybra_unreachable!(),
        }
    }
}
