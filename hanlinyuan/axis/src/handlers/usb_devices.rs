// USB/外部设备管理处理器（SQLite 持久化版）

use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::database::usb_device_store::SqliteUsbDeviceRepository;

#[derive(Debug, Deserialize)]
pub struct UsbDeviceQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub device_type: Option<String>,
}

/// GET /api/v1/usb-devices — USB 设备列表（分页 + status/device_type 筛选）
pub async fn list_usb_devices(
    query: web::Query<UsbDeviceQuery>,
    repo: web::Data<Arc<SqliteUsbDeviceRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(20), 100);

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
pub async fn get_usb_device(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteUsbDeviceRepository>>,
) -> Result<HttpResponse> {
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
pub async fn eject_usb_device(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteUsbDeviceRepository>>,
) -> Result<HttpResponse> {
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
