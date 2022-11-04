use quote::{format_ident, quote};
use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::py_ast::kybra_types::KybraExpr;
use cdk_framework::{
    nodes::{data_type_nodes::ActPrimitiveLit, ActFnParam},
    ActDataType, CanisterMethod, CanisterMethodType, ToActDataType,
};

use super::KybraStmt;

impl KybraStmt<'_> {
    pub fn is_canister_method_stmt(&self) -> bool {
        self.is_canister_method_type(CanisterMethodType::Update)
            || self.is_canister_method_type(CanisterMethodType::Query)
    }

    pub fn is_canister_method_type(&self, canister_method_type: CanisterMethodType) -> bool {
        self.is_decorator_same_as(match canister_method_type {
            CanisterMethodType::Heartbeat => "heartbeat",
            CanisterMethodType::Init => "init",
            CanisterMethodType::InspectMessage => "inspect_message",
            CanisterMethodType::PostUpgrade => "post_upgrade",
            CanisterMethodType::PreUpgrade => "pre_upgrade",
            CanisterMethodType::Query => "query",
            CanisterMethodType::Update => "update",
        })
    }

    fn is_decorator_same_as(&self, decorator_name: &str) -> bool {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { decorator_list, .. } => {
                decorator_list
                    .iter()
                    .any(|expr_kind| match &expr_kind.node {
                        ExprKind::Name { id, .. } => id == decorator_name,
                        _ => false,
                    })
            }
            _ => false,
        }
    }

    fn build_ast_params(&self) -> Vec<ActFnParam> {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { args, .. } => {
                args.args
                    .iter()
                    .fold(vec![], |acc, arg| match &arg.node.annotation {
                        Some(annotation) => {
                            let name = arg.node.arg.clone();
                            let kybra_annotation = KybraExpr {
                                located_expr: &annotation,
                                source_map: &self.source_map,
                            };
                            let data_type = kybra_annotation.to_act_data_type(&None);
                            vec![acc, vec![ActFnParam { name, data_type }]].concat()
                        }
                        None => todo!("Param type needs type annotation"),
                    })
            }
            _ => todo!(),
        }
    }

    pub fn as_canister_method(&self) -> Option<CanisterMethod> {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef {
                name,
                args,
                returns,
                ..
            } => {
                let param_conversions = args.args.iter().map(|arg| {
                    let param_name = format_ident!("{}", arg.node.arg);
                    quote! {
                        #param_name.try_into_vm_value(vm).unwrap()
                    }
                });

                let params_comma = if param_conversions.len() == 1 {
                    quote!(,)
                } else {
                    quote!()
                };
                let body = quote! {
                    unsafe {
                        let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                        let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                        let vm = &_kybra_interpreter.vm;

                        let method_py_object_ref = _kybra_scope.globals.get_item(#name, vm).unwrap();

                        let invoke_result = vm.invoke(&method_py_object_ref, (#(#param_conversions),*#params_comma));

                        match invoke_result {
                            Ok(py_object_ref) => _kybra_async_result_handler(vm, &py_object_ref, vm.ctx.none()).await.try_from_vm_value(vm).unwrap(),
                            Err(err) => {
                                let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                                panic!("{}", err_string);
                            }
                        }
                    }
                };

                let params: Vec<ActFnParam> = self.build_ast_params();
                let return_type: ActDataType = match returns {
                    Some(return_type) => {
                        let kybra_return_type = KybraExpr {
                            located_expr: &return_type,
                            source_map: &self.source_map,
                        };
                        kybra_return_type.to_act_data_type(&None)
                    }
                    None => ActPrimitiveLit::Void.to_act_data_type(&None),
                };
                Some(CanisterMethod {
                    body,
                    params,
                    is_manual: false,
                    name: name.clone(),
                    return_type,
                })
            }
            _ => None,
        }
    }
}
