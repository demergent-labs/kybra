mod array;
mod errors;
mod generate_inline_name;
mod generator;
mod hash;
mod opt;
mod to_act_data_type;
mod tuple;

use rustpython_parser::ast::{ExprKind, Located};

use crate::source_map::SourceMap;

pub struct KybraExpr<'a> {
    pub located_expr: &'a Located<ExprKind>,
    pub source_map: &'a SourceMap,
}
