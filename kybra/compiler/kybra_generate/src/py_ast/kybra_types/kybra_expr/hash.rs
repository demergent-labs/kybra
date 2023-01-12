use super::{to_hash_string::ToHashString, KybraExpr};
use core::hash::Hash;
// use std::hash::Hash;
// use syn::__private::Hash;

impl Hash for KybraExpr<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_hashable_string().hash(state)
    }
}
