use actix_identity::Identity;
use actix_web::{
    web::{Data, Json},
    HttpMessage, HttpRequest, HttpResponse,
};
use serde_json::json;
use validator::Validate;

use crate::{
    controllers::Response,
    errors::Error::BadRequest,
    models::{
        users::{
            auth::Registrar,
            codes::{Code, CodeType},
            User,
        },
        IntoJson,
    },
    state::State,
};

pub async fn register(
    Json(registrar): Json<Registrar>,
    state: Data<State>,
    request: HttpRequest,
) -> Response {
    registrar.validate()?;

    let email = registrar.email.to_owned();
    let candidate = registrar.code.to_owned();

    let invalid_code =
        BadRequest("Invalid registration code, please check your email and try again".to_owned());

    // check if the code is valid
    match Code::find_one_by_email(email, CodeType::Registration, &state.database).await? {
        Some(code) => {
            // if code is not expired, return error
            if !code.is_valid(candidate) {
                return Err(invalid_code);
            }

            // if code is valid, deactivate it
            code.deactivate_by_id(&state.database).await?;
        }
        None => {
            // if no code found, return error
            return Err(invalid_code);
        }
    }

    let new_user = registrar.build();

    User::create(&new_user, &state.database).await?;

    Identity::login(&request.extensions(), new_user.id.to_hex())?;

    let value = new_user.into_json();

    Ok(HttpResponse::Created().json(json!({ "user": value })))
}
