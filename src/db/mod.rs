use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use super::config::Config;

pub async fn get_pool() -> Pool<Postgres> {
    let config = Config::new();
    let database_url = config.get_database_url();

    log::info!("Connectiong to database...");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            log::error!("Failed to connect to database: {:#?}", err);
            std::process::exit(1);
        }
    };

    log::info!("Connected to database");

    pool
}
