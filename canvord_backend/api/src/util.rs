use std::env;
use dto::app_error::AppError;
use dto::app_response::AppResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;
use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, Service, Payload}, Error, HttpMessage, FromRequest, HttpRequest};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use once_cell::sync::Lazy;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey,};
use std::task::{Context, Poll};
use std::rc::Rc;
use chrono::{Duration, Utc};

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

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if let Some(token) = auth_str.strip_prefix("Bearer ") {
                        if let Ok(data) = decode::<Claims>(
                            token,
                            &DecodingKey::from_secret(SECRET_KEY.as_ref()),
                            &Validation::new(Algorithm::HS256),
                        ) {
                            let claims = data.claims;
                            req.extensions_mut().insert(AuthenticatedUser {
                                user_id: claims.sub,
                                role: claims.role,
                            });
                        }
                    }
                }
            }

            // ❗️如果没有用户信息，拒绝请求
            if req.extensions().get::<AuthenticatedUser>().is_none() {
                return Err(actix_web::error::ErrorUnauthorized("Missing or invalid token"));
            }

            service.call(req).await
        })
    }
}


static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    dotenvy::dotenv().ok(); // 加载 .env 文件
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
}

pub struct AuthenticatedUser {
    pub user_id: String,
    pub role: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    if let Ok(data) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
                        &Validation::new(Algorithm::HS256),
                    ) {
                        let claims = data.claims;
                        return ready(Ok(AuthenticatedUser {
                            user_id: claims.sub,
                            role: claims.role,
                        }));
                    }
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Invalid or missing token")))
    }
}