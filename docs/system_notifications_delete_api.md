# 系统通知删除 API

## Phase 208

## 接口说明

删除指定的系统通知。

## 请求

`DELETE /api/v1/system/notifications/{id}`

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

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can delete system notifications",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 通知不存在或非系统通知

```json
{
  "success": false,
  "error": "System notification 999 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "删除系统通知失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 删除系统通知

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/notifications/15" \
  -H "Authorization: Bearer <jwt_token>"
```

响应：`204 No Content`

### 删除不存在的通知

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/notifications/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "System notification 999 not found",
  "code": "NOT_FOUND"
}
```

### 删除个人通知（非系统通知）

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/notifications/10" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "System notification 10 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问
- 仅允许删除系统通知（target_user_id IS NULL）

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 查询通知是否存在（404 Not Found）
4. 验证是系统通知（target_user_id IS NULL）
5. 执行删除
6. 返回 204 No Content

## 通知类型

| 类型 | 说明 |
| ---- | ---- |
| system | 系统通知 |
| alert | 告警通知 |
| info | 信息通知 |
| warning | 警告通知 |
| error | 错误通知 |

## 版本历史

- **Phase 208** (2026-03-28): 通知管理模块 - 系统通知删除 API
