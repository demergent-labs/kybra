pub mod act;
pub mod act_node;
pub mod generators;
pub mod nodes;

pub trait ToTokenStream {
    fn to_token_stream(&self) -> proc_macro2::TokenStream;
}
