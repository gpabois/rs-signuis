use futures::future::BoxFuture;
use signuis_core::{services::{ServicePool,authentication::traits::Authentication as TraitAuthentication}, model::{session::Session, credentials::Credential}, log::info};
use tonic::{transport::Server, Request, Response, Status};

pub mod codegen {
    tonic::include_proto!("signuis"); 
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("signuis_descriptor");
}

pub mod middlewares;

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

impl Into<codegen::AuthenticationResult> for Result<Session, signuis_core::Error> {
    fn into(self) -> codegen::AuthenticationResult {
        todo!()
    }
}

pub fn into_status(error: signuis_core:: Error) -> Status {
    Status::new(tonic::Code::Internal, error.message())
}

#[derive(Clone)]
pub struct Authentication(ServicePool);

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


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    signuis_core::config::Config::init(Default::default()).unwrap();

    let addr = "[::1]:5051".parse()?;
    let pool = signuis_core::services::ServicePool::from_config().await.unwrap();

    let reflection = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(codegen::FILE_DESCRIPTOR_SET)
    .build()
    .unwrap();

    info!("Starting gRPC server, listenning to {addr}");
    Server::builder()
        .add_service(reflection)
        .add_service(codegen::authentication_server::AuthenticationServer::new(Authentication(pool.clone())))
        .serve(addr)
        .await?;

    Ok(())
}