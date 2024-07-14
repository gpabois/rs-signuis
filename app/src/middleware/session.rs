use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use signuis_core::{
    models::session::Session, services::authentication::CheckUserSessionToken, Signuis,
};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::error::ServerError;

pub struct SessionMiddleware(signuis_core::Signuis);

impl SessionMiddleware {
    pub fn new(signuis: signuis_core::Signuis) -> Self {
        Self(signuis)
    }
}

impl<S, B> Transform<S, ServiceRequest> for SessionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = SessionMiddlewareInstance<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionMiddlewareInstance {
            service: Rc::new(service),
            signuis: self.0.clone(),
        }))
    }
}

pub struct SessionMiddlewareInstance<S> {
    service: Rc<S>,
    signuis: Signuis,
}

impl<S, B> Service<ServiceRequest> for SessionMiddlewareInstance<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let sg = self.signuis.clone();
        let service = self.service.clone();

        Box::pin(async move {
            let maybe_session_token = req.cookie("SIGNUIS_SESSION_TOKEN");

            let session: Session = match maybe_session_token {
                None => Session::Anonymous,
                Some(token) => sg
                    .service
                    .send(CheckUserSessionToken::new(token.value()))
                    .await
                    .map_err(ServerError::from)?
                    .map_err(ServerError::from)?
                    .map(Session::User)
                    .unwrap_or_else(|| Session::Anonymous),
            };

            req.extensions_mut().insert(session);

            service.call(req).await
        })
    }
}
