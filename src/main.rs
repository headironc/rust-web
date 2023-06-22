use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, SessionMiddleware};
use actix_web::{cookie::time::Duration, middleware::Logger, web::Data, App, HttpServer};
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
        let redis = state.redis.to_owned();

        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(redis.store, redis.key)
                    .cookie_name("headiron-session".to_string())
                    .cookie_secure(cfg!(release))
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(3)))
                    .build(),
            )
            .wrap(Logger::new("%a %r %s"))
            .configure(configure)
    })
    .bind(addrs)?
    .run()
    .await
}
