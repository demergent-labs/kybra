use regex::Regex;
use rustpython_parser::ast::{
    AliasData, ArgData, Arguments, Boolop, Cmpop, Comprehension, Constant, ExcepthandlerKind,
    ExprContext, ExprKind, KeywordData, Located, MatchCase, Mod, Operator, PatternKind, StmtKind,
    TypeIgnore, Unaryop, Withitem,
};

pub trait TokenLength {
    fn get_token_length(&self) -> usize;
}

impl TokenLength for Mod {
    fn get_token_length(&self) -> usize {
        match &self {
            Mod::Module { body, type_ignores } => {
                body.get_token_length() + type_ignores.get_token_length()
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
        match &self {
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
                // + decorator_list.get_token_length() // See the decorator list from FunctionDefs
                eprintln!("The length of a class is {}", result);
                result
            }
            StmtKind::Return { value } => match value {
                Some(returns) => "return".len() + returns.get_token_length(),
                None => 0,
            },
            StmtKind::Delete { targets: _ } => {
                todo!();
                // targets.get_token_length()
            }
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
            StmtKind::AugAssign {
                target: _,
                op: _,
                value: _,
            } => {
                todo!();
                // target.get_token_length() + op.get_token_length() + value.get_token_length()
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
                target: _,
                iter: _,
                body: _,
                orelse: _,
                type_comment: _,
            } => {
                todo!();
                // target.get_token_length()
                //     + iter.get_token_length()
                //     + body.get_token_length()
                //     + orelse.get_token_length()
                //     + match type_comment {
                //         Some(type_comment) => type_comment.get_token_length(),
                //         None => 0,
                //     }
            }
            StmtKind::AsyncFor {
                target: _,
                iter: _,
                body: _,
                orelse: _,
                type_comment: _,
            } => {
                todo!();
                // target.get_token_length()
                //     + iter.get_token_length()
                //     + body.get_token_length()
                //     + orelse.get_token_length()
                //     + match type_comment {
                //         Some(type_comment) => type_comment.get_token_length(),
                //         None => 0,
                //     }
            }
            StmtKind::While {
                test: _,
                body: _,
                orelse: _,
            } => {
                todo!();
                // test.get_token_length() + body.get_token_length() + orelse.get_token_length()
            }
            StmtKind::If {
                test: _,
                body: _,
                orelse: _,
            } => {
                todo!();
                // test.get_token_length() + body.get_token_length() + orelse.get_token_length()
            }
            StmtKind::With {
                items: _,
                body: _,
                type_comment: _,
            } => {
                todo!();
                // items.get_token_length()
                //     + body.get_token_length()
                //     + match type_comment {
                //         Some(type_comment) => type_comment.get_token_length(),
                //         None => 0,
                //     }
            }
            StmtKind::AsyncWith {
                items: _,
                body: _,
                type_comment: _,
            } => {
                todo!();
                // items.get_token_length()
                //     + body.get_token_length()
                //     + match type_comment {
                //         Some(type_comment) => type_comment.get_token_length(),
                //         None => 0,
                //     }
            }
            StmtKind::Match {
                subject: _,
                cases: _,
            } => {
                todo!();
                // subject.get_token_length() + cases.get_token_length()
            }
            StmtKind::Raise { exc: _, cause: _ } => {
                todo!();
                // (match exc {
                //     Some(exc) => exc.get_token_length(),
                //     None => 0,
                // }) + match cause {
                //     Some(cause) => cause.get_token_length(),
                //     None => 0,
                // }
            }
            StmtKind::Try {
                body: _,
                handlers: _,
                orelse: _,
                finalbody: _,
            } => {
                todo!();
                // body.get_token_length()
                //     + handlers.get_token_length()
                //     + orelse.get_token_length()
                //     + finalbody.get_token_length()
            }
            StmtKind::Assert { test: _, msg: _ } => {
                todo!();
                // test.get_token_length()
                //     + match msg {
                //         Some(msg) => msg.get_token_length(),
                //         None => 0,
                //     }
            }
            StmtKind::Import { names } => "import".len() + names.get_token_length(),
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
            StmtKind::Global { names: _ } => {
                todo!();
                // names.get_token_length()
            }
            StmtKind::Nonlocal { names: _ } => {
                todo!();
                // names.get_token_length()
            }
            StmtKind::Expr { value } => value.get_token_length(),
            StmtKind::Pass => {
                todo!();
                // "pass".len()
            }
            StmtKind::Break => "break".len(),
            StmtKind::Continue => {
                todo!();
                // "continue".len()
            }
        }
    }
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
        match self {
            Unaryop::Invert => todo!("Invert"),
            Unaryop::Not => "not",
            Unaryop::UAdd => todo!("UAdd"),
            Unaryop::USub => todo!("USub"),
        }
        .len()
    }
}

