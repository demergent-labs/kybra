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
                #name.try_into_vm_value(vm).unwrap_or_trap()
            }
        })
        .collect();

    let params = tuple::generate_tuple(&param_conversions);

    Ok(quote! {
        let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
            .unwrap_or_trap("SystemError: missing python interpreter");
        let vm = &interpreter.vm;
        let params = #params;

        let final_return_value = call_global_python_function(#name, params)
            .await
            .unwrap_or_else(|err| ic_cdk::trap(err.as_str()));

        #return_expression
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
        final_return_value
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
