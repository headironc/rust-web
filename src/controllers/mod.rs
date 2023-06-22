use actix_web::HttpResponse;

use crate::errors::Error;

pub mod users;

pub type Response = Result<HttpResponse, Error>;
