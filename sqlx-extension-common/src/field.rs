use syn::Ident;

use crate::attribute::Attribute;

/// This struct describes a single field of an
/// entity.
#[derive(Debug)]
pub struct Field {
    /// This is the identifier (i.e. the name)
    /// of an field.
    pub identifier: Ident,
    /// Those are all the attributes a given field has.
    pub attributes: Vec<Attribute>,
}
