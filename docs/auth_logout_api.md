# 用户登出 API

**Phase 100** - 用户认证 API 之用户登出接口

---

## 接口信息

- **端点:** `POST /api/v1/auth/logout`
- **认证:** 需要 JWT Bearer Token（需要登录状态）
- **权限:** 所有已认证用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |
| `Content-Type` | 否 | `application/json` |

### 请求体

```json
{
  "all_sessions": false
}
```

### 参数说明

| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `all_sessions` | boolean | 否 | false | 是否登出所有会话 |

### 请求示例

```bash
# 登出当前会话
curl -X POST "http://localhost:8080/api/v1/auth/logout" \
  -H "Authorization: Bearer <JWT_TOKEN>"

# 登出所有会话
curl -X POST "http://localhost:8080/api/v1/auth/logout" \
  -H "Authorization: Bearer <JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "all_sessions": true
  }'
```

---

## 响应

### 200 OK - 登出成功

```json
{
  "success": true,
  "message": "User 1 logged out successfully"
}
```

或（登出所有会话）

```json
{
  "success": true,
  "message": "User 1 logged out from all sessions"
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

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **Token 黑名单**: 登出后 token 将被加入黑名单（Redis 缓存）
3. **所有会话**: all_sessions=true 时，用户所有活跃会话将被终止

---

## 响应字段说明

### LogoutResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |

---

## 实现细节

- **文件位置:** `src/handlers/auth_logout.rs`
- **路由注册:** `src/main.rs` - `POST /api/v1/auth/logout`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `Redis` - Token 黑名单缓存（模拟）

---

## 相关接口

- `POST /api/v1/auth/login` - 用户登录
- `POST /api/v1/auth/refresh` - 刷新 Token
- `GET /api/v1/users` - 用户列表

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 100 初始实现 |
