use crate::cdk_act_old::{
    act::AbstractCanisterTree,
    act_node::{ActNode, PrimitiveInfo},
    nodes::canister_method::{CanisterMethod, CanisterMethodActNode},
};
use quote::{format_ident, quote};
use rustpython_parser::{ast, mode};

pub fn generate_act(python_source: &str) -> AbstractCanisterTree {
    let py_mod = rustpython_parser::parser::parse(python_source, mode::Mode::Module, "").unwrap();

    let randomness_implementation = AbstractCanisterTree::generate_randomness_implementation();
    let candid_file_generation = AbstractCanisterTree::generate_candid_file_generation("main.did");
    // TODO why do we split up query and update??
    let canister_method_act_nodes = get_canister_method_act_nodes(&py_mod);

    AbstractCanisterTree {
        randomness_implementation,
        candid_file_generation,
        query_methods: canister_method_act_nodes,
        update_methods: vec![],
    }
}

fn get_canister_method_act_nodes(py_mod: &ast::Mod) -> Vec<CanisterMethodActNode> {
    match py_mod {
        ast::Mod::Module { body, type_ignores } => body
            .iter()
            .filter(|located_stmt| match &located_stmt.node {
                ast::StmtKind::FunctionDef {
                    name,
                    args,
                    body,
                    decorator_list,
                    returns,
                    type_comment,
                } => {
                    let is_query = is_stmt_kind_function_def_a_query(&located_stmt.node);
                    let is_update = is_stmt_kind_function_def_an_update(&located_stmt.node);

                    is_query || is_update
                }
                _ => false,
            })
            .map(|stmt| match &stmt.node {
                ast::StmtKind::FunctionDef {
                    name,
                    args,
                    body,
                    decorator_list,
                    returns,
                    type_comment,
                } => {
                    let param_conversions = args.args.iter().map(|arg| {
                        let param_name = format_ident!("{}", arg.node.arg);
                        quote! {
                            #param_name.try_into_vm_value(vm).unwrap()
                        }
                    });

                    let params_comma = if param_conversions.len() == 1 { quote!(,) } else { quote!() };

                    let canister_method = CanisterMethod {
                        body: quote! {
                            unsafe {
                                let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                                let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                                let result = _kybra_interpreter.enter(|vm| {
                                    let method_py_object_ref = _kybra_scope.globals.get_item(#name, vm).unwrap();

                                    let result_py_object_ref = vm.invoke(&method_py_object_ref, (#(#param_conversions),*#params_comma));

                                    match result_py_object_ref {
                                        Ok(py_object_ref) => py_object_ref.try_from_vm_value(vm).unwrap(),
                                        Err(err) => {
                                            let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                                            panic!(err_string);
                                        }
                                    }
                                });

                                result
                            }
                        },
                        param_names: args.args.iter().map(|arg| {
                            format_ident!("{}", arg.node.arg)
                        }).collect(),
                        param_types: args.args.iter().map(|arg| {
                            expr_kind_to_act_node(&temp(&arg.node.annotation))
                        }).collect(),
                        inline_types: Box::new(vec![]),
                        is_manual: false,
                        name: name.clone(),
                        return_type: expr_kind_to_act_node(&temp(returns)),
                    };

                    let is_query = is_stmt_kind_function_def_a_query(&stmt.node);

                    if is_query {
                        CanisterMethodActNode::QueryMethod(canister_method)
                    }
                    else {
                        CanisterMethodActNode::UpdateMethod(canister_method)
                    }
                }
                _ => panic!(""),
            })
            .collect(),
        _ => vec![],
    }
}

fn temp(monkey: &Option<Box<ast::Located<ast::ExprKind>>>) -> Option<&ast::ExprKind> {
    if let Some(hello) = monkey {
        let node = &hello.node;

        Some(node.clone())
    } else {
        None
    }
}

// TODO combine these two functions
// TODO these should probably be an enum
fn is_stmt_kind_function_def_a_query(stmt_kind: &ast::StmtKind) -> bool {
    match stmt_kind {
        ast::StmtKind::FunctionDef { decorator_list, .. } => {
            decorator_list
                .iter()
                .any(|expr_kind| match &expr_kind.node {
                    ast::ExprKind::Name { id, .. } => match &id[..] {
                        "query" => true,
                        _ => false,
                    },
                    _ => false,
                })
        }
        _ => false,
    }
}

fn is_stmt_kind_function_def_an_update(stmt_kind: &ast::StmtKind) -> bool {
    match stmt_kind {
        ast::StmtKind::FunctionDef { decorator_list, .. } => {
            decorator_list
                .iter()
                .any(|expr_kind| match &expr_kind.node {
                    ast::ExprKind::Name { id, .. } => match &id[..] {
                        "update" => true,
                        _ => false,
                    },
                    _ => false,
                })
        }
        _ => false,
    }
}

fn expr_kind_to_act_node(expr_kind_option: &Option<&ast::ExprKind>) -> ActNode {
    if let Some(expr_kind) = expr_kind_option {
        match expr_kind {
            ast::ExprKind::Name { id, .. } => match &id[..] {
                "blob" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(Vec<u8>),
                }),
                "empty" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(ic_cdk::export::candid::Empty),
                }),
                "float64" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(f64),
                }),
                "float32" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(f32),
                }),
                "int" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(i128),
                }),
                "int64" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(i64),
                }),
                "int32" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(i32),
                }),
                "int16" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(i16),
                }),
                "int8" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(i8),
                }),
                "nat" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(u128),
                }),
                "nat64" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(u64),
                }),
                "nat32" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(u32),
                }),
                "nat16" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(u16),
                }),
                "nat8" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(u8),
                }),
                "null" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!((())),
                }),
                "Principal" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(ic_cdk::export::Principal),
                }),
                "bool" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(bool),
                }),
                "reserved" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(ic_cdk::export::candid::Reserved),
                }),
                "str" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(String),
                }),
                "text" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(String),
                }),
                _ => panic!(""),
            },
            _ => panic!(""),
        }
    } else {
        // TODO figure things out with this type here, we should be able to represent an empty ActNode, or maybe have an option in the canister method return type
        ActNode::Primitive(PrimitiveInfo {
            identifier: quote!(),
        })
    }
}
