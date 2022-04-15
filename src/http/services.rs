use actix_web::{get, web, HttpResponse, Responder};
use crate::storage::port::PortStore;


#[get("/{port}")]
async fn next(store: web::Data<PortStore>, port: web::Path<u32>) -> impl Responder {
    let value = store.next(Some(port.into_inner()));
    format!("{}", value)
}

#[get("/reserve/{port}")]
async fn reserve(store: web::Data<PortStore>, port: web::Path<u32>) -> impl Responder {
    let value = store.reserve_next(Some(port.into_inner()));
    format!("{}", value)
}

#[get("/release/{port}")]
async fn release(store: web::Data<PortStore>, port: web::Path<u32>) -> HttpResponse {
    store.release(port.into_inner());
    HttpResponse::Ok().body("Ok") // FIXME
}


pub fn next_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/next")
            .service(next)
            .service(reserve)
            .service(release)
    );
}