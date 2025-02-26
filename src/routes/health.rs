use actix_web::web;

use super::super::handlers::health_handler;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health").route(web::get().to(health_handler::health_check_handler)),
    );
}
