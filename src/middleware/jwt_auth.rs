// JWT 权限校验中间件
// 功能：从 Authorization Header 提取 Token，验证并解析 Claims，注入到请求上下文

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, web, HttpResponse,
};
use futures_util::future::{ok, Ready};
use std::rc::Rc;

use crate::models::jwt::JwtClaims;
use crate::services::jwt_service::JwtService;

// JWT 中间件
pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware { service: Rc::new(service) })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Bug #79 修复：公开路径不需要认证
        let path = req.path();
        let public_paths = [
            "/api/v1/health",
            "/api/v1/auth/login",
            "/api/v1/auth/refresh",
        ];
        
        if public_paths.contains(&path) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        // 从 Header 中获取 Authorization
        let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());

        // Bug #79 修复：无 token 或无效 token 应拒绝请求
        if auth_header.is_none() {
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized("Authentication required"))
            });
        }

        let auth = auth_header.unwrap();
        
        if let Some(jwt_data) = req.app_data::<web::Data<Rc<JwtService>>>() {
            let jwt_service = jwt_data.get_ref();

            // 解析 JWT Token
            match extract_claims_from_auth(jwt_service, auth) {
                Ok(claims) => {
                    // 将 Claims 存储到请求扩展中
                    req.extensions_mut().insert(claims);
                }
                Err(e) => {
                    // Bug #79 修复：无效 token 拒绝请求
                    return Box::pin(async move {
                        Err(actix_web::error::ErrorUnauthorized("Invalid or expired token"))
                    });
                }
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

// 从 Authorization Header 解析 JWT Claims
fn extract_claims_from_auth(
    jwt_service: &JwtService,
    auth_header: &str,
) -> Result<JwtClaims, Error> {
    let parts: Vec<&str> = auth_header.split_whitespace().collect();
    if parts.len() != 2 || parts[0] != "Bearer" {
        return Err(Error::from(actix_web::error::ErrorUnauthorized("Invalid Authorization header format")));
    }

    let token = parts[1];
    let claims = jwt_service.validate_token(token).map_err(|e| {
        log::error!("Invalid JWT token: {}", e);
        Error::from(actix_web::error::ErrorUnauthorized("Invalid token"))
    })?;

    Ok(claims)
}
