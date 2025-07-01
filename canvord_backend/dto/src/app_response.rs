use crate::app_error::AppError;
use actix_web::{HttpResponse, Responder};
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct AppResponse<T: JsonSchema> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: JsonSchema> AppResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "OK".to_string(),
            data: Some(data),
        }
    }

    pub fn message(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn from_error(err: &AppError) -> Self {
        Self {
            code: err.code(),
            message: err.to_string(),
            data: None,
        }
    }
}

impl<T> Responder for AppResponse<T>
where
    T: Serialize + JsonSchema,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(self)
    }
}