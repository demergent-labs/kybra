use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use rustpython_parser::ast::ExprKind;

use crate::py_ast::{
    kybra_types::{KybraExpr, KybraStmt},
    traits::GetDependencies,
};

impl GetDependencies for KybraExpr<'_> {
    fn get_dependent_types(
        &self,
        type_alias_lookup: &HashMap<String, KybraStmt>,
        found_type_names: &HashSet<String>,
    ) -> HashSet<String> {
        match &self.located_expr.node {
            ExprKind::Subscript { slice, .. } => KybraExpr {
                located_expr: slice,
                source_map: self.source_map,
            }
            .get_dependent_types(type_alias_lookup, found_type_names),
            ExprKind::Tuple { elts, .. } => elts.iter().fold(HashSet::new(), |acc, elt| {
                let dependencies = KybraExpr {
                    located_expr: elt,
                    source_map: self.source_map,
                }
                .get_dependent_types(type_alias_lookup, found_type_names);
                acc.union(&dependencies).cloned().collect()
            }),
            ExprKind::Constant { .. } => HashSet::new(),
            ExprKind::Name { id, .. } => match &id[..] {
                "blob" => HashSet::new(),
                "empty" => HashSet::new(),
                "float64" => HashSet::new(),
                "float32" => HashSet::new(),
                "int" => HashSet::new(),
                "int64" => HashSet::new(),
                "int32" => HashSet::new(),
                "int16" => HashSet::new(),
                "int8" => HashSet::new(),
                "nat" => HashSet::new(),
                "nat64" => HashSet::new(),
                "nat32" => HashSet::new(),
                "nat16" => HashSet::new(),
                "nat8" => HashSet::new(),
                "null" => HashSet::new(),
                "Principal" => HashSet::new(),
                "bool" => HashSet::new(),
                "reserved" => HashSet::new(),
                "str" => HashSet::new(),
                "text" => HashSet::new(),
                _ => {
                    let name = id.to_string();
                    if found_type_names.contains(&name) {
                        return HashSet::new();
                    }
                    match type_alias_lookup.clone().get(&name) {
                        Some(decl) => {
                            let new_type: HashSet<String> =
                                HashSet::from_iter(vec![name].iter().cloned());
                            let found_type_names: HashSet<String> =
                                found_type_names.clone().union(&new_type).cloned().collect();
                            // When finding a new type return it and all of it's dependents
                            found_type_names
                                .union(
                                    &decl.get_dependent_types(type_alias_lookup, &found_type_names),
                                )
                                .cloned()
                                .collect()
                        }
                        None => HashSet::new(),
                    }
                }
            },
            _ => HashSet::new(),
        }
    }
}
