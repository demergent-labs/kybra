use std::ops::Deref;

use rustpython_parser::ast::{
    AliasData, ArgData, Arguments, Boolop, Cmpop, Comprehension, Constant, ExcepthandlerKind,
    ExprContext, ExprKind, KeywordData, Located, MatchCase, Mod, Operator, PatternKind, StmtKind,
    TypeIgnore, Unaryop, Withitem,
};

use crate::{
    py_ast::PyAst,
    source_map::{token_length::TokenLength, SourceMapped},
};

pub trait Analyze {
    fn analyze(&self);
}

impl PyAst {
    pub fn analyze(&self) -> &PyAst {
        let kps: &Vec<SourceMapped<Mod>> = &self.source_mapped_mods;
        for kybra_program in kps {
            kybra_program.analyze()
        }
        self
    }
}

impl Analyze for SourceMapped<Mod> {
    fn analyze(&self) {
        eprintln!("#########################");
        eprintln!("### This is a kybra file: {}", self.source_map.file_name);
        eprintln!("#########################");
        if "/home/bdemann/code/demergent_labs/kybra/examples/query/src/main.py"
            == self.source_map.file_name
            || false
        {
            eprintln!("{} is the length of this program", self.get_token_length());
            match &self.deref() {
                rustpython_parser::ast::Mod::Module { body, type_ignores } => {
                    give_analysis("Module", 2);
                    give_analysis("body", 3);
                    body.analyze();
                    give_analysis("type_ignores", 3);
                    type_ignores.analyze();
                }
                rustpython_parser::ast::Mod::Interactive { body } => {
                    give_analysis("Interactive", 2);
                    give_analysis("body", 3);
                    body.analyze();
                }
                rustpython_parser::ast::Mod::Expression { body } => {
                    give_analysis("Expression", 2);
                    give_analysis("body", 3);
                    body.analyze();
                }
                rustpython_parser::ast::Mod::FunctionType { argtypes, returns } => {
                    give_analysis("FunctionType", 2);
                    give_analysis("argtypes", 3);
                    argtypes.analyze();
                    give_analysis("returns", 3);
                    returns.analyze();
                }
            };
        }
        eprintln!("#########################");
        eprintln!("### End of kybra file: {}", self.source_map.file_name);
        eprintln!("#########################");
    }
}

impl<T> Analyze for Vec<T>
where
    T: Analyze,
{
    fn analyze(&self) {
        give_analysis("Vec", 1);
        for (i, t) in self.iter().enumerate() {
            give_analysis(format!("{}th value in vec", i).as_str(), 2);
            t.analyze()
        }
    }
}

