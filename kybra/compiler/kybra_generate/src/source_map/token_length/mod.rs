use regex::Regex;
use rustpython_parser::ast::{
    AliasData, ArgData, Arguments, Boolop, Cmpop, Comprehension, Constant, ExcepthandlerKind,
    ExprContext, ExprKind, KeywordData, Located, MatchCase, Mod, Operator, PatternKind, StmtKind,
    TypeIgnore, Unaryop, Withitem,
};

use crate::debug::WhatIsIt;

pub const SPECIAL_CHARACTERS: &str = r"[^A-Za-z0-9]";
pub const REGULAR_CHARACTERS: &str = r"[A-Za-z0-9]";

pub trait TokenLength {
    fn get_token_length(&self) -> usize;
}

impl TokenLength for Mod {
    fn get_token_length(&self) -> usize {
        match &self {
            Mod::Module {
                body,
                type_ignores: _,
            } => {
                body.get_token_length()
                //+ type_ignores.get_token_length() I don't think we need to take these into account
            }
            Mod::Interactive { body } => body.get_token_length(),
            Mod::Expression { body } => body.get_token_length(),
            Mod::FunctionType { argtypes, returns } => {
                argtypes.get_token_length() + returns.get_token_length()
            }
        }
    }
}

impl<T> TokenLength for Vec<T>
where
    T: TokenLength,
{
    fn get_token_length(&self) -> usize {
        let result = self.iter().fold(0, |acc, t| {
            let thing = t.get_token_length();
            acc + thing
        });
        result
    }
}

