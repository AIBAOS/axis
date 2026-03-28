# 系统通知详情 API 文档

## 概述

本文档描述 Axis NAS 系统中获取系统通知详情 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/system/notifications/{id}`
- **版本**: v1
- **Phase**: 209

## 认证

- **类型**: JWT Bearer Token
- **权限**: 登录用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `id` | number | 是 | 通知 ID |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "id": 1,
    "type": "info",
    "title": "系统更新",
    "message": "系统已更新到最新版本",
    "source": "system",
    "status": "unread",
    "created_at": 1711500000,
    "read_at": null,
    "metadata": {
      "priority": "normal",
      "target_user_id": null,
      "action_url": "/settings/update"
    }
  }
}
```

### 错误响应

#### 404 Not Found - 通知不存在

```json
{
  "success": false,
  "error": "Notification 999 not found",
  "code": "NOT_FOUND"
}
```

#### 403 Forbidden - 无权查看

```json
{
  "success": false,
  "error": "只能查看自己的通知",
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
  "error": "查询通知失败：数据库错误",
  "code": "DATABASE_ERROR"
}
```

## 数据模型

### NotificationDetail

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 通知 ID |
| `type` | string | 通知类型：`info` / `warning` / `error` / `critical` |
| `title` | string | 通知标题 |
| `message` | string | 通知内容 |
| `source` | string? | 通知来源 |
| `status` | string | 状态：`read` / `unread` |
| `created_at` | number | 创建时间（Unix 时间戳） |
| `read_at` | number? | 已读时间（Unix 时间戳） |
| `metadata` | NotificationMetadata? | 元数据 |

### NotificationMetadata

| 字段 | 类型 | 描述 |
|------|------|------|
| `priority` | string? | 优先级：`low` / `normal` / `high` / `urgent` |
| `target_user_id` | number? | 目标用户 ID（NULL 为系统通知） |
| `action_url` | string? | 操作链接 |

### NotificationDetailResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | NotificationDetail | 通知详情 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 无权查看此通知 |
| `NOT_FOUND` | 404 | 通知不存在 |
| `DATABASE_ERROR` | 500 | 数据库查询失败 |

## 权限说明

### 通知归属规则

| 用户类型 | 可查看的通知 |
|----------|-------------|
| **Admin 用户** | 所有通知（系统通知 + 个人通知） |
| **普通用户** | 仅自己的通知（target_user_id = user_id）+ 系统通知（target_user_id IS NULL） |

### 示例场景

1. **Admin 查看系统通知** → ✅ 允许
2. **Admin 查看他人通知** → ✅ 允许
3. **普通用户查看系统通知** → ✅ 允许
4. **普通用户查看自己的通知** → ✅ 允许
5. **普通用户查看他人通知** → ❌ 403 Forbidden

## 示例

### 请求

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications/1" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### 响应（系统通知）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "type": "info",
    "title": "系统更新",
    "message": "系统已更新到最新版本",
    "source": "system",
    "status": "unread",
    "created_at": 1711500000,
    "read_at": null,
    "metadata": {
      "priority": "normal",
      "target_user_id": null,
      "action_url": "/settings/update"
    }
  }
}
```

### 响应（个人通知）

```json
{
  "success": true,
  "data": {
    "id": 2,
    "type": "warning",
    "title": "存储空间不足",
    "message": "您的存储空间已使用 90%",
    "source": "storage",
    "status": "unread",
    "created_at": 1711600000,
    "read_at": null,
    "metadata": {
      "priority": "high",
      "target_user_id": 123,
      "action_url": "/storage"
    }
  }
}
```

## 实现细节

### 数据库查询

- **查询**: `get_notification_by_id(id)`
- **数据库**: SQLite
- **框架**: Actix-web
- **仓库**: SqliteNotificationRepository

### 状态判断

- `is_read = true` → `status = "read"`
- `is_read = false` → `status = "unread"`

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
- `DELETE /api/v1/system/notifications/{id}` - 删除系统通知

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试查看系统通知
curl -X GET "http://localhost:8080/api/v1/system/notifications/1" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 预期：200 OK + 通知详情

# 测试查看不存在的通知
curl -X GET "http://localhost:8080/api/v1/system/notifications/999" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 预期：404 Not Found
```

## 版本历史

- **Phase 209** (2026-03-28): 初始实现，SQLite 持久化