impl Analyze for StmtKind {
    fn analyze(&self) {
        eprintln!("####################");
        give_analysis("StmtKind", 1);
        eprintln!("####################");
        eprintln!(
            "The length of this statement is {}",
            self.get_token_length()
        );
        match &self {
            StmtKind::FunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
            } => {
                give_analysis("FunctionDef", 2);
                give_analysis("name", 3);
                name.analyze();
                give_analysis("args", 3);
                args.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("decorator_list", 3);
                decorator_list.analyze();
                give_analysis("returns", 3);
                match returns {
                    Some(returns) => returns.analyze(),
                    None => eprintln!("No returns"),
                }
                give_analysis("type_comment", 3);
                match type_comment {
                    Some(type_comment) => type_comment.analyze(),
                    None => eprintln!("No type comment"),
                }
            }
            StmtKind::AsyncFunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
            } => {
                give_analysis("AsyncFunctionDef", 2);
                give_analysis("name", 3);
                name.analyze();
                give_analysis("args", 3);
                args.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("decorator_list", 3);
                decorator_list.analyze();
                give_analysis("returns", 3);
                match returns {
                    Some(returns) => returns.analyze(),
                    None => eprintln!("No returns"),
                }
                give_analysis("type_comment", 3);
                match type_comment {
                    Some(type_comment) => type_comment.analyze(),
                    None => eprintln!("No type comment"),
                }
            }
            StmtKind::ClassDef {
                name,
                bases,
                keywords,
                body,
                decorator_list,
            } => {
                give_analysis("ClassDef", 2);
                give_analysis("name", 3);
                name.analyze();
                give_analysis("bases", 3);
                bases.analyze();
                give_analysis("keywords", 3);
                keywords.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("decorator_list", 3);
                decorator_list.analyze();
            }
            StmtKind::Return { value } => {
                give_analysis("Return", 2);
                give_analysis("value", 3);
                match value {
                    Some(returns) => returns.analyze(),
                    None => eprintln!("No returns"),
                }
            }
            StmtKind::Delete { targets } => {
                give_analysis("Delete", 2);
                give_analysis("targets", 3);
                targets.analyze()
            }
            StmtKind::Assign {
                targets,
                value,
                type_comment,
            } => {
                give_analysis("Assign", 2);
                give_analysis("targets", 3);
                targets.analyze();
                give_analysis("value", 3);
                value.analyze();
                give_analysis("type_comment", 3);
                match type_comment {
                    Some(type_comment) => type_comment.analyze(),
                    None => eprintln!("No type comment"),
                }
            }
            StmtKind::AugAssign { target, op, value } => {
                give_analysis("AugAssign", 2);
                give_analysis("target", 3);
                target.analyze();
                give_analysis("op", 3);
                op.analyze();
                give_analysis("value", 3);
                value.analyze();
            }
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                simple,
            } => {
                give_analysis("AnnAssign", 2);
                give_analysis("target", 3);
                target.analyze();
                give_analysis("annotation", 3);
                annotation.analyze();
                give_analysis("value", 3);
                match value {
                    Some(value) => value.analyze(),
                    None => eprintln!("No returns"),
                }
                give_analysis("simple", 3);
                simple.analyze();
            }
            StmtKind::For {
                target,
                iter,
                body,
                orelse,
                type_comment,
            } => {
                give_analysis("For", 2);
                give_analysis("target", 3);
                target.analyze();
                give_analysis("iter", 3);
                iter.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("orelse", 3);
                orelse.analyze();
                give_analysis("type_comment", 3);
                match type_comment {
                    Some(type_comment) => type_comment.analyze(),
                    None => eprintln!("No type comment"),
                }
            }
            StmtKind::AsyncFor {
                target,
                iter,
                body,
                orelse,
                type_comment,
            } => {
                give_analysis("AsyncFor", 2);
                give_analysis("target", 3);
                target.analyze();
                give_analysis("iter", 3);
                iter.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("orelse", 3);
                orelse.analyze();
                give_analysis("type_comment", 3);
                match type_comment {
                    Some(type_comment) => type_comment.analyze(),
                    None => eprintln!("No type comment"),
                }
            }
            StmtKind::While { test, body, orelse } => {
                give_analysis("While", 2);
                give_analysis("test", 3);
                test.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("orelse", 3);
                orelse.analyze();
            }
            StmtKind::If { test, body, orelse } => {
                give_analysis("If", 2);
                give_analysis("test", 3);
                test.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("orelse", 3);
                orelse.analyze();
            }
            StmtKind::With {
                items,
                body,
                type_comment,
            } => {
                give_analysis("With", 2);
                give_analysis("items", 3);
                items.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("type_comment", 3);
                match type_comment {
                    Some(type_comment) => type_comment.analyze(),
                    None => eprintln!("No type comment"),
                }
            }
            StmtKind::AsyncWith {
                items,
                body,
                type_comment,
            } => {
                give_analysis("AsyncWith", 2);
                give_analysis("items", 3);
                items.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("type_comment", 3);
                match type_comment {
                    Some(type_comment) => type_comment.analyze(),
                    None => eprintln!("No type comment"),
                }
            }
            StmtKind::Match { subject, cases } => {
                give_analysis("Match", 2);
                give_analysis("subject", 3);
                subject.analyze();
                give_analysis("cases", 3);
                cases.analyze();
            }
            StmtKind::Raise { exc, cause } => {
                give_analysis("Raise", 2);
                give_analysis("exc", 3);
                match exc {
                    Some(exc) => exc.analyze(),
                    None => eprintln!("No exc"),
                }
                give_analysis("cause", 3);
                match cause {
                    Some(cause) => cause.analyze(),
                    None => eprintln!("No cause"),
                }
            }
            StmtKind::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                give_analysis("Try", 2);
                give_analysis("body", 3);
                body.analyze();
                give_analysis("handlers", 3);
                handlers.analyze();
                give_analysis("orelse", 3);
                orelse.analyze();
                give_analysis("finalbody", 3);
                finalbody.analyze();
            }
            StmtKind::Assert { test, msg } => {
                give_analysis("Assert", 2);
                give_analysis("test", 3);
                test.analyze();
                give_analysis("msg", 3);
                match msg {
                    Some(msg) => msg.analyze(),
                    None => eprintln!("No msg"),
                }
            }
            StmtKind::Import { names } => {
                give_analysis("Import", 2);
                give_analysis("names", 3);
                names.analyze()
            }
            StmtKind::ImportFrom {
                module,
                names,
                level,
            } => {
                give_analysis("ImportFrom", 2);
                give_analysis("module", 3);
                match module {
                    Some(msg) => msg.analyze(),
                    None => eprintln!("No msg"),
                }
                give_analysis("names", 3);
                names.analyze();
                give_analysis("level", 3);
                match level {
                    Some(level) => level.analyze(),
                    None => eprintln!("No level"),
                }
            }
            StmtKind::Global { names } => {
                give_analysis("Global", 2);
                give_analysis("name", 3);
                names.analyze()
            }
            StmtKind::Nonlocal { names } => {
                give_analysis("Nonlocal", 2);
                give_analysis("names", 3);
                names.analyze()
            }
            StmtKind::Expr { value } => {
                give_analysis("Expr", 2);
                give_analysis("value", 3);
                value.analyze()
            }
            StmtKind::Pass => give_analysis("Pass", 2),
            StmtKind::Break => give_analysis("Break", 2),
            StmtKind::Continue => give_analysis("Continue", 2),
        }
        eprintln!("####################");
        give_analysis("End of StmtKind", 1);
        eprintln!("####################");
    }
}