impl TokenLength for StmtKind {
    fn get_token_length(&self) -> usize {
        let len = match &self {
            StmtKind::FunctionDef {
                name,
                args,
                body,
                decorator_list: _,
                returns,
                type_comment,
            } => {
                "def".len()
                    + name.get_token_length()
                    + args.get_token_length()
                    + body.get_token_length()
                    // + decorator_list.get_token_length() // I don't think we need the decorator list since the token start is at the def key word so its after the decorators are already past
                    + match returns {
                        Some(returns) => returns.get_token_length(),
                        None => 0,
                    }
                    + match type_comment {
                        Some(type_comment) => type_comment.get_token_length(),
                        None => 0,
                    }
            }
            StmtKind::AsyncFunctionDef {
                name,
                args,
                body,
                decorator_list: _,
                returns,
                type_comment,
            } => {
                "async".len()
                    + "def".len()
                    + name.get_token_length()
                    + args.get_token_length()
                    + body.get_token_length()
                    // + decorator_list.get_token_length() // See the decorator list from FunctionDefs
                    + match returns {
                        Some(returns) => returns.get_token_length(),
                        None => 0,
                    }
                    + match type_comment {
                        Some(type_comment) => type_comment.get_token_length(),
                        None => 0,
                    }
            }
            StmtKind::ClassDef {
                name,
                bases,
                keywords,
                body,
                decorator_list: _,
            } => {
                let result = "class".len()
                    + name.get_token_length()
                    + bases.get_token_length()
                    + keywords.get_token_length()
                    + body.get_token_length();
                // eprintln!(
                //     "My guess is its the body which is {} long",
                //     body.get_token_length()
                // );
                // for stmt in body {
                //     eprintln!(
                //         "This is a stmt in the body and it's {} long",
                //         stmt.get_token_length()
                //     );
                //     stmt.what_is_it()
                // }
                // + decorator_list.get_token_length() // See the decorator list from FunctionDefs
                // eprintln!("The length of a class is {}", result);
                result
            }
            StmtKind::Return { value } => match value {
                Some(returns) => {
                    // eprintln!("We are about to return the length of a return statement");
                    // eprintln!("I think that it is {} long", returns.get_token_length());
                    // returns.what_is_it();
                    // eprintln!(
                    //     "Plus the length of the return keyword which should be {} long",
                    //     "return".len()
                    // );
                    "return".len() + returns.get_token_length()
                }
                None => 0,
            },
            StmtKind::Delete { targets } => "del".len() + targets.get_token_length(),
            StmtKind::Assign {
                targets,
                value,
                type_comment,
            } => {
                targets.get_token_length()
                    + value.get_token_length()
                    + match type_comment {
                        Some(type_comment) => type_comment.get_token_length(),
                        None => 0,
                    }
            }
            StmtKind::AugAssign { target, op, value } => {
                target.get_token_length() + op.get_token_length() + value.get_token_length()
            }
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                simple,
            } => {
                target.get_token_length()
                    + annotation.get_token_length()
                    + match value {
                        Some(value) => value.get_token_length(),
                        None => 0,
                    }
                    + simple.get_token_length()
            }
            StmtKind::For {
                target,
                iter,
                body,
                orelse,
                type_comment,
            } => calculate_for_loop_length(target, iter, body, orelse, type_comment),
            StmtKind::AsyncFor {
                target,
                iter,
                body,
                orelse,
                type_comment,
            } => {
                "async".len() + calculate_for_loop_length(target, iter, body, orelse, type_comment)
            }
            StmtKind::While { test, body, orelse } => {
                "while".len()
                    + test.get_token_length()
                    + body.get_token_length()
                    + if orelse.len() > 0 { "else".len() } else { 0 }
                    + orelse.get_token_length()
            }
            StmtKind::If { test, body, orelse } => {
                "if".len()
                    + test.get_token_length()
                    + body.get_token_length()
                    + if orelse.len() > 0 { "else".len() } else { 0 }
                    + orelse.get_token_length()
            }
            StmtKind::With {
                items,
                body,
                type_comment: _,
            } => {
                let result = calculate_with_length(items, body);
                // eprintln!("Lets see the result for the with is {}", result);
                result
            }
            StmtKind::AsyncWith {
                items,
                body,
                type_comment: _,
            } => "async".len() + calculate_with_length(items, body),
            StmtKind::Match { subject, cases } => {
                "match".len() + subject.get_token_length() + cases.get_token_length()
            }
            StmtKind::Raise { exc, cause } => {
                // eprintln!("This is the exc");
                // exc.as_ref().unwrap().what_is_it();
                // eprintln!("This is the cause");
                // cause.as_ref().unwrap().what_is_it();
                "raise".len()
                    + match exc {
                        Some(exc) => exc.get_token_length(),
                        None => 0,
                    }
                    + match cause {
                        Some(cause) => "from".len() + cause.get_token_length(),
                        None => 0,
                    }
            }
            StmtKind::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                for ore in orelse {
                    ore.what_is_it()
                }
                "try".len()
                    + body.get_token_length()
                    + handlers.iter().fold(0, |acc, _| acc + "except".len())
                    + handlers.get_token_length()
                    + orelse.get_token_length()
                    + if finalbody.len() > 0 {
                        "finally".len()
                    } else {
                        0
                    }
                    + finalbody.get_token_length()
            }
            StmtKind::Assert { test, msg } => {
                "assert".len()
                    + test.get_token_length()
                    + match msg {
                        Some(msg) => msg.get_token_length(),
                        None => 0,
                    }
            }
            StmtKind::Import { names } => {
                // eprintln!(
                //     "This is the length we are giving:\nimport: {}\nnames: {}",
                //     "import".len(),
                //     names.get_token_length()
                // );
                "import".len() + names.get_token_length()
            }
            StmtKind::ImportFrom {
                module,
                names,
                level,
            } => {
                "from".len()
                    + match module {
                        Some(msg) => msg.get_token_length(),
                        None => 0,
                    }
                    + "import".len()
                    + names.get_token_length()
                    + match level {
                        Some(level) => level.get_token_length(),
                        None => 0,
                    }
            }
            StmtKind::Global { names } => "global".len() + names.get_token_length(),
            StmtKind::Nonlocal { names } => "nonlocal".len() + names.get_token_length(),
            StmtKind::Expr { value } => value.get_token_length(),
            StmtKind::Pass => "pass".len(),
            StmtKind::Break => "break".len(),
            StmtKind::Continue => "continue".len(),
        };
        // eprintln!("We are looking at an STMT KIND:");
        // self.what_is_it();
        // eprintln!("This stmt kind is {} characters long", len);
        len
    }
}

fn calculate_with_length(items: &Vec<Withitem>, body: &Vec<Located<StmtKind>>) -> usize {
    // eprintln!("with: {}", "with".len());
    // eprintln!("as: {}", "as".len());
    // eprintln!("items: {}", items.get_token_length());
    // eprintln!("body: {}", body.get_token_length());
    "with".len() + "as".len() + items.get_token_length() + body.get_token_length()
}

