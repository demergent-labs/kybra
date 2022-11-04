use cdk_framework::{nodes::ActFnParam, ToActDataType};
use rustpython_parser::ast::Arguments;

use crate::source_map::SourceMap;

use super::KybraExpr;

pub struct KybraArguments<'a> {
    pub arguments: &'a Arguments,
    pub source_map: &'a SourceMap,
}

impl KybraArguments<'_> {
    pub fn to_act_fn_params(&self) -> Result<Vec<ActFnParam>, String> {
        if self.arguments.kwarg.is_some() {
            return Err("the dictionary unpacking operator (**) is not supported".to_string());
        }

        if self.arguments.vararg.is_some() {
            return Err("the iterator unpacking operator (*) is not supported".to_string());
        }

        if self.arguments.args.len() == 0 {
            return Err(
                "method must be an instance method. Add \"self\" as the first parameter"
                    .to_string(),
            );
        }

        if self.arguments.args[0].node.arg != "self".to_string() {
            return Err("first parameter must be \"self\"".to_string());
        }

        // Ignore the first param, which is always "self"
        self.arguments.args[1..]
            .iter()
            .map(|arg| {
                let name = &arg.node.arg;

                match &arg.node.annotation {
                    Some(annotation) => {
                        let data_type = KybraExpr {
                            located_expr: annotation.as_ref(),
                            source_map: self.source_map,
                        }
                        .to_act_data_type(&None);

                        Ok(ActFnParam {
                            name: name.to_string(),
                            data_type,
                        })
                    }
                    None => Err(format!("parameter \"{}\" is missing a type annotation. All method parameters must specify their type.", &name)),
                }
            })
            .collect()
    }
}
