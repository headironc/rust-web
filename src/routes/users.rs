use actix_web::{
    web::{post, resource, scope},
    Scope,
};

use crate::controllers::users::{auth::register, codes::send_registration_code};

pub fn router() -> Scope {
    scope("users")
        .service(resource("registration-code").route(post().to(send_registration_code)))
        .service(resource("register").route(post().to(register)))
        .service(scope("{id}"))
}
