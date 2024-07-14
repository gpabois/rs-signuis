use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use futures::future::LocalBoxFuture;

use crate::{
    error::Error,
    events::{EventBus, UserRegistered},
    forms::account::RegisterUserForm,
    models::user::{UserId, UserRole},
    repositories::{
        user::{InsertUser, UserWithUsernameOrEmailExists},
        Repository,
    },
    validation::{Validation, Validator},
};

#[derive(Clone)]
pub struct Account(Addr<AccountActor>);

impl Account {
    pub fn new(repos: Repository, events: EventBus) -> Self {
        Self(AccountActor::new(repos, events).start())
    }

    pub async fn execute<O: AccountOp>(&self, op: O) -> Result<O::Return, Error> {
        self.0.send(ExecuteAccountOp(op)).await?
    }
}

pub struct AccountActor {
    repos: Repository,
    events: EventBus,
}

impl AccountActor {
    pub fn new(repos: Repository, events: EventBus) -> Self {
        Self { repos, events }
    }
}

impl Actor for AccountActor {
    type Context = Context<Self>;
}

impl<O> Handler<ExecuteAccountOp<O>> for AccountActor
where
    O: AccountOp,
{
    type Result = ResponseFuture<Result<O::Return, Error>>;

    fn handle(&mut self, msg: ExecuteAccountOp<O>, _ctx: &mut Self::Context) -> Self::Result {
        let fut = msg.0.execute(self);

        Box::pin(fut)
    }
}

/// Une opération à executer auprès du service de gestion
/// des comptes utilisateurs.
pub trait AccountOp: Sync + Send + 'static {
    type Return: Sync + Send;

    fn execute<'fut>(
        self,
        auth: &mut AccountActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>>;
}

pub struct ExecuteAccountOp<O>(O)
where
    O: AccountOp;

impl<O> Message for ExecuteAccountOp<O>
where
    O: AccountOp,
{
    type Result = Result<O::Return, Error>;
}

/// Enregistre un nouvel utilisateur.
pub struct RegisterUser {
    form: RegisterUserForm,
}

impl AccountOp for RegisterUser {
    type Return = UserId;

    fn execute<'fut>(
        self,
        accounts: &mut AccountActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>> {
        let repos = accounts.repos.clone();
        let events = accounts.events.clone();

        Box::pin(async move {
            let mut validator = Validator::default();
            self.form.assert(&mut validator);
            validator.check().inspect_err(|_| {})?;

            let exists = repos
                .execute(UserWithUsernameOrEmailExists {
                    username: self.form.username.clone(),
                    email: self.form.email.clone(),
                })
                .await?;

            validator.assert_false(
                exists.username_exists,
                Some("le nom d'utilisateur est déjà pris"),
                ["username"],
            );

            validator.assert_false(
                exists.email_exists,
                Some("l'adresse courriel est déjà pris"),
                ["email"],
            );

            let user_id = repos
                .execute(InsertUser {
                    username: self.form.username,
                    email: self.form.email,
                    password: Some(self.form.password),
                    role: UserRole::default(),
                })
                .await?;

            // notifie les autres systèmes
            events.notify(UserRegistered(user_id));

            Ok(user_id)
        })
    }
}
