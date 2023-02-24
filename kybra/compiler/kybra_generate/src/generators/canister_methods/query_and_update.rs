use cdk_framework::act::node::{data_type::Primitive, DataType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{generators::tuple, source_map::SourceMapped};

pub fn generate_body(kybra_statement: &SourceMapped<&Located<StmtKind>>) -> TokenStream {
    let act_params = kybra_statement.build_params();

    let name = match kybra_statement.get_name() {
        Some(name) => name,
        None => todo!(),
    };

    let param_conversions = act_params
        .iter()
        .map(|param| {
            let name = format_ident!("{}", param.prefixed_name());
            quote! {
                #name.try_into_vm_value(vm).unwrap()
            }
        })
        .collect();

    let params = tuple::generate_tuple(&param_conversions);

    let return_expression = generate_return_expression(kybra_statement);

    quote! {
        unsafe {
            let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
            let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

            let vm = &_kybra_interpreter.vm;

            let method_py_object_ref = _kybra_unwrap_rust_python_result(_kybra_scope.globals.get_item(#name, vm), vm);

            let invoke_result = vm.invoke(&method_py_object_ref, #params);

            match invoke_result {
                Ok(py_object_ref) => {
                    let _kybra_final_return_value = _kybra_async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;

                    #return_expression
                },
                Err(err) => {
                    let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                    panic!("{}", err_string);
                }
            }
        }
    }
}

fn generate_return_expression(kybra_statement: &SourceMapped<&Located<StmtKind>>) -> TokenStream {
    if kybra_statement.is_manual() {
        return quote! {
            ic_cdk::api::call::ManualReply::empty()
        };
    }

    let return_type = kybra_statement.build_return_type();
    if type_is_null_or_void(return_type) {
        return quote! {
            return;
        };
    }

    quote! {
        _kybra_final_return_value.try_from_vm_value(vm).unwrap()
    }
}

fn type_is_null_or_void(act_type: DataType) -> bool {
    match act_type {
        DataType::Primitive(primitive) => match primitive {
            Primitive::Null => true,
            Primitive::Void => true,
            _ => false,
        },
        _ => false,
    }
}
