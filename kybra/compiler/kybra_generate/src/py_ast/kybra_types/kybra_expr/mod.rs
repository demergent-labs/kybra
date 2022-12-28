mod array;
mod errors;
mod generate_inline_name;
mod generator;
pub mod get_dependencies;
mod hash;
mod opt;
mod to_act_data_type;
mod tuple;

use rustpython_parser::ast::{ExprKind, Located};

use crate::source_map::SourceMap;

pub struct KybraExpr<'a> {
    pub located_expr: &'a Located<ExprKind>,
    pub source_map: SourceMap,
}

impl KybraExpr<'_> {
    pub fn get_name(&self) -> Option<String> {
        match &self.located_expr.node {
            ExprKind::Name { id, .. } => Some(id.clone()),
            _ => None,
        }
    }
}

impl KybraExpr<'_> {
    pub fn is_manual(&self) -> bool {
        match &self.located_expr.node {
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => {
                    if id == "manual" {
                        return true;
                    } else {
                        return KybraExpr {
                            located_expr: slice,
                            source_map: self.source_map.clone(),
                        }
                        .is_manual();
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }
}
