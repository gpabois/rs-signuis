use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}