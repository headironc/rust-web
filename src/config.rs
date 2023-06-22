use log::{error, info};
use std::{env::var, net::Ipv4Addr, process};

#[derive(Debug, Clone)]
pub struct Config {
    pub addrs: (Ipv4Addr, u16),
    pub mongo_url: String,
    pub db_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    fn new() -> Self {
        let (mongo_url, db_name) = Self::read_mongo_config();
        let addrs = Self::read_addrs();

        Self {
            mongo_url,
            db_name,
            addrs,
        }
    }

    fn read_addrs() -> (Ipv4Addr, u16) {
        // read env variables
        let host = match var("HOST") {
            Ok(host) => match host.parse::<Ipv4Addr>() {
                Ok(addr) => {
                    if addr.is_loopback() {
                        addr
                    } else {
                        error!("Invalid HOST environment variable, using default");
                        Ipv4Addr::LOCALHOST
                    }
                }
                Err(_) => {
                    error!("Invalid HOST environment variable, using default");
                    Ipv4Addr::LOCALHOST
                }
            },
            Err(_) => {
                info!("HOST environment variable not set, using default");
                Ipv4Addr::LOCALHOST
            }
        };

        let port = match var("PORT") {
            Ok(port) => match port.parse::<u16>() {
                Ok(port) => port,
                Err(_) => {
                    error!("Invalid PORT environment variable, using default");
                    5008
                }
            },
            Err(_) => {
                info!("PORT environment variable not set, using default");
                5008
            }
        };

        (host, port)
    }

    fn read_mongo_config() -> (String, String) {
        // read env variables
        let mongo_url = match var("MONGO_URL") {
            Ok(mongo_url) => mongo_url,
            Err(_) => {
                error!("Please set MONGO_URL environment variable");
                process::exit(1);
            }
        };

        let db_name = match var("DB_NAME") {
            Ok(db_name) => db_name,
            Err(_) => {
                error!("Please set DB_NAME environment variable");
                process::exit(1);
            }
        };

        (mongo_url, db_name)
    }
}
