use actix_web::{get, web, Responder};
use log::{debug, error, info, warn};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    info!("greet: {}", name);
    debug!("greet: {}", name);
    warn!("greet: {}", name);
    error!("greet: {}", name);
    format!("Hello {name}!")
}
