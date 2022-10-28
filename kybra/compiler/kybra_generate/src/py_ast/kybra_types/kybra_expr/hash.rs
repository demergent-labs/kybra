use super::KybraExpr;
use core::hash::Hash;
// use std::hash::Hash;
// use syn::__private::Hash;

impl Hash for KybraExpr<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // TODO we have got to figure out a better has then just the tokens location in the file
        self.located_expr.location.column().hash(state);
        self.located_expr.location.row().hash(state);
    }
}
