# 删除用户 API

## Phase 104

## 接口说明

删除指定用户，仅限 admin 角色访问。

## 请求

`DELETE /api/v1/users/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 用户 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "User deleted"
}
```

### 错误响应

#### 400 Bad Request - 尝试删除自己

```json
{
  "success": false,
  "error": "Cannot delete yourself",
  "code": "CANNOT_DELETE_SELF"
}
```

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足（非 admin）

```json
{
  "success": false,
  "error": "Only admin users can delete users",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 用户不存在

```json
{
  "success": false,
  "error": "User 999 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to delete user: database error",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 删除用户

```bash
curl -X DELETE "http://localhost:8080/api/v1/users/1" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "message": "User deleted"
}
```

### 删除不存在的用户

```bash
curl -X DELETE "http://localhost:8080/api/v1/users/999" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "User 999 not found",
  "code": "NOT_FOUND"
}
```

### 非 admin 用户尝试删除

```bash
curl -X DELETE "http://localhost:8080/api/v1/users/1" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can delete users",
  "code": "FORBIDDEN"
}
```

### 尝试删除自己

```bash
curl -X DELETE "http://localhost:8080/api/v1/users/1" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Cannot delete yourself",
  "code": "CANNOT_DELETE_SELF"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问
- 不能删除自己

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 检查是否尝试删除自己（400 Bad Request）
4. 查询用户是否存在（404 Not Found）
5. 使用 SqliteUserRepository 删除用户
6. 返回删除成功消息

## 安全说明

- 此接口仅限 admin 用户调用
- 防止 admin 删除自己导致系统无管理员
- 删除操作不可逆，建议添加二次确认机制
- 建议添加操作审计日志

## 版本历史

- **Phase 104** (2026-03-28): 用户管理模块 - 删除用户 API
- **Phase 51** (2026-03-26): 初始实现（返回 204 No Content）
- **Phase 104 增强** (2026-03-28): 返回 200 OK + JSON 响应
