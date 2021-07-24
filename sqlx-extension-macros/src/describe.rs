use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, parse_macro_input, DataStruct};

use crate::utils::parse_field;

/// This macro is used to generate an implementation of the describe
/// trait.
pub fn describe_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let res:Option<TokenStream> = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields_named ), ..}) => {
            let fields = fields_named.named.into_iter().filter_map(parse_field).collect::<Vec<_>>();
            println!("{:#?}", fields);
            None
        },
        _ => None
    };

    res.unwrap_or(TokenStream::default())
}