impl Analyze for Boolop {
    fn analyze(&self) {
        give_analysis("Boolop", 1);
        let boolop = match self {
            Boolop::And => "And",
            Boolop::Or => "Or",
        };
        give_analysis(format!("{}", boolop).as_str(), 2)
    }
}

impl Analyze for Unaryop {
    fn analyze(&self) {
        give_analysis("Unaryop", 1);
        let unaryop = match self {
            Unaryop::Invert => "Invert",
            Unaryop::Not => "Not",
            Unaryop::UAdd => "UAdd",
            Unaryop::USub => "USub",
        };
        give_analysis(format!("{}", unaryop).as_str(), 2)
    }
}

impl Analyze for Comprehension {
    fn analyze(&self) {
        give_analysis("Comprehension", 1);
        give_analysis("target", 2);
        self.target.analyze();
        give_analysis("iter", 2);
        self.iter.analyze();
        give_analysis("ifs", 2);
        self.ifs.analyze();
        give_analysis("is_async", 2);
        self.is_async.analyze();
    }
}

impl Analyze for Cmpop {
    fn analyze(&self) {
        give_analysis("Cmpop", 1);
        let cmpop = match self {
            Cmpop::Eq => "Eq",
            Cmpop::NotEq => "NotEq",
            Cmpop::Lt => "Lt",
            Cmpop::LtE => "LtE",
            Cmpop::Gt => "Gt",
            Cmpop::GtE => "GtE",
            Cmpop::Is => "Is",
            Cmpop::IsNot => "IsNot",
            Cmpop::In => "In",
            Cmpop::NotIn => "NotIn",
        };
        give_analysis(format!("{}", cmpop).as_str(), 2);
    }
}

