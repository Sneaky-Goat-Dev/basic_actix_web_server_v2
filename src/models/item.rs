use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Item {
    id: i32,
    name: String,
    description: Option<String>,
    created_at: NaiveDateTime,
}
