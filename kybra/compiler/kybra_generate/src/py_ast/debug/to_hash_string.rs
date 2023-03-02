use std::ops::Deref;

use rustpython_parser::ast::{
    ArgData, Arguments, Boolop, Cmpop, Comprehension, Constant, ExprContext, ExprKind, KeywordData,
    Located, Operator, Unaryop,
};

use crate::source_map::SourceMapped;

pub trait ToHashString {
    fn to_hashable_string(&self) -> String;
}

impl ToHashString for SourceMapped<Located<ExprKind>> {
    fn to_hashable_string(&self) -> String {
        format!(
            "SourceMapped: {} from {}",
            self.deref().to_hashable_string(),
            self.source_map.file_name
        )
    }
}

impl<T> ToHashString for Vec<T>
where
    T: ToHashString,
{
    fn to_hashable_string(&self) -> String {
        let vec_string = self.iter().fold("Vec: ".to_string(), |acc, hashable| {
            format!("{} {},", acc, hashable.to_hashable_string())
        });
        vec_string[..vec_string.len() - 1].to_string()
    }
}

impl<T> ToHashString for Located<T>
where
    T: ToHashString,
{
    fn to_hashable_string(&self) -> String {
        format!(
            "Located Node: {} located at {}:{}",
            self.node.to_hashable_string(),
            self.location.row(),
            self.location.column()
        )
    }
}

