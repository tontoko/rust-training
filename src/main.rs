use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer, Responder};
use anyhow::bail;
use anyhow::{Context, Result};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World"
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
