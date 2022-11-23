use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::py_ast::kybra_types::{KybraExpr, KybraStmt};
use cdk_framework::{
    nodes::data_type_nodes::{
        act_funcs::{Func, FuncTypeAlias},
        ActFunc, LiteralOrTypeAlias, ToIdent,
    },
    ActDataType, ToActDataType,
};

mod errors;

impl KybraStmt<'_> {
    pub fn as_func(&self) -> ActDataType {
        match &self.stmt_kind.node {
            StmtKind::AnnAssign { target, value, .. } => match &value {
                Some(value) => match &value.node {
                    ExprKind::Call { args, .. } => {
                        let name = match &target.node {
                            ExprKind::Name { id, .. } => id.clone(),
                            _ => todo!(),
                        };
                        if args.len() != 1 {
                            todo!()
                        }
                        let mode = match &args[0].node {
                            ExprKind::Subscript { value, .. } => match &value.node {
                                ExprKind::Name { id, .. } => {
                                    if !["Oneway", "Update", "Query"].contains(&id.as_str()) {
                                        todo!()
                                    }
                                    id
                                }
                                _ => todo!(),
                            },
                            _ => todo!(),
                        }
                        .to_string();
                        let params = match &args[0].node {
                            ExprKind::Subscript { slice, .. } => match &slice.node {
                                ExprKind::Tuple { elts, .. } => {
                                    if elts.len() != 2 {
                                        todo!()
                                    }
                                    match &elts[0].node {
                                        ExprKind::List { elts, .. } => elts
                                            .iter()
                                            .map(|elt| {
                                                KybraExpr {
                                                    located_expr: elt,
                                                    source_map: self.source_map,
                                                }
                                                .to_act_data_type(&None)
                                            })
                                            .collect(),
                                        _ => todo!(),
                                    }
                                }
                                _ => todo!(),
                            },
                            _ => todo!(),
                        };
                        let return_type = Box::from(if mode == "Oneway" {
                            None
                        } else {
                            match &args[0].node {
                                ExprKind::Subscript { slice, .. } => match &slice.node {
                                    ExprKind::Tuple { elts, .. } => {
                                        if elts.len() != 2 {
                                            todo!()
                                        }
                                        Some(
                                            KybraExpr {
                                                located_expr: &elts[1],
                                                source_map: self.source_map,
                                            }
                                            .to_act_data_type(&None),
                                        )
                                    }
                                    _ => todo!(),
                                },
                                _ => todo!(),
                            }
                        });
                        ActDataType::Func(ActFunc {
                            act_type: LiteralOrTypeAlias::TypeAlias(FuncTypeAlias {
                                func: Func {
                                    to_vm_value: generate_func_to_vm_value(&name),
                                    list_to_vm_value: generate_func_list_to_vm_value(&name),
                                    from_vm_value: generate_func_from_vm_value(&name),
                                    list_from_vm_value: generate_func_list_from_vm_value(&name),
                                    name,
                                    params,
                                    return_type,
                                    mode,
                                },
                            }),
                        })
                    }
                    _ => todo!(),
                },
                None => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn is_func(&self) -> bool {
        match &self.stmt_kind.node {
            StmtKind::AnnAssign {
                annotation, value, ..
            } => {
                let is_type_alias = match &annotation.node {
                    ExprKind::Name { id, .. } => id == "TypeAlias",
                    _ => false,
                };
                let is_func = match &value {
                    Some(value) => match &value.node {
                        ExprKind::Call { func, .. } => match &func.node {
                            ExprKind::Name { id, .. } => id == "Func",
                            _ => false,
                        },
                        _ => false,
                    },
                    None => false,
                };
                is_type_alias && is_func
            }
            _ => false,
        }
    }

    pub fn get_func_args(&self) -> Option<&Vec<Located<ExprKind>>> {
        if !self.is_func() {
            return None;
        }

        match &self.stmt_kind.node {
            StmtKind::AnnAssign { value, .. } => match &value {
                Some(value) => match &value.node {
                    ExprKind::Call { args, .. } => Some(args),
                    _ => None,
                },
                None => None,
            },
            _ => None,
        }
    }
}

fn generate_func_to_vm_value(name: &String) -> TokenStream {
    let type_alias_name = name.to_identifier().to_token_stream();
    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for #type_alias_name {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                self.0.try_into_vm_value(vm)
            }
        }
    }
}

fn generate_func_list_to_vm_value(name: &String) -> TokenStream {
    let type_alias_name = name.to_identifier().to_token_stream();
    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for Vec<#type_alias_name> {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                try_into_vm_value_generic_array(self, vm)
            }
        }
    }
}

fn generate_func_from_vm_value(name: &String) -> TokenStream {
    let type_alias_name = name.to_identifier().to_token_stream();
    quote! {
        impl CdkActTryFromVmValue<#type_alias_name, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<#type_alias_name, CdkActTryFromVmValueError> {
                let candid_func: candid::Func = self.try_from_vm_value(vm).unwrap();
                Ok(candid_func.into())
            }
        }
    }
}

fn generate_func_list_from_vm_value(name: &String) -> TokenStream {
    let type_alias_name = name.to_identifier().to_token_stream();
    quote! {
        impl CdkActTryFromVmValue<Vec<#type_alias_name>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<#type_alias_name>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }
    }
}
