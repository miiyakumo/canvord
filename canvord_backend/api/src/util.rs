use dto::app_error::AppError;
use dto::app_response::AppResponse;
use schemars::JsonSchema;
use serde::Serialize;
use validator::Validate;

pub async fn handle_api_result<T>(res: Result<T, AppError>) -> AppResponse<T>
where
    T: Serialize + JsonSchema,
{
    match res {
        Ok(data) => AppResponse::ok(data),
        Err(e) => AppResponse::from_error(&e),
    }
}

pub fn validate<T: Validate>(val: &T) -> Result<(), AppError> {
    val.validate().map_err(AppError::from)
}