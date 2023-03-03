pub mod errors;
pub mod returns;

use cdk_framework::act::node::param::Param;
use rustpython_parser::ast::{ArgData, Arguments, Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};

pub enum InternalOrExternal {
    Internal,
    External,
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn build_params(
        &self,
        internal_or_external: InternalOrExternal,
    ) -> KybraResult<Vec<Param>> {
        match &self.node {
            StmtKind::FunctionDef { args, .. } => {
                SourceMapped::new(args.as_ref(), self.source_map.clone())
                    .to_param_list(internal_or_external)
            }
            _ => panic!("Unreachable"),
        }
    }
}

impl SourceMapped<&Arguments> {
    pub fn to_param_list(
        &self,
        internal_or_external: InternalOrExternal,
    ) -> KybraResult<Vec<Param>> {
        if self.kwarg.is_some() {
            todo!(
                "{}",
                "the dictionary unpacking operator (**) is not supported".to_string()
            );
        }

        if self.vararg.is_some() {
            todo!(
                "{}",
                "the iterator unpacking operator (*) is not supported".to_string()
            );
        }

        let param_results = match internal_or_external {
            InternalOrExternal::Internal => {
                if self.args[0].node.arg == "self".to_string() {
                    todo!("{}", "first parameter must not be \"self\"".to_string());
                }
                self.args
                    .iter()
                    .map(|arg| SourceMapped::new(arg, self.source_map.clone()).to_param())
                    .collect()
            }
            InternalOrExternal::External => {
                if self.args.len() == 0 {
                    todo!(
                        "{}",
                        "method must be an instance method. Add \"self\" as the first parameter"
                            .to_string(),
                    );
                }

                if self.args[0].node.arg != "self".to_string() {
                    todo!("{}", "first parameter must be \"self\"".to_string());
                }
                self.args[1..]
                    .iter()
                    .map(|arg| SourceMapped::new(arg, self.source_map.clone()).to_param())
                    .collect()
            }
        };

        // Ignore the first param, which is always "self"
        Ok(crate::errors::collect_kybra_results(param_results)?)
    }
}

impl SourceMapped<&Located<ArgData>> {
    pub fn to_param(&self) -> KybraResult<Param> {
        let param_name = self.node.arg.clone();
        match &self.node.annotation {
            Some(annotation) => {
                let data_type = SourceMapped::new(
                    annotation.as_ref(),
                    self.source_map.clone(),
                )
                .to_data_type()?;

                Ok(Param {
                    name: param_name,
                    type_: data_type,
                })
            }
            None => todo!("{}", format!("parameter \"{}\" is missing a type annotation. All method parameters must specify their type.", param_name)),
        }
    }
}
