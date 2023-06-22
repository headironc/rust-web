use actix_web::{
    body::BoxBody, http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError,
};
use serde_json::json;
use thiserror::Error as ThisError;

pub mod json;
mod mongo;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("MongoDB error: {0}")]
    MongoDBError(#[from] mongodb::error::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            MongoDBError(error) => mongo::mongo_error_handler(error).0,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        use Error::*;

        let message = match self {
            MongoDBError(error) => mongo::mongo_error_handler(error).1,
        };

        HttpResponseBuilder::new(self.status_code()).json(json!({
            "message": message,
        }))
    }
}
