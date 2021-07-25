/// This trait is used to define what a single entity looks like.
pub trait Describe {
    /// This function returns the name of the table.
    fn table_name() -> &'static str;

    /// This function returns an insert statement which requires all
    /// parameters.
    fn insert() -> &'static str;
    
    /// This function returns an insert statement where all ID fields are left out.
    fn insert_without_id() -> &'static str;

    /// This function returns a statement which inserts the given entity without its id, returning all its values.
    fn insert_return() -> &'static str;

    /// This function returns a statement which inserts the given entity without its ID, returning
    /// the ID.
    fn insert_return_id() -> &'static str;
}