impl ToHashString for ExprKind {
    fn to_hashable_string(&self) -> String {
        match &self {
            rustpython_parser::ast::ExprKind::BoolOp { op, values } => {
                format!(
                    "BoolOp: op: {}, values: {}",
                    op.to_hashable_string(),
                    values.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::NamedExpr { target, value } => {
                format!(
                    "NamedExpr: target: {}, value: {}",
                    target.to_hashable_string(),
                    value.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::BinOp { left, op, right } => {
                format!(
                    "BinOp: left: {}, op: {}, right: {}",
                    left.to_hashable_string(),
                    op.to_hashable_string(),
                    right.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::UnaryOp { op, operand } => {
                format!(
                    "ExprKind: op: {}, operand: {}",
                    op.to_hashable_string(),
                    operand.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::Lambda { args, body } => {
                format!(
                    "Lambda: args: {}, body: {}",
                    args.to_hashable_string(),
                    body.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::IfExp { test, body, orelse } => {
                format!(
                    "IfExp: test: {}, body: {}, orelse: {}",
                    test.to_hashable_string(),
                    body.to_hashable_string(),
                    orelse.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::Dict { keys, values } => {
                format!(
                    "Dict: keys: {}, values: {}",
                    keys.to_hashable_string(),
                    values.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::Set { elts } => {
                format!("Set: elts: {}", elts.to_hashable_string())
            }
            rustpython_parser::ast::ExprKind::ListComp { elt, generators } => {
                format!(
                    "ListComp: elt: {}, generators: {}",
                    elt.to_hashable_string(),
                    generators.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::SetComp { elt, generators } => {
                format!(
                    "SetComp: elt: {}, generators: {}",
                    elt.to_hashable_string(),
                    generators.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::DictComp {
                key,
                value,
                generators,
            } => {
                format!(
                    "Dict Comp: key: {}, value: {}, generators: {}",
                    key.to_hashable_string(),
                    value.to_hashable_string(),
                    generators.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::GeneratorExp { elt, generators } => format!(
                "Generator Expression: elt: {}, generators: {}",
                elt.to_hashable_string(),
                generators.to_hashable_string()
            ),
            rustpython_parser::ast::ExprKind::Await { value } => {
                format!("Await: {}", value.to_hashable_string())
            }
            rustpython_parser::ast::ExprKind::Yield { value } => {
                format!(
                    "Yield: {}",
                    match value {
                        Some(yield_value) => yield_value.to_hashable_string(),
                        None => "None".to_string(),
                    }
                )
            }
            rustpython_parser::ast::ExprKind::YieldFrom { value } => {
                format!("Yield From: {}", value.to_hashable_string())
            }
            rustpython_parser::ast::ExprKind::Compare {
                left,
                ops,
                comparators,
            } => {
                format!(
                    "Compare: left: {}, ops: {}, comparators: {}",
                    left.to_hashable_string(),
                    ops.to_hashable_string(),
                    comparators.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::Call {
                func,
                args,
                keywords,
            } => {
                format!(
                    "Call: func: {}, args: {}, keywords: {}",
                    func.to_hashable_string(),
                    args.to_hashable_string(),
                    keywords.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::FormattedValue {
                value,
                conversion,
                format_spec,
            } => {
                format!(
                    "Formatted Value: value: {}, conversion: {}, format_spc: {}",
                    value.to_hashable_string(),
                    conversion.to_string(),
                    match format_spec {
                        Some(format_spec) => format_spec.to_hashable_string(),
                        None => "None".to_string(),
                    }
                )
            }
            rustpython_parser::ast::ExprKind::JoinedStr { values } => {
                format!("Joined Str: values: {}", values.to_hashable_string())
            }
            rustpython_parser::ast::ExprKind::Constant { value, kind } => format!(
                "Constant: value: {}, kind: {}",
                value.to_hashable_string(),
                match kind {
                    Some(kind) => kind,
                    None => "None",
                }
            ),
            rustpython_parser::ast::ExprKind::Attribute { value, attr, ctx } => format!(
                "Attribute: value: {}, attr: {}, ctx: {}",
                value.to_hashable_string(),
                attr,
                ctx.to_hashable_string()
            ),
            rustpython_parser::ast::ExprKind::Subscript { value, slice, ctx } => format!(
                "Subscript: value: {}, slice: {}, ctx: {}",
                value.to_hashable_string(),
                slice.to_hashable_string(),
                ctx.to_hashable_string()
            ),
            rustpython_parser::ast::ExprKind::Starred { value, ctx } => {
                format!(
                    "Starred: value: {}, ctx: {}",
                    value.to_hashable_string(),
                    ctx.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::Name { id, ctx } => {
                format!("Name: id: {}, ctx: {}", id, ctx.to_hashable_string())
            }
            rustpython_parser::ast::ExprKind::List { elts, ctx } => {
                format!(
                    "List: elts: {}, ctx: {}",
                    elts.to_hashable_string(),
                    ctx.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::Tuple { elts, ctx } => {
                format!(
                    "Tuple: elts: {}, ctx: {}",
                    elts.to_hashable_string(),
                    ctx.to_hashable_string()
                )
            }
            rustpython_parser::ast::ExprKind::Slice { lower, upper, step } => format!(
                "Slice: lower: {}, upper: {}, step: {}",
                match lower {
                    Some(lower) => lower.to_hashable_string(),
                    None => "None".to_string(),
                },
                match upper {
                    Some(upper) => upper.to_hashable_string(),
                    None => "None".to_string(),
                },
                match step {
                    Some(step) => step.to_hashable_string(),
                    None => "None".to_string(),
                }
            ),
        }
    }
}

impl ToHashString for ArgData {
    fn to_hashable_string(&self) -> String {
        format!(
            "ArgData: arg: {}, annotation: {}, type_comment: {}",
            self.arg.to_string(),
            match &self.annotation {
                Some(annotation) => annotation.to_hashable_string(),
                None => "None".to_string(),
            },
            match &self.type_comment {
                Some(type_comment) => type_comment,
                None => "None",
            }
        )
    }
}

impl ToHashString for Arguments {
    fn to_hashable_string(&self) -> String {
        format!("Arguments: posonlyargs: {}, args: {}, vararg: {}, kwonlyargs: {}, kw_defaults: {}, kwarg: {}, defaults: {}",
            self.posonlyargs.to_hashable_string(),
            self.args.to_hashable_string(),
            match &self.vararg {
                Some(vararg) => vararg.to_hashable_string(),
                None => "None".to_string(),
            },
            self.kwonlyargs.to_hashable_string(),
            self.kw_defaults.to_hashable_string(),
            match &self.kwarg {
                Some(kwarg) => kwarg.to_hashable_string(),
                None => "None".to_string(),
            },
            self.defaults.to_hashable_string()
        )
    }
}

impl ToHashString for Boolop {
    fn to_hashable_string(&self) -> String {
        format!(
            "Bool Operator: {}",
            match self {
                Boolop::And => "and",
                Boolop::Or => "or",
            }
        )
    }
}

impl ToHashString for Comprehension {
    fn to_hashable_string(&self) -> String {
        format!(
            "Comprehension: target: {}, iter: {}, ifs: {}, is_async: {}",
            self.target.to_hashable_string(),
            self.iter.to_hashable_string(),
            self.ifs.to_hashable_string(),
            self.is_async.to_string()
        )
    }
}

impl ToHashString for Cmpop {
    fn to_hashable_string(&self) -> String {
        format!(
            "Cmpop: {}",
            match self {
                Cmpop::Eq => "=",
                Cmpop::NotEq => "!=",
                Cmpop::Lt => "<",
                Cmpop::LtE => "<=",
                Cmpop::Gt => ">",
                Cmpop::GtE => ">=",
                Cmpop::Is => "is",
                Cmpop::IsNot => "is not",
                Cmpop::In => "in",
                Cmpop::NotIn => "not in",
            }
        )
    }
}

impl ToHashString for u8 {
    fn to_hashable_string(&self) -> String {
        self.to_string()
    }
}

impl ToHashString for Constant {
    fn to_hashable_string(&self) -> String {
        format!(
            "Constant: {}",
            match &self {
                Constant::None => "None".to_string(),
                Constant::Bool(boolean) => boolean.to_string(),
                Constant::Str(str) => str.to_string(),
                Constant::Bytes(bytes) => bytes.to_hashable_string(),
                Constant::Int(int) => int.to_string(),
                Constant::Tuple(tuple) => tuple.to_hashable_string(),
                Constant::Float(float) => float.to_string(),
                Constant::Complex { real, imag } =>
                    format!("{}j {}i", real.to_string(), imag.to_string()),
                Constant::Ellipsis => "...".to_string(),
            }
        )
    }
}

impl ToHashString for ExprContext {
    fn to_hashable_string(&self) -> String {
        format!(
            "Expr Context: {}",
            match &self {
                ExprContext::Load => "load",
                ExprContext::Store => "store",
                ExprContext::Del => "del",
            }
        )
    }
}

impl ToHashString for KeywordData {
    fn to_hashable_string(&self) -> String {
        format!(
            "Keyword Data: arg: {}, value: {}",
            match &self.arg {
                Some(arg) => arg.to_string(),
                None => "None".to_string(),
            },
            self.value.to_hashable_string()
        )
    }
}

impl ToHashString for Operator {
    fn to_hashable_string(&self) -> String {
        format!(
            "Operator: {}",
            match &self {
                Operator::Add => "+",
                Operator::Sub => "-",
                Operator::Mult => "*",
                Operator::MatMult => "@",
                Operator::Div => "/",
                Operator::Mod => "%",
                Operator::Pow => "^^",
                Operator::LShift => "<<",
                Operator::RShift => ">>",
                Operator::BitOr => "|",
                Operator::BitXor => "^",
                Operator::BitAnd => "&",
                Operator::FloorDiv => "//",
            }
        )
    }
}

impl ToHashString for Unaryop {
    fn to_hashable_string(&self) -> String {
        format!(
            "Unary Operator: {}",
            match &self {
                Unaryop::Invert => "~",
                Unaryop::Not => "not",
                Unaryop::UAdd => "+",
                Unaryop::USub => "-",
            }
        )
    }
}
