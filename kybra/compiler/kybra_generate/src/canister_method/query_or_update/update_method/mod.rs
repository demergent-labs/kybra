use cdk_framework::act::node::canister_method::{CanisterMethodType, UpdateMethod};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

impl PyAst {
    pub fn build_update_methods(&self) -> KybraResult<Vec<UpdateMethod>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_update_method())
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_update_method(&self) -> KybraResult<Option<UpdateMethod>> {
        if !self.is_canister_method_type(CanisterMethodType::Update) {
            return Ok(None);
        }
        Ok(Some(UpdateMethod {
            definition: self.as_query_or_update_definition()?,
        }))
    }
}
