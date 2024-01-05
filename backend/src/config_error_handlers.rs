use actix_web::{
    body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, HttpResponseBuilder, ResponseError,
};
use derive_more::Display;

use crate::common::Error;

#[derive(Debug, Display)]
struct JsonConfigErrorHandler<Err: actix_web::ResponseError> {
    error: Err,
}

impl<Err: actix_web::ResponseError> From<Err> for JsonConfigErrorHandler<Err> {
    fn from(value: Err) -> Self {
        Self { error: value }
    }
}

impl<Err: actix_web::ResponseError> ResponseError for JsonConfigErrorHandler<Err> {
    fn status_code(&self) -> StatusCode {
        self.error.status_code()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(self.status_code()).json(Error::new(format!("{}", self.error)))
    }
}

pub fn json_config_error_handler<Err: actix_web::ResponseError + 'static>(
    err: Err,
    _: &HttpRequest,
) -> actix_web::Error {
    JsonConfigErrorHandler::from(err).into()
}
