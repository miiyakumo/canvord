use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use redis::{AsyncCommands, Client};
use std::{future::{ready, Ready}, rc::Rc};
use actix_web::error::ErrorInternalServerError;

/// 缓存中间件构造器（实现 Transform）
pub struct CacheMiddleware {
    pub client: Client
}

impl<S, B> Transform<S, ServiceRequest> for CacheMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = CacheMiddlewareImpl<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CacheMiddlewareImpl {
            service: Rc::new(service),
            client: self.client.clone()
        }))
    }
}

/// 缓存中间件实现体（Service）
pub struct CacheMiddlewareImpl<S> {
    service: Rc<S>,
    client: Client
}

impl<S, B> Service<ServiceRequest> for CacheMiddlewareImpl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        if req.method() != actix_web::http::Method::GET {
            let fut = self.service.call(req);
            return Box::pin(async move {
                fut.await.map(|res| res.map_into_left_body())
            });
        }

        let path = req.path().to_owned();
        let svc = Rc::clone(&self.service);
        let client = self.client.clone();

        Box::pin(async move {
            // 初始化 Redis 客户端和连接
            let mut conn = client
                .get_multiplexed_async_connection()
                .await
                .map_err(ErrorInternalServerError)?;

            // 读取缓存
            let cached_opt: Option<String> = conn.get(&path).await.map_err(ErrorInternalServerError)?;

            if let Some(cached) = cached_opt {
                // 有缓存，直接返回缓存内容
                return Ok(ServiceResponse::new(
                    req.request().clone(),
                    HttpResponse::Ok().body(cached).map_into_right_body(),
                ));
            }

            // 缓存未命中，执行下游服务
            let res = svc.call(req).await?;

            // 将响应体转成 Bytes 以读取内容
            let (req, res) = res.into_parts();
            let res_body = res.into_body();  // 这里拿到响应体，impl MessageBody
            let body_bytes = actix_web::body::to_bytes(res_body).await
                .map_err(|_| ErrorInternalServerError("to_bytes error"))?;
            
            let body_string = String::from_utf8_lossy(&body_bytes).to_string();

            // 写入缓存
            let _: () = conn.set_ex(&path, body_string.clone(), 660).await
                .map_err(ErrorInternalServerError)?;

            // 构造新的响应返回
            let response = HttpResponse::Ok()
                .body(body_string)
                .map_into_right_body();

            Ok(ServiceResponse::new(req, response))
        })
    }
}