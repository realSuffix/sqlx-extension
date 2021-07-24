mod describe;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::describe::describe_macro;

#[proc_macro_derive(Describe, attributes(rename))]
pub fn describe(input: TokenStream) -> TokenStream {
    describe_macro(input)
}