impl Analyze for Constant {
    fn analyze(&self) {
        give_analysis("Constant", 1);
        let constant = match self {
            Constant::None => "None".to_string(),
            Constant::Bool(bool) => format!("Bool: {}", bool),
            Constant::Str(string) => format!("Str: {}", string),
            Constant::Bytes(bytes) => format!("Bytes: {:?}", bytes),
            Constant::Int(int) => format!("Int: {}", int),
            Constant::Tuple(tuple) => format!("Tuple: {:?}", tuple),
            Constant::Float(float) => format!("Float: {}", float),
            Constant::Complex { real, imag } => format!("Complex: real({}), imag({})", real, imag),
            Constant::Ellipsis => "Ellipsis".to_string(),
        };
        give_analysis(format!("{}", constant).as_str(), 2)
    }
}

impl Analyze for ExprContext {
    fn analyze(&self) {
        give_analysis("ExprContext", 1);
        let expr_context = match self {
            ExprContext::Load => "Load",
            ExprContext::Store => "Store",
            ExprContext::Del => "Del",
        };
        give_analysis(format!("{}", expr_context).as_str(), 2)
    }
}

impl Analyze for ExprKind {
    fn analyze(&self) {
        eprintln!("____________________");
        give_analysis("ExprKind", 1);
        eprintln!("____________________");
        eprintln!(
            "The length of this expr kind is {}",
            self.get_token_length()
        );
        match &self {
            ExprKind::BoolOp { op, values } => {
                give_analysis("BoolOp", 2);
                give_analysis("op", 3);
                op.analyze();
                give_analysis("values", 3);
                values.analyze();
            }
            ExprKind::NamedExpr { target, value } => {
                give_analysis("NamedExpr", 2);
                give_analysis("target", 3);
                target.analyze();
                give_analysis("value", 3);
                value.analyze();
            }
            ExprKind::BinOp { left, op, right } => {
                give_analysis("BinOp", 2);
                give_analysis("left", 3);
                left.analyze();
                give_analysis("op", 3);
                op.analyze();
                give_analysis("right", 3);
                right.analyze();
            }
            ExprKind::UnaryOp { op, operand } => {
                give_analysis("UnaryOp", 2);
                give_analysis("op", 3);
                op.analyze();
                give_analysis("operand", 3);
                operand.analyze();
            }
            ExprKind::Lambda { args, body } => {
                give_analysis("Lambda", 2);
                give_analysis("args", 3);
                args.analyze();
                give_analysis("body", 3);
                body.analyze();
            }
            ExprKind::IfExp { test, body, orelse } => {
                give_analysis("IfExp", 2);
                give_analysis("test", 3);
                test.analyze();
                give_analysis("body", 3);
                body.analyze();
                give_analysis("orelse", 3);
                orelse.analyze();
            }
            ExprKind::Dict { keys, values } => {
                give_analysis("Dict", 2);
                give_analysis("keys", 3);
                keys.analyze();
                give_analysis("values", 3);
                values.analyze();
            }
            ExprKind::Set { elts } => {
                give_analysis("Set", 2);
                give_analysis("elts", 3);
                elts.analyze()
            }
            ExprKind::ListComp { elt, generators } => {
                give_analysis("ListComp", 2);
                give_analysis("elt", 3);
                elt.analyze();
                give_analysis("generators", 3);
                generators.analyze();
            }
            ExprKind::SetComp { elt, generators } => {
                give_analysis("SetComp", 2);
                give_analysis("elt", 3);
                elt.analyze();
                give_analysis("generators", 3);
                generators.analyze();
            }
            ExprKind::DictComp {
                key,
                value,
                generators,
            } => {
                give_analysis("DictComp", 2);
                give_analysis("key", 3);
                key.analyze();
                give_analysis("value", 3);
                value.analyze();
                give_analysis("generator", 3);
                generators.analyze();
            }
            ExprKind::GeneratorExp { elt, generators } => {
                give_analysis("GeneratorExp", 2);
                give_analysis("elt", 3);
                elt.analyze();
                give_analysis("generators", 3);
                generators.analyze();
            }
            ExprKind::Await { value } => {
                give_analysis("Await", 2);
                give_analysis("value", 3);
                value.analyze()
            }
            ExprKind::Yield { value } => {
                give_analysis("Yield Expr:", 2);
                give_analysis("value", 3);
                match &value {
                    Some(value) => value.analyze(),
                    None => eprintln!("No Value"),
                }
            }
            ExprKind::YieldFrom { value } => {
                give_analysis("YieldFrom", 2);
                give_analysis("value", 3);
                value.analyze()
            }
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => {
                give_analysis("Compare", 2);
                give_analysis("left", 3);
                left.analyze();
                give_analysis("op", 2);
                ops.analyze();
                give_analysis("comparators", 3);
                comparators.analyze();
            }
            ExprKind::Call {
                func,
                args,
                keywords,
            } => {
                give_analysis("Call", 2);
                give_analysis("func", 3);
                func.analyze();
                give_analysis("args", 2);
                args.analyze();
                give_analysis("keywords", 2);
                keywords.analyze()
            }
            ExprKind::FormattedValue {
                value,
                conversion,
                format_spec,
            } => {
                give_analysis("FormattedValue", 2);
                give_analysis("value", 3);
                value.analyze();
                give_analysis("conversion", 3);
                conversion.analyze();
                give_analysis("format_spec", 3);
                match format_spec {
                    Some(format_spec) => format_spec.analyze(),
                    None => eprintln!("No format spec"),
                }
            }
            ExprKind::JoinedStr { values } => {
                give_analysis("JoinedStr", 2);
                give_analysis("values", 3);
                values.analyze()
            }
            ExprKind::Constant { value, kind } => {
                give_analysis("Constant", 2);
                give_analysis("value", 3);
                value.analyze();
                give_analysis("kind", 3);
                match &kind {
                    Some(kind) => kind.analyze(),
                    None => eprintln!("No kind"),
                }
            }
            ExprKind::Attribute { value, attr, ctx } => {
                give_analysis("Attribute", 2);
                give_analysis("value", 3);
                value.analyze();
                give_analysis("attr", 3);
                attr.analyze();
                give_analysis("ctx", 3);
                ctx.analyze();
            }
            ExprKind::Subscript { value, slice, ctx } => {
                give_analysis("Subscript", 2);
                give_analysis("value", 3);
                value.analyze();
                give_analysis("slice", 3);
                slice.analyze();
                give_analysis("ctx", 3);
                ctx.analyze();
            }
            ExprKind::Starred { value, ctx } => {
                give_analysis("Starred", 2);
                give_analysis("value", 3);
                value.analyze();
                give_analysis("ctx", 3);
                ctx.analyze();
            }
            ExprKind::Name { id, ctx } => {
                give_analysis("Name", 2);
                give_analysis("id", 3);
                id.analyze();
                give_analysis("ctx", 3);
                ctx.analyze();
            }
            ExprKind::List { elts, ctx } => {
                give_analysis("List", 2);
                give_analysis("elts", 3);
                elts.analyze();
                give_analysis("ctx", 3);
                ctx.analyze();
            }
            ExprKind::Tuple { elts, ctx } => {
                give_analysis("Tuple", 2);
                give_analysis("elts", 3);
                elts.analyze();
                give_analysis("ctx", 3);
                ctx.analyze();
            }
            ExprKind::Slice { lower, upper, step } => {
                give_analysis("Slice", 2);
                give_analysis("lower", 3);
                match lower {
                    Some(lower) => lower.analyze(),
                    None => eprintln!("No lower"),
                }
                give_analysis("upper", 3);
                match upper {
                    Some(upper) => upper.analyze(),
                    None => eprintln!("No lower"),
                }
                give_analysis("step", 3);
                match step {
                    Some(step) => step.analyze(),
                    None => eprintln!("No lower"),
                }
            }
        }
        eprintln!("____________________");
        give_analysis("End of ExprKind", 1);
        eprintln!("____________________");
    }
}

