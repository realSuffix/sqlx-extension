use std::convert::TryFrom;

use proc_macro::TokenStream;
use proc_macro2::{Group, Literal};
use sqlx_extension::models::{
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
    let attributes = parse_attributes(attrs).collect();

    Some(CustomField {
        identifier,
        attributes,
    })
}

/// This function converts a SYN-attribute to a more usable
/// attribute from the common crate.
pub(crate) fn parse_raw_attribute(attr: Attribute) -> Option<RawAttribute> {
    let name = ident_from_path(attr.path)?;

    // convert attribute to group
    let input = TokenStream::from(attr.tokens);
    let group = parse::<Group>(input).ok();

    let value = if let Some(group) = group {
        // convert tokens within group to literal
        let input = TokenStream::from(group.stream());
        let value = parse::<Literal>(input).ok()?;
        Some(value.to_string())
    } else {
        None
    };

    Some(RawAttribute { name, value })
}

/// This function attempts to retrieve the underlying identifier
/// of a path.
pub(crate) fn ident_from_path(path: Path) -> Option<Ident> {
    path.segments.into_iter().map(|s| s.ident).next()
}

/// This function parses all the given attributes.
pub(crate) fn parse_attributes(
    attrs: impl IntoIterator<Item = Attribute>,
) -> impl Iterator<Item = CustomAttribute> {
    attrs
        .into_iter()
        .filter_map(parse_raw_attribute)
        .map(CustomAttribute::try_from)
        .filter_map(Result::ok)
}
