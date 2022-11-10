use std::collections::{HashMap, HashSet};

use rustpython_parser::ast::StmtKind;

use crate::py_ast::{kybra_types::KybraExpr, traits::GetDependencies};

use super::KybraStmt;

impl GetDependencies for KybraStmt<'_> {
    fn get_dependent_types(
        &self,
        type_alias_lookup: &HashMap<String, KybraStmt>,
        found_type_names: &HashSet<String>,
    ) -> HashSet<String> {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { args, returns, .. } => {
                let arg_dependencies =
                    args.args.iter().fold(found_type_names.clone(), |acc, arg| {
                        match &arg.node.annotation {
                            Some(annotation) => {
                                let dependencies = KybraExpr {
                                    located_expr: &annotation,
                                    source_map: self.source_map,
                                }
                                .get_dependent_types(type_alias_lookup, found_type_names);
                                acc.union(&dependencies).cloned().collect()
                            }
                            None => acc,
                        }
                    });
                let return_dependencies = match returns {
                    Some(returns) => KybraExpr {
                        located_expr: returns,
                        source_map: self.source_map,
                    }
                    .get_dependent_types(type_alias_lookup, found_type_names),
                    None => HashSet::new(),
                };
                arg_dependencies
                    .union(&return_dependencies)
                    .cloned()
                    .collect()
            }
            StmtKind::ClassDef { body, .. } => {
                body.iter().fold(found_type_names.clone(), |acc, member| {
                    let dependency = KybraStmt {
                        stmt_kind: member,
                        source_map: self.source_map,
                    }
                    .get_dependent_types(type_alias_lookup, found_type_names);
                    acc.union(&dependency).cloned().collect()
                })
            }
            StmtKind::Assign { value, .. } => KybraExpr {
                located_expr: value,
                source_map: self.source_map,
            }
            .get_dependent_types(type_alias_lookup, found_type_names),
            StmtKind::AnnAssign { annotation, .. } => {
                if self.is_func() {
                    match self.get_func_args() {
                        Some(args) => args.iter().fold(found_type_names.clone(), |acc, arg| {
                            let dependencies = KybraExpr {
                                located_expr: arg,
                                source_map: self.source_map,
                            }
                            .get_dependent_types(type_alias_lookup, found_type_names);
                            acc.union(&dependencies).cloned().collect()
                        }),
                        None => found_type_names.clone(),
                    }
                } else {
                    KybraExpr {
                        located_expr: annotation,
                        source_map: self.source_map,
                    }
                    .get_dependent_types(type_alias_lookup, found_type_names)
                }
            }
            _ => HashSet::new(),
        }
    }
}
