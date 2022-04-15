use std::io;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};

use crate::reserved_ports::ReservedPorts;

mod reserved_ports;

#[derive(Clone)]
struct AppState {
    reserved_ports: ReservedPorts,
}

async fn next(app_state: web::Data<AppState>, port: web::Path<u32>) -> impl Responder {
    let next = app_state.reserved_ports.next(Some(port.into_inner()));
    format!("{}", next)
}

async fn reserve(app_state: web::Data<AppState>, port: web::Path<u32>) -> impl Responder {
    let next = app_state.reserved_ports.reserve_next(Some(port.into_inner()));
    format!("{}", next)
}

async fn release(app_state: web::Data<AppState>, port: web::Path<u32>) -> HttpResponse {
    app_state.reserved_ports.release(port.into_inner());
    HttpResponse::Ok().body("Ok") // FIXME
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let reserved_ports = ReservedPorts::default();
    let app_state = AppState {
        reserved_ports
    };
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(app_state.clone()))
            .route("/next/{port}", web::to(next))
            .route("/next/reserve/{port}", web::to(reserve))
            .route("/next/release/{port}", web::to(release))
    }).bind(("127.0.0.1", 3000))?
      .run()
      .await
}
