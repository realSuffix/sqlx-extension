use sqlx::database::HasArguments;

use sqlx::query::{Query, QueryAs};
use sqlx::{query, query_as, Database, FromRow};

use super::sql_describe::SqlDescribe;

pub trait Entity: SqlDescribe {
    /// This is the type by which this entity is uniquely identified within
    /// the database.
    type Identifier;

    /// This method generates a query which expects *all* attributes a certain entity
    /// has to be bound.
    fn insert<'q, DB: Database>() -> Query<'q, DB, <DB as HasArguments<'q>>::Arguments> {
        query(<Self as SqlDescribe>::insert())
    }

    /// This function inserts the entity without the annotated primary keys,
    /// returning the entire entity.
    fn insert_without_pk<'q, DB>() -> QueryAs<'q, DB, Self, <DB as HasArguments<'q>>::Arguments>
    where
        DB: Database,
        Self: Send + Sized + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        query_as(<Self as SqlDescribe>::insert_without_pk())
    }

    /// This function inserts the entity without the annotated primary keys,
    /// returning the primary key of the entity.
    /// Note: This also works in case the primary key consists of multiple parameters; it will
    /// return all of them!
    fn insert_without_pk_return_pk<'q, DB: Database>(
    ) -> QueryAs<'q, DB, Self::Identifier, <DB as HasArguments<'q>>::Arguments>
    where
        Self::Identifier: for<'r> FromRow<'r, DB::Row>,
    {
        query_as(<Self as SqlDescribe>::insert_without_pk_return_pk())
    }

    /// This function retrieves all entities within a given table.
    fn retrieve_all<'q, R, DB>() -> QueryAs<'q, DB, R, <DB as HasArguments<'q>>::Arguments>
    where
        R: for<'r> FromRow<'r, DB::Row> + Send + Unpin,
        DB: Database,
    {
        query_as(<Self as SqlDescribe>::retrieve_all())
    }

    /// This function retrieves a entity via its PK.
    fn retrieve_by_pk<'q, DB, R>() -> QueryAs<'q, DB, R, <DB as HasArguments<'q>>::Arguments>
    where
        DB: Database,
        R: for<'r> FromRow<'r, DB::Row> + Send + Unpin,
    {
        query_as(<Self as SqlDescribe>::retrieve_by_pk())
    }

    /// This function deletes a single entity via the PK of the table.
    fn delete_by_pk<'q, DB>() -> Query<'q, DB, <DB as HasArguments<'q>>::Arguments>
    where
        DB: Database,
    {
        query(<Self as SqlDescribe>::retrieve_by_pk())
    }

    /// This function generates a query with the given SQL, resolving to the passed type.
    fn custom_query_as<'q, DB: Database, O>(
        sql: &'q str,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DB: Database,
        O: for<'r> FromRow<'r, DB::Row>,
    {
        query_as(sql)
    }

    /// This function generates a query with the given SQL, resolving to the passed type.
    fn custom_query_as_self<'q, DB: Database>(
        sql: &'q str,
    ) -> QueryAs<'q, DB, Self, <DB as HasArguments<'q>>::Arguments>
    where
        DB: Database,
        Self: for<'r> FromRow<'r, DB::Row>,
    {
        query_as(sql)
    }
}
