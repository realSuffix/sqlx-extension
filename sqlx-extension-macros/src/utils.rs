use sqlx_extension_common::{field::Field as CustomField, attribute::Attribute as CustomAttribute};
use proc_macro::TokenStream;
use syn::{Attribute, Field, MetaNameValue};
use proc_macro2::Group;
use syn::parse;

/// This function converts a SYN-field to a more usable
/// field from the common crate.
pub(crate) fn parse_field(field: Field) -> Option<CustomField> {
    let Field { attrs, ident, .. } = field;

    // parse identifier
    let identifier = ident?;

    // parse attributes
    let attributes = attrs.into_iter().filter_map(parse_attribute).collect();

    Some(CustomField {
        identifier,
        attributes
    })
}

/// This function converts a SYN-attribute to a more usable
/// attribute from the common crate.
pub(crate) fn parse_attribute(attr: Attribute) -> Option<CustomAttribute> {
   // convert attribute to group
   let input = TokenStream::from(attr.tokens);
   let group = parse::<Group>(input).ok()?;

   // convert tokens within group to `MetaNameValue`
   let input = TokenStream::from(group.stream());
   let meta = parse::<MetaNameValue>(input).ok()?;

   // TODO extract relevant values from `MetaNameValue`

   None
}
