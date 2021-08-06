use sqlx::{Database, Pool, Transaction, Result};

/// This enum represents a transaction which may
/// or may not have been opened.
pub enum MaybeTransaction<'a, DB: Database> {
    Pool(&'a Pool<DB>),
    Transaction(Transaction<'a, DB>),
}

impl<'a, DB: Database> MaybeTransaction<'a, DB> {
    /// This function retrieves the underlying transaction in case it was already opened
    /// or opens a new one in case a pool was contained.
    pub async fn get(&'a mut self) -> Result<&'a mut Transaction<'a, DB>> {
        match self {
            MaybeTransaction::Transaction(ref mut transaction) => Ok(transaction),
            MaybeTransaction::Pool(pool) => {
                let transaction = pool.begin().await?;
                *self = MaybeTransaction::Transaction(transaction);

                if let MaybeTransaction::Transaction(ref mut transaction) = self {
                    Ok(transaction)
                } else {
                    unreachable!()
                }

            }
        }
    }

    /// This function closes the transaction in case it was still open.
    /// Note: If you pass some function just a &mut MaybeTransaction, you can
    /// therefore be sure that it wont stand a chance at ever closing the transaction!
    pub async fn commit(self) -> Result<()> {
        // close transaction in case it was still open
        if let MaybeTransaction::Transaction(transaction) = self {
            transaction.commit().await?;
        }
        Ok(())
    }
}
