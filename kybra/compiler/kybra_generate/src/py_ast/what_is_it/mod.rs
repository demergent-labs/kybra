use rustpython_parser::ast::{
    ArgData, Arguments, Constant, ExprKind, KeywordData, Located, StmtKind,
};

use super::kybra_types::KybraStmt;

pub trait WhatIsIt {
    fn what_is_it(&self) -> ();
}

pub trait ToDisplayString {
    fn to_display_string(&self) -> String;
}

impl WhatIsIt for KybraStmt<'_> {
    fn what_is_it(&self) -> () {
        self.stmt_kind.what_is_it();
    }
}

impl WhatIsIt for Located<StmtKind> {
    fn what_is_it(&self) -> () {
        match &self.node {
            StmtKind::ClassDef {
                name,
                bases,
                keywords,
                body,
                decorator_list,
            } => {
                eprintln!("--------------------------------------------");
                eprintln!("This is a Class Def");
                eprintln!("This is the name {}", name);
                let base_strings: Vec<String> =
                    bases.iter().map(|base| base.to_display_string()).collect();
                let keyword_string: Vec<String> = keywords
                    .iter()
                    .map(|keyword| keyword.to_display_string())
                    .collect();
                let decorator_string: Vec<String> = decorator_list
                    .iter()
                    .map(|decorator| decorator.to_display_string())
                    .collect();
                let body_strings: Vec<String> =
                    body.iter().map(|stmt| stmt.to_display_string()).collect();
                eprintln!("These are the bases {:?}", base_strings);
                eprintln!("These are the keywords {:?}", keyword_string);
                eprintln!("These are the decorators {:?}", decorator_string);
                eprintln!("These are the body {:?}", body_strings);
                eprintln!("--------------------------------------------");
            }
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                simple,
            } => {
                eprintln!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
                eprintln!("This is a AnnAssign");
                eprintln!("The target is: {}", target.to_display_string());
                eprintln!("The annotation is: {}", annotation.to_display_string());
                let value = match value {
                    Some(value) => value.to_display_string(),
                    None => "None".to_string(),
                };
                eprintln!("The value is: {}", value);
                eprintln!("The simple is: {}", simple);
                eprintln!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            }
            StmtKind::FunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
            } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a function def");
                eprintln!("The name is: {}", name);
                eprintln!("The args are: {:?}", args.to_display_string());
                let body_strings: Vec<String> =
                    body.iter().map(|stmt| stmt.to_display_string()).collect();
                eprintln!("The body is: {:?}", body_strings);
                let decorator_string: Vec<String> = decorator_list
                    .iter()
                    .map(|decorator| decorator.to_display_string())
                    .collect();
                eprintln!("The decorators are: {:?}", decorator_string);
                eprintln!(
                    "The returns are: {:?}",
                    match returns {
                        Some(returns) => returns.to_display_string(),
                        None => "None".to_string(),
                    }
                );
                eprintln!("The type_comment is: {:?}", type_comment);
                eprintln!("--------------------------------------");
            }
            StmtKind::AsyncFunctionDef { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a async function def");
                eprintln!("--------------------------------------");
            }
            StmtKind::Return { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a return");
                eprintln!("--------------------------------------");
            }
            StmtKind::Delete { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a delete");
                eprintln!("--------------------------------------");
            }
            StmtKind::Assign {
                targets,
                value,
                type_comment,
            } => {
                eprintln!("--------------------------------------");
                eprintln!("This is an assign");
                let targets: Vec<String> = targets
                    .iter()
                    .map(|target| target.to_display_string())
                    .collect();
                eprintln!("The target are {:?}", targets);
                eprintln!("The value is {}", value.to_display_string());
                let type_comment = match type_comment {
                    Some(type_comment) => type_comment,
                    None => "None",
                };
                eprintln!("The type_comment is {}", type_comment);
                eprintln!("--------------------------------------");
            }
            StmtKind::AugAssign { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is an aug assign");
                eprintln!("--------------------------------------");
            }
            StmtKind::For { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a for");
                eprintln!("--------------------------------------");
            }
            StmtKind::AsyncFor { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a async for");
                eprintln!("--------------------------------------");
            }
            StmtKind::While { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a while");
                eprintln!("--------------------------------------");
            }
            StmtKind::If { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is an if");
                eprintln!("--------------------------------------");
            }
            StmtKind::With { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a with");
                eprintln!("--------------------------------------");
            }
            StmtKind::AsyncWith { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is an async with");
                eprintln!("--------------------------------------");
            }
            StmtKind::Match { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a match");
                eprintln!("--------------------------------------");
            }
            StmtKind::Raise { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a raise");
                eprintln!("--------------------------------------");
            }
            StmtKind::Try { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a try");
                eprintln!("--------------------------------------");
            }
            StmtKind::Assert { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a assert");
                eprintln!("--------------------------------------");
            }
            StmtKind::Import { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is an import");
                eprintln!("--------------------------------------");
            }
            StmtKind::ImportFrom { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is an import from");
                eprintln!("--------------------------------------");
            }
            StmtKind::Global { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a global");
                eprintln!("--------------------------------------");
            }
            StmtKind::Nonlocal { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a nonlocal");
                eprintln!("--------------------------------------");
            }
            StmtKind::Expr { .. } => {
                eprintln!("--------------------------------------");
                eprintln!("This is a expr");
                eprintln!("--------------------------------------");
            }
            StmtKind::Pass => {
                eprintln!("--------------------------------------");
                eprintln!("This is a pass");
                eprintln!("--------------------------------------");
            }
            StmtKind::Break => {
                eprintln!("--------------------------------------");
                eprintln!("This is a break");
                eprintln!("--------------------------------------");
            }
            StmtKind::Continue => {
                eprintln!("--------------------------------------");
                eprintln!("This is a continue");
                eprintln!("--------------------------------------");
            }
        }
    }
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

