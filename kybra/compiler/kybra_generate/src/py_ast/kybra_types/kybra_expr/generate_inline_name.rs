use crate::py_ast::traits::GenerateInlineName;

use super::KybraExpr;

impl GenerateInlineName for KybraExpr<'_> {
    fn generate_inline_name(&self) -> String {
        format!("KybraInlineType{}", self.calculate_hash())
    }
}
