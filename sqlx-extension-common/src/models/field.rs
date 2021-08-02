use std::borrow::Cow;

use syn::Ident;

use crate::models::attribute::Attribute;

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

impl Field {
    /// This method tries to retrieve the "real" name of a field:
    /// If the field has a rename attribute, then the first encountered
    /// version of it will be used, else the normal name is used.
    pub fn name<'a>(&'a self) -> Cow<'a, str> {
        self.attributes
            .iter()
            .find_map(|a| {
                if let Attribute::Rename { new_name } = a {
                    Some(Cow::Borrowed(&new_name[..]))
                } else {
                    None
                }
            })
            .unwrap_or(Cow::Owned(self.identifier.to_string()))
    }
}
