/// This trait describes a given entity using SQL statements.
/// It should almost never be implemented directly, but rather using the derive macro
/// which comes with this trait.
pub trait SqlDescribe {
    /// This function returns a statement which simply inserts a given
    /// entity, requiring all attributes.
    fn insert() -> &'static str;

    /// This function returns a statement which inserts the entity without
    /// requiring the fields annoted with `p_key`, returning the entire entity.
    fn insert_without_pk() -> &'static str;

    /// This function returns a statement which inserts the entity without
    /// requiring the fields annotated with `p_key`, returning *just* the fields
    /// annotated with `p_key`.
    fn insert_without_pk_return_pk() -> &'static str;

    /// This function returns a statement which retrieves all entities from
    /// the current table.
    fn retrieve_all() -> &'static str;

    /// This function returns a statement which retrieves all entities; filtering
    /// the entities by the primary key of the entity.
    fn retrieve_by_pk() -> &'static str;

    /// This function returns a statement which deletes all entities in the table
    /// matching a given primary key.
    fn delete_by_pk() -> &'static str;
}
