pub mod database;
pub mod account;
pub mod authentication;
pub mod authorization;

use sqlx::{Postgres, Pool, Transaction};

use crate::{repositories::Repository, Error};

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

    pub async fn begin<'q>(&self) -> Result<ServiceTx<'q>, Error> {
        let tx = self.querier.begin().await?;
        Ok(ServiceTx{
            querier: tx, 
            repos: self.repos
        })
    }

}
