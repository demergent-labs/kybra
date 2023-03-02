use cdk_framework::{act::node::data_type::Opt, ToDataType};
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_opt(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "opt",
                _ => false,
            },
            _ => false,
        }
    }

    pub(super) fn to_opt(&self) -> Result<Opt, Message> {
        if !self.is_opt() {
            return Err(self.not_opt_error());
        }
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "opt" {
                            panic!("{}", self.not_opt_error())
                        }
                    }
                    _ => panic!("{}", self.not_opt_error()),
                }
                let kybra_expr = SourceMapped {
                    inner: slice.as_ref(),
                    source_map: self.source_map.clone(),
                };
                Ok(Opt {
                    enclosed_type: Box::from(kybra_expr.to_data_type()),
                })
            }
            _ => Err(self.not_opt_error()),
        }
    }

    pub fn not_opt_error(&self) -> Message {
        self.create_error_message("This is is not an opt", "", None)
    }
}
