use actix_web::{
    cookie::{time::OffsetDateTime, Cookie, Expiration},
    post,
    web::{Data, Form},
    HttpResponse, Responder,
};

use signuis_core::{
    forms::authentication::CredentialForm, models::session::Session,
    services::authentication::AuthenticateWithCredential, Signuis,
};

use crate::error::ServerError;

#[post("/login")]
pub async fn authenticate_with_credential(
    Form(form): Form<CredentialForm>,
    session: Data<Session>,
    sg: Data<Signuis>,
) -> Result<impl Responder, actix_web::Error> {
    let session = session.get_ref().clone();

    let user_session = sg
        .service
        .send(AuthenticateWithCredential::new(form, session))
        .await
        .map_err(ServerError::from)?
        .map_err(ServerError::from)?;

    let tok_cookie = Cookie::build("SIGNUIS_SESSION_TOKEN", user_session.token)
        .secure(true)
        .http_only(true)
        .expires(Expiration::DateTime(
            OffsetDateTime::from_unix_timestamp(user_session.expires_at.timestamp()).unwrap(),
        ))
        .finish();

    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", "/"))
        .cookie(tok_cookie)
        .finish())
}
