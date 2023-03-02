use cdk_framework::{act::node::param::Param, ToDataType};
use rustpython_parser::ast::{ArgData, Arguments, Located};

use crate::source_map::SourceMapped;

impl SourceMapped<&Arguments> {
    pub fn to_param_list(&self) -> Result<Vec<Param>, String> {
        if self.node.kwarg.is_some() {
            return Err("the dictionary unpacking operator (**) is not supported".to_string());
        }

        if self.node.vararg.is_some() {
            return Err("the iterator unpacking operator (*) is not supported".to_string());
        }

        if self.node.args.len() == 0 {
            return Err(
                "method must be an instance method. Add \"self\" as the first parameter"
                    .to_string(),
            );
        }

        if self.node.args[0].node.arg != "self".to_string() {
            return Err("first parameter must be \"self\"".to_string());
        }

        // Ignore the first param, which is always "self"
        Ok(self.node.args[1..]
            .iter()
            .map(|arg| {
                SourceMapped {
                    node: arg,
                    source_map: self.source_map.clone(),
                }
                .to_param()
            })
            .collect())
    }
}

impl SourceMapped<&Located<ArgData>> {
    pub fn to_param(&self) -> Param {
        let param_name = self.node.node.arg.clone();
        match &self.node.node.annotation {
            Some(annotation) => {
                let data_type = SourceMapped {
                    node: annotation.as_ref(),
                    source_map: self.source_map.clone(),
                }
                .to_data_type();

                Param {
                    name: param_name,
                    type_: data_type,
                }
            }
            None => todo!("{}", format!("parameter \"{}\" is missing a type annotation. All method parameters must specify their type.", param_name)),
        }
    }
}