impl Analyze for KeywordData {
    fn analyze(&self) {
        give_analysis("KeywordData", 1);
        give_analysis("value", 2);
        self.value.analyze()
    }
}

impl<T> Analyze for Located<T>
where
    T: Analyze,
{
    fn analyze(&self) {
        give_analysis("Located", 1);
        give_analysis("node", 2);
        self.node.analyze()
    }
}

impl Analyze for TypeIgnore {
    fn analyze(&self) {
        give_analysis("TypeIgnore", 1);
        give_analysis("TypeIgnore", 2);
        match self {
            TypeIgnore::TypeIgnore { lineno, tag } => {
                give_analysis("lineno", 3);
                lineno.analyze();
                give_analysis("tag", 3);
                tag.analyze();
            }
        }
    }
}

impl Analyze for String {
    fn analyze(&self) {
        eprintln!("String: {}", self)
    }
}

impl Analyze for usize {
    fn analyze(&self) {
        eprintln!("usize: {}", self)
    }
}

impl Analyze for Arguments {
    fn analyze(&self) {
        give_analysis("Arguments", 1);
        give_analysis("posonlyargs", 2);
        self.posonlyargs.analyze();
        give_analysis("args", 2);
        self.args.analyze();
        give_analysis("vararg", 2);
        match &self.vararg {
            Some(vararg) => vararg.analyze(),
            None => eprintln!("No vararg"),
        }
        give_analysis("kwonlyargs", 2);
        self.kwonlyargs.analyze();
        give_analysis("kw_defaults", 2);
        self.kw_defaults.analyze();
        give_analysis("kwarg", 2);
        match &self.kwarg {
            Some(kwarg) => kwarg.analyze(),
            None => eprintln!("No vararg"),
        }
        give_analysis("defaults", 2);
        self.defaults.analyze();
        give_analysis("args", 2);
        self.args.analyze();
    }
}

