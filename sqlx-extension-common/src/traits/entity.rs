use async_trait::async_trait;
use sqlx::Database;
use sqlx::query::{Query, QueryAs};
use sqlx::database::HasArguments;

#[async_trait(?Send)]
pub trait Entity {
    /// This method generates a query which expects *all* attributes a certain entity
    /// has to be bound.
    async fn insert<'q, DB>() -> Query<'q, DB, <DB as HasArguments<'q>>::Arguments>
        where DB: Database;

    /// This function inserts the entity without the annotated primary keys,
    /// returning the entire entity.
    async fn insert_without_pk<'q, DB>() -> QueryAs<'q, DB, Self, <DB as HasArguments<'q>>::Arguments>
        where DB: Database,
              Self: Send + Sized +
                  for<'r> sqlx::FromRow<'r, DB::Row>;

    /// This function inserts the entity without the annotated primary keys,
    /// returning the primary key of the entity.
    /// Note: This also works in case the primary key consists of multiple parameters; it will
    /// return all of them!
    async fn insert_without_pk_return_pk<'q, DB, ID>() -> QueryAs<'q, DB, ID, <DB as HasArguments<'q>>::Arguments>
        where DB: Database,
              ID: Send + Sized +
                  for<'r> sqlx::FromRow<'r, DB::Row>;
}
