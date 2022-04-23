mod data;
mod http;
mod storage;

use std::io;
use std::path::Path;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::{Env};
use rocksdb::DB;

use crate::http::configuration_service;
use crate::storage::configuration_store::ConfigurationStore;
use crate::storage::port_store::PortStore;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let path = "/tmp";
    let directory = Path::new(path);
    let port_store = PortStore::new(DB::open_default(directory.join("ports.db")).unwrap());
    let configuration_store = ConfigurationStore::new(DB::open_default(directory.join("configurations.db")).unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(port_store.clone()))
            .app_data(web::Data::new(configuration_store.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/configuration")
                    .service(configuration_service::create)
                    .service(configuration_service::get)
                    .service(configuration_service::delete)
            )


    }).bind(("0.0.0.0", 3000))?
        .run()
        .await
}
