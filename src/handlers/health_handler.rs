use actix_web::{HttpResponse, Responder};

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}
