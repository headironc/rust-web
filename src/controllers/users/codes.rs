use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use rand::{thread_rng, Rng};
use validator::Validate;

use crate::{
    controllers::Response,
    errors::Error::BadRequest,
    models::users::{
        codes::{Code, CodeType::Registration},
        mail_validator::MailValidator,
        User,
    },
    state::State,
};

pub async fn send_registration_code(
    Json(email_validator): Json<MailValidator>,
    state: Data<State>,
) -> Response {
    email_validator.validate()?;

    let email = email_validator.email;

    // check if user with this email already exists
    if User::find_one_by_email(email.to_owned(), &state.database)
        .await?
        .is_some()
    {
        return Err(BadRequest(format!(
            "User with email `{}` already exists.",
            email
        )));
    }

    // check if registration code for this email already exists
    if let Some(code) =
        Code::find_one_by_email(email.to_owned(), Registration, &state.database).await?
    {
        // if code is not expired, return error
        if !code.is_expired() {
            return Err(BadRequest(format!(
                "Registration code for email `{}` already exists, please check your email or try again later.",
                email
            )));
        } else {
            // if code is expired, deactivate it
            code.deactivate_by_id(&state.database).await?;
        }
    }

    // generate new code
    let mut rng = thread_rng();
    let code = rng.gen_range(100000..999999).to_string();

    // send email with code
    state
        .email
        .send_registration_code(
            email.to_owned(),
            "Register An Account",
            "Please use the following code to register an account",
            code.to_owned(),
            state.config.code_expire,
        )
        .await?;

    // create new code instance
    let code = Code::new(email, code, Registration, state.config.code_expire);

    // save code to database
    Code::create(code, &state.database).await?;

    Ok(HttpResponse::Created().finish())
}
