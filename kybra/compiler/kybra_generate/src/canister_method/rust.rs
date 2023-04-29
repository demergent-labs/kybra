use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustpython_parser::ast::Located;
use rustpython_parser::ast::StmtKind;

use crate::{
    errors::KybraResult, method_utils::params::InternalOrExternal, source_map::SourceMapped, tuple,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn generate_call_to_py_function(&self) -> KybraResult<TokenStream> {
        match self.node {
            StmtKind::FunctionDef { .. } => {
                let function_name = self.get_function_name()?;
                let params = self.build_params(InternalOrExternal::Internal)?;

                let param_conversions = params
                    .iter()
                    .map(|param| {
                        let name = format_ident!("{}", param.get_prefixed_name());
                        quote! {
                            #name.try_into_vm_value(vm).unwrap()
                        }
                    })
                    .collect();
                let params = tuple::generate_tuple(&param_conversions);

                Ok(quote! {
                    let interpreter = INTERPRETER_OPTION.as_mut().unwrap();
                    let scope = SCOPE_OPTION.as_mut().unwrap();

                    interpreter.enter(|vm| {
                        let method_py_object_ref = unwrap_rust_python_result(scope.globals.get_item(#function_name, vm), vm);

                        let result_py_object_ref = vm.invoke(&method_py_object_ref, #params);

                        match result_py_object_ref {
                            Ok(py_object_ref) => py_object_ref.try_from_vm_value(vm).unwrap(),
                            Err(err) => {
                                let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                                panic!("{}", err_string);
                            }
                        }
                    });
                })
            }
            _ => Err(crate::errors::unreachable()),
        }
    }
}
