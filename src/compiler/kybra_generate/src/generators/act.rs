use crate::cdk_act::{
    act::AbstractCanisterTree,
    act_node::{ActNode, PrimitiveInfo},
    nodes::canister_method::{CanisterMethod, CanisterMethodActNode},
};
use quote::quote;
use rustpython_parser::{ast, mode};

pub fn generate_act(python_source: &str) -> AbstractCanisterTree {
    let py_mod = rustpython_parser::parser::parse(python_source, mode::Mode::Module, "").unwrap();

    // TODO why do we split up query and update??
    let canister_method_act_nodes = get_canister_method_act_nodes(&py_mod);

    AbstractCanisterTree {
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
                } => true,
                _ => false,
            })
            .map(|stmt| {
                eprint!("{:#?}", &stmt.node);

                match &stmt.node {
                    ast::StmtKind::FunctionDef {
                        name,
                        args,
                        body,
                        decorator_list,
                        returns,
                        type_comment,
                    } => CanisterMethodActNode::QueryMethod(CanisterMethod {
                        body: quote! {
                            unsafe {
                                let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                                let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                                let result = _kybra_interpreter.enter(|vm| {
                                    let method_py_object_ref = _kybra_scope.globals.get_item(#name, vm).unwrap();

                                    // let result_py_object_ref = vm.invoke(&hello_world_py_object_ref, (x.try_into_vm_value(vm).unwrap(), y.try_into_vm_value(vm).unwrap())).unwrap();
                                    let result_py_object_ref = vm.invoke(&method_py_object_ref, ()).unwrap();

                                    result_py_object_ref.try_from_vm_value(vm).unwrap()
                                });

                                result
                            }
                        },
                        param_names: vec![],
                        param_types: vec![],
                        inline_types: Box::new(vec![]),
                        is_manual: false,
                        name: name.clone(),
                        return_type: expr_kind_to_act_node(&temp(returns)),
                    }),
                    _ => panic!(""),
                }
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
fn is_stmt_kind_function_def_a_query(stmt_kind: &ast::StmtKind) -> bool {
    match stmt_kind {
        ast::StmtKind::FunctionDef { decorator_list, .. } => {
            // TODO find a query decorator
            true
        }
        _ => false,
    }
}

fn is_stmt_kind_function_def_an_update(stmt_kind: &ast::StmtKind) -> bool {
    match stmt_kind {
        ast::StmtKind::FunctionDef { decorator_list, .. } => {
            // TODO find an update decorator
            true
        }
        _ => false,
    }
}

fn expr_kind_to_act_node(expr_kind_option: &Option<&ast::ExprKind>) -> ActNode {
    if let Some(expr_kind) = expr_kind_option {
        match expr_kind {
            ast::ExprKind::Name { id, .. } => match &id[..] {
                "bool" => ActNode::Primitive(PrimitiveInfo {
                    identifier: quote!(bool),
                }),
                "str" => ActNode::Primitive(PrimitiveInfo {
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
