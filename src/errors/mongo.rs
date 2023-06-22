use actix_web::http::StatusCode;
use mongodb::error::{
    BulkWriteFailure, CommandError, Error,
    ErrorKind::{BulkWrite, Command, Write},
    WriteError, WriteFailure,
};

use crate::{database::Collection, utils::regex::REGEX_DUPLICATE_KEY};

const DEFAULT_ERROR_MESSAGE: &str = "Something went wrong, please try again later or contact us.";

pub fn mongo_error_handler(error: &Error) -> (StatusCode, String) {
    let error_kind = error.kind.as_ref();
    let default_error_message = DEFAULT_ERROR_MESSAGE.to_string();

    match error_kind {
        Write(WriteFailure::WriteError(WriteError { code, message, .. })) if *code == 11000 => {
            capture(message)
        }
        BulkWrite(BulkWriteFailure { write_errors, .. }) => {
            if let Some(errors) = write_errors.to_owned() {
                if let Some(bulk_write_error) = errors.into_iter().next() {
                    if bulk_write_error.code == 11000 {
                        return capture(&bulk_write_error.message);
                    }
                }
            }

            (StatusCode::INTERNAL_SERVER_ERROR, default_error_message)
        }
        Command(CommandError { code, message, .. }) if *code == 11000 => capture(message),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, default_error_message),
    }
}

fn capture(message: &str) -> (StatusCode, String) {
    let default_error_message = DEFAULT_ERROR_MESSAGE.to_string();

    if let Some(captures) = REGEX_DUPLICATE_KEY.captures(message) {
        use Collection::*;

        let collection = captures.get(2).unwrap().as_str();
        let field = captures.get(3).unwrap().as_str();
        let value = captures.get(4).unwrap().as_str();

        let current = collection.parse::<Collection>().unwrap();

        match (current, field) {
            (Users, "email") => (
                StatusCode::BAD_REQUEST,
                format!("Email: `{}` already exists.", value),
            ),
            (Users, "username") => (
                StatusCode::BAD_REQUEST,
                format!("Username: `{}` already exists.", value),
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, default_error_message),
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, default_error_message)
    }
}
