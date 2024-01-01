use std::{net::SocketAddr, str::FromStr};

use futures::future::BoxFuture;
use signuis_core::{services::{ServicePool, authentication::traits::Authentication}, model::{session::Session, client::Client}};
use tower::{Layer, Service};
use tonic::{body::BoxBody, transport::{Body, server::{TcpConnectInfo, TlsConnectInfo}}, codegen::http::{request::Request, response::Response}};

#[derive(Clone)]
pub struct AuthenticationLayer(pub ServicePool);

impl<S> Layer<S> for AuthenticationLayer {
    type Service = AuthenticationMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthenticationMiddleware(service, self.0.clone())
    }
}


#[derive(Clone)]
pub struct AuthenticationMiddleware<S>(S, ServicePool);

// Return the remote address behind the request
pub fn remote_addr(req: &Request<Body>) -> Option<SocketAddr> {
    req.extensions()
    .get::<TcpConnectInfo>()
    .and_then(|i| i.remote_addr())
    .or_else(|| {
        req.extensions()
            .get::<TlsConnectInfo<TcpConnectInfo>>()
            .and_then(|i| i.get_ref().remote_addr())
    })
}

pub fn client_addr(req: &Request<Body>) -> Option<SocketAddr> {
    if let Some(Ok(addr)) = req.headers().get("X-Forwarded-For").map(|h| std::str::from_utf8(h.as_bytes())) {
        if let Ok(socket_addr) = SocketAddr::from_str(addr) {
            return Some(socket_addr)
        }
    }

    remote_addr(req)
}

pub fn user_agent(req: &Request<Body>) -> Option<&str> {
    if let Some(Ok(s)) = req.headers().get("User-Agent").map(|h| std::str::from_utf8(h.as_bytes())) {
        return Some(s)
    }

   None
}

pub fn client_from_request(req: &Request<Body>) -> Client {
    Client::new(
        &client_addr(req).unwrap().to_string(),
        user_agent(req).unwrap()
    )
}

impl<S> Service<Request<Body>> for AuthenticationMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let clone = self.0.clone();
        let mut inner = std::mem::replace(&mut self.0, clone);

        let pool = self.1.clone();

        Box::pin(async move {
            let session = Session::anonymous(client_from_request(&req));

            req.extensions_mut().insert(session);

            if let Some(token_raw_value) = req.headers().get("X-Authenticate") {
                let token = std::str::from_utf8(token_raw_value.as_bytes()).unwrap();
                let mut tx = pool.begin().await.unwrap();
                if let Ok(session) = tx.check_session_token(token).await {
                    req.extensions_mut()
                        .get_mut::<Session>()
                        .map(|sess| *sess = session);
                }
            }
            // Do extra async work here...
            let response = inner.call(req).await?;

            Ok(response)
        })
    }

}