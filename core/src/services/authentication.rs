use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use chrono::{Duration, Utc};
use futures::future::LocalBoxFuture;
use std::ops::Add;
use uuid::Uuid;

use crate::crypto::generate_token;
use crate::error::Error;
use crate::events::{AuthenticationFailed, EventBus};
use crate::forms::authentication::CredentialForm;
use crate::issues::{Issue, Issues};
use crate::models::session::{Session, UserSession};
use crate::repositories::credential::MaybeFindOneCredentialByNameOrEmail;
use crate::repositories::user_session::{InsertUserSession, MaybeFindOneValidUserSessionByToken};
use crate::repositories::Repository;

#[derive(Clone)]
pub struct Authentication(Addr<AuthenticationActor>);

impl Authentication {
    pub fn new(repos: Repository, events: EventBus) -> Self {
        Self(AuthenticationActor::new(repos, events).start())
    }

    pub async fn execute<O: AuthenticationOp>(&self, op: O) -> Result<O::Return, Error> {
        self.0.send(ExecuteAuthenticationOp(op)).await?
    }
}

pub struct AuthenticationActor {
    repos: Repository,
    events: EventBus,
}

impl AuthenticationActor {
    pub fn new(repos: Repository, events: EventBus) -> Self {
        Self { repos, events }
    }
}

impl Actor for AuthenticationActor {
    type Context = Context<Self>;
}

impl<O> Handler<ExecuteAuthenticationOp<O>> for AuthenticationActor
where
    O: AuthenticationOp,
{
    type Result = ResponseFuture<Result<O::Return, Error>>;

    fn handle(
        &mut self,
        msg: ExecuteAuthenticationOp<O>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let fut = msg.0.execute(self);

        Box::pin(fut)
    }
}

pub trait AuthenticationOp: Sync + Send + 'static {
    type Return: Sync + Send;

    fn execute<'fut>(
        self,
        auth: &mut AuthenticationActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>>;
}

pub struct ExecuteAuthenticationOp<O>(O)
where
    O: AuthenticationOp;

impl<O> Message for ExecuteAuthenticationOp<O>
where
    O: AuthenticationOp,
{
    type Result = Result<O::Return, Error>;
}

/// Authentifie avec des identifiants simples.
pub struct AuthenticateWithCredential {
    pub form: CredentialForm,
    pub session: Session,
}

impl AuthenticateWithCredential {
    pub fn new(form: CredentialForm, session: Session) -> Self {
        Self { form, session }
    }
}

impl AuthenticationOp for AuthenticateWithCredential {
    type Return = CreatedUserSession;

    fn execute<'fut>(
        self,
        auth: &mut AuthenticationActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>> {
        let repos = auth.repos.clone();
        let events = auth.events.clone();
        let user_session_lifetime = Duration::hours(8);

        Box::pin(async move {
            let credential = repos
                .execute(MaybeFindOneCredentialByNameOrEmail(
                    self.form.username_or_email.to_string(),
                ))
                .await?
                .ok_or_else(|| {
                    Issues::new()
                        .add(invalid_credential_issue())
                        .to_owned()
                        .into_error()
                })?;

            if !credential.verify(&self.form.password)? {
                events.notify(AuthenticationFailed(credential.id));

                return Err(Issues::new()
                    .add(invalid_credential_issue())
                    .to_owned()
                    .into_error());
            }

            let user_id = credential.id;
            let token = generate_token(16);
            let expires_at = Utc::now().add(user_session_lifetime);

            repos
                .execute(InsertUserSession {
                    user_id,
                    token: token.clone(),
                    expires_at,
                })
                .await?;

            Ok(CreatedUserSession {
                user_id,
                token,
                expires_at,
            })
        })
    }
}

pub struct CreatedUserSession {
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: chrono::DateTime<Utc>,
}

/// Vérifie le jeton de la session utilisateur.
pub struct CheckUserSessionToken {
    pub token: String,
}

impl CheckUserSessionToken {
    pub fn new<S: ToString>(token: S) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}

impl AuthenticationOp for CheckUserSessionToken {
    type Return = Option<UserSession>;

    fn execute<'fut>(
        self,
        auth: &mut AuthenticationActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>> {
        let repos = auth.repos.clone();

        Box::pin(async move {
            repos
                .execute(MaybeFindOneValidUserSessionByToken(self.token))
                .await
        })
    }
}

#[inline]
fn invalid_credential_issue() -> Issue {
    Issue::new(
        "invalid_form",
        "Les identifiants sont incorrects",
        Vec::<String>::default(),
    )
}
