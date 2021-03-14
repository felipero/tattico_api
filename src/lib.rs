use std::env;
use std::net::TcpListener;

use actix_tls::accept::openssl::{SslAcceptor, SslAcceptorBuilder};
use actix_tls::connect::ssl::openssl::SslMethod;
use actix_web::{App, HttpResponse, HttpServer, middleware, Result, web};
use actix_web::dev::Server;
use actix_web::web::ServiceConfig;
use openssl::ssl::SslFiletype;

mod accounting;
pub mod controllers;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    logger_setup();
    println!("Starting server at: https://{}", listener.local_addr()?);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(controllers::routes)
            .configure(health_check)
    })
    .listen_openssl(listener, ssl_builder())?
    .run();

    Ok(server)
}

pub fn health_check(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/health_check").route(web::get().to(HttpResponse::Ok)));
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