fn calculate_for_loop_length(
    target: &Box<Located<ExprKind>>,
    iter: &Box<Located<ExprKind>>,
    body: &Vec<Located<StmtKind>>,
    orelse: &Vec<Located<StmtKind>>,
    _type_comment: &Option<String>,
) -> usize {
    // eprintln!("for len: {}", "for".len());
    // eprintln!("target len: {}", target.get_token_length());
    // eprintln!("in len: {}", "in".len());
    // eprintln!("iter len: {}", iter.get_token_length());
    // eprintln!("body len: {}", body.get_token_length());
    // eprintln!(
    //     "else len: {}",
    //     if orelse.len() > 0 { "else".len() } else { 0 }
    // );
    // eprintln!("orelse len: {}", orelse.get_token_length());
    // let result = "for".len()
    //     + target.get_token_length()
    //     + "in".len()
    //     + iter.get_token_length()
    //     + body.get_token_length()
    //     + if orelse.len() > 0 { "else".len() } else { 0 }
    //     + orelse.get_token_length();
    // eprintln!("for a total of {}", result);
    "for".len()
        + target.get_token_length()
        + "in".len()
        + iter.get_token_length()
        + body.get_token_length()
        + if orelse.len() > 0 { "else".len() } else { 0 }
        + orelse.get_token_length()
}

impl TokenLength for Boolop {
    fn get_token_length(&self) -> usize {
        match self {
            Boolop::And => "and",
            Boolop::Or => "or",
        }
        .len()
    }
}

impl TokenLength for Unaryop {
    fn get_token_length(&self) -> usize {
        let uop = match self {
            Unaryop::Invert => "~",
            Unaryop::Not => "not",
            Unaryop::UAdd => "+",
            Unaryop::USub => "-",
        };
        let re = Regex::new(SPECIAL_CHARACTERS).unwrap();
        re.replace_all(uop, "").len()
    }
}

impl TokenLength for Comprehension {
    fn get_token_length(&self) -> usize {
        "for".len()
            + "in".len()
            + self.target.get_token_length()
            + self.iter.get_token_length()
            + self.ifs.iter().fold(0, |acc, _| acc + "if".len())
            + self.ifs.get_token_length()
            + self.is_async.get_token_length()
    }
}

impl TokenLength for Cmpop {
    fn get_token_length(&self) -> usize {
        let cmpop = match self {
            Cmpop::Eq => "==",
            Cmpop::NotEq => "!=",
            Cmpop::Lt => "<",
            Cmpop::LtE => "<=",
            Cmpop::Gt => ">",
            Cmpop::GtE => ">=",
            Cmpop::Is => "is",
            Cmpop::IsNot => "is not",
            Cmpop::In => "in",
            Cmpop::NotIn => "not in",
        };
        let re = Regex::new(SPECIAL_CHARACTERS).unwrap();
        re.replace_all(cmpop, "").len()
    }
}

impl TokenLength for Constant {
    fn get_token_length(&self) -> usize {
        let re = Regex::new(SPECIAL_CHARACTERS).unwrap();
        match self {
            Constant::None => "None".len(),
            Constant::Bool(bool) => {
                if *bool {
                    "True".len()
                } else {
                    "False".len()
                }
            }
            Constant::Str(string) => re.replace_all(&string, "").len(),
            Constant::Bytes(bytes) => bytes
                .iter()
                .fold(0, |acc, byte| acc + byte.to_string().len()),
            Constant::Int(int) => int.to_string().len(),
            Constant::Tuple(tuple) => tuple.get_token_length(),
            Constant::Float(float) => re.replace_all(&float.to_string(), "").len(),
            Constant::Complex { real, imag } => {
                "j".len()
                    + re.replace_all(&real.to_string(), "").len()
                    + re.replace_all(&imag.to_string(), "").len()
            }
            Constant::Ellipsis => re.replace_all("...", "").len(),
        }
    }
}

impl TokenLength for ExprContext {
    fn get_token_length(&self) -> usize {
        0
    }
}

