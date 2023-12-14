use core::fmt;
use std::pin::Pin;

use async_stream::{try_stream, stream};
use async_trait::async_trait;
use futures::{Future, StreamExt};
use sqlx::{Postgres, Pool, Executor, Acquire, Transaction};
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::{Migrator, MigrateError};
use futures::stream::BoxStream;
use futures::future::{BoxFuture, LocalBoxFuture};

use crate::Error;

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone, Debug)]
pub struct Database<C> {
    pub executor: C,
}

// Pool database
pub type DatabasePool = Database<Pool<Postgres>>;

// Database transaction
pub type DatabaseTx<'c> = Database<Transaction<'c, Postgres>>;

pub mod traits {
    use futures::future::LocalBoxFuture;
    use sqlx::migrate::MigrateError;

    pub trait Database<'c> {
        fn migrate(self) -> LocalBoxFuture<'c, Result<(), MigrateError>>;
        fn revert(self) -> LocalBoxFuture<'c, Result<(), MigrateError>>;
    }
}

pub struct DatabaseArgs<'a> {
    url: &'a str,
}

impl<'a> DatabaseArgs<'a> {
    pub fn new(url: &'a str) -> Self {
        return Self{url}
    }
}

impl DatabasePool {
    pub async fn new<'a>(args: DatabaseArgs<'a>) -> Result<Self, Error> {
        let executor = PgPoolOptions::new().max_connections(5).connect(args.url).await?;   

        Result::Ok(Self{executor})
    }
    
    pub async fn begin(&self) -> Result<DatabaseTx<'static>, Error> {
        let tx = self.executor.begin().await?;
        return Result::Ok(DatabaseTx::new(tx))
    }
} 

impl<'c> DatabaseTx<'c> {
    pub fn new(tx: Transaction<'c, Postgres>) -> Self {
        Self{executor: tx}
    }
}

impl<'c, C> traits::Database<'c> for &'c Database<C> 
    where &'c C: Acquire<'c, Database = Postgres> + std::marker::Send
{
    fn revert(self) -> LocalBoxFuture<'c, Result<(), MigrateError>> {
       Box::pin(MIGRATOR.undo(&self.executor, -1))
    }

    fn migrate(self) -> LocalBoxFuture<'c, Result<(), MigrateError>>  {
        Box::pin(MIGRATOR.run(&self.executor))
    }
}

impl<'c, C> traits::Database<'c> for &'c mut Database<C> 
    where &'c mut C: Acquire<'c, Database = Postgres> , C: std::marker::Send + std::marker::Sync + 'c 
{
    fn revert(self) -> LocalBoxFuture<'c, Result<(), MigrateError>> {
        Box::pin(MIGRATOR.undo(&mut self.executor, -1))
    }

    fn migrate(self) -> LocalBoxFuture<'c, Result<(), MigrateError>> {
        Box::pin(MIGRATOR.run(&mut self.executor))
    }
}

impl<'c, C> Executor<'c> for &'c mut Database<C>
    where for<'a> &'a mut C: Acquire<'c, Database = Postgres>, C: std::marker::Send + std::fmt::Debug {
        type Database = Postgres;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row>,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database> {
            let fut = Box::pin(try_stream! {
                let mut conn = self.executor.acquire().await?;
                let mut stream = conn.fetch_many(query);

                while let Some(v) = stream.next().await {
                    yield v?
                }
            });

            fut
        }

        fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database> {
            Box::pin(async move {
                let conn = self.executor.acquire().await;
                if let Result::Err(error) = conn {
                    Result::Err(error)
                } else {
                    conn.unwrap().fetch_optional(query).await
                }
            })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>>
    where
        'c: 'e {
            let future = self.executor.acquire();

            let future = async move {
                let conn = future.await;
                if let Result::Err(error) = conn {
                    Result::Err(error)
                } else {
                    conn.unwrap().prepare_with(sql, parameters).await
                }
            };
    
            Box::pin(future)
    }

        fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e {
            let future = self.executor.acquire();

            let future = async move {
                let conn = future.await;
                if let Result::Err(error) = conn {
                    Result::Err(error)
                } else {
                    conn.unwrap().describe(sql).await
                }
            };
    
            Box::pin(future)
    }
    }

impl<'c, C> Executor<'c> for &'_ Database<C> 
    where for<'a> &'a C: Executor<'c, Database = Postgres>, 
            C: std::marker::Sync + std::marker::Send + std::fmt::Debug 
{
    type Database = Postgres;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row>,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database> {
        self.executor.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database> {
        self.executor.fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>>
    where
        'c: 'e {
        self.executor.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e {
        self.executor.describe(sql)
    }
}

impl<'c, C> Executor<'c> for Database<C> 
    where C: Executor<'c, Database = Postgres> 
{
    type Database = Postgres;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row>,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database> {
        self.executor.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database> {
        self.executor.fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>>
    where
        'c: 'e {
        self.executor.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e {
        self.executor.describe(sql)
    }
}