impl Analyze for ArgData {
    fn analyze(&self) {
        give_analysis("ArgData", 1);
        give_analysis("arg", 2);
        self.arg.analyze();
        give_analysis("annotation", 2);
        match &self.annotation {
            Some(annotation) => annotation.analyze(),
            None => eprintln!("No annotation"),
        }
        give_analysis("type_comment", 2);
        match &self.type_comment {
            Some(type_comment) => type_comment.analyze(),
            None => eprintln!("No type comment"),
        }
    }
}

impl Analyze for Operator {
    fn analyze(&self) {
        give_analysis("Operator", 1);
        give_analysis("op", 2);
        let op = match &self {
            Operator::Add => "Add",
            Operator::Sub => "Sub",
            Operator::Mult => "Mult",
            Operator::MatMult => "MatMult",
            Operator::Div => "Div",
            Operator::Mod => "Mod",
            Operator::Pow => "Pow",
            Operator::LShift => "LShift",
            Operator::RShift => "RShift",
            Operator::BitOr => "BitOr",
            Operator::BitXor => "BitXor",
            Operator::BitAnd => "BitAnd",
            Operator::FloorDiv => "FloorDiv",
        };
        eprintln!("{}", op)
    }
}

impl Analyze for Withitem {
    fn analyze(&self) {
        give_analysis("Withitem", 1);
        give_analysis("context_expr", 2);
        self.context_expr.analyze();
        give_analysis("optional_vars", 2);
        match &self.optional_vars {
            Some(optional_vars) => optional_vars.analyze(),
            None => eprintln!("No optional vars"),
        }
    }
}