impl TokenLength for ExprKind {
    fn get_token_length(&self) -> usize {
        let len = match &self {
            ExprKind::BoolOp { op, values } => op.get_token_length() + values.get_token_length(),
            ExprKind::NamedExpr { target, value } => {
                target.get_token_length() + value.get_token_length()
            }
            ExprKind::BinOp { left, op, right } => {
                left.get_token_length() + op.get_token_length() + right.get_token_length()
            }
            ExprKind::UnaryOp { op, operand } => op.get_token_length() + operand.get_token_length(),
            ExprKind::Lambda { args, body } => {
                "lambda".len() + args.get_token_length() + body.get_token_length()
            }
            ExprKind::IfExp { test, body, orelse } => {
                // TODO elif is going to be a bit of a challenge that has not been figured out yet
                "if".len()
                    + test.get_token_length()
                    + body.get_token_length()
                    + "else".len()
                    + orelse.get_token_length()
            }
            ExprKind::Dict { keys, values } => keys.get_token_length() + values.get_token_length(),
            ExprKind::Set { elts } => elts.get_token_length(),
            ExprKind::ListComp { elt, generators } => {
                elt.get_token_length() + generators.get_token_length()
            }
            ExprKind::SetComp { elt, generators } => {
                elt.get_token_length() + generators.get_token_length()
            }
            ExprKind::DictComp {
                key,
                value,
                generators,
            } => key.get_token_length() + value.get_token_length() + generators.get_token_length(),
            ExprKind::GeneratorExp { elt, generators } => {
                elt.get_token_length() + generators.get_token_length()
            }
            ExprKind::Await { value } => "await".len() + value.get_token_length(),
            ExprKind::Yield { value } => {
                "yield".len()
                    + match &value {
                        Some(value) => value.get_token_length(),
                        None => 0,
                    }
            }
            ExprKind::YieldFrom { value } => {
                "yield".len() + "from".len() + value.get_token_length()
            }
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => left.get_token_length() + ops.get_token_length() + comparators.get_token_length(),
            ExprKind::Call {
                func,
                args,
                keywords,
            } => {
                // let f_len = func.get_token_length();
                // let a_len = args.get_token_length();
                // let k_len = keywords.get_token_length();
                // eprintln!(
                //     "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<the func is {} long, the args are {} long, the keywords are {} long",
                //     f_len, a_len, k_len
                // );
                func.get_token_length() + args.get_token_length() + keywords.get_token_length()
            }
            ExprKind::FormattedValue {
                value,
                conversion,
                format_spec,
            } => {
                value.get_token_length()
                    + conversion.get_token_length()
                    + match format_spec {
                        Some(format_spec) => format_spec.get_token_length(),
                        None => 0,
                    }
            }
            ExprKind::JoinedStr { values } => values.get_token_length(),
            ExprKind::Constant { value, kind } => {
                value.get_token_length()
                    + match &kind {
                        Some(kind) => kind.get_token_length(),
                        None => 0,
                    }
            }
            ExprKind::Attribute { value, attr, ctx } => {
                // let v_len = value.get_token_length();
                // let a_len = attr.get_token_length();
                // let c_len = ctx.get_token_length();
                // eprintln!("What is the value?");
                // value.what_is_it();
                // match &value.node {
                //     ExprKind::Constant { value, kind } => match &value {
                //         Constant::None => todo!(),
                //         Constant::Bool(_) => todo!(),
                //         Constant::Str(string) => {
                //             eprintln!("This is the string rep of the value: {}", string)
                //         }
                //         Constant::Bytes(_) => todo!(),
                //         Constant::Int(_) => todo!(),
                //         Constant::Tuple(_) => todo!(),
                //         Constant::Float(_) => todo!(),
                //         Constant::Complex { real, imag } => todo!(),
                //         Constant::Ellipsis => todo!(),
                //     },
                //     _ => eprintln!("It wasn't a constant"),
                // }
                // eprintln!("What is the attr?");
                // eprintln!("{}", attr);
                // eprintln!("What is the ctx?");
                // ctx.what_is_it();
                // eprintln!(
                //     ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>the value is {} long, the attr is {} long, the ctx are {} long",
                //     v_len, a_len, c_len
                // );
                // We aren't going to take into account the value length of the
                // attribute because it should already be take into account
                // earlier, I think For example "hello".upper() "hello" counts
                // as a constant and then looking at upper it will show up as an
                // attribute on hello and "hello" would be the value. but it was
                // also counted for it's self earlier
                value.get_token_length() + attr.get_token_length() + ctx.get_token_length()
            }
            ExprKind::Subscript { value, slice, ctx } => {
                value.get_token_length() + slice.get_token_length() + ctx.get_token_length()
            }
            ExprKind::Starred { value, ctx } => value.get_token_length() + ctx.get_token_length(),
            ExprKind::Name { id, ctx } => id.get_token_length() + ctx.get_token_length(),
            ExprKind::List { elts, ctx } => elts.get_token_length() + ctx.get_token_length(),
            ExprKind::Tuple { elts, ctx } => elts.get_token_length() + ctx.get_token_length(),
            ExprKind::Slice { lower, upper, step } => {
                (match lower {
                    Some(lower) => lower.get_token_length(),
                    None => 0,
                }) + match upper {
                    Some(upper) => upper.get_token_length(),
                    None => 0,
                } + match step {
                    Some(step) => step.get_token_length(),
                    None => 0,
                }
            }
        };
        // eprintln!("We are looking at an EXPR KIND:");
        // self.what_is_it();
        // eprintln!("This expr kind is {} characters long", len);
        len
    }
}

impl TokenLength for KeywordData {
    fn get_token_length(&self) -> usize {
        self.value.get_token_length()
            + match &self.arg {
                Some(arg) => arg.get_token_length(),
                None => 0,
            }
    }
}

