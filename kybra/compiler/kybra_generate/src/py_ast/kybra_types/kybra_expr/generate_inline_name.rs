use crate::py_ast::traits::GenerateInlineName;

use super::KybraExpr;

impl GenerateInlineName for KybraExpr<'_> {
    fn generate_inline_name(&self) -> String {
        format!("AzleInlineType{}", self.calculate_hash())
    }
}
