use actix_web::{HttpResponse, web, get, post};
use actix_web::http::StatusCode;
use log::info;
use crate::{ConfigurationStore, PortStore};
use crate::storage::template_store::TemplateStore;

#[post("")]
pub async fn create(store: web::Data<TemplateStore>, content: String) -> HttpResponse {
    HttpResponse::Ok().json(store.create(&content))
}

#[get("")]
pub async fn get_all(store: web::Data<TemplateStore>) ->  HttpResponse {
    HttpResponse::Ok().json(store.get_all())
}

#[get("/{uuid}/next")]
pub async fn next(store: web::Data<TemplateStore>,
                  configuration_store:  web::Data<ConfigurationStore>,
                  port_store: web::Data<PortStore>,
                  uuid: web::Path<String>) ->  HttpResponse {
    let uuid = uuid.into_inner();
    if let Some(template) = store.get(uuid.clone()) {
        info!("{}", uuid);
        let configuration = configuration_store.create(template, port_store.into_inner().clone());
        HttpResponse::Ok().json(configuration)
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }

}

#[get("/{uuid}")]
pub async fn get(store: web::Data<TemplateStore>, uuid: web::Path<String>) -> HttpResponse {
    let uuid = uuid.into_inner();
    if let Some (tmpl) = store.get(uuid.clone()) {
        HttpResponse::Ok().json(tmpl)
    } else {
        info!("Template with uuid {} not found", uuid);
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

