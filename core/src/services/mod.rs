pub mod database;
pub mod account;
pub mod authentication;
pub mod authorization;
pub mod logger;
pub mod reporting;

use futures::future::BoxFuture;
use log::info;
use sqlx::{Postgres, Pool, Transaction};

use crate::{repositories::Repository, Error, config::Config};

use self::database::{DatabasePool, DatabasePoolArgs};

#[derive(Clone)]
pub struct Service<Q> {
    pub querier: Q,
    pub repos: Repository
}

pub type ServicePool = Service<Pool<Postgres>>;
pub type ServiceTx<'q> = Service<Transaction<'q, Postgres>>;

impl<Q> Service<Q> {
    pub fn borrow_repository(&self) -> &Repository {
        return &self.repos
    }
}

impl<'q> ServiceTx<'q> {
    pub async fn commit(self) -> Result<(), Error> {
        self.querier.rollback().await.map_err(Error::from)
    }

    pub async fn rollback(self) -> Result<(), Error> {
        self.querier.rollback().await.map_err(Error::from)
    }
}


impl Service<Pool<Postgres>> {
    pub fn new (pool: Pool<Postgres>) -> Self {
        Self { querier: pool, repos: Default::default() }
    }

    pub async fn from_config() -> Result<Self, Error> {
        info!("Creating service pool");
        let database_url = Config::try_get_database_url()?;
        let pool = DatabasePool::new(DatabasePoolArgs::new(database_url.as_str())).await?;
        Ok(Self::new(pool))
    }

    pub async fn begin<'q>(&self) -> Result<ServiceTx<'q>, Error> {
        let tx = self.querier.begin().await?;
        Ok(ServiceTx{
            querier: tx, 
            repos: self.repos
        })
    }

    pub fn with<D, E, F>(&self, f: F) -> BoxFuture<'_, Result<D, E>>
    where 
        for<'a, 'f, 'g> F:  std::ops::FnOnce(&'a mut ServiceTx<'g>) -> BoxFuture<'a, Result<D, E>> + Send + 'a, D: Send, E: From<Error> + Send {        
        Box::pin(async {
            let mut tx = self.begin().await?;
            match f(&mut tx).await {
                Ok(val) => {
                    tx.commit().await?;
                    Ok(val)
                },
                Err(err) => {
                    tx.rollback().await?;
                    Err(err)
                }
            }
        })
    }

}
