use cdk_framework::{act::node::param::Param, ToDataType};
use rustpython_parser::ast::Arguments;

use crate::source_map::SourceMapped;

impl SourceMapped<&Arguments> {
    pub fn to_act_fn_params(&self) -> Result<Vec<Param>, String> {
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
        self.node.args[1..]
            .iter()
            .map(|arg| {
                let name = &arg.node.arg;

                match &arg.node.annotation {
                    Some(annotation) => {
                        let data_type = SourceMapped {
                            node: annotation.as_ref(),
                            source_map: self.source_map.clone(),
                        }
                        .to_data_type();

                        Ok(Param {
                            name: name.to_string(),
                            type_: data_type,
                        })
                    }
                    None => Err(format!("parameter \"{}\" is missing a type annotation. All method parameters must specify their type.", &name)),
                }
            })
            .collect()
    }
}
