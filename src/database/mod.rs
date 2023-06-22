use log::{error, info};
use mongodb::{bson::doc, options::ClientOptions, Client, Database as MongoDatabase};
use std::process;

#[derive(Debug, Clone)]
pub struct Database {
    pub client: Client,
    pub db: MongoDatabase,
}

impl Database {
    pub async fn new(mongo_url: &str, db_name: &str) -> Self {
        let client = Self::connect(mongo_url).await;
        let db = client.database(db_name);

        Self { client, db }
    }

    async fn connect(mongo_url: &str) -> Client {
        let option = match ClientOptions::parse(mongo_url).await {
            Ok(option) => option,
            Err(e) => {
                error!("Failed to parse mongo url: {}", e);
                process::exit(1);
            }
        };

        match Client::with_options(option) {
            Ok(client) => {
                // test connect
                match client
                    .database("admin")
                    .run_command(doc! {"ping": 1}, None)
                    .await
                {
                    Ok(_) => {
                        info!("Connected to mongodb");
                    }
                    Err(e) => {
                        error!("Failed to connect to mongo: {}", e);
                        process::exit(1);
                    }
                };

                client
            }
            Err(e) => {
                error!("Failed to create mongo client: {}", e);
                process::exit(1);
            }
        }
    }
}
