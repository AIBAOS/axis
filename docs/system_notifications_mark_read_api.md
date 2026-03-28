# 系统通知标记已读 API 文档

## 概述

本文档描述 Axis NAS 系统中系统通知标记已读 API 的实现细节。

## API 端点

### 端点 1: 标记系统通知为已读 (Phase 199)

- **路径**: `PUT /api/v1/system/notifications/{id}/read`
- **版本**: v1
- **Phase**: 199

### 端点 2: 标记通知为已读 (Phase 200)

- **路径**: `POST /api/v1/system/notifications/{id}/mark-read`
- **版本**: v1
- **Phase**: 200

## 认证

- **类型**: JWT Bearer Token
- **权限**: 登录用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `id` | number | 是 | 通知 ID |

## 响应格式

### 成功响应

```json
{
  "success": true,
  "message": "通知 '系统更新' 已标记为已读",
  "notification": {
    "id": 1,
    "title": "系统更新",
    "message": "系统已更新到最新版本",
    "type": "info",
    "is_read": true,
    "read_at": 1711600000
  }
}
```

### 错误响应

```json
{
  "success": false,
  "error": "错误描述",
  "code": "ERROR_CODE"
}
```

## 数据模型

### MarkAsReadResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `notification` | NotificationSummary? | 通知摘要（可选） |

### NotificationSummary

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 通知 ID |
| `title` | string | 通知标题 |
| `message` | string | 通知内容 |
| `type` | string | 通知类型：`info` / `warning` / `error` / `critical` |
| `is_read` | boolean | 是否已读 |
| `read_at` | number? | 已读时间（Unix 时间戳） |

## 错误代码

| 代码 | 描述 |
|------|------|
| `UNAUTHORIZED` | 未提供或无效的认证令牌 |
| `NOT_FOUND` | 通知不存在 |
| `FORBIDDEN` | 无权标记此通知（仅 Phase 200） |
| `CONFLICT` | 通知已是已读状态 |
| `DATABASE_ERROR` | 数据库操作失败 |

## 权限说明

### PUT /api/v1/system/notifications/{id}/read (Phase 199)

- **适用范围**: 仅系统通知（target_user_id IS NULL）
- **权限**: 登录用户可标记任意系统通知

### POST /api/v1/system/notifications/{id}/mark-read (Phase 200)

- **适用范围**: 所有通知
- **权限规则**:
  - 普通用户：只能标记自己的通知（target_user_id = user_id）或系统通知
  - Admin 用户：可标记任意通知

## 示例

### 请求

```bash
# Phase 199 - 标记系统通知
curl -X PUT "http://localhost:8080/api/v1/system/notifications/1/read" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# Phase 200 - 标记个人通知
curl -X POST "http://localhost:8080/api/v1/system/notifications/1/mark-read" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "message": "通知 '系统更新' 已标记为已读",
  "notification": {
    "id": 1,
    "title": "系统更新",
    "message": "系统已更新到最新版本",
    "type": "info",
    "is_read": true,
    "read_at": 1711600000
  }
}
```

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

## 实现细节

- **状态更新**: `is_read = 1`, `read_at = 当前时间戳`
- **数据库**: SQLite
- **框架**: Actix-web
- **仓库**: SqliteNotificationRepository

## 相关接口

- `GET /api/v1/system/notifications` - 获取系统通知列表
- `GET /api/v1/notifications` - 获取个人通知列表
- `DELETE /api/v1/notifications/read` - 删除所有已读通知

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test
```

## 版本历史

- **Phase 199** (2026-03-28): 初始实现 PUT /api/v1/system/notifications/{id}/read
- **Phase 200** (2026-03-28): 新增 POST /api/v1/system/notifications/{id}/mark-read，支持个人通知
