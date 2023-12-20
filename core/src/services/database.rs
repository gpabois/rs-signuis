use sqlx::{Postgres, Pool, Acquire};
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::Migrator;

use crate::Error;

use super::{ServicePool, ServiceTx};

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct DatabasePoolArgs<'a> {
    url: &'a str,
}

impl<'a> DatabasePoolArgs<'a> {
    pub fn new(url: &'a str) -> Self {
        return Self{url}
    }
}

pub struct DatabasePool {}

impl DatabasePool {
    pub async fn new<'a>(args: DatabasePoolArgs<'a>) -> Result<Pool<Postgres>, Error> {
        let pool = PgPoolOptions::new().max_connections(5).connect(args.url).await?;
        Ok(pool)
    }
} 

impl ServicePool {
    pub async fn migrate_database(&self) -> Result<(), Error> {
        let mut executor = self.querier.acquire().await?;
        MIGRATOR.run(&mut executor).await?;
        Ok(())
    }

    pub async fn revert_database(self) -> Result<(), Error> {
        let mut executor = self.querier.acquire().await?;
        MIGRATOR.undo(&mut executor, -1).await?;
        Ok(())
    }
}

impl<'q> ServiceTx<'q> {
    pub async fn migrate_database(&mut self) -> Result<(), Error> {
        let executor = self.querier.acquire().await?;
        MIGRATOR.run(executor).await?;
        Ok(())
    }

    pub async fn revert_database(&mut self) -> Result<(), Error> {
        let executor = self.querier.acquire().await?;
        MIGRATOR.undo( executor, -1).await?;
        Ok(())
    }
}