mod records;

use rustpython_parser::ast::{ExprKind, KeywordData, Located, StmtKind};

use crate::{cdk_act::ActDataType, WhatIsIt};

use super::KybraStmt;

impl KybraStmt<'_> {
    pub fn build_act_data_type(&self) -> ActDataType {
        match &self.stmt_kind.node {
            StmtKind::ClassDef { .. } => self.as_record(),
            StmtKind::FunctionDef { .. } => todo!(),
            StmtKind::AsyncFunctionDef { .. } => todo!(),
            StmtKind::Return { .. } => todo!(),
            StmtKind::Delete { .. } => todo!(),
            StmtKind::Assign { .. } => todo!(),
            StmtKind::AugAssign { .. } => todo!(),
            StmtKind::AnnAssign { .. } => todo!(),
            StmtKind::For { .. } => todo!(),
            StmtKind::AsyncFor { .. } => todo!(),
            StmtKind::While { .. } => todo!(),
            StmtKind::If { .. } => todo!(),
            StmtKind::With { .. } => todo!(),
            StmtKind::AsyncWith { .. } => todo!(),
            StmtKind::Match { .. } => todo!(),
            StmtKind::Raise { .. } => todo!(),
            StmtKind::Try { .. } => todo!(),
            StmtKind::Assert { .. } => todo!(),
            StmtKind::Import { .. } => todo!(),
            StmtKind::ImportFrom { .. } => todo!(),
            StmtKind::Global { .. } => todo!(),
            StmtKind::Nonlocal { .. } => todo!(),
            StmtKind::Expr { .. } => todo!(),
            StmtKind::Pass => todo!(),
            StmtKind::Break => todo!(),
            StmtKind::Continue => todo!(),
        }
    }
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
                let base_strings: Vec<String> = bases.iter().map(|base| base.to_cool()).collect();
                let keyword_string: Vec<String> =
                    keywords.iter().map(|keyword| keyword.to_cool()).collect();
                let decorator_string: Vec<String> = decorator_list
                    .iter()
                    .map(|decorator| decorator.to_cool())
                    .collect();
                let body_strings: Vec<String> = body.iter().map(|base| base.to_cool()).collect();
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
                eprintln!("The target is: {}", target.to_cool());
                eprintln!("The annotation is: {}", annotation.to_cool());
                let value = match value {
                    Some(value) => value.to_cool(),
                    None => "None".to_string(),
                };
                eprintln!("The value is: {}", value);
                eprintln!("The simple is: {}", simple);
                eprintln!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            }
            _ => (),
        }
    }
}

impl CoolBoy for Located<StmtKind> {
    fn to_cool(&self) -> String {
        let stmt_kind = match &self.node {
            StmtKind::FunctionDef { .. } => "function def".to_string(),
            StmtKind::AsyncFunctionDef { .. } => "asyc function def".to_string(),
            StmtKind::ClassDef { .. } => "class def".to_string(),
            StmtKind::Return { .. } => "return".to_string(),
            StmtKind::Delete { .. } => "delete".to_string(),
            StmtKind::Assign { .. } => "assign".to_string(),
            StmtKind::AugAssign { .. } => "aug assign".to_string(),
            StmtKind::AnnAssign { .. } => "ann assign".to_string(),
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

impl WhatIsIt for Located<ExprKind> {
    fn what_is_it(&self) -> () {
        let thing = match &self.node {
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
            ExprKind::Call { .. } => "call".to_string(),
            ExprKind::FormattedValue { .. } => "formatted value".to_string(),
            ExprKind::JoinedStr { .. } => "joined str".to_string(),
            ExprKind::Constant { .. } => "constant".to_string(),
            ExprKind::Attribute { .. } => "attribute".to_string(),
            ExprKind::Subscript { .. } => "subscript".to_string(),
            ExprKind::Starred { .. } => "starred".to_string(),
            ExprKind::Name { id, .. } => format!("name: {}", id),
            ExprKind::List { .. } => "list".to_string(),
            ExprKind::Tuple { .. } => "tuple".to_string(),
            ExprKind::Slice { .. } => "slice".to_string(),
        };
        eprintln!("This Expr Kind is a {}", thing);
    }
}

trait CoolBoy {
    fn to_cool(&self) -> String;
}

impl CoolBoy for Located<KeywordData> {
    fn to_cool(&self) -> String {
        let arg = match &self.node.arg {
            Some(arg) => arg.clone(),
            None => "no args".to_string(),
        };
        let value = self.node.value.to_cool();
        format!("arg: {}, value: {}", arg, value)
    }
}

impl CoolBoy for Located<ExprKind> {
    fn to_cool(&self) -> String {
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
            ExprKind::Call { .. } => "call".to_string(),
            ExprKind::FormattedValue { .. } => "formatted value".to_string(),
            ExprKind::JoinedStr { .. } => "joined str".to_string(),
            ExprKind::Constant { .. } => "constant".to_string(),
            ExprKind::Attribute { .. } => "attribute".to_string(),
            ExprKind::Subscript { .. } => "subscript".to_string(),
            ExprKind::Starred { .. } => "starred".to_string(),
            ExprKind::Name { id, .. } => format!("name: {}", id),
            ExprKind::List { .. } => "list".to_string(),
            ExprKind::Tuple { .. } => "tuple".to_string(),
            ExprKind::Slice { .. } => "slice".to_string(),
        };
        format!("Located<ExprKind>: {}", expr_kind)
    }
}
