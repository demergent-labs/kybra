pub mod not_exactly_one_target;

use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Unreachable},
    source_map::SourceMapped,
    Error,
};

pub use not_exactly_one_target::NotExactlyOneTarget;

impl SourceMapped<&Located<ExprKind>> {
    pub fn invalid_subscriptable_error(&self) -> Error {
        let title =
            "Only Async, Vec, Manual, Opt, or Tuple are allowed subscriptables for candid values";
        let annotation = "Invalid subscriptable here";
        Error::InvalidSubscriptable(self.create_error_message(title, annotation, None))
    }

    pub fn none_cant_be_a_type_error(&self) -> Error {
        Error::NoneCantBeAType(self.create_error_message("None must not be used as a type, but only as a value. Please specify either kybra.null or kybra.void.", "Ambiguous None here", None))
    }

    pub fn unsupported_type_error(&self) -> Error {
        let expression_name = match &self.node {
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
        let title = format!("{} is not allowed here.", expression_name);
        let annotation = "Illegal expression used here";
        Error::UnsupportedType(self.create_error_message(&title, annotation, None))
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn invalid_class_error(&self) -> Error {
        let title = "For a class to be included in your canister definition it must be either a Record or a Variant.";
        let annotation = "illegal class here";
        Error::InvalidClass(self.create_error_message(title, annotation, None))
    }

    pub fn invalid_assign_error(&self) -> Error {
        let title = "For a global assignment to be included in your canister definition it must be be either a Tuple or a Type Alias";
        let annotation = "illegal assignment here";
        Error::InvalidAssign(self.create_error_message(title, annotation, None))
    }

    pub fn invalid_annotation_assign_error(&self) -> Error {
        let title = "For a global annotation assignment to be included in your canister definition it must be be either a Func or a Type Alias";
        let annotation = "illegal annotation assignment here";
        Error::InvalidAnnAssign(self.create_error_message(title, annotation, None))
    }

    pub fn unsupported_type_error(&self) -> Error {
        let stmt_kind_name = match &self.node {
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
        let title = format!(
            "{} are not allowed here. They cannot be represented as a candid type.",
            stmt_kind_name
        );
        let annotation = "Illegal expression used here";
        Error::UnsupportedType(self.create_error_message(&title, annotation, None))
    }
}
