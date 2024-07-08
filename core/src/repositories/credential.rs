use sqlx::Executor;

use crate::models::credential::Credential;

use super::Repository;

/// On implémente les fonctions de répertoire pour les informations d'identification.
impl Repository 
{
    async fn find_one_credential_by_name_or_email<'c, E: Executor<'c>>(&self, executor: E, name_or_email: &str) -> Result<Option<Credential>, crate::error::Error>
    {
        let cred: Credential = sqlx::query("SELECT id, password FROM users WHERE username=$1 OR WHERE email=$1")
            .bind(name_or_email)
            .fetch_optional(executor)
            .await?;

        Ok(cred)
    }
}

