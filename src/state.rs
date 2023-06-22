use crate::{
    config::Config,
    database::{redis::Redis, Database},
    utils::email::Email,
};

#[derive(Clone)]
pub struct State {
    pub config: Config,
    pub database: Database,
    pub redis: Redis,
    pub email: Email,
}

impl State {
    pub async fn new() -> Self {
        let config = Config::default();
        let mongo_config = config.mongo_config.to_owned();
        let email_config = config.email_config.to_owned();

        let database = Database::new(mongo_config).await;
        let redis = Redis::new(config.redis_url.to_owned()).await;
        let email = Email::new(email_config);

        Self {
            config,
            database,
            redis,
            email,
        }
    }
}
