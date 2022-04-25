use std::io;
use std::path::Path;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use env_logger::Env;
use rocksdb::{DBCompressionType, DBWithThreadMode, MultiThreaded, Options, DB};

use crate::http::configuration_service;
use crate::storage::configuration_store::ConfigurationStore;
use crate::storage::port_store::PortStore;

mod data;
mod http;
mod storage;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'd', long = "data-dir")]
    data_dir: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = Args::parse();
    let path = &args.data_dir;
    let directory = Path::new(path);
    let port_db = create_port_db(directory);
    let port_store = PortStore::new(port_db);
    let configuration_store =
        ConfigurationStore::new(DB::open_default(directory.join("configurations.db")).unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(port_store.clone()))
            .app_data(web::Data::new(configuration_store.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/configuration")
                    .service(configuration_service::create)
                    .service(configuration_service::get)
                    .service(configuration_service::delete),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

fn create_port_db(directory: &Path) -> DBWithThreadMode<MultiThreaded> {
    let mut cf_opts = Options::default();
    cf_opts.set_max_write_buffer_number(16);
    cf_opts.set_compression_type(DBCompressionType::None);
    DB::open(&cf_opts, directory.join("ports.db")).unwrap()
}
