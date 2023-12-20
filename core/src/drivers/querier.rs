use sqlx::{Executor, Postgres, Pool,  Acquire, Transaction, PgConnection};

pub trait DatabaseQuerier<'c>: Executor<'c, Database=Postgres>{}
impl<'c> DatabaseQuerier<'c> for &Pool<Postgres> {}
impl<'c> DatabaseQuerier<'c> for &'c mut PgConnection {}

pub trait DatabaseQuerierAcquire<'c>: Acquire<'c, Database = Postgres>{}
impl<'c> DatabaseQuerierAcquire<'c> for &'c mut Transaction<'c, Postgres>{}