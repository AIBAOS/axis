# 创建用户 API

**Phase 102** - 用户管理 API 之创建用户接口

---

## 接口信息

- **端点:** `POST /api/v1/users`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <ADMIN_JWT_TOKEN>` |
| `Content-Type` | 是 | `application/json` |

### 请求体

```json
{
  "username": "string",
  "password": "string",
  "email": "string",
  "role": "string (可选)"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `username` | string | 是 | 用户名（必须唯一） |
| `password` | string | 是 | 密码（至少 8 位） |
| `email` | string | 是 | 邮箱地址（必须包含 @ 和 .） |
| `role` | string | 否 | 角色：admin/user/guest（默认 user） |

### 请求示例

```bash
# 创建普通用户
curl -X POST "http://localhost:8080/api/v1/users" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newuser",
    "password": "SecurePass123",
    "email": "newuser@example.com",
    "role": "user"
  }'

# 创建管理员用户
curl -X POST "http://localhost:8080/api/v1/users" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newadmin",
    "password": "SecurePass123",
    "email": "newadmin@example.com",
    "role": "admin"
  }'
```

---

## 响应

### 201 Created - 创建成功

```json
{
  "success": true,
  "message": "User created successfully",
  "data": {
    "id": 100,
    "username": "newuser",
    "email": "newuser@example.com",
    "role": "user",
    "created_at": 1711526400,
    "updated_at": 1711526400
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "username is required",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "password must be at least 8 characters long",
  "code": "WEAK_PASSWORD"
}
```

或

```json
{
  "success": false,
  "error": "invalid email format",
  "code": "INVALID_EMAIL"
}
```

或

```json
{
  "success": false,
  "error": "Invalid role 'superuser'. Valid roles: admin, user, guest",
  "code": "INVALID_ROLE"
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

### 403 Forbidden - 无权限

```json
{
  "success": false,
  "error": "Only admin users can create users",
  "code": "FORBIDDEN"
}
```

### 409 Conflict - 用户名已存在

```json
{
  "success": false,
  "error": "Username 'admin' already exists",
  "code": "CONFLICT"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以创建用户
3. **密码强度**: 密码必须至少 8 个字符
4. **用户名唯一性**: 用户名必须全局唯一
5. **邮箱格式**: 必须包含 @ 和 .

---

## 响应字段说明

### CreateUserResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 用户信息 |

### UserInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 用户 ID |
| `username` | string | 用户名 |
| `email` | string | 邮箱地址 |
| `role` | string | 角色：admin/user/guest |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

---

## 实现细节

- **文件位置:** `src/handlers/users_create.rs`
- **路由注册:** `src/main.rs` - `POST /api/v1/users`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/users` - 用户列表（Phase 99）
- `GET /api/v1/users/{id}` - 用户详情（Phase 101）
- `PUT /api/v1/users/{id}` - 更新用户
- `DELETE /api/v1/users/{id}` - 删除用户

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 102 初始实现 |
