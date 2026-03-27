# 用户登录 API

**Phase 52** - 用户认证 API 之登录接口

---

## 接口信息

- **端点:** `POST /api/v1/auth/login`
- **认证:** 无需认证（公开接口）
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Content-Type` | 是 | `application/json` |

### 请求体

```json
{
  "username": "string",
  "password": "string"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `username` | string | 是 | 用户名 |
| `password` | string | 是 | 密码（明文） |

### 请求示例

```bash
curl -X POST "http://localhost:8080/api/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "SecurePass123"
  }'
```

---

## 响应

### 200 OK - 登录成功

```json
{
  "success": true,
  "message": "Login successful",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "user": {
      "id": 1,
      "username": "admin",
      "email": "admin@axis.local",
      "roles": ["admin"]
    }
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "username and password are required",
  "code": "INVALID_PARAMS"
}
```

### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Invalid username or password",
  "code": "UNAUTHORIZED"
}
```

**注意：** 出于安全考虑，不泄露具体是用户名不存在还是密码错误。

### 403 Forbidden - 账户被禁用

```json
{
  "success": false,
  "error": "User account is inactive",
  "code": "ACCOUNT_INACTIVE"
}
```

### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Login failed",
  "code": "INTERNAL_ERROR"
}
```

或

```json
{
  "success": false,
  "error": "Token generation failed",
  "code": "INTERNAL_ERROR"
}
```

---

## 安全说明

1. **密码加密**: 密码使用 bcrypt 算法存储，登录时进行 bcrypt 验证
2. **错误模糊化**: 认证失败时不泄露具体是用户名不存在还是密码错误
3. **JWT Token**: 成功后返回 JWT Bearer Token，用于后续请求认证
4. **Token 有效期**: 默认 60 分钟过期（可在配置中调整）
5. **账户状态检查**: 禁用的账户（is_active=false）无法登录

---

## 响应字段说明

### LoginData

| 字段 | 类型 | 说明 |
|------|------|------|
| `access_token` | string | JWT 访问令牌 |
| `token_type` | string | Token 类型：Bearer |
| `expires_in` | integer | Token 有效期（秒） |
| `user` | object | 用户基本信息 |

### UserInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 用户 ID |
| `username` | string | 用户名 |
| `email` | string | 邮箱地址 |
| `roles` | array | 角色列表 |

---

## 实现细节

- **文件位置:** `src/handlers/auth_login.rs`
- **路由注册:** `src/main.rs` - `POST /api/v1/auth/login`
- **依赖:**
  - `bcrypt` - 密码验证
  - `jsonwebtoken` - JWT Token 生成
  - `SqliteUserRepository` - 用户数据存储

---

## 相关接口

- `POST /api/v1/auth/logout` - 登出
- `POST /api/v1/auth/refresh` - 刷新 Token
- `POST /api/v1/users` - 创建用户（Phase 47）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 52 初始实现 |