impl ToDisplayString for Located<ArgData> {
    fn to_display_string(&self) -> String {
        let type_annotation = match &self.node.annotation {
            Some(annotation) => annotation.to_display_string(),
            None => "None".to_string(),
        };
        let type_comment = match &self.node.type_comment {
            Some(type_comment) => type_comment.clone(),
            None => "None".to_string(),
        };
        format!(
            "ArgData: arg({}) type_annotation({}) type_comment({})",
            self.node.arg, type_annotation, type_comment,
        )
    }
}

impl ToDisplayString for Located<StmtKind> {
    fn to_display_string(&self) -> String {
        let stmt_kind = match &self.node {
            StmtKind::FunctionDef { .. } => "function def".to_string(),
            StmtKind::AsyncFunctionDef { .. } => "asyc function def".to_string(),
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

impl WhatIsIt for Located<ArgData> {
    fn what_is_it(&self) -> () {
        eprintln!("--------------------------------------");
        eprintln!("This is an arg data");
        let annotation = match &self.node.annotation {
            Some(annotation) => annotation.to_display_string(),
            None => "None".to_string(),
        };
        let type_comment = match &self.node.type_comment {
            Some(type_comment) => type_comment.clone(),
            None => "None".to_string(),
        };
        eprintln!(
            "arg({}), annotation({}), type_comment({})",
            self.node.arg, annotation, type_comment
        );
        eprintln!("--------------------------------------");
    }
}

impl WhatIsIt for Located<ExprKind> {
    fn what_is_it(&self) -> () {
        eprintln!(
            "This Expr Kind is a {}. It's located at {}:{}",
            self.to_display_string(),
            self.location.row(),
            self.location.column()
        );
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

impl ToDisplayString for Located<KeywordData> {
    fn to_display_string(&self) -> String {
        let arg = match &self.node.arg {
            Some(arg) => arg.clone(),
            None => "no args".to_string(),
        };
        let value = self.node.value.to_display_string();
        format!("arg: {}, value: {}", arg, value)
    }
}

impl ToDisplayString for Located<ExprKind> {
    fn to_display_string(&self) -> String {
        let expr_kind = match &self.node {
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
