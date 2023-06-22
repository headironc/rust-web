use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use env_logger::{init_from_env, Env};

use headiron_rust::{routes::configure, state::State};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    init_from_env(Env::new().default_filter_or("info"));

    let state = State::new().await;

    let config = state.config.to_owned();
    let addrs = config.addrs;

    log::info!("Starting server at: {:?}", addrs);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(Logger::new("%a %r %s"))
            .configure(configure)
    })
    .bind(addrs)?
    .run()
    .await
}
