# 打印机详情 API 文档 (Phase 54)

## 概述

打印机详情 API 允许管理员获取单个打印机的详细信息。

## 接口详情

### GET /api/v1/printers/{id}

获取指定打印机的详细信息。

#### 认证要求

需要有效的 JWT Token，且用户必须具有 `admin` 角色。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 打印机 ID |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "message": "打印机详情获取成功",
  "data": {
    "id": 1,
    "name": "HP LaserJet Pro",
    "model": "HP LaserJet Pro MFP M125nw",
    "manufacturer": "HP",
    "status": "idle",
    "location": "一楼办公区",
    "ip_address": "192.168.1.101",
    "port": 9100,
    "usb_device": null,
    "network_path": "\\\\192.168.1.101\\printer",
    "queue_jobs_count": 0,
    "created_at": "2026-03-15T10:00:00Z",
    "updated_at": "2026-03-18T15:30:00Z"
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `message`: 响应消息
- `data`: 打印机详细信息
  - `id`: 打印机 ID
  - `name`: 打印机名称
  - `model`: 型号
  - `manufacturer`: 制造商
  - `status`: 当前状态
  - `location`: 位置
  - `ip_address`: IP 地址
  - `port`: 端口号（默认 9100）
  - `usb_device`: USB 设备路径（可选）
  - `network_path`: 网络共享路径（可选）
  - `queue_jobs_count`: 打印队列中的任务数
  - `created_at`: 创建时间
  - `updated_at`: 更新时间

**错误响应 (404 Not Found) - 打印机不存在**

```json
{
  "success": false,
  "message": "打印机不存在",
  "code": "NOT_FOUND"
}
```

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "message": "仅管理员可访问打印机详情",
  "code": "FORBIDDEN"
}
```

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "message": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：获取打印机详情

```bash
curl -X GET "http://localhost:8080/api/v1/printers/1" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 示例 2：获取不存在的打印机（404）

```bash
curl -X GET "http://localhost:8080/api/v1/printers/999" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "message": "打印机不存在",
  "code": "NOT_FOUND"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可访问
- 非 admin 角色返回 403 Forbidden

## 实现文件

- `src/handlers/printers_get.rs` - 打印机详情处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **权限控制**：仅 admin 角色可访问打印机详情
2. **404 处理**：打印机不存在时返回 404
3. **数据实时性**：打印机状态可能变化，建议定期刷新

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `NOT_FOUND` | 404 | 打印机不存在 |

## 相关 API

- **GET /api/v1/printers** - 获取打印机列表 (Phase 53)
- **GET /api/v1/printers/{id}/jobs** - 获取打印队列
- **POST /api/v1/printers/{id}/jobs** - 创建打印任务
