use std::io;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};

use crate::reserved_ports::ReservedPorts;

mod reserved_ports;


#[get("/next/{port}")]
async fn next(reserved_ports: web::Data<ReservedPorts>, port: web::Path<u32>) -> impl Responder {
    let value = reserved_ports.next(Some(port.into_inner()));
    format!("{}", value)
}

#[get("/next/reserve/{port}")]
async fn reserve(reserved_ports: web::Data<ReservedPorts>, port: web::Path<u32>) -> impl Responder {
    let value = reserved_ports.reserve_next(Some(port.into_inner()));
    format!("{}", value)
}

#[get("/next/release/{port}")]
async fn release(reserved_ports: web::Data<ReservedPorts>, port: web::Path<u32>) -> HttpResponse {
    reserved_ports.release(port.into_inner());
    HttpResponse::Ok().body("Ok") // FIXME
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let reserved_ports = ReservedPorts::default();
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(reserved_ports.clone()))
            .service(next)
            .service(reserve)
            .service(release)
    }).bind(("127.0.0.1", 3000))?
      .run()
      .await
}
