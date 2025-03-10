pub struct Config {
    database_url: String,
}

impl Config {
    pub fn new() -> Self {
        log::info!("Loading environment variables...");

        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(err) => {
                log::error!("Failed to load DATABASE_URL from env: {:#?}", err);
                std::process::exit(1)
            }
        };

        log::info!("Successfully loaded environment variables");

        Config { database_url }
    }

    pub fn get_database_url(&self) -> &str {
        &self.database_url
    }
}
