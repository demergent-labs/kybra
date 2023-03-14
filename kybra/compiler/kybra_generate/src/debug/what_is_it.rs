use std::ops::Deref;

use rustpython_parser::ast::{ArgData, ExprContext, ExprKind, Located, StmtKind};

use crate::{debug::ToDisplayString, source_map::SourceMapped};

pub trait WhatIsIt {
    fn what_is_it(&self) -> ();
}

impl<T> WhatIsIt for SourceMapped<T>
where
    T: WhatIsIt,
{
    fn what_is_it(&self) -> () {
        self.deref().what_is_it()
    }
}

impl WhatIsIt for ExprContext {
    fn what_is_it(&self) -> () {
        let context = match &self {
            ExprContext::Load => "load",
            ExprContext::Store => "store",
            ExprContext::Del => "del",
        };
        eprintln!("The Expr Context is {}", context)
    }
}

impl WhatIsIt for StmtKind {
    fn what_is_it(&self) -> () {
        match &self {
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

impl WhatIsIt for ArgData {
    fn what_is_it(&self) -> () {
        eprintln!("--------------------------------------");
        eprintln!("This is an arg data");
        let annotation = match &self.annotation {
            Some(annotation) => annotation.to_display_string(),
            None => "None".to_string(),
        };
        let type_comment = match &self.type_comment {
            Some(type_comment) => type_comment.clone(),
            None => "None".to_string(),
        };
        eprintln!(
            "arg({}), annotation({}), type_comment({})",
            self.arg, annotation, type_comment
        );
        eprintln!("--------------------------------------");
    }
}

impl<T> WhatIsIt for Located<T>
where
    T: WhatIsIt,
{
    fn what_is_it(&self) -> () {
        self.node.what_is_it();
        eprintln!(
            "It's located at {}:{}",
            self.location.row(),
            self.location.column()
        );
    }
}

impl WhatIsIt for ExprKind {
    fn what_is_it(&self) -> () {
        eprintln!("This Expr Kind is a {}.", self.to_display_string(),);
    }
}
