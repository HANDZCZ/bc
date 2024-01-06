use actix_web::{
    body::BoxBody, http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError,
};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn new<T>(error: T) -> Self
    where
        T: Into<String>,
    {
        Error {
            error: error.into(),
        }
    }
}

#[derive(Debug)]
pub struct JsonError {
    error: Error,
    status_code: StatusCode,
}

impl JsonError {
    pub fn new(error: Error, status_code: StatusCode) -> Self {
        JsonError { error, status_code }
    }
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for JsonError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(self.status_code()).json(&self.error)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonString(pub Box<RawValue>);

impl From<String> for JsonString {
    fn from(value: String) -> Self {
        JsonString(RawValue::from_string(value).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tournament_type")]
pub enum TournamentType {
    FFA,
    OneBracketTwoFinalPositions,
    OneBracketOneFinalPositions,
}