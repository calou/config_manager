use crate::data::configuration::Configuration;
use crate::data::template::Template;
use crate::{ConfigurationStore, PortStore};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, web, HttpResponse};

fn create_response(configuration: Configuration) -> HttpResponse {
    let content = configuration.content;
    HttpResponse::Ok()
        .insert_header(("x-configuration-uuid", configuration.uuid))
        .content_type("text/plain; charset=utf-8")
        .take()
        .body(content)
}

#[post("")]
pub async fn create(
    store: web::Data<ConfigurationStore>,
    port_store: web::Data<PortStore>,
    content: String,
) -> HttpResponse {
    let template = Template::create(&content);
    let configuration = store.create(template, port_store.into_inner());
    create_response(configuration)
}

#[get("/{uuid}")]
pub async fn get(store: web::Data<ConfigurationStore>, uuid: web::Path<String>) -> HttpResponse {
    let uuid = uuid.into_inner();
    if let Some(configuration) = store.get(uuid) {
        create_response(configuration)
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

#[delete("/{uuid}")]
pub async fn delete(
    store: web::Data<ConfigurationStore>,
    port_store: web::Data<PortStore>,
    uuid: web::Path<String>,
) -> HttpResponse {
    let uuid = uuid.into_inner();
    let port_store = port_store.into_inner();
    if let Some(configuration) = store.delete(uuid) {
        for port in configuration.clone().ports {
            port_store.release(port);
        }
        create_response(configuration)
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}
