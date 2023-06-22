use actix_web::web::{route, scope, JsonConfig, ServiceConfig};
use serde_qs::actix::QsQueryConfig;

use crate::errors::json::json_error_handler;

mod default;
mod users;

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(scope("/api").service(scope("/v1").service(users::router())))
        .default_service(route().to(default::not_found))
        .app_data(JsonConfig::default().error_handler(json_error_handler))
        .app_data(QsQueryConfig::default());
}
