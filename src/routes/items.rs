use actix_web::web;

use super::super::handlers::items_handler;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/items")
            .route(web::get().to(items_handler::get_items))
            .route(web::post().to(items_handler::post_item)),
    );
}
