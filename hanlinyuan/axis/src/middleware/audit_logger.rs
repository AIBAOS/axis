use axum::{body::Body, async_trait, extract::RequestParts, http::Request, middleware::Next, response::Response};
use serde_json::json;
use crate::AppState;

pub async fn audit_logger<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, axum::extract::rejection::JsonRejection>
where
    B: axum::body::HttpBody + Send + Sync + 'static,
    B::Data: Send,
    B::Error: Send + Sync + 'static,
{
    let method = req.method().clone();
    let uri = req.uri().clone();
    let user_id = 0; // TODO: 从 JWT claims 获取实际 user_id
    
    let response = next.run(req).await;
    
    let status = response.status().as_u16();
    
    let action = format!("{} {}", method, uri);
    let resource = uri.path().to_string();
    let details = Some(json!({
        "status": status,
        "method": method.to_string()
    }).to_string());
    
    let ip_address = "127.0.0.1".to_string(); // TODO: 从请求中提取真实 IP
    
    Ok(response)
}
