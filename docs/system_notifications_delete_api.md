# 系统通知删除 API 文档

## 概述

本文档描述 Axis NAS 系统中删除系统通知 API 的实现细节。

## API 端点

- **路径**: `DELETE /api/v1/system/notifications/{id}`
- **版本**: v1
- **Phase**: 208

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `id` | number | 是 | 系统通知 ID |

## 响应格式

### 成功响应 (204 No Content)

```
HTTP/1.1 204 No Content
```

### 错误响应

#### 404 Not Found - 通知不存在

```json
{
  "success": false,
  "error": "System notification 999 not found",
  "code": "NOT_FOUND"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can delete system notifications",
  "code": "FORBIDDEN"
}
```

#### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "删除系统通知失败：数据库错误",
  "code": "DATABASE_ERROR"
}
```

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试删除 |
| `NOT_FOUND` | 404 | 通知不存在或非系统通知 |
| `DATABASE_ERROR` | 500 | 数据库操作失败 |

## 示例

### 请求

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/notifications/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 成功响应

```
HTTP/1.1 204 No Content
```

### 错误响应（非系统通知）

```json
{
  "success": false,
  "error": "System notification 2 not found",
  "code": "NOT_FOUND"
}
```

## 权限说明

- **Admin 用户**: 可删除任意系统通知（target_user_id IS NULL）
- **普通用户**: 无权访问（返回 403 Forbidden）

## 删除条件

### 允许删除
- 通知存在且为系统通知（target_user_id IS NULL）
- 用户具有 admin 权限

### 禁止删除
- 通知不存在（404）
- 通知为个人通知（target_user_id 有值）（404）
- 用户无 admin 权限（403）

## 实现细节

### 系统通知验证

- 仅允许删除 `target_user_id IS NULL` 的系统通知
- 个人通知（target_user_id 有值）返回 404 Not Found

### 数据库操作

- **查询**: `get_notification_by_id(id)`
- **删除**: `delete_notification(id)`
- **数据库**: SQLite
- **框架**: Actix-web
- **仓库**: SqliteNotificationRepository

## 数据库表结构

```sql
CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    type TEXT NOT NULL DEFAULT 'info',
    priority TEXT NOT NULL DEFAULT 'normal',
    source TEXT,
    target_user_id INTEGER,
    is_read INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    read_at INTEGER,
    action_url TEXT
);
```

## 相关接口

- `GET /api/v1/system/notifications` - 获取系统通知列表
- `PUT /api/v1/system/notifications/{id}/read` - 标记系统通知为已读
- `POST /api/v1/system/notifications/{id}/mark-read` - 标记通知为已读

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试删除系统通知
curl -X DELETE "http://localhost:8080/api/v1/system/notifications/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：204 No Content

# 测试删除个人通知（应失败）
curl -X DELETE "http://localhost:8080/api/v1/system/notifications/2" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：404 Not Found
```

## 版本历史

- **Phase 208** (2026-03-28): 初始实现，SQLite 持久化
