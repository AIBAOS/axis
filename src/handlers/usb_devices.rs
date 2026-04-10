// USB/外部设备管理处理器（SQLite 持久化版）

use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::database::usb_device_store::SqliteUsbDeviceRepository;
use crate::services::jwt_service::JwtService;

#[derive(Debug, Deserialize)]
pub struct UsbDeviceQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub device_type: Option<String>,
}

/// JWT 认证辅助函数
fn validate_auth(req: &HttpRequest, jwt_service: &web::Data<JwtService>) -> Result<crate::models::jwt::JwtClaims, HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Err(HttpResponse::Unauthorized().json(json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    jwt_service.validate_token(&token.expect("Token should exist"))
        .map_err(|_| HttpResponse::Unauthorized().json(json!({
            "success": false,
            "message": "Invalid token"
        })))
}

/// GET /api/v1/usb-devices — USB 设备列表（分页 + status/device_type 筛选）
/// 需要登录用户访问
pub async fn list_usb_devices(
    http_req: HttpRequest,
    query: web::Query<UsbDeviceQuery>,
    repo: web::Data<Arc<SqliteUsbDeviceRepository>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let page = query.page.unwrap_or(1).max(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(20).max(1), 100); // Bug #89 修复

    match repo.get_usb_devices(
        query.status.as_deref(),
        query.device_type.as_deref(),
        page,
        per_page,
    ) {
        Ok((devices, total)) => {
            let total_pages = if total == 0 { 1 } else { (total as u32 + per_page - 1) / per_page };
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": devices,
                "pagination": {
                    "page": page,
                    "per_page": per_page,
                    "total": total,
                    "total_pages": total_pages
                }
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询 USB 设备列表失败: {}", e)
        }))),
    }
}

/// GET /api/v1/usb-devices/{id} — USB 设备详情
/// 需要登录用户访问
pub async fn get_usb_device(
    http_req: HttpRequest,
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteUsbDeviceRepository>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let id = path.into_inner();
    match repo.get_usb_device_by_id(id) {
        Ok(Some(device)) => Ok(HttpResponse::Ok().json(json!({ "success": true, "data": device }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("USB device '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询 USB 设备失败: {}", e)
        }))),
    }
}

/// POST /api/v1/usb-devices/{id}/eject — 安全弹出设备
/// 需要登录用户访问
pub async fn eject_usb_device(
    http_req: HttpRequest,
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteUsbDeviceRepository>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let id = path.into_inner();
    match repo.get_usb_device_by_id(id) {
        Ok(Some(device)) => {
            if device.status == "ejected" {
                return Ok(HttpResponse::Conflict().json(json!({
                    "success": false,
                    "message": format!("设备 '{}' 已弹出", device.name)
                })));
            }
            match repo.eject_device(id) {
                Ok(true) => Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "message": format!("设备 '{}' 已安全弹出", device.name)
                }))),
                Ok(false) => Ok(HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "message": "弹出设备失败：状态更新异常"
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "message": format!("弹出设备失败: {}", e)
                }))),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("USB device '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询 USB 设备失败: {}", e)
        }))),
    }
}
