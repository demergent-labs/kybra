use rustpython_parser::ast::StmtKind;

use crate::{
    errors::{CreateMessage, Message},
    py_ast::kybra_types::KybraStmt,
};

impl KybraStmt<'_> {
    pub fn invalid_class_error(&self) -> Message {
        let title = "For a class to be included in your canister definition it must be either a Record or a Variant.";
        let annotation = "illegal class here";
        self.create_error_message(title, annotation, None)
    }

    pub fn invalid_assign_error(&self) -> Message {
        let title = "For a global assignment to be included in your canister definition it must be be either a Tuple or a Type Alias";
        let annotation = "illegal assignment here";
        self.create_error_message(title, annotation, None)
    }

    pub fn invalid_annotation_assign_error(&self) -> Message {
        let title = "For a global annotation assignment to be included in your canister definition it must be be either a Func or a Type Alias";
        let annotation = "illegal annotation assignment here";
        self.create_error_message(title, annotation, None)
    }

    pub fn unsupported_type_error(&self) -> Message {
        let stmt_kind_name = match &self.stmt_kind.node {
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
            _ => {
                panic!("Unreachable: This type should be supported. I don't know how we got here.")
            }
        };
        let title = format!(
            "{} are not allowed here. They cannot be represented as a candid type.",
            stmt_kind_name
        );
        let annotation = "Illegal expression used here";
        self.create_error_message(&title, annotation, None)
    }
}
