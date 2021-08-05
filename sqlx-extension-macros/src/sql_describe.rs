mod queries;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use sqlx_extension_core::{
    models::{attribute::Attribute, field::Field},
    query_builder::QueryBuilder,
};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};

use crate::utils::{parse_attributes, parse_field};

use self::queries::create_query_fn;

/// This macro is used to generate an implementation of the describe
/// trait.
pub fn sql_describe_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let res: Option<TokenStream> = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => {
            let struct_name = &input.ident;

            let struct_attributes: Vec<_> = parse_attributes(input.attrs).collect();

            // parse name of table
            let table_name = parse_table_name(struct_attributes.iter())
                .expect("Table name was not provided! aborting...");

            // parse fields
            let fields = fields_named
                .named
                .into_iter()
                .filter_map(parse_field)
                .collect::<Vec<_>>();

            // create version which doesn't own the fields
            let fields_ref = fields.iter().collect::<Vec<_>>();

            // create all functions
            let mut queries = Vec::new();
            create_queries(&mut queries, &fields_ref, &table_name);

            // create trait implementation by interpolating all queries
            Some(
                quote! {
                    impl sqlx_extension::traits::sql_describe::SqlDescribe for #struct_name {
                        #(#queries)*

                        /// The name of the table for the current entity.
                        fn table_name() -> &'static str {
                            #table_name
                        }
                    }
                }
                .into(),
            )
        }
        _ => None,
    };

    res.unwrap_or_default()
}

/// This function parses the name of a table from the attributes of a struct.
fn parse_table_name<'a>(attrs: impl IntoIterator<Item = &'a Attribute> + 'a) -> Option<&'a str> {
    attrs.into_iter().find_map(|a| {
        if let Attribute::Table { table_name } = a {
            Some(&table_name[..])
        } else {
            None
        }
    })
}

/// This method fills the given array with all queries for a given object.
fn create_queries(all_queries: &mut Vec<TokenStream2>, all_fields: &[&Field], table_name: &str) {
    // split the fields into contained / not contained in primary key
    let (fields_not_in_pk, fields_in_pk): (Vec<&Field>, Vec<&Field>) =
        all_fields
            .iter()
            .fold((Vec::new(), Vec::new()), |(mut not_pk, mut pk), curr| {
                // check whether the current field is marked as primary key
                let is_pk = curr.attributes.contains(&Attribute::PrimaryKey);

                // yes --> add it to the vector of pk-fields
                if is_pk {
                    pk.push(curr);
                } else {
                    // nope --> add it to the vector of non-pks
                    not_pk.push(curr);
                }

                (not_pk, pk)
            });

    // create query builder
    let builder = QueryBuilder::new()
        .table_name(table_name)
        .entity_attributes(all_fields);

    // create queries
    let insert = create_query_fn(
        &create_ident("insert"),
        &builder.build_insert(&[]).unwrap_or_default(),
    );
    all_queries.push(insert);

    // make sure to change entity attributes to be just the ones which
    // are not in the PK
    let builder = builder.entity_attributes(&fields_not_in_pk);

    let insert_without_pk = create_query_fn(
        &create_ident("insert_without_pk"),
        &builder.build_insert(&all_fields).unwrap_or_default(),
    );
    all_queries.push(insert_without_pk);

    let insert_without_pk_return_pk = create_query_fn(
        &create_ident("insert_without_pk_return_pk"),
        &builder.build_insert(&fields_in_pk).unwrap_or_default(),
    );
    all_queries.push(insert_without_pk_return_pk);

    // retrieve queries
    let retrieve_all = create_query_fn(
        &create_ident("retrieve_all"),
        &builder.build_retrieve().unwrap_or_default(),
    );
    all_queries.push(retrieve_all);

    let retrieve_by_pk = create_query_fn(
        &create_ident("retrieve_by_pk"),
        &builder
            .build_retrieve_where(&fields_in_pk)
            .unwrap_or_default(),
    );
    all_queries.push(retrieve_by_pk);

    // delete queries
    let delete_by_pk = create_query_fn(
        &create_ident("delete_by_pk"),
        &builder
            .build_delete_where(&fields_in_pk)
            .unwrap_or_default(),
    );
    all_queries.push(delete_by_pk);
}

/// This helper function just creates a ident with the span `call_site`.
fn create_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}
