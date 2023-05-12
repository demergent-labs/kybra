use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct UnsupportedType {
    pub expression_name: String,
    pub location: Location,
}

impl UnsupportedType {
    pub fn err_from_expr(expr_kind: &SourceMapped<&Located<ExprKind>>) -> Error {
        let expression_name = match &expr_kind.node {
            ExprKind::BoolOp { .. } => "boolean operator",
            ExprKind::NamedExpr { .. } => "named expression",
            ExprKind::BinOp { .. } => "binary operator",
            ExprKind::UnaryOp { .. } => "unary operator",
            ExprKind::Lambda { .. } => "lambda",
            ExprKind::IfExp { .. } => "if expression",
            ExprKind::Dict { .. } => "dictionary",
            ExprKind::Set { .. } => "set",
            ExprKind::ListComp { .. } => "list comprehension",
            ExprKind::SetComp { .. } => "set comprehension",
            ExprKind::DictComp { .. } => "dict comprehension",
            ExprKind::GeneratorExp { .. } => "generator expression",
            ExprKind::Await { .. } => "await expression",
            ExprKind::Yield { .. } => "yield expression",
            ExprKind::YieldFrom { .. } => "yield from expression",
            ExprKind::Compare { .. } => "compare expression",
            ExprKind::Call { .. } => "call expression",
            ExprKind::FormattedValue { .. } => "formatted values expression",
            ExprKind::JoinedStr { .. } => "joined string expression",
            ExprKind::Attribute { .. } => "attribute expression",
            ExprKind::Starred { .. } => "starred expression",
            ExprKind::List { .. } => "list expression",
            ExprKind::Tuple { .. } => "tuple expression",
            ExprKind::Slice { .. } => "slice expression",
            _ => "This type",
        }
        .to_string();
        Self {
            location: expr_kind.create_location(),
            expression_name,
        }
        .into()
    }
}

impl From<UnsupportedType> for Error {
    fn from(value: UnsupportedType) -> Self {
        Self::UnsupportedType(value)
    }
}

impl Display for UnsupportedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!("{} is not allowed here.", self.expression_name);
        let annotation = "Illegal expression used here".to_string();
        let suggestion = None;

        write!(
            f,
            "{}",
            CompilerOutput {
                title,
                location: self.location.clone(),
                annotation,
                suggestion,
            }
            .to_string(AnnotationType::Error),
        )
    }
}