impl TokenLength for Comprehension {
    fn get_token_length(&self) -> usize {
        todo!();
        // self.target.get_token_length()
        //     + self.iter.get_token_length()
        //     + self.ifs.get_token_length()
        //     + self.is_async.get_token_length()
    }
}

// TODO this one is a little weird because the isnot and notin could have any
// number of space in the middle of the operator. However I think we are
// unlikely to runinto those operators in kybra right?
impl TokenLength for Cmpop {
    fn get_token_length(&self) -> usize {
        // match self {
        //     Cmpop::Eq => "==",
        //     Cmpop::NotEq => "!=",
        //     Cmpop::Lt => "<",
        //     Cmpop::LtE => "<=",
        //     Cmpop::Gt => ">",
        //     Cmpop::GtE => ">=",
        //     Cmpop::Is => "is",
        //     Cmpop::IsNot => "isnot",
        //     Cmpop::In => "in",
        //     Cmpop::NotIn => "notin",
        // }
        // .len();
        todo!("I think that most of these will be sorted out but maybe not the isnot in and not in")
    }
}

impl TokenLength for Constant {
    fn get_token_length(&self) -> usize {
        match self {
            Constant::None => "None".len(),
            Constant::Bool(bool) => {
                if *bool {
                    "True".len()
                } else {
                    "False".len()
                }
            }
            Constant::Str(string) => {
                let re = Regex::new(r"\s+").unwrap();
                re.replace_all(&string, "").len()
            }
            Constant::Bytes(_bytes) => todo!(),
            Constant::Int(int) => int.to_string().len(),
            Constant::Tuple(_tuple) => todo!(),
            Constant::Float(_float) => todo!(),
            Constant::Complex { real: _, imag: _ } => todo!(),
            Constant::Ellipsis => "...".len(),
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
        match &self {
            ExprKind::BoolOp { op, values } => op.get_token_length() + values.get_token_length(),
            ExprKind::NamedExpr { target, value } => {
                target.get_token_length() + value.get_token_length()
            }
            ExprKind::BinOp { left, op, right } => {
                left.get_token_length() + op.get_token_length() + right.get_token_length()
            }
            ExprKind::UnaryOp { op, operand } => op.get_token_length() + operand.get_token_length(),
            ExprKind::Lambda { args, body } => args.get_token_length() + body.get_token_length(),
            ExprKind::IfExp { test, body, orelse } => {
                test.get_token_length() + body.get_token_length() + orelse.get_token_length()
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
            ExprKind::Await { value } => value.get_token_length(),
            ExprKind::Yield { value } => match &value {
                Some(value) => value.get_token_length(),
                None => 0,
            },
            ExprKind::YieldFrom { value } => value.get_token_length(),
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => left.get_token_length() + ops.get_token_length() + comparators.get_token_length(),
            ExprKind::Call {
                func,
                args,
                keywords,
            } => func.get_token_length() + args.get_token_length() + keywords.get_token_length(),
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
        }
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
        match self {
            TypeIgnore::TypeIgnore { lineno, tag } => {
                lineno.get_token_length() + tag.get_token_length()
            }
        }
    }
}

impl TokenLength for String {
    fn get_token_length(&self) -> usize {
        let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
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
            + match &self.type_comment {
                Some(type_comment) => type_comment.get_token_length(),
                None => 0,
            }
    }
}

impl TokenLength for Operator {
    fn get_token_length(&self) -> usize {
        match &self {
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
        }
        .len()
            * 0 // We are removing special characters so all of these will eventually be 0 length
    }
}

impl TokenLength for Withitem {
    fn get_token_length(&self) -> usize {
        self.context_expr.get_token_length();
        match &self.optional_vars {
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
        self.pattern.get_token_length()
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
                    Some(name) => name.get_token_length(),
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
                Some(asname) => asname.get_token_length(),
                None => 0,
            }
    }
}
