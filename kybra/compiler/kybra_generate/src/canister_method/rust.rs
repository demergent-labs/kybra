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
                    let interpreter = INTERPRETER_OPTION
                        .as_mut()
                        .unwrap_or_trap("SystemError: missing python interpreter");
                    let scope = SCOPE_OPTION
                        .as_mut()
                        .unwrap_or_trap("SystemError: missing python scope");

                    interpreter.enter(|vm| {
                        let method_py_object_ref = scope.globals.get_item(#function_name, vm).unwrap_or_trap(vm);

                        let result_py_object_ref = method_py_object_ref.call(#params, vm);

                        match result_py_object_ref {
                            Ok(py_object_ref) => py_object_ref.try_from_vm_value(vm).unwrap_or_trap(),
                            Err(err) => {
                                let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                                panic!("{}", err_string);
                            }
                        }
                    });
                })
            }
            _ => kybra_unreachable!(),
        }
    }
}
