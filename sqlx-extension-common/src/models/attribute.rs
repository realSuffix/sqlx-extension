use std::convert::TryFrom;

use syn::{Ident, Type};

/// This enum represents an actual attribute on a field.
#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    /// The rename attribute: it contains the
    /// new name of the field.
    Rename { new_name: String },
    /// The table attribute: it contains the name
    /// of the table.
    Table { table_name: String },
    /// The current field should be marked as a primary key.
    PrimaryKey,
    /// The type chosen for the identifier (also derives `Entity`!)
    Identifier { ident_type: Type },
}

/// This struct represents a raw attribute which hasn't been mapped to a specific variant yet.
#[derive(Debug)]
pub struct RawAttribute {
    /// The name of the attribute
    pub name: Ident,
    /// The value of the attribute
    pub value: Option<String>,
}

impl TryFrom<RawAttribute> for Attribute {
    type Error = ();

    /// This implementation tries to map a given raw attribute to its
    /// corresponding attribute (if the name matches).
    fn try_from(value: RawAttribute) -> Result<Self, Self::Error> {
        match value.name.to_string().as_str() {
            "rename" => Ok(Attribute::Rename {
                new_name: value.value.unwrap(),
            }),
            "p_key" => Ok(Attribute::PrimaryKey),
            "table" => Ok(Attribute::Table {
                table_name: value.value.unwrap(),
            }),
            "ident" => Ok(Attribute::Identifier {
                ident_type: syn::parse_str(&value.value.unwrap().replace("\"", ""))
                    .expect("panicked here"),
            }),
            _ => Err(()),
        }
    }
}
