use rustpython_parser::ast::{
    ArgData, Arguments, Constant, ExprKind, KeywordData, Located, StmtKind,
};

pub trait ToDisplayString {
    fn to_display_string(&self) -> String;
}

impl ToDisplayString for Arguments {
    fn to_display_string(&self) -> String {
        let pos_only_args: Vec<String> = self
            .posonlyargs
            .iter()
            .map(|arg| arg.to_display_string())
            .collect();
        let args: Vec<String> = self
            .args
            .iter()
            .map(|arg| arg.to_display_string())
            .collect();
        let var_arg = match &self.vararg {
            Some(var_arg) => var_arg.to_display_string(),
            None => "None".to_string(),
        };
        let kw_only_args: Vec<String> = self
            .kwonlyargs
            .iter()
            .map(|arg| arg.to_display_string())
            .collect();
        let kw_defaults: Vec<String> = self
            .kw_defaults
            .iter()
            .map(|arg| arg.to_display_string())
            .collect();
        let kw_arg = match &self.kwarg {
            Some(var_arg) => var_arg.to_display_string(),
            None => "None".to_string(),
        };
        let defaults = self.defaults.iter().map(|arg| arg.to_display_string());
        format!(
            "Arguments: pos_only_args({:?}) args({:?}) var_arg({var_arg}) kw_only_args({:?}) kw_defaults({:?}) kw_arg({kw_arg}) defaults({:?})",
            pos_only_args, args, kw_only_args, kw_defaults, defaults
        )
    }
}

impl ToDisplayString for ArgData {
    fn to_display_string(&self) -> String {
        let type_annotation = match &self.annotation {
            Some(annotation) => annotation.to_display_string(),
            None => "None".to_string(),
        };
        let type_comment = match &self.type_comment {
            Some(type_comment) => type_comment.clone(),
            None => "None".to_string(),
        };
        format!(
            "ArgData: arg({}) type_annotation({}) type_comment({})",
            self.arg, type_annotation, type_comment,
        )
    }
}

impl ToDisplayString for StmtKind {
    fn to_display_string(&self) -> String {
        let stmt_kind = match &self {
            StmtKind::FunctionDef { .. } => "function def".to_string(),
            StmtKind::AsyncFunctionDef { .. } => "async function def".to_string(),
            StmtKind::ClassDef { .. } => "class def".to_string(),
            StmtKind::Return { .. } => "return".to_string(),
            StmtKind::Delete { .. } => "delete".to_string(),
            StmtKind::Assign { .. } => "assign".to_string(),
            StmtKind::AugAssign { .. } => "aug assign".to_string(),
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                simple,
            } => format!(
                "target({}), annotation({}), value({}), simple({})",
                target.to_display_string(),
                annotation.to_display_string(),
                match value {
                    Some(value) => value.to_display_string(),
                    None => "None".to_string(),
                },
                simple
            ),
            StmtKind::For { .. } => "for".to_string(),
            StmtKind::AsyncFor { .. } => "async for".to_string(),
            StmtKind::While { .. } => "while".to_string(),
            StmtKind::If { .. } => "if".to_string(),
            StmtKind::With { .. } => "with".to_string(),
            StmtKind::AsyncWith { .. } => "async with".to_string(),
            StmtKind::Match { .. } => "match".to_string(),
            StmtKind::Raise { .. } => "raise".to_string(),
            StmtKind::Try { .. } => "try".to_string(),
            StmtKind::Assert { .. } => "assert".to_string(),
            StmtKind::Import { .. } => "import".to_string(),
            StmtKind::ImportFrom { .. } => "import from".to_string(),
            StmtKind::Global { .. } => "global".to_string(),
            StmtKind::Nonlocal { .. } => "nonlocal".to_string(),
            StmtKind::Expr { .. } => "expr".to_string(),
            StmtKind::Pass => "pass".to_string(),
            StmtKind::Break => "break".to_string(),
            StmtKind::Continue => "continue".to_string(),
        };
        format!("Located<StmtKind>: {}", stmt_kind)
    }
}

