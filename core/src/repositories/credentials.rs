use async_stream::stream;
use futures::{stream::BoxStream, StreamExt};
use sqlx::{FromRow, postgres::PgRow, Row};

use crate::{models::credentials::{HashedCredential, CredentialFilter}, drivers};

pub mod traits {
    use futures::{stream::BoxStream, TryStreamExt, future::BoxFuture};

    use crate::{models::credentials::{HashedCredential, CredentialFilter}, Error, drivers};

    pub trait CredentialRepository<'q>: Sized + std::marker::Send {
        // Find credentials based on a filter
        fn find_credentials_by<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, filter: CredentialFilter) 
            -> BoxStream<'b, Result<HashedCredential, Error>> 
        where 'a: 'b, 'q: 'b, Q: 'b;

        // Find one credential based on the filter
        fn find_one_credential_by<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, filter: CredentialFilter) 
        -> BoxFuture<'b, Result<Option<HashedCredential>, Error>> 
        where 'a: 'b, 'q: 'b, Q: 'b, Self: 'b {
            Box::pin(async {
                self.find_credentials_by(
                    querier, // Explicit reborrow
                    filter
                )
                .try_next()
                .await
            })

        }
    }
}

impl<'r> FromRow<'r, PgRow> for HashedCredential {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get::<uuid::Uuid, _>("id")?.into(),
            pwd_hash: row.try_get("password")?
        })
    }
}

/// Module permettant de générer des requêtes SQL
mod sql_query {
    use sea_query::{Query, PostgresQueryBuilder, Cond, Expr};
    use sea_query_binder::{SqlxValues, SqlxBinder};

    use crate::{models::credentials::CredentialFilter, sql::UserIden};

    /// Génère une requête filtrée de récupération d'informations d'identification.
    pub fn find_credentials_by(filter: CredentialFilter) -> (String, SqlxValues) {
        Query::select()
        .from(UserIden::Table)
        .columns([UserIden::ID, UserIden::Password])
        .conditions(filter.name_or_email_eq.is_some(), |s| {
            let name_or_email = filter.name_or_email_eq.unwrap();
            s.cond_where(
                Cond::any()
                .add(Expr::col(UserIden::Name).eq(name_or_email.clone()))
                .add(Expr::col(UserIden::Email).eq(name_or_email))
            );
        }, |_|{})
        .build_sqlx(PostgresQueryBuilder)
    }
}

/// On implémente les fonctions de répertoire pour les informations d'identification.
impl<'q> traits::CredentialRepository<'q> for &'q super::Repository {
    fn find_credentials_by<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, filter: CredentialFilter) 
        -> BoxStream<'b, Result<crate::models::credentials::HashedCredential, crate::Error>> 
    where 'a: 'b, 'q: 'b, Q: 'b {
        Box::pin(stream! {
            let (sql, arguments) = sql_query::find_credentials_by(filter);
            let mut stream = sqlx::query_with(&sql, arguments).fetch(querier);
            
            while let Some(result) = stream.next().await {
                match result {
                    Ok(row) => yield HashedCredential::from_row(&row).map_err(crate::Error::from),
                    Err(error) => yield Err(crate::Error::from(error))
                }  
            } 
        })

    }
}