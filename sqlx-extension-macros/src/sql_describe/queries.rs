use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

/// This function creates a query with a given name.
pub fn create_query_fn(fn_name: &Ident, sql: &str) -> TokenStream2 {
    quote! {
        fn #fn_name() -> &'static str {
            #sql
        }
    }
}
