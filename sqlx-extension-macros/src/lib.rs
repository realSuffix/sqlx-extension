mod sql_describe;
mod utils;

use proc_macro::TokenStream;
use crate::sql_describe::sql_describe_macro;

#[proc_macro_derive(Describe, attributes(rename, p_key, table))]
pub fn sql_describe(input: TokenStream) -> TokenStream {
    sql_describe_macro(input)
}
