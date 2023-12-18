use sqlx::{Executor, Postgres};

pub trait DatabaseQuerier<'c>: Executor<'c, Database=Postgres>{}