use log::{error, info};
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Collection as MongoCollection, Database as MongoDatabase, IndexModel,
};
use std::{process, str::FromStr};

use crate::{config::MongoConfig, models::users::User};

pub mod redis;

#[derive(Debug, Clone)]
pub struct Database {
    db: MongoDatabase,
}

impl Database {
    pub async fn new(mongo_config: MongoConfig) -> Self {
        use Collection::*;

        let client = Self::connect(&mongo_config.mongo_url).await;
        let db = client.database(&mongo_config.db_name);

        Self::create_unique_indexes::<User>(&db, Users, vec!["email", "username"]).await;

        Self { db }
    }

    pub fn collection<T>(&self, collection: Collection) -> MongoCollection<T> {
        self.db.collection::<T>(collection.into())
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

    async fn create_unique_indexes<T>(
        database: &MongoDatabase,
        collection: Collection,
        keys: Vec<&'static str>,
    ) {
        let name: &str = collection.into();

        let connection = database.collection::<T>(name);

        let indexes = keys.into_iter().map(|key| {
            let options = IndexOptions::builder()
                .name(key.to_string())
                .unique(true)
                .build();

            IndexModel::builder()
                .keys(doc! {
                    key: 1
                })
                .options(options)
                .build()
        });

        match connection.create_indexes(indexes, None).await {
            Ok(_) => {
                info!("Created unique indexes for {}", name);
            }
            Err(e) => {
                error!("Failed to create unique indexes for {}: {}", name, e);
                process::exit(1);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Collection {
    Users,
    Codes,
}

impl From<Collection> for &str {
    fn from(collection: Collection) -> Self {
        use Collection::*;

        match collection {
            Users => "users",
            Codes => "codes",
        }
    }
}

impl FromStr for Collection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Collection::*;

        match s {
            "users" => Ok(Users),
            "codes" => Ok(Codes),
            _ => Err("Invalid collection name, must be one of: users"),
        }
    }
}
