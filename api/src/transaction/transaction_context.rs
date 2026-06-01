use crate::application_context::Db;
use sqlx::{Database, Pool, Transaction};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::task_local;

task_local! {
    static TRANSACTION: Arc<Mutex<Transaction<'static, Db>>>;
}

type DbConnection = <Db as Database>::Connection;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Transaction is still referenced outside the scope")]
    StillReferenced,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub struct TransactionContext;

impl TransactionContext {
    pub fn current_transaction() -> Option<Arc<Mutex<Transaction<'static, Db>>>> {
        TRANSACTION.try_with(|tx| tx.clone()).ok()
    }

    pub async fn scope<F, R>(
        tx: Transaction<'static, Db>,
        f: F,
    ) -> Result<(R, Transaction<'static, Db>), TransactionError>
    where
        F: Future<Output = R>,
{
        let shared = Arc::new(Mutex::new(tx));
        let result = TRANSACTION.scope(shared.clone(), f).await;

        let tx = Arc::try_unwrap(shared)
            .map_err(|_| TransactionError::StillReferenced)?
            .into_inner();

        Ok((result, tx))
    }

    pub async fn execute_with<F, R>(
        pool: &Pool<Db>,
        f: F,
    ) -> Result<R, sqlx::Error>
    where
        F: for<'c> AsyncFnOnce(&'c mut DbConnection) -> Result<R, sqlx::Error>,
    {
        match Self::current_transaction() {
            Some(tx) => {
                let mut guard = tx.lock().await;
                f(&mut **guard).await
            }
            None => {
                let mut conn = pool.acquire().await?;
                f(&mut *conn).await
            }
        }
    }
}
