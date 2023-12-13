use sqlx::{Postgres, Pool, Executor};
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::Migrator;
use futures::stream::BoxStream;
use futures::future::BoxFuture;

use crate::Error;

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

pub struct DatabaseArgs<'a> {
    url: &'a str,
}

impl<'a> DatabaseArgs<'a> {
    pub fn new(url: &'a str) -> Self {
        return Self{url}
    }
}

impl Database {
    pub async fn new<'a>(args: DatabaseArgs<'a>) -> Result<Self, Error> {
        let pool = PgPoolOptions::new().max_connections(5).connect(args.url).await?;   

        Result::Ok(Self{
            pool
        })
    }

    pub async fn revert(&self) -> Result<(), Error> {
        MIGRATOR.undo(&self.pool, -1).await?;
        Result::Ok(())
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        MIGRATOR.run(&self.pool).await?;
        Result::Ok(())
    }
} 

impl Executor<'static> for Database {
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
        'static: 'e,
        E: sqlx::Execute<'q, Self::Database> {
        self.pool.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'static: 'e,
        E: sqlx::Execute<'q, Self::Database> {
        self.pool.fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>>
    where
        'static: 'e {
        self.pool.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'static: 'e {
        self.pool.describe(sql)
    }
}