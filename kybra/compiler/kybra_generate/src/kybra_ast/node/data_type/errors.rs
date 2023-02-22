use rustpython_parser::ast::{ExprKind, Located};

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
