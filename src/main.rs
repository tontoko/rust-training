use actix_web::middleware::Logger;
use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use anyhow::bail;
use anyhow::{Context, Result};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello World"
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind_openssl("127.0.0.1:8443", builder)?
        .run()
        .await
}
