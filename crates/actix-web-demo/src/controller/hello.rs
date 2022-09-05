use actix_web::{get, Responder, web};
use log::{info, debug, warn, error};


#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    info!("greet: {}", name);
    debug!("greet: {}", name);
    warn!("greet: {}", name);
    error!("greet: {}", name);
    format!("Hello {name}!")
}
