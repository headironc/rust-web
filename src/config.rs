use log::{error, info};
use std::{env::var, net::Ipv4Addr, process};

#[derive(Debug, Clone)]
pub struct Config {
    pub addrs: (Ipv4Addr, u16),
    pub mongo_config: MongoConfig,
    pub redis_url: String,
    pub email_config: EmailConfig,
    pub code_expire: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    fn new() -> Self {
        let addrs = Self::read_addrs();
        let mongo_config = MongoConfig::new();
        let redis_url = Self::read_redis_url();
        let email_config = EmailConfig::new();
        let code_expire = Self::read_code_expire();

        Self {
            addrs,
            mongo_config,
            redis_url,
            email_config,
            code_expire,
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

    fn read_redis_url() -> String {
        // read env variables
        match var("REDIS_URL") {
            Ok(redis_url) => redis_url,
            Err(_) => {
                error!("Please set REDIS_URL environment variable");
                process::exit(1);
            }
        }
    }

    fn read_code_expire() -> i64 {
        // read env variables
        match var("CODE_EXPIRE") {
            Ok(code_expire) => match code_expire.parse::<i64>() {
                Ok(code_expire) => code_expire,
                Err(_) => {
                    error!("Invalid CODE_EXPIRE environment variable, using default 15 minutes");
                    15
                }
            },
            Err(_) => {
                info!("CODE_EXPIRE environment variable not set, using default 15 minutes");
                15
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MongoConfig {
    pub mongo_url: String,
    pub db_name: String,
}

impl MongoConfig {
    fn new() -> Self {
        let (mongo_url, db_name) = Self::read_mongo_config();

        Self { mongo_url, db_name }
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

#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub host: String,
    pub port: u16,
    pub from: String,
    pub reply_to: String,
    pub username: String,
    pub password: String,
}

impl EmailConfig {
    fn new() -> Self {
        let (host, port, from, reply_to, username, password) = Self::read_email_config();

        Self {
            host,
            port,
            from,
            reply_to,
            username,
            password,
        }
    }

    fn read_email_config() -> (String, u16, String, String, String, String) {
        let host = match var("EMAIL_HOST") {
            Ok(host) => host,
            Err(_) => {
                error!("Please set EMAIL_HOST environment variable");
                process::exit(1);
            }
        };

        let port = match var("EMAIL_PORT") {
            Ok(port) => match port.parse::<u16>() {
                Ok(port) => port,
                Err(_) => {
                    error!("Please set a valid EMAIL_PORT environment variable");
                    process::exit(1);
                }
            },
            Err(_) => {
                error!("Please set EMAIL_PORT environment variable");
                process::exit(1);
            }
        };

        let from = match var("EMAIL_FROM") {
            Ok(from) => from,
            Err(_) => {
                error!("Please set EMAIL_FROM environment variable");
                process::exit(1);
            }
        };

        let reply_to = match var("EMAIL_REPLY_TO") {
            Ok(reply_to) => reply_to,
            Err(_) => {
                error!("Please set EMAIL_REPLY_TO environment variable");
                process::exit(1);
            }
        };

        let username = match var("EMAIL_USERNAME") {
            Ok(username) => username,
            Err(_) => {
                error!("Please set EMAIL_USERNAME environment variable");
                process::exit(1);
            }
        };

        let password = match var("EMAIL_PASSWORD") {
            Ok(password) => password,
            Err(_) => {
                error!("Please set EMAIL_PASSWORD environment variable");
                process::exit(1);
            }
        };

        (host, port, from, reply_to, username, password)
    }
}
