use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::{item::Item, post_item::InsertedItem, post_item::PostItem};

pub async fn get_items(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Item>("SELECT * FROM items;")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(err) => {
            log::error!("Failed to retrieve items: {:#?}", err);
            HttpResponse::InternalServerError().body("Failed to retrieve items")
        }
    }
}

pub async fn post_item(pool: web::Data<PgPool>, item: web::Json<PostItem>) -> impl Responder {
    let name = item.get_name();
    let description = match item.get_description() {
        Some(value) => value,
        None => "",
    };

    let result = sqlx::query_as!(
        InsertedItem,
        "INSERT INTO items (name, description) VALUES ($1, $2) RETURNING id",
        name,
        description
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record),
        Err(err) => {
            log::error!("Failed to insert item: {:#?}", err);
            HttpResponse::InternalServerError().body("Failed to insert item")
        }
    }
}
