mod pages;

use actix_web::{App, HttpServer, web};
use crate::pages::home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}