use actix_web::{Middleware, Service, ServiceRequest, ServiceResponse, Error};
use actix_web::dev::ServiceExt;
use std::rc::Rc;

/// JWT 认证中间件
pub struct JwtAuth {
    // 中间件配置
}

impl JwtAuth {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Middleware<S> for JwtAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    fn start(&self, req: ServiceRequest) -> Result<S::Future, Error> {
        // 从 Authorization 头获取 Token
        let auth_header = req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());
        
        match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                // Token 存在，继续服务
                req.into_future().await
            }
            _ => {
                // 无 Token，返回 401
                let (req, _) = req.into_parts();
                Ok(req.into_future())
            }
        }
    }
}
