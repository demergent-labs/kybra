use cdk_framework::{
    act::node::{
        candid::{self, Primitive},
        node_parts::mode::Mode,
        CandidType, ReturnType,
    },
    traits::CollectResults,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::CollectResults as CrateCollectResults, py_ast::PyAst, source_map::SourceMapped, Error,
};

use self::errors::{FuncCallTakesOneArg, FuncFormatting, ReturnTypeMode};

pub mod errors;
mod rust;

impl PyAst {
    pub fn build_funcs(&self) -> Result<Vec<candid::Func>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_func())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

struct Func<'a> {
    mode: &'a String,
    params: &'a Vec<Located<ExprKind>>,
    returns: &'a Located<ExprKind>,
}

impl SourceMapped<&Located<ExprKind>> {
    fn get_func_arg(&self) -> Result<Option<&Located<ExprKind>>, Error> {
        if let ExprKind::Call { func, args, .. } = &self.node {
            if let ExprKind::Name { id, .. } = &func.node {
                if id == "Func" {
                    if args.len() != 1 {
                        return Err(FuncCallTakesOneArg::err_from_expr(self, args.len()).into());
                    }
                    return Ok(Some(&args[0]));
                }
            }
        }
        Ok(None)
    }

    fn get_func(&self) -> Result<Option<Func>, Error> {
        match self.get_func_arg()? {
            Some(thing) => match &thing.node {
                ExprKind::Subscript { value, slice, .. } => {
                    let mode = match &value.node {
                        ExprKind::Name { id, .. } => id,
                        _ => return Err(ReturnTypeMode::err_from_expr(self)),
                    };
                    let (params, returns) = if let ExprKind::Tuple { elts, .. } = &slice.node {
                        if elts.len() != 2 {
                            return Err(FuncFormatting::err_from_expr(self));
                        }
                        let params = match &elts[0].node {
                            ExprKind::List { elts, .. } => elts,
                            _ => return Err(FuncFormatting::err_from_expr(self)),
                        };
                        (params, &elts[1])
                    } else {
                        return Err(FuncFormatting::err_from_expr(self));
                    };

                    Ok(Some(Func {
                        mode,
                        params,
                        returns,
                    }))
                }
                _ => Err(ReturnTypeMode::err_from_expr(self)),
            },
            None => Ok(None),
        }
    }

    fn as_func(&self, name: Option<String>) -> Result<Option<candid::Func>, Vec<Error>> {
        match self.get_func().map_err(Into::<Vec<Error>>::into)? {
            Some(func) => {
                let mode = match func.mode.as_str() {
                    "Oneway" => Mode::Oneway,
                    "Update" => Mode::Update,
                    "Query" => Mode::Query,
                    _ => return Err(ReturnTypeMode::err_from_expr(self).into()),
                };
                let (params, return_type) = (
                    func.params
                        .into_iter()
                        .map(|param| {
                            SourceMapped::new(param, self.source_map.clone()).to_candid_type()
                        })
                        .collect(),
                    match mode {
                        Mode::Query => Ok(CandidType::Primitive(Primitive::Void)),
                        _ => SourceMapped::new(func.returns, self.source_map.clone())
                            .to_candid_type(),
                    },
                )
                    .collect_results()?;
                Ok(Some(candid::Func {
                    to_vm_value: |name: String| rust::generate_func_to_vm_value(&name),
                    list_to_vm_value: |name: String| rust::generate_func_list_to_vm_value(&name),
                    from_vm_value: |name: String| rust::generate_func_from_vm_value(&name),
                    list_from_vm_value: |name: String| {
                        rust::generate_func_list_from_vm_value(&name)
                    },
                    name,
                    params,
                    return_type: Box::from(ReturnType::new(return_type)),
                    mode,
                }))
            }
            None => Ok(None),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_func(&self) -> Result<Option<candid::Func>, Vec<Error>> {
        match &self.node {
            StmtKind::Assign { value, .. }
            | StmtKind::AnnAssign {
                value: Some(value), ..
            } => {
                let name = self.get_name_or_err()?;
                Ok(SourceMapped::new(value.as_ref(), self.source_map.clone())
                    .as_func(Some(name))?)
            }
            _ => Ok(None),
        }
    }
}
