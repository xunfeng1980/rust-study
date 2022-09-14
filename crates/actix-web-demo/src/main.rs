use actix_identity::IdentityMiddleware;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use actix_session::{Session, SessionMiddleware, storage::RedisSessionStore};
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;

use log::info;

use controller::hello::*;
use controller::account::{index, login, logout};

pub mod controller;
pub mod middleware;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // todo: log4rs path is too long
    log4rs::init_file("./crates/actix-web-demo/src/resources/config/log4rs.yaml",
                      Default::default()).unwrap();
    info!("Server start on 127.0.0.1:8080");
    let secret_key = Key::generate();
    HttpServer::new(move || {
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                // disable secure cookie for local testing
                .cookie_secure(false)
                .build();

        App::new()
            // Install the identity framework first.
            .wrap(IdentityMiddleware::default())
            // The identity system is built on top of sessions. You must install the session
            // middleware to leverage `actix-identity`. The session middleware must be mounted
            // AFTER the identity middleware: `actix-web` invokes middleware in the OPPOSITE
            // order of registration when it receives an incoming request.
            .wrap(session_mw)
            .wrap(Logger::default())
            .service(index)
            .service(login)
            .service(logout)
            .service(greet)

    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}