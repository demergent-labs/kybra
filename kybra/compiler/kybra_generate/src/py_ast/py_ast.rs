use rustpython_parser::{
    ast::Mod,
    parser::{self, Mode},
};

use super::Error;
use crate::source_map::{SourceMap, SourceMapped};

pub struct PyAst {
    pub source_mapped_mods: Vec<SourceMapped<Mod>>,
    pub entry_module_name: String,
}

impl PyAst {
    pub fn new(py_file_names: &Vec<&str>, entry_module_name: &str) -> Result<PyAst, Vec<Error>> {
        // TODO: Use collect_results from CDK Framework instead once
        // https://github.com/demergent-labs/cdk_framework/pull/75 is merged.
        let (source_mapped_mods, errors) = py_file_names
            .iter()
            .map(|py_file_name| -> Result<SourceMapped<Mod>, Error> {
                let source = std::fs::read_to_string(py_file_name)?;

                let module = parser::parse(&source, Mode::Module, "")?;
                Ok(SourceMapped::new(
                    module,
                    SourceMap::new(source, py_file_name),
                ))
            })
            .fold((vec![], vec![]), |mut acc, result| {
                match result {
                    Ok(source_mapped_mod) => acc.0.push(source_mapped_mod),
                    Err(err) => acc.1.push(err),
                }
                acc
            });

        if errors.is_empty() {
            return Ok(PyAst {
                source_mapped_mods,
                entry_module_name: entry_module_name.to_string(),
            });
        }

        Err(errors)
    }
}
