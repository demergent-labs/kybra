mod record_members;

use cdk_framework::act::node::candid;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CollectResults, py_ast::PyAst, source_map::SourceMapped, Error};

impl PyAst {
    pub fn build_records(&self) -> Result<Vec<candid::Record>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_record())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn as_record(&self) -> Result<Option<candid::Record>, Vec<Error>> {
        match self.get_child_class_of("Record") {
            Some(record) => {
                let members = record
                    .body
                    .iter()
                    .map(|stmt| SourceMapped::new(stmt, self.source_map.clone()).as_record_member())
                    .collect_results()?
                    .into_iter()
                    .filter_map(|member_option| member_option)
                    .collect();
                Ok(Some(candid::Record {
                    name: Some(record.name.clone()),
                    members,
                    type_params: vec![].into(),
                }))
            }
            None => Ok(None),
        }
    }
}