impl Analyze for PatternKind {
    fn analyze(&self) {
        give_analysis("PatternKind", 1);
        match &self {
            PatternKind::MatchValue { value } => {
                give_analysis("MatchValue", 2);
                give_analysis("value", 3);
                value.analyze();
            }
            PatternKind::MatchSingleton { value } => {
                give_analysis("MatchSingleton", 2);
                give_analysis("value", 3);
                value.analyze();
            }
            PatternKind::MatchSequence { patterns } => {
                give_analysis("MatchSequence", 2);
                give_analysis("patterns", 3);
                patterns.analyze();
            }
            PatternKind::MatchMapping {
                keys,
                patterns,
                rest,
            } => {
                give_analysis("MatchMapping", 2);
                give_analysis("keys", 3);
                keys.analyze();
                give_analysis("patterns", 3);
                patterns.analyze();
                give_analysis("rest", 3);
                match rest {
                    Some(rest) => rest.analyze(),
                    None => eprintln!("No rest"),
                }
            }
            PatternKind::MatchClass {
                cls,
                patterns,
                kwd_attrs,
                kwd_patterns,
            } => {
                give_analysis("MatchClass", 2);
                give_analysis("cls", 3);
                cls.analyze();
                give_analysis("patterns", 3);
                patterns.analyze();
                give_analysis("kwd_attrs", 3);
                kwd_attrs.analyze();
                give_analysis("kwd_patterns", 3);
                kwd_patterns.analyze();
            }
            PatternKind::MatchStar { name } => {
                give_analysis("MatchStar", 2);
                give_analysis("name", 3);
                match name {
                    Some(name) => name.analyze(),
                    None => eprintln!("No match star"),
                }
            }
            PatternKind::MatchAs { pattern, name } => {
                give_analysis("MatchAs", 2);
                give_analysis("pattern", 3);
                match pattern {
                    Some(pattern) => pattern.analyze(),
                    None => eprintln!("No pattern"),
                };
                give_analysis("name", 3);
                match name {
                    Some(name) => name.analyze(),
                    None => eprintln!("No name"),
                };
            }
            PatternKind::MatchOr { patterns } => {
                give_analysis("MatchOr", 3);
                give_analysis("patterns", 2);
                patterns.analyze();
            }
        }
    }
}

impl Analyze for MatchCase {
    fn analyze(&self) {
        give_analysis("MatchCase", 1);
        give_analysis("pattern", 2);
        self.pattern.analyze();
        give_analysis("guard", 2);
        match &self.guard {
            Some(guard) => guard.analyze(),
            None => eprintln!("No guard"),
        }
        give_analysis("body", 2);
        self.body.analyze();
    }
}

impl Analyze for ExcepthandlerKind {
    fn analyze(&self) {
        give_analysis("ExcepthandlerKind", 1);
        match &self {
            ExcepthandlerKind::ExceptHandler { type_, name, body } => {
                give_analysis("type", 2);
                match &type_ {
                    Some(type_) => type_.analyze(),
                    None => eprintln!("No type"),
                };
                give_analysis("name", 2);
                match &name {
                    Some(name) => name.analyze(),
                    None => eprintln!("No name"),
                };
                give_analysis("body", 2);
                body.analyze();
            }
        }
    }
}

impl Analyze for AliasData {
    fn analyze(&self) {
        give_analysis("AliasData", 1);
        give_analysis("name", 2);
        self.name.analyze();
        give_analysis("asname", 2);
        match &self.asname {
            Some(asname) => asname.analyze(),
            None => eprintln!("No asname"),
        }
    }
}

fn give_analysis(name: &str, level: u8) {
    let appendages = match level {
        1 => "##",
        2 => ">>",
        3 => "**",
        _ => "--",
    };
    eprintln!("{}{}{}", appendages, name, appendages)
}
