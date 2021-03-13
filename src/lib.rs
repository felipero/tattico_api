use std::env;
use std::net::TcpListener;

use actix_files as fs;
use actix_tls::accept::openssl::{SslAcceptor, SslAcceptorBuilder};
use actix_tls::connect::ssl::openssl::SslMethod;
use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder, Result};
use openssl::ssl::SslFiletype;

mod accounting;
pub mod controllers;

async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?.set_status_code(StatusCode::OK))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    logger_setup();
    println!("Starting server at: https://{}", listener.local_addr()?);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(controllers::routes)
            .service(web::resource("/health_check").route(web::get().to(health_check)))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .listen_openssl(listener, ssl_builder())?
    .run();

    Ok(server)
}

fn logger_setup() {
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    }
    let _ = env_logger::try_init();
}

fn ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    builder
}
