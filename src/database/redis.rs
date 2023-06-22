use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
use log::error;
use std::process;

#[derive(Clone)]
pub struct Redis {
    pub key: Key,
    pub store: RedisSessionStore,
}

impl Redis {
    pub async fn new(redis_url: String) -> Self {
        let key = Key::generate();
        let store = match RedisSessionStore::new(redis_url).await {
            Ok(store) => {
                log::info!("Connected to redis");
                store
            }
            Err(err) => {
                error!("Failed to connect to redis: {}", err);
                process::exit(1);
            }
        };

        Self { key, store }
    }
}
