# Phase 226 - 用户详情 API 文档

## 接口说明

获取单个用户的详细信息，支持权限控制。

## 接口定义

```
GET /api/v1/users/{id}
```

## 请求参数

### 路径参数

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| id | integer | 是 | 用户 ID |

### 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "id": 1,
    "username": "admin",
    "email": "admin@axis.local",
    "roles": ["admin"],
    "is_active": true,
    "created_at": 1710500000,
    "updated_at": 1710500000,
    "last_login": 1710600000
  }
}
```

### 404 Not Found - 用户不存在

```json
{
  "success": false,
  "error": "User 999 not found",
  "code": "NOT_FOUND"
}
```

### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can view other users' details",
  "code": "FORBIDDEN"
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

## 使用示例

### 1. Admin 查看任意用户

```bash
# Admin 查看用户 ID 为 2 的详情
curl -X GET "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 2. 普通用户查看自己的详情

```bash
# 用户 ID 为 2 查看自己的详情
curl -X GET "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 3. 普通用户尝试查看他人详情（返回 403）

```bash
# 用户 ID 为 2 尝试查看用户 ID 为 3 的详情
curl -X GET "http://localhost:8080/api/v1/users/3" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
# 返回 403 Forbidden
```

### 4. 查看不存在的用户（返回 404）

```bash
curl -X GET "http://localhost:8080/api/v1/users/999" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
# 返回 404 Not Found
```

## 字段说明

### UserDetail

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 用户 ID |
| username | string | 用户名 |
| email | string | 邮箱地址 |
| roles | array | 角色列表（如 ["admin"], ["user"]） |
| is_active | boolean | 是否激活 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |
| last_login | integer\|null | 最后登录时间（Unix 时间戳） |

### 安全说明

- **密码不包含在响应中** - 出于安全考虑，密码字段永远不会返回
- **权限控制** - Admin 可查看任意用户，普通用户只能查看自己

## 权限控制

| 用户角色 | 可查看的用户 |
|----------|--------------|
| admin | 任意用户 |
| user | 仅自己 |
| guest | 仅自己 |

## 错误码说明

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| UNAUTHORIZED | 401 | 未提供 token 或 token 无效 |
| FORBIDDEN | 403 | 普通用户尝试查看他人详情 |
| NOT_FOUND | 404 | 用户 ID 不存在 |
| DATABASE_ERROR | 500 | 数据库查询错误 |

## 实现细节

- **文件位置:** `src/handlers/users_get_by_id.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/users/{id}`
- **数据库:** 使用 `SqliteUserRepository` 实现真实查询
- **依赖:**
  - `SqliteUserRepository` - 用户数据存储
  - `SqliteRbacRepository` - 角色数据存储
  - `JwtService` - JWT 认证

## 相关接口

- `GET /api/v1/users` - 用户列表（Phase 225）
- `POST /api/v1/users` - 创建用户（Phase 227）
- `PUT /api/v1/users/{id}` - 更新用户（Phase 228）
- `DELETE /api/v1/users/{id}` - 删除用户（Phase 229）
- `PUT /api/v1/users/{id}/password` - 修改密码（Phase 230）

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-28 | 2.0 | Phase 226：使用真实数据库实现，添加权限控制 |
| 2026-03-27 | 1.0 | Phase 101：初始实现（模拟数据） |

---

**兵部尚书 签发**
2026-03-28 08:00 UTC
