pub mod errors;
pub mod returns;

use cdk_framework::act::node::param::Param;
use rustpython_parser::ast::{ArgData, Located, StmtKind};

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
                if args.kwarg.is_some() {
                    return Err(self.dictionary_unpacking_operator_not_supported_error());
                }

                if args.vararg.is_some() {
                    return Err(self.iterator_unpacking_operator_not_supported_error());
                }

                let param_results = match internal_or_external {
                    InternalOrExternal::Internal => {
                        if args.args.len() > 0 && args.args[0].node.arg == "self".to_string() {
                            return Err(self.first_parameter_must_not_be_self_error());
                        }
                        args.args
                            .iter()
                            .map(|arg| SourceMapped::new(arg, self.source_map.clone()).to_param())
                            .collect()
                    }
                    InternalOrExternal::External => {
                        if args.args.len() == 0 {
                            return Err(self.first_parameter_must_be_self_error());
                        }

                        if args.args[0].node.arg != "self".to_string() {
                            return Err(self.first_parameter_must_be_self_error());
                        }

                        // Ignore the first param, which is always "self"
                        args.args[1..]
                            .iter()
                            .map(|arg| SourceMapped::new(arg, self.source_map.clone()).to_param())
                            .collect()
                    }
                };

                Ok(crate::errors::collect_kybra_results(param_results)?)
            }
            _ => Err(crate::errors::unreachable()),
        }
    }
}

impl SourceMapped<&Located<ArgData>> {
    pub fn to_param(&self) -> KybraResult<Param> {
        let param_name = self.node.arg.clone();
        match &self.node.annotation {
            Some(annotation) => {
                let candid_type = SourceMapped::new(annotation.as_ref(), self.source_map.clone())
                    .to_data_type()?;

                Ok(Param {
                    name: param_name,
                    candid_type,
                })
            }
            None => Err(self.param_type_annotation_required_error()),
        }
    }
}
