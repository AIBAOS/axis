# 删除用户 API

**Phase 51** - 用户管理 API 之删除用户接口

---

## 接口信息

- **端点:** `DELETE /api/v1/users/{id}`
- **认证:** 需要 JWT Bearer Token
- **权限:** 仅 `admin` 角色可访问
- **响应:** 204 No Content

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 用户 ID |

### 请求示例

```bash
curl -X DELETE "http://localhost:8080/api/v1/users/123" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"
```

---

## 响应

### 204 No Content - 删除成功

无响应体。

### 400 Bad Request - 不能删除自己

```json
{
  "success": false,
  "error": "Cannot delete yourself",
  "code": "CANNOT_DELETE_SELF"
}
```

### 401 Unauthorized - 未认证

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

或

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden - 无权限

```json
{
  "success": false,
  "error": "Only admin users can delete users",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 用户不存在

```json
{
  "success": false,
  "error": "User 123 not found",
  "code": "NOT_FOUND"
}
```

### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to delete user: <error message>",
  "code": "INTERNAL_ERROR"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以删除用户
3. **自我保护**: 不能删除当前登录用户自己
4. **操作审计**: 删除操作会记录日志（包含被删除用户 ID 和用户名）

---

## 实现细节

- **文件位置:** `src/handlers/users_delete.rs`
- **路由注册:** `src/main.rs` - `DELETE /api/v1/users/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteUserRepository` - 用户数据存储
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/users` - 用户列表（Phase 48）
- `GET /api/v1/users/{id}` - 用户详情（Phase 49）
- `POST /api/v1/users` - 创建用户（Phase 47）
- `PUT /api/v1/users/{id}` - 更新用户（Phase 50）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 51 初始实现 |
| 2026-03-26 | 1.1 | 增强 JWT 认证和 admin 权限校验 |
