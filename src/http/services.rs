use actix_web::web;

use crate::http::next_service;
use crate::http::template_service;

pub fn next(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/next")
            .service(next_service::get)
            .service(next_service::reserve)
            .service(next_service::release)
    );
}

pub fn template(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/template")
            .service(template_service::create)
            .service(template_service::get)
            .service(template_service::get_all)
    );
}