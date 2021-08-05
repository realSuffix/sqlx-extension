mod entity;
mod sql_describe;
mod utils;

use crate::sql_describe::sql_describe_macro;
use entity::entity_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Describe, attributes(rename, p_key, table))]
pub fn sql_describe(input: TokenStream) -> TokenStream {
    sql_describe_macro(input)
}

#[proc_macro_derive(Entity, attributes(ident))]
pub fn entity(input: TokenStream) -> TokenStream {
    entity_macro(input)
}
