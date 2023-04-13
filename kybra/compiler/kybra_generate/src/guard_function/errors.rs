use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub fn guard_functions_param_error(&self) -> Error {
        Error::GuardFunctionParam(self.create_error_message(
            "Guards functions can't have parameters",
            "",
            None,
        ))
    }

    pub fn guard_function_return_error(&self) -> Error {
        Error::GuardFunctionReturn(self.create_error_message(
            "Guard functions must return GuardResult",
            "",
            None,
        ))
    }
}
