use actix_web::{HttpResponse, web, get, post};
use crate::storage::template_store::TemplateStore;

#[post("")]
pub async fn create(store: web::Data<TemplateStore>, content: String) -> HttpResponse {
    HttpResponse::Ok().json(store.create(&content))
}

#[get("")]
pub async fn get_all(store: web::Data<TemplateStore>) ->  HttpResponse {
    HttpResponse::Ok().json(store.get_all())
}

#[get("/{uuid}")]
pub async fn get(store: web::Data<TemplateStore>, uuid: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(store.get(uuid.into_inner()))
}