use dto::app_error::AppError;
use dto::app_response::AppResponse;
use schemars::JsonSchema;
use serde::Serialize;

pub async fn handle_api_result<T>(res: Result<T, AppError>) -> AppResponse<T>
where
    T: Serialize + JsonSchema,
{
    match res {
        Ok(data) => AppResponse::ok(data),
        Err(e) => AppResponse::from_error(&e),
    }
}