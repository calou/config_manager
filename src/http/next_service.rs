use actix_web::{HttpResponse, Responder, web, get};
use crate::storage::port_store::PortStore;

#[get("/{port}")]
pub async fn get(store: web::Data<PortStore>, port: web::Path<u32>) -> impl Responder {
    let value = store.next(Some(port.into_inner()));
    format!("{}", value)
}

#[get("/reserve/{port}")]
pub async fn reserve(store: web::Data<PortStore>, port: web::Path<u32>) -> impl Responder {
    let value = store.reserve_next(Some(port.into_inner()));
    format!("{}", value)
}

#[get("/release/{port}")]
pub async fn release(store: web::Data<PortStore>, port: web::Path<u32>) -> HttpResponse {
    store.release(port.into_inner());
    HttpResponse::Ok().body("Ok") // FIXME
}