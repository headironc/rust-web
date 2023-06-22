use actix_web::{
    error::{Error, InternalError, JsonPayloadError},
    HttpRequest, HttpResponse,
};

use crate::errors::json;

pub fn json_error_handler(error: JsonPayloadError, _req: &HttpRequest) -> Error {
    use JsonPayloadError::{ContentType, Deserialize};

    let res = match &error {
        ContentType => HttpResponse::UnsupportedMediaType().json(json!({
            "message": "Unsupported content type.",
        })),
        Deserialize(error) => HttpResponse::BadRequest().json(json!({
            "message": error.to_string() + ".",
        })),
        _ => HttpResponse::BadRequest().json(json!({
            "message": "Bad request.",
        })),
    };

    InternalError::from_response(error, res).into()
}
