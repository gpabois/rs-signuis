use sea_query::RcOrArc;
use sqlx::{Postgres, Pool, Acquire, Transaction, PgConnection, Connection};
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::{Migrator, MigrateError};
use futures::future::{BoxFuture, LocalBoxFuture};
use std::sync::Mutex;
use std::sync::Arc;

use crate::Error;

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone, Debug)]
pub struct Database<C> {
    pub executor: C,
}

pub mod traits {
    use futures::future::LocalBoxFuture;
    use sqlx::migrate::MigrateError;

    pub trait Database<'c> {
        fn migrate(self) -> LocalBoxFuture<'c, Result<(), MigrateError>>;
        fn revert(self) -> LocalBoxFuture<'c, Result<(), MigrateError>>;
    }
}

pub struct DatabasePoolArgs<'a> {
    url: &'a str,
}

impl<'a> DatabasePoolArgs<'a> {
    pub fn new(url: &'a str) -> Self {
        return Self{url}
    }
}

pub struct DatabasePool {
    pool: Pool<Postgres>
}

impl DatabasePool {
    pub async fn new<'a>(args: DatabasePoolArgs<'a>) -> Result<Self, Error> {
        let pool = PgPoolOptions::new().max_connections(5).connect(args.url).await?;   

        Result::Ok(Self{pool})
    }
    
    pub async fn begin(&self) -> Result<DatabaseTx<'static, Postgres>, Error> {
        let tx = self.executor.begin().await?;
        return Result::Ok(DatabaseTx::new(tx))
    }
} 

pub struct DatabaseTx<'c, DB> {
    tx: Transaction<'c, DB>
}

impl<'c, DB> DatabaseTx<'c, DB> {
    pub fn new(tx: Transaction<'c, Postgres>) -> Self {
        Self{tx}
    }

    pub fn into_shared(self) -> SharedDatabaseTx<'c, DB> {
        SharedDatabaseTx::new(self)
    }
}

impl <'c, DB> Acquire<'c, DB> for &'_ mut DatabaseTx<'c, DB> {
    type Database = DB;
    type Connection: DB::Connection;

    fn acquire(self) -> BoxFuture<'c, Result<Self::Connection, sqlx::Error>> {
        self.tx.acquire()
    }

    fn begin(self) -> BoxFuture<'c, Result<Transaction<'c, Self::Database>, sqlx::Error>> {
        self.tx.begin()
    }
}

#[derive(Clone)]
pub struct SharedDatabaseTx<'c, DB> {
    inner: Arc<Mutex<InnerDatabaseTx<'c, DB>>>
}

impl<'c, DB> SharedDatabaseTx<'c, DB> {
    pub fn new(tx: DatabaseTx<'c, DB>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(tx))
        }
    }
}

impl <'c, DB> Acquire<'c, DB> for &'_ SharedDatabaseTx<'c, DB> {
    type Database = DB;
    type Connection: DB::Connection;

    fn acquire(self) -> BoxFuture<'c, Result<Self::Connection, sqlx::Error>> {
        self.tx.acquire()
    }

    fn begin(self) -> BoxFuture<'c, Result<Transaction<'c, Self::Database>, sqlx::Error>> {
        self.tx.begin()
    }
}
