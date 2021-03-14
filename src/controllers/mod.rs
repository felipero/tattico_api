use actix_web::web::ServiceConfig;
use actix_web::{guard, web};

mod entries;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1")
                .guard(guard::Header("Content-Type", "application/json"))
                .service(entries::services()),
        ),
    );
}
