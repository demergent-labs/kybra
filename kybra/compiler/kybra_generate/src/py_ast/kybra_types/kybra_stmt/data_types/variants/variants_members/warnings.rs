// use crate::{
//     errors::{Contents, Message},
//     py_ast::kybra_types::KybraStmt,
//     source_map::GetSourceInfo,
// };

// impl KybraStmt<'_> {
//     pub(super) fn variant_default_value_warning(&self) -> Message {
//         Message::Warning(Contents {
//             title: "WARNING: Default values are not supported and will be ignored".to_string(),
//             origin: self.get_origin(),
//             line_number: self.get_line_number(),
//             source: self.get_source(),
//             range: self.get_range(),
//             annotation: "Default value used here".to_string(),
//             suggestion: None,
//         })
//     }
// }
