mod data;
mod http;
mod storage;

use std::io;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::{Env};

use http::services::{next, template};
use crate::storage::configuration_store::ConfigurationStore;
use crate::storage::port_store::PortStore;
use crate::storage::template_store::TemplateStore;

#[actix_web::main]
async fn main() -> io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let port_store = PortStore::default();
    let template_store = TemplateStore::default();
    let configuration_store = ConfigurationStore::default();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(port_store.clone()))
            .app_data(web::Data::new(template_store.clone()))
            .app_data(web::Data::new(configuration_store.clone()))
            .configure(next)
            .configure(template)
            .wrap(Logger::default())
    }).bind(("127.0.0.1", 3000))?
      .run()
      .await
}
