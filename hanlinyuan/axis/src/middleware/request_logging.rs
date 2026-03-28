use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error};
use tracing::{info, debug};
use uuid::Uuid;
use std::time::Instant;

// 日志中间件结构体（实现 Transform trait 适配 actix-web 4.x）
pub struct RequestLogging;

impl<S> actix_web::dev::Transform<S, ServiceRequest> for RequestLogging
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = RequestLoggingMiddleware<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(RequestLoggingMiddleware { service }))
    }
}

pub struct RequestLoggingMiddleware<S> {
    service: S,
}

impl<S> actix_web::dev::Service<ServiceRequest> for RequestLoggingMiddleware<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        let client_ip = req
            .connection_info()
            .peer_addr()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let method = req.method().to_string();
        let path = req.path().to_string();
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();
        let content_length = req
            .headers()
            .get("content-length")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.parse::<u64>().ok())
            .flatten()
            .unwrap_or(0);
        let start_time = Instant::now();

        debug!(
            request_id = request_id,
            client_ip = client_ip,
            method = method,
            path = path,
            user_agent = user_agent,
            "Incoming request"
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let status = res.status().as_u16();
            let latency = start_time.elapsed().as_millis();

            info!(
                request_id = request_id,
                status = status,
                latency_ms = latency,
                path = path,
                content_length = content_length
            );

            Ok(res)
        })
    }
}
