mod database;
mod account;
mod authentication;

pub use database::*;
pub use authentication::*;
pub use account::*;
use sqlx::Postgres;

use self::traits::AuthenticationContainer;

pub mod traits {
    pub use super::authentication::traits::Authentication;
    pub use super::database::traits::Database;

    pub trait Container<'a, Entity> {
        fn with() ->  &'a Box<dyn Entity>;
    }
    
}

/// Service pool
pub struct ServicePool {
    db: DatabasePool
}

impl Container<traits::Database> for &'_ ServicePool {
    type Authentication = Authentication<DatabasePool>;

    fn get(self) -> Self::Authentication {
        Self::Authentication::new(AuthenticationArgs::new(&self.db))
    }
}

/// Service transaction
pub struct ServiceTx<'a> {
    db:     SharedDatabaseTx<'a, Postgres>,
    auth:   Box<Authentication<SharedDatabaseTx<'a, Postgres>>>
}

pub struct ServiceTxArgs<'a> {
    db: DatabaseTx<'a>
}

impl<'a> ServiceTxArgs<'a> {
    pub fn new(db: DatabaseTx<'a>) -> Self {
        Self{db}
    }
}

impl<'a> ServiceTx<'a> {
    pub fn new(args: ServiceTxArgs<'a>) -> Self {
        let shared_db = args.db.into_shared();

        Self {
            db: shared_db.clone(),
            auth: Box::new(
                Authentication::new(
                    AuthenticationArgs::new(shared_db.clone())
                )
            )
        }
    }
}

impl<'a> Container<'a, traits::Authentication> for &'a ServiceTx<'a> {
    fn with(self) -> &'a Box<dyn traits::Authentication> {
        return &self.auth
    }
}