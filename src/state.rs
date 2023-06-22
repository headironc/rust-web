use crate::{config::Config, database::Database};

#[derive(Debug, Clone)]
pub struct State {
    pub config: Config,
    pub database: Database,
}

impl State {
    pub async fn new() -> Self {
        let config = Config::default();

        let database = Database::new(&config.mongo_url, &config.db_name).await;

        Self { config, database }
    }
}
