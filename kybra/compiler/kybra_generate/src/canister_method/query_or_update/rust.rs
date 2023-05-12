use cdk_framework::{
    act::node::{candid::Primitive, CandidType},
    traits::CollectResults,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{method_utils::params::InternalOrExternal, source_map::SourceMapped, tuple, Error};

pub fn generate_body(
    source_mapped_located_stmtkind: &SourceMapped<&Located<StmtKind>>,
) -> Result<TokenStream, Vec<Error>> {
    let (params, return_expression) = (
        source_mapped_located_stmtkind.build_params(InternalOrExternal::Internal),
        generate_return_expression(source_mapped_located_stmtkind),
    )
        .collect_results()?;

    let name = source_mapped_located_stmtkind.get_name_or_err()?;

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
        unsafe {
            let interpreter = INTERPRETER_OPTION.as_mut().unwrap();
            let scope = SCOPE_OPTION.as_mut().unwrap();

            let vm = &interpreter.vm;

            let method_py_object_ref = unwrap_rust_python_result(scope.globals.get_item(#name, vm), vm);

            let invoke_result = vm.invoke(&method_py_object_ref, #params);

            match invoke_result {
                Ok(py_object_ref) => {
                    let final_return_value = async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;

                    #return_expression
                },
                Err(err) => {
                    let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                    panic!("{}", err_string);
                }
            }
        }
    })
}

fn generate_return_expression(
    kybra_statement: &SourceMapped<&Located<StmtKind>>,
) -> Result<TokenStream, Vec<Error>> {
    if kybra_statement.is_manual() {
        return Ok(quote! {
            ic_cdk::api::call::ManualReply::empty()
        });
    }

    if type_is_null_or_void(kybra_statement.build_return_type()?) {
        return Ok(quote! {
            return;
        });
    }

    Ok(quote! {
        final_return_value.try_from_vm_value(vm).unwrap()
    })
}

fn type_is_null_or_void(candid_type: CandidType) -> bool {
    match candid_type {
        CandidType::Primitive(primitive) => match primitive {
            Primitive::Null => true,
            Primitive::Void => true,
            _ => false,
        },
        _ => false,
    }
}