impl<T> TokenLength for Located<T>
where
    T: TokenLength,
{
    fn get_token_length(&self) -> usize {
        self.node.get_token_length()
    }
}

impl TokenLength for TypeIgnore {
    fn get_token_length(&self) -> usize {
        todo!();
        // match self {
        //     TypeIgnore::TypeIgnore { lineno, tag } => {
        //         lineno.get_token_length() + tag.get_token_length()
        //     }
        // }
    }
}

impl TokenLength for String {
    fn get_token_length(&self) -> usize {
        let re = Regex::new(SPECIAL_CHARACTERS).unwrap();
        re.replace_all(self, "").len()
    }
}

impl TokenLength for usize {
    fn get_token_length(&self) -> usize {
        0
    }
}

impl TokenLength for Arguments {
    fn get_token_length(&self) -> usize {
        self.posonlyargs.get_token_length()
            + self.args.get_token_length()
            + match &self.vararg {
                Some(vararg) => vararg.get_token_length(),
                None => 0,
            }
            + self.kwonlyargs.get_token_length()
            + self.kw_defaults.get_token_length()
            + match &self.kwarg {
                Some(kwarg) => kwarg.get_token_length(),
                None => 0,
            }
            + self.defaults.get_token_length()
    }
}

impl TokenLength for ArgData {
    fn get_token_length(&self) -> usize {
        self.arg.get_token_length()
            + match &self.annotation {
                Some(annotation) => annotation.get_token_length(),
                None => 0,
            }
        // + match &self.type_comment {
        //     Some(type_comment) => type_comment.get_token_length(),
        //     None => 0,
        // }
    }
}

impl TokenLength for Operator {
    fn get_token_length(&self) -> usize {
        let op = match &self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mult => "*",
            Operator::MatMult => "@",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::Pow => "**",
            Operator::LShift => "<<",
            Operator::RShift => ">>",
            Operator::BitOr => "|",
            Operator::BitXor => "^",
            Operator::BitAnd => "&",
            Operator::FloorDiv => "//",
        };
        let re = Regex::new(SPECIAL_CHARACTERS).unwrap();
        re.replace_all(op, "").len()
    }
}

impl TokenLength for Withitem {
    fn get_token_length(&self) -> usize {
        self.context_expr.get_token_length()
            + match &self.optional_vars {
                Some(optional_vars) => optional_vars.get_token_length(),
                None => 0,
            }
    }
}

impl TokenLength for PatternKind {
    fn get_token_length(&self) -> usize {
        match &self {
            PatternKind::MatchValue { value } => value.get_token_length(),
            PatternKind::MatchSingleton { value } => value.get_token_length(),
            PatternKind::MatchSequence { patterns } => patterns.get_token_length(),
            PatternKind::MatchMapping {
                keys,
                patterns,
                rest,
            } => {
                keys.get_token_length()
                    + patterns.get_token_length()
                    + match rest {
                        Some(rest) => rest.get_token_length(),
                        None => 0,
                    }
            }
            PatternKind::MatchClass {
                cls,
                patterns,
                kwd_attrs,
                kwd_patterns,
            } => {
                cls.get_token_length()
                    + patterns.get_token_length()
                    + kwd_attrs.get_token_length()
                    + kwd_patterns.get_token_length()
            }
            PatternKind::MatchStar { name } => match name {
                Some(name) => name.get_token_length(),
                None => 0,
            },
            PatternKind::MatchAs { pattern, name } => {
                (match pattern {
                    Some(pattern) => pattern.get_token_length(),
                    None => 0,
                }) + match name {
                    Some(name) => name.get_token_length(),
                    None => 0,
                }
            }
            PatternKind::MatchOr { patterns } => patterns.get_token_length(),
        }
    }
}

impl TokenLength for MatchCase {
    fn get_token_length(&self) -> usize {
        "case".len()
            + self.pattern.get_token_length()
            + match &self.guard {
                Some(guard) => guard.get_token_length(),
                None => 0,
            }
            + self.body.get_token_length()
    }
}

impl TokenLength for ExcepthandlerKind {
    fn get_token_length(&self) -> usize {
        match &self {
            ExcepthandlerKind::ExceptHandler { type_, name, body } => {
                (match &type_ {
                    Some(type_) => type_.get_token_length(),
                    None => 0,
                }) + match &name {
                    Some(name) => "as".len() + name.get_token_length(),
                    None => 0,
                } + body.get_token_length()
            }
        }
    }
}

impl TokenLength for AliasData {
    fn get_token_length(&self) -> usize {
        self.name.get_token_length()
            + match &self.asname {
                Some(asname) => "as".len() + asname.get_token_length(),
                None => 0,
            }
    }
}
