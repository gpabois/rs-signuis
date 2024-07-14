use crate::error::Error;
use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use futures::future::LocalBoxFuture;
use sqlx::{Executor, PgPool, Postgres};
use sqlx_postgres::PgPoolOptions;

pub mod credential;
pub mod nuisance_family;
pub mod nuisance_report;
pub mod nuisance_type;
pub mod user;
pub mod user_session;

#[derive(Clone)]
pub struct RepositorySettings {
    pub max_connections: u32,
    pub database_url: String,
}

impl Default for RepositorySettings {
    fn default() -> Self {
        Self {
            max_connections: 5,
            database_url: std::env::var("DATABASE_URL").unwrap_or_default(),
        }
    }
}

impl RepositorySettings {
    /// Définit le nombre maximum de connections dans la pool.
    pub fn set_max_connections(&mut self, value: u32) -> &mut Self {
        self.max_connections = value;
        self
    }
}

/// Un repertoire de données.
#[derive(Clone)]
pub struct Repository(Addr<RepositoryActor>);

impl Repository {
    /// Crée une nouvelle connection au répertoire de données.
    pub async fn new(settings: &RepositorySettings) -> Result<Self, Error> {
        RepositoryActor::new(settings)
            .await
            .map(Actor::start)
            .map(Self)
    }

    /// Execute une opération sur le répertoire de données.
    pub async fn execute<O: RepositoryOp + 'static>(&self, op: O) -> Result<O::Return, Error> {
        self.0.send(ExecRepositoryOp::from(op)).await?
    }
}

#[derive(Clone)]
pub struct RepositoryActor {
    pool: PgPool,
}

impl RepositoryActor {
    pub async fn new(settings: &RepositorySettings) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(settings.max_connections)
            .connect(&settings.database_url)
            .await?;

        Ok(Self { pool })
    }
}

impl Actor for RepositoryActor {
    type Context = Context<Self>;
}

impl<T> Handler<ExecRepositoryOp<T>> for RepositoryActor
where
    T: RepositoryOp + 'static,
{
    type Result = ResponseFuture<Result<<T as RepositoryOp>::Return, Error>>;

    fn handle(&mut self, msg: ExecRepositoryOp<T>, _ctx: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        Box::pin(async move { msg.0.execute(&pool).await })
    }
}

pub struct BeginTx {}

impl RepositoryOp for BeginTx {
    type Return = ();

    fn execute<'c, E>(self, executor: E) -> LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: Executor<'c, Database = Postgres> + 'c,
    {
        Box::pin(async {
            sqlx::query("BEGIN").execute(executor).await?;
            Ok(())
        })
    }
}

/// Message sollicitant l'exécution d'une opération sur le répertoire de données.
///
pub struct ExecRepositoryOp<T>(T)
where
    T: RepositoryOp;

impl<T> From<T> for ExecRepositoryOp<T>
where
    T: RepositoryOp,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Message for ExecRepositoryOp<T>
where
    T: RepositoryOp,
{
    type Result = Result<<T as RepositoryOp>::Return, Error>;
}

pub trait RepositoryOp: Sync + Send {
    type Return: Sync + Send + 'static;

    fn execute<'c, E>(self, executor: E) -> LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: Executor<'c, Database = Postgres> + 'c;
}
