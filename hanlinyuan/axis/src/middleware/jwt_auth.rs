// JWT 权限校验中间件
// 功能：从 Authorization Header 提取 Token，验证并解析 Claims，注入到请求上下文

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, web,
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
        // 从 Header 中获取 Authorization
        let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(auth) = auth_header {
            if let Some(jwt_data) = req.app_data::<web::Data<Rc<JwtService>>>() {
                let jwt_service = jwt_data.get_ref();

                // 解析 JWT Token
                match extract_claims_from_auth(jwt_service, auth) {
                    Ok(claims) => {
                        // 将 Claims 存储到请求扩展中
                        req.extensions_mut().insert(claims);
                    }
                    Err(_) => {
                        // Token 无效，注入空 Claims（后续 handlers 可检查）
                        req.extensions_mut().insert(JwtClaims {
                            sub: "0".to_string(),
                            user_id: 0,
                            username: "".to_string(),
                            issuer: "".to_string(),
                            audience: "".to_string(),
                            exp: 0,
                            iat: 0,
                            roles: Vec::new(),
                            permissions: Vec::new(),
                        });
                    }
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