impl ToDisplayString for Constant {
    fn to_display_string(&self) -> String {
        format!(
            "Constant: {}",
            match self {
                Constant::None => "None",
                Constant::Bool(_) => "bool",
                Constant::Str(_) => "str",
                Constant::Bytes(_) => "bytes",
                Constant::Int(_) => "int",
                Constant::Tuple(_) => "tuple",
                Constant::Float(_) => "float",
                Constant::Complex { .. } => "complex",
                Constant::Ellipsis => "ellipsis",
            },
        )
    }
}

impl ToDisplayString for KeywordData {
    fn to_display_string(&self) -> String {
        let arg = match &self.arg {
            Some(arg) => arg.clone(),
            None => "no args".to_string(),
        };
        let value = self.value.to_display_string();
        format!("arg: {}, value: {}", arg, value)
    }
}

impl<T> ToDisplayString for Located<T>
where
    T: ToDisplayString,
{
    fn to_display_string(&self) -> String {
        self.node.to_display_string()
    }
}

impl ToDisplayString for ExprKind {
    fn to_display_string(&self) -> String {
        let expr_kind = match &self {
            ExprKind::BoolOp { .. } => "bool op".to_string(),
            ExprKind::NamedExpr { .. } => "named expr".to_string(),
            ExprKind::BinOp { .. } => "bin op".to_string(),
            ExprKind::UnaryOp { .. } => "unary op".to_string(),
            ExprKind::Lambda { .. } => "lambda".to_string(),
            ExprKind::IfExp { .. } => "if exp".to_string(),
            ExprKind::Dict { .. } => "dict".to_string(),
            ExprKind::Set { .. } => "set".to_string(),
            ExprKind::ListComp { .. } => "list comp".to_string(),
            ExprKind::SetComp { .. } => "set comp".to_string(),
            ExprKind::DictComp { .. } => "dict comp".to_string(),
            ExprKind::GeneratorExp { .. } => "generator exp".to_string(),
            ExprKind::Await { .. } => "await".to_string(),
            ExprKind::Yield { .. } => "yield".to_string(),
            ExprKind::YieldFrom { .. } => "yield from".to_string(),
            ExprKind::Compare { .. } => "compare".to_string(),
            ExprKind::Call {
                func,
                args,
                keywords,
            } => {
                let func = func.to_display_string();
                let args: Vec<String> = args.iter().map(|arg| arg.to_display_string()).collect();
                let keywords: Vec<String> =
                    keywords.iter().map(|arg| arg.to_display_string()).collect();
                format!(
                    "call: func({}), args({:?}), keywords({:?})",
                    func, args, keywords
                )
            }
            ExprKind::FormattedValue { .. } => "formatted value".to_string(),
            ExprKind::JoinedStr { .. } => "joined str".to_string(),
            ExprKind::Constant { value, kind } => {
                let value = value.to_display_string();
                let kind = match kind {
                    Some(kind) => kind,
                    None => "None",
                };
                format!("constant: value({}) kind({})", value, kind)
            }
            ExprKind::Attribute { .. } => "attribute".to_string(),
            ExprKind::Subscript { value, slice, .. } => {
                format!(
                    "subscript: value({}), slice({})",
                    value.to_display_string(),
                    slice.to_display_string()
                )
            }
            ExprKind::Starred { .. } => "starred".to_string(),
            ExprKind::Name { id, .. } => format!("name: {}", id),
            ExprKind::List { .. } => "list".to_string(),
            ExprKind::Tuple { elts, .. } => {
                let types: Vec<String> = elts.iter().map(|elt| elt.to_display_string()).collect();
                format!("tuple: {:?}", types)
            }
            ExprKind::Slice { .. } => "slice".to_string(),
        };
        format!("Located<ExprKind>: {}", expr_kind)
    }
}
