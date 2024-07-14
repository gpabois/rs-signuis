use actix::MailboxError;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use signuis_core::error::ErrorKind;

#[derive(Debug)]
pub struct ServerError(ErrorKind);

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            ErrorKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Invalid(_) => StatusCode::NOT_ACCEPTABLE,
            ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match &self.0 {
            ErrorKind::InternalError => HttpResponse::InternalServerError()
                .append_header(("Location", "/500"))
                .finish(),
            ErrorKind::DatabaseError => HttpResponse::InternalServerError()
                .append_header(("Location", "/500"))
                .finish(),
            ErrorKind::Invalid(issues) => HttpResponse::NotAcceptable().json(issues.clone()),
            ErrorKind::Unauthorized => HttpResponse::InternalServerError()
                .append_header(("Location", "/401"))
                .finish(),
        }
    }
}

impl From<MailboxError> for ServerError {
    fn from(_: MailboxError) -> Self {
        Self(ErrorKind::InternalError)
    }
}

impl From<signuis_core::error::Error> for ServerError {
    fn from(value: signuis_core::error::Error) -> Self {
        Self(value.kind)
    }
}
