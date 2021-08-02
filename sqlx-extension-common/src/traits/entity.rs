use async_trait::async_trait;
use sqlx::database::HasArguments;
use sqlx::query::{Query, QueryAs};
use sqlx::{Database, Encode, Executor, FromRow, IntoArguments, Result, Type, query, query_as};

use super::sql_describe::SqlDescribe;

#[async_trait(?Send)]
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
    async fn retrieve_all<DB, R, E>(exec: E) -> Result<Vec<R>>
    where
        DB: Database,
        R: for<'r> FromRow<'r, DB::Row> + Send + Unpin,
        E: for<'e> Executor<'e, Database = DB>,
        for<'db> <DB as HasArguments<'db>>::Arguments: IntoArguments<'db, DB> 
    {
        query_as(<Self as SqlDescribe>::retrieve_all())
            .fetch_all(exec)
            .await
    }

    /// This function retrieves a single entity of the table via the PK (if any).
    async fn retrieve_by_pk_optional<'a, DB, R, E, I>(exec: E, params: I) -> Result<Option<R>>
    where
        DB: Database,
        R: for<'r> FromRow<'r, DB::Row> + Send + Unpin,
        E: for<'e> Executor<'e, Database = DB>,
        for<'db> <DB as HasArguments<'db>>::Arguments: IntoArguments<'db, DB>,
        I: IntoIterator<Item = &'a Self::Identifier>,
        Self::Identifier: 
            'a +
            Sync +
            for<'p> Encode<'p, DB> +
            Type<DB>

    {
        let mut query = query_as(<Self as SqlDescribe>::retrieve_by_pk());
        for item in params {
            query = query.bind(item);
        }

        query
            .fetch_optional(exec)
            .await
    }

    /// This function retrieves a single entity of the table via the PK.
    async fn retrieve_by_pk_one<'a, DB, R, E, I>(exec: E, params: I) -> Result<Option<R>>
    where
        DB: Database,
        R: for<'r> FromRow<'r, DB::Row> + Send + Unpin,
        E: for<'e> Executor<'e, Database = DB>,
        for<'db> <DB as HasArguments<'db>>::Arguments: IntoArguments<'db, DB>,
        I: IntoIterator<Item = &'a Self::Identifier>,
        Self::Identifier: 
            'a +
            Sync +
            for<'p> Encode<'p, DB> +
            Type<DB>

    {
        let mut query = query_as(<Self as SqlDescribe>::retrieve_by_pk());
        for item in params {
            query = query.bind(item);
        }

        query
            .fetch_optional(exec)
            .await
    }

    /// This function deletes a single entity via the PK of the table.
    async fn delete_by_pk<'a, DB, E, I>(exec: E, params: I) -> Result<()>
    where
        DB: Database,
        E: for<'e> Executor<'e, Database = DB>,
        for<'db> <DB as HasArguments<'db>>::Arguments: IntoArguments<'db, DB>,
        I: IntoIterator<Item = &'a Self::Identifier>,
        Self::Identifier: 
            'a +
            Sync +
            for<'p> Encode<'p, DB> +
            Type<DB>

    {
        let mut query = query(<Self as SqlDescribe>::retrieve_by_pk());

        for item in params {
            query = query.bind(item);
        }

        query
            .execute(exec)
            .await?;

        Ok(())
    }
}
