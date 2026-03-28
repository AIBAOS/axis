# 通知删除 API

## Phase 208

## 接口说明

删除系统通知。系统通知是指 `target_user_id` 为 NULL 的全局通知。

## 请求

`DELETE /api/v1/system/notifications/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 通知 ID |

### 请求体

无

## 响应

### 成功响应（204 No Content）

删除成功，无响应体。

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 非 admin 用户

```json
{
  "success": false,
  "error": "仅 admin 可删除通知",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 通知不存在

```json
{
  "success": false,
  "error": "Notification 123 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "删除通知失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 删除系统通知（admin）

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/notifications/123" \
  -H "Authorization: Bearer <jwt_token>"
```

### 响应（成功）

```
204 No Content
```

### 响应（非 admin）

```json
{
  "success": false,
  "error": "仅 admin 可删除通知",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- **仅 admin 用户可访问**

## 业务逻辑

1. 验证 JWT Token 有效性
2. 验证用户角色是否为 admin
3. 查询通知是否存在（404 Not Found）
4. 删除通知
5. 返回 204 No Content

## 版本历史

- **Phase 208** (2026-03-28): 通知删除 API - 仅 admin 可访问，返回 204 No Content