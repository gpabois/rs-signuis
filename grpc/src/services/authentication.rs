use futures::future::BoxFuture;
use signuis_core::{services::{ServicePool,authentication::traits::Authentication as TraitAuthentication}, model::{session::Session, credentials::Credential}};
use tonic::{Request, Response, Status};

use crate::error::into_status;
use crate::codegen::{self};

#[derive(Clone)]
pub struct Authentication(pub ServicePool);

impl codegen::authentication_server::Authentication for Authentication {
    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn authenticate_with_credential<'a,'b>(&'a self, request: Request<codegen::Credential>) 
        ->  BoxFuture<'b, Result<Response<codegen::AuthenticationResult>, Status>> 
    where 'a:'b, Self: 'b {
        Box::pin(async move {
            self.0.with(|tx| {
            Box::pin(async move {
                let actor = request.extensions().get::<Session>().unwrap();
                let result = tx.authenticate_with_credentials(&
                    request.get_ref()
                    .clone()
                    .into(), 
                    actor
                ).await;
                Ok(Response::new(result.into()))
            })
        }).await.map_err(into_status)})
    }
}

impl Into<Credential> for codegen::Credential {
    fn into(self) -> Credential {
        Credential {
            name_or_email: self.name_or_email,
            password: self.password
        }
    }
}

impl Into<codegen::Issue> for signuis_core::Issue {
    fn into(self) -> codegen::Issue {
        codegen::Issue {
            code: self.code,
            message: self.message,
            path: self.path
        }
    }
}

impl Into<codegen::Error> for signuis_core::Error {
    fn into(self) -> codegen::Error {
        codegen::Error {
            code: self.code(),
            args: Some(match self {
                Self::ValidationError(issues) =>
                codegen::error::Args::Issues(
                    codegen::Issues{
                        issues: issues.into_iter().map(|f| f.into()).collect()
                    }
                ),
                _ => codegen::error::Args::Message(self.message())
            })
        }
    }
}

impl Into<codegen::authentication_result::Args> for Result<Session, signuis_core::Error> {
    fn into(self) -> codegen::authentication_result::Args {
        match self {
            Ok(data) => codegen::authentication_result::Args::Data(data.token),
            Err(err) => codegen::authentication_result::Args::Error(err.into())
        }
    }
}

impl Into<codegen::AuthenticationResult> for Result<Session, signuis_core::Error> {
    fn into(self) -> codegen::AuthenticationResult {
        codegen::AuthenticationResult { 
            ok: self.is_ok(), 
            args: Some(self.into()) 
        }
    }
}

