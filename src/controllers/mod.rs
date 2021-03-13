use actix_web::web;
use actix_web::web::ServiceConfig;

mod entries;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").service(web::scope("/v1").service(entries::services())));
}

