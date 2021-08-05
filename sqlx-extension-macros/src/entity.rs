use proc_macro::TokenStream;
use quote::quote;
use sqlx_extension::models::attribute::Attribute;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Type};

use crate::utils::parse_attributes;

pub fn entity_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let res: Option<TokenStream> = match input.data {
        Data::Struct(DataStruct { .. }) => {
            let struct_name = &input.ident;
            let struct_attributes: Vec<_> = parse_attributes(input.attrs).collect();

            // parse identifier for table (if any)
            parse_identifier(struct_attributes.iter()).map(|i| {
                let result = quote! {
                    impl sqlx_extension::traits::entity::Entity for #struct_name {
                        type Identifier = #i;
                    }
                };
                result.into()
            })
        }
        _ => None,
    };

    res.unwrap_or_default()
}

/// This function parses the identifier of a given table (if any).
fn parse_identifier<'a>(attrs: impl IntoIterator<Item = &'a Attribute> + 'a) -> Option<&'a Type> {
    attrs.into_iter().find_map(|a| {
        if let Attribute::Identifier { ident_type } = a {
            Some(ident_type)
        } else {
            None
        }
    })
}
