mod describe;
mod utils;

use proc_macro::TokenStream;
use crate::describe::describe_macro;

#[proc_macro_derive(Describe, attributes(rename))]
pub fn describe(input: TokenStream) -> TokenStream {
    describe_macro(input)
}
