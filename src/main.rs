mod data;
mod http;
mod storage;

use std::io;

use actix_web::{App, HttpServer, web};

use http::services::{next, template};
use crate::storage::port_store::PortStore;
use crate::storage::template_store::TemplateStore;

#[actix_web::main]
async fn main() -> io::Result<()> {

    let port_store = PortStore::default();
    let template_store = TemplateStore::default();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(port_store.clone()))
            .app_data(web::Data::new(template_store.clone()))
            .configure(next)
            .configure(template)
    }).bind(("127.0.0.1", 3000))?
      .run()
      .await
}
