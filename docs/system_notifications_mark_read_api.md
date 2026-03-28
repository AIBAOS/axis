# 系统通知标记已读 API

## Phase 199

## 接口说明

将指定的系统通知标记为已读状态。

## 请求

`PUT /api/v1/system/notifications/{id}/read`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 通知 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "通知 '系统维护通知' 已标记为已读",
  "notification": {
    "id": 15,
    "title": "系统维护通知",
    "message": "系统将于今晚 23:00 进行例行维护",
    "type": "system",
    "is_read": true,
    "read_at": 1711670400
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| success | boolean | 操作是否成功 |
| message | string | 操作结果消息 |
| notification | object | 通知摘要（可选） |
| notification.id | integer | 通知 ID |
| notification.title | string | 通知标题 |
| notification.message | string | 通知内容 |
| notification.type | string | 通知类型 |
| notification.is_read | boolean | 是否已读 |
| notification.read_at | integer | 已读时间（Unix 时间戳） |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "只能标记系统通知为已读",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 通知不存在

```json
{
  "success": false,
  "error": "Notification 999 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "标记已读失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 标记系统通知为已读

```bash
curl -X PUT "http://localhost:8080/api/v1/system/notifications/15/read" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "message": "通知 '系统维护通知' 已标记为已读",
  "notification": {
    "id": 15,
    "title": "系统维护通知",
    "message": "系统将于今晚 23:00 进行例行维护",
    "type": "system",
    "is_read": true,
    "read_at": 1711670400
  }
}
```

#### 409 Conflict - 通知已是已读状态

```json
{
  "success": false,
  "error": "通知已是已读状态",
  "code": "CONFLICT"
}
```

### 标记不存在的通知

```bash
curl -X PUT "http://localhost:8080/api/v1/system/notifications/999/read" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Notification 999 not found",
  "code": "NOT_FOUND"
}
```

### 标记个人通知（非系统通知）

```bash
curl -X PUT "http://localhost:8080/api/v1/system/notifications/10/read" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "只能标记系统通知为已读",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问
- 仅允许标记系统通知（target_user_id IS NULL）

## 业务逻辑

1. 验证 JWT Token 有效性
2. 查询通知是否存在（404 Not Found）
3. 验证是系统通知（target_user_id IS NULL）
4. 如果已是已读状态，直接返回
5. 更新 is_read = 1, read_at = 当前时间戳
6. 返回更新后的通知摘要

## 通知类型

| 类型 | 说明 |
| ---- | ---- |
| system | 系统通知 |
| alert | 告警通知 |
| info | 信息通知 |
| warning | 警告通知 |
| error | 错误通知 |

## 版本历史

- **Phase 199** (2026-03-28): 通知管理模块 - 系统通知标记已读 API
