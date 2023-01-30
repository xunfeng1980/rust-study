use actix_identity::Identity;
use actix_web::{
    cookie::Key, get, middleware::Logger, post, App, HttpMessage, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use log::{debug, error, info, warn};

#[get("/account")]
pub async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[post("/account/login")]
pub async fn login(request: HttpRequest) -> impl Responder {
    // Some kind of authentication should happen here -
    // e.g. password-based, biometric, etc.
    // [...]

    // Attached a verified user identity to the active
    // session.
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

#[post("/account/logout")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::NoContent()
}
