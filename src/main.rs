mod data;
mod http;
mod storage;

use std::io;

use actix_web::{App, HttpServer, web};

use http::services::next_service;
use crate::storage::port::PortStore;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let reserved_ports = PortStore::default();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(reserved_ports.clone()))
            .configure(next_service)
    }).bind(("127.0.0.1", 3000))?
      .run()
      .await
}
