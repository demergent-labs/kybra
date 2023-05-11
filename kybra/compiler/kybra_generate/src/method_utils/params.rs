use cdk_framework::{act::node::Param, traits::CollectResults};
use rustpython_parser::ast::{ArgData, Located, StmtKind};

use crate::{
    errors::CollectResults as OtherCollectResults, kybra_unreachable, source_map::SourceMapped,
    Error,
};

use super::errors::{
    DictionaryUnpackingOperatorNotSupported, FirstParamMustBeSelf, FirstParamMustNotBeSelf,
    IteratorUnpackingOperatorNotSupported, ParamTypeAnnotationRequired, TooManyParams,
};

#[derive(Debug, Clone)]
pub enum InternalOrExternal {
    Internal,
    External,
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn build_params(
        &self,
        internal_or_external: InternalOrExternal,
    ) -> Result<Vec<Param>, Vec<Error>> {
        match &self.node {
            StmtKind::FunctionDef { args, .. } => {
                (
                    match args.kwarg {
                        None => Ok(()),
                        _ => {
                            Err(DictionaryUnpackingOperatorNotSupported::err_from_stmt(self).into())
                        }
                    },
                    match args.vararg {
                        None => Ok(()),
                        _ => Err(IteratorUnpackingOperatorNotSupported::err_from_stmt(self).into()),
                    },
                )
                    .collect_results()?;

                let param_results: Vec<_> = match internal_or_external {
                    InternalOrExternal::Internal => {
                        if args.args.len() > 0 && args.args[0].node.arg == "self".to_string() {
                            return Err(FirstParamMustNotBeSelf::err_from_stmt(self).into());
                        }
                        if args.args.len() > 5 {
                            return Err(TooManyParams::err_from_stmt(
                                self,
                                InternalOrExternal::Internal,
                            )
                            .into());
                        }

                        args.args
                            .iter()
                            .map(|arg| SourceMapped::new(arg, self.source_map.clone()).to_param())
                            .collect()
                    }
                    InternalOrExternal::External => {
                        if args.args.len() == 0 {
                            return Err(FirstParamMustBeSelf::err_from_stmt(self).into());
                        }

                        if args.args[0].node.arg != "self".to_string() {
                            return Err(FirstParamMustBeSelf::err_from_stmt(self).into());
                        }

                        if args.args.len() > 6 {
                            return Err(TooManyParams::err_from_stmt(
                                self,
                                InternalOrExternal::External,
                            )
                            .into());
                        }

                        // Ignore the first param, which is always "self"
                        args.args[1..]
                            .iter()
                            .map(|arg| SourceMapped::new(arg, self.source_map.clone()).to_param())
                            .collect()
                    }
                };

                param_results.into_iter().collect_results()
            }
            _ => kybra_unreachable!(),
        }
    }
}

impl SourceMapped<&Located<ArgData>> {
    pub fn to_param(&self) -> Result<Param, Vec<Error>> {
        let param_name = self.node.arg.clone();
        match &self.node.annotation {
            Some(annotation) => {
                let candid_type = SourceMapped::new(annotation.as_ref(), self.source_map.clone())
                    .to_candid_type()?;

                Ok(Param {
                    name: param_name,
                    candid_type,
                })
            }
            None => Err(ParamTypeAnnotationRequired::err_from_arg_data(self).into()),
        }
    }
}
