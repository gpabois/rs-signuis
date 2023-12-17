use std::ops::Deref;

use sqlx::{Postgres, Pool, Acquire};
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::{Migrator, Migrate};

use crate::Error;

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

pub struct Database{}

impl Database {
    pub async fn migrate<'c, A>(executor: A) -> Result<(), Error> 
        where A: Acquire<'c>, 
                <A::Connection as Deref>::Target: Migrate 
    {
        MIGRATOR.run(executor).await?;
        Ok(())
    }

    pub async fn revert<'c, A>(executor: A) -> Result<(), Error> where A: Acquire<'c>, <A::Connection as Deref>::Target: Migrate {
        MIGRATOR.undo(executor, -1).await?;
        Ok(())
    }
}