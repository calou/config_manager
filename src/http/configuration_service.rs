use actix_web::{HttpResponse, web, get, post, delete};
use actix_web::http::StatusCode;
use crate::{ConfigurationStore, PortStore};
use crate::data::template::Template;

#[post("")]
pub async fn create(store: web::Data<ConfigurationStore>, port_store: web::Data<PortStore>, content: String) -> HttpResponse {
    let template = Template::create(&content);
    let configuration = store.create(template, port_store.into_inner());
    let content = configuration.content;
    HttpResponse::Ok()
        .insert_header(("x-configuration-uuid", configuration.uuid))
        .content_type("text/plain; charset=utf-8")
        .take()
        .body(content)
}

#[get("/{uuid}")]
pub async fn get(store: web::Data<ConfigurationStore>,
                 uuid: web::Path<String>) -> HttpResponse {
    let uuid = uuid.into_inner();
    if let Some(configuration) = store.get(uuid.clone()) {
        HttpResponse::Ok().json(configuration)
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

#[delete("/{uuid}")]
pub async fn delete(store: web::Data<ConfigurationStore>,
                    port_store: web::Data<PortStore>,
                    uuid: web::Path<String>) -> HttpResponse {
    let uuid = uuid.into_inner();
    let port_store = port_store.into_inner();
    if let Some(configuration) = store.delete(uuid.clone()) {
        for port in configuration.ports{
            port_store.release(port);
        }
        HttpResponse::Accepted()
            .insert_header(("x-configuration-uuid", configuration.uuid))
            .content_type("text/plain; charset=utf-8")
            .take()
            .body(configuration.content)
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}
