use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location, Unreachable},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct UnsupportedType {
    pub message: String,
    pub location: Location,
}

impl UnsupportedType {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        let stmt_kind_name = match &stmt_kind.node {
            StmtKind::FunctionDef { .. } => "Function Def",
            StmtKind::AsyncFunctionDef { .. } => "Async Function Def",
            StmtKind::Return { .. } => "Return",
            StmtKind::Delete { .. } => "Delete",
            StmtKind::AugAssign { .. } => "AugAssign",
            StmtKind::For { .. } => "For",
            StmtKind::AsyncFor { .. } => "Async For",
            StmtKind::While { .. } => "While",
            StmtKind::If { .. } => "If",
            StmtKind::With { .. } => "With",
            StmtKind::AsyncWith { .. } => "Async With",
            StmtKind::Match { .. } => "Match",
            StmtKind::Raise { .. } => "Raise",
            StmtKind::Try { .. } => "Try",
            StmtKind::Assert { .. } => "Assert",
            StmtKind::Import { .. } => "Import",
            StmtKind::ImportFrom { .. } => "Import From",
            StmtKind::Global { .. } => "Global",
            StmtKind::Nonlocal { .. } => "Nonlocal",
            StmtKind::Expr { .. } => "Expression",
            StmtKind::Pass => "Pass",
            StmtKind::Break => "Break",
            StmtKind::Continue => "Continue",
            _ => return Unreachable::new_err(),
        };
        let message = format!(
            "{} are not allowed here. They cannot be represented as a candid type.",
            stmt_kind_name
        );
        Self {
            message,
            location: stmt_kind.create_location(),
        }
        .into()
    }

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
        };
        let message = format!("{} is not allowed here.", expression_name);
        Self {
            location: expr_kind.create_location(),
            message,
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
        let annotation = "Illegal expression used here".to_string();
        let suggestion = None;

        write!(
            f,
            "{}",
            CompilerOutput {
                title: self.message.clone(),
                location: self.location.clone(),
                annotation,
                suggestion,
            }
            .to_string(AnnotationType::Error),
        )
    }
}
