/// This enum represents an actual attribute on a field.
#[derive(Debug)]
pub enum Attribute {
    /// The rename attribute: it contains the 
    /// new name of the field.
    Rename {
        new_name: String
    },
    /// The table attribute: it contains the name
    /// of the table.
    Table {
        table_name: String
    }
}
