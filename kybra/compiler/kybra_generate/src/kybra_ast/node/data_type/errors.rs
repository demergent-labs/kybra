use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<ExprKind>> {
    pub fn invalid_subscript_value_error(&self) -> Message {
        let title =
            "Only Async, list, manual, opt, or tuple are allowed subscripts for candid values";
        let annotation = "Invalid subscript here";
        self.create_error_message(title, annotation, None)
    }

    pub fn none_cant_be_a_type_error(&self) -> Message {
        self.create_error_message("None must not be used as a type, but only as a value. Please specify either kybra.null or kybra.void.", "Ambiguous None here", None)
    }

    pub fn unsupported_type_error(&self) -> Message {
        let expression_name = match &self.node.node {
            ExprKind::BoolOp { .. } => "boolean operators",
            ExprKind::NamedExpr { .. } => "named expressions",
            ExprKind::BinOp { .. } => "binary operators",
            ExprKind::UnaryOp { .. } => "unary operators",
            ExprKind::Lambda { .. } => "lambdas",
            ExprKind::IfExp { .. } => "if expressions",
            ExprKind::Dict { .. } => "dictionaries",
            ExprKind::Set { .. } => "sets",
            ExprKind::ListComp { .. } => "list comprehensions",
            ExprKind::SetComp { .. } => "set comprehensions",
            ExprKind::DictComp { .. } => "dict comprehensions",
            ExprKind::GeneratorExp { .. } => "generator expressions",
            ExprKind::Await { .. } => "await expressions",
            ExprKind::Yield { .. } => "yield expressions",
            ExprKind::YieldFrom { .. } => "yield from expressions",
            ExprKind::Compare { .. } => "compare expressions",
            ExprKind::Call { .. } => "call expressions",
            ExprKind::FormattedValue { .. } => "formatted values expressions",
            ExprKind::JoinedStr { .. } => "joined string expressions",
            ExprKind::Attribute { .. } => "attribute expressions",
            ExprKind::Starred { .. } => "starred expressions",
            ExprKind::List { .. } => "list expressions",
            ExprKind::Tuple { .. } => "tuple expressions",
            ExprKind::Slice { .. } => "slice expressions",
            _ => panic!("Unreachable: This type should be supported. I don' know how we got here."),
        };
        let title = format!("{} are not allowed here.", expression_name);
        let annotation = "Illegal expression used here";
        self.create_error_message(&title, annotation, None)
    }
}

impl SourceMapped<&Located<StmtKind>> {
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
        let stmt_kind_name = match &self.node.node {
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
