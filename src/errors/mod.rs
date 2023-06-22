use actix_web::{
    body::BoxBody, http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError,
};
use log::error;
use serde_json::json;
use thiserror::Error as ThisError;

pub mod json;
mod mongo;
mod validation;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("MongoDB error: {0}")]
    MongoDBError(#[from] mongodb::error::Error),
    #[error("Validation error: {0}")]
    ValidationErrors(#[from] validator::ValidationErrors),
    #[error("Lettre error: {0}")]
    LettreError(#[from] lettre::error::Error),
    #[error("Lettre SMTP error: {0}")]
    LettreSmtpError(#[from] lettre::transport::smtp::Error),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Handlebars render error: {0}")]
    HandlebarsRenderError(#[from] handlebars::RenderError),
    #[error("Handlebars template error: {0}")]
    HandlebarsTemplateError(#[from] Box<handlebars::TemplateError>),
    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            MongoDBError(error) => mongo::mongo_error_handler(error).0,
            ValidationErrors(_) | BadRequest(_) => StatusCode::BAD_REQUEST,
            LettreError(_)
            | LettreSmtpError(_)
            | HandlebarsRenderError(_)
            | HandlebarsTemplateError(_)
            | AnyhowError(_)
            | InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let default_error_message =
            "Something went wrong. Please try again later or contact us.".to_string();

        use Error::*;

        error!("Error: {:?}", self);

        let message = match self {
            MongoDBError(error) => mongo::mongo_error_handler(error).1,
            ValidationErrors(error) => validation::validation_error_handler(error),
            LettreError(_)
            | LettreSmtpError(_)
            | HandlebarsRenderError(_)
            | HandlebarsTemplateError(_)
            | AnyhowError(_) => default_error_message,
            BadRequest(message) => message.to_string(),
            NotFound(message) => message.to_string(),
            InternalServerError(message) => {
                if cfg!(debug_assertions) {
                    message.to_string()
                } else {
                    default_error_message
                }
            }
        };

        HttpResponseBuilder::new(self.status_code()).json(json!({
            "message": message,
        }))
    }
}
