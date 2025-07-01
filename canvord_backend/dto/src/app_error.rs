use crate::app_response::AppResponse;
use actix_web::{HttpResponse, ResponseError};
use apistos::ApiComponent;
use schemars::JsonSchema;
use sea_orm::DbErr;
use thiserror::Error;
use validator::ValidationError;

#[derive(Debug, Error, JsonSchema, ApiComponent)]
pub enum AppError {
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Internal Server Error")]
    InternalError,
    #[error("Database Error: {0}")]
    DbError(String),
}

impl AppError {
    pub fn code(&self) -> i32 {
        match self {
            AppError::NotFound(_) => 404,
            AppError::BadRequest(_) => 400,
            AppError::InternalError => 500,
            AppError::DbError(_) => 500,
        }
    }

    pub fn http_code(&self) -> u16 {
        self.code() as u16
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(actix_web::http::StatusCode::from_u16(self.http_code()).unwrap())
            .json(AppResponse::<()> {
                code: self.code(),
                message: self.to_string(),
                data: None,
            })
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DbError(err.to_string())
    }
}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::BadRequest(err.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::BadRequest(err.to_string())
    }
}
