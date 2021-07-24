use std::convert::TryFrom;

use proc_macro::TokenStream;
use proc_macro2::{Group, Literal};
use sqlx_extension_common::{
    attribute::{Attribute as CustomAttribute, RawAttribute},
    field::Field as CustomField,
};
use syn::parse;
use syn::{Attribute, Field, Ident, Path};

/// This function converts a SYN-field to a more usable
/// field from the common crate.
pub(crate) fn parse_field(field: Field) -> Option<CustomField> {
    let Field { attrs, ident, .. } = field;

    // parse identifier
    let identifier = ident?;

    // parse attributes
    let attributes = attrs
        .into_iter()
        // parse the attribute
        .filter_map(parse_attribute)
        // convert it to a known attribute
        .map(CustomAttribute::try_from)
        // filter out the ones that couldn't be converted
        .filter_map(Result::ok)
        .collect();

    Some(CustomField {
        identifier,
        attributes,
    })
}

/// This function converts a SYN-attribute to a more usable
/// attribute from the common crate.
pub(crate) fn parse_attribute(attr: Attribute) -> Option<RawAttribute> {
    let name = ident_from_path(attr.path)?;

    // convert attribute to group
    let input = TokenStream::from(attr.tokens);
    let group = parse::<Group>(input).ok()?;

    // convert tokens within group to literal
    let input = TokenStream::from(group.stream());
    let value = parse::<Literal>(input).ok()?;
    let value = value.to_string();

    Some(RawAttribute { name, value })
}

/// This function attempts to retrieve the underlying identifier
/// of a path.
pub(crate) fn ident_from_path(path: Path) -> Option<Ident> {
    path.segments.into_iter().map(|s| s.ident).next()
}
