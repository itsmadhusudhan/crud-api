use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// error
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum CustomError {
    #[error("Name cannot be empty")]
    NameEmpty,
    #[error("Task not found")]
    NotFound,
}

impl ResponseError for CustomError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CustomError::NameEmpty => actix_web::http::StatusCode::BAD_REQUEST,
            CustomError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .json(serde_json::json!({ "message": self.to_string(), "code": self.status_code().as_u16() }))
    }
}
