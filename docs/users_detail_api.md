# 用户详情 API (Phase 226)

## 接口说明

获取单个用户的详细信息。支持权限控制：admin 用户可查看任意用户详情，普通用户只能查看自己的详情。

## 接口定义

```
GET /api/v1/users/{id}
```

## 请求参数

### 路径参数

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| id | integer | 是 | 用户 ID |

### 查询参数

无查询参数

## 请求头

| Header | 必需 | 说明 |
|--------|------|------|
| Authorization | 是 | `Bearer <JWT_TOKEN>` |

## 权限控制

- **admin 角色**：可以查看任意用户的详细信息
- **普通用户**：只能查看自己的用户详情，尝试访问其他用户会返回 403 Forbidden

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "id": 1,
    "username": "admin",
    "email": "admin@example.com",
    "roles": ["admin"],
    "is_active": true,
    "created_at": 1710500000,
    "updated_at": 1710600000,
    "last_login": 1710600000
  }
}
```

### 错误响应

#### 401 Unauthorized - 未授权

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

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can view other users' details",
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

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "Database error: Failed to connect to database",
  "code": "DATABASE_ERROR"
}
```

## 使用示例

### 1. 管理员获取任意用户详情

```bash
curl -X GET "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 2. 普通用户获取自己的详情

```bash
# 普通用户 id=3，查看自己的详情
curl -X GET "http://localhost:8080/api/v1/users/3" \
  -H "Authorization: Bearer <user_jwt_token>"
```

### 3. 普通用户尝试查看其他用户（会失败）

```bash
# 普通用户 id=3，尝试查看用户 id=2，返回 403
curl -X GET "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <user_jwt_token>"

# 返回：
# Status: 403 Forbidden
# {
#   "success": false,
#   "error": "Only admin users can view other users' details",
#   "code": "FORBIDDEN"
# }
```

### 4. 查看不存在的用户

```bash
curl -X GET "http://localhost:8080/api/v1/users/999" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 返回：
# Status: 404 Not Found
# {
#   "success": false,
#   "error": "User 999 not found",
#   "code": "NOT_FOUND"
# }
```

## 响应字段说明

### UserDetail

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 用户 ID |
| username | string | 用户名 |
| email | string | 邮箱地址 |
| roles | array | 用户角色数组 |
| is_active | boolean | 是否激活 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |
| last_login | integer\|null | 最后登录时间（Unix 时间戳），未登录为 null |

| 字段名 | 类型 | 是否返回密码 |
|--------|------|--------------|
| 普通用户自己 | 自己的角色 | 否 |
| admin 查看任意用户 | 所有角色 | 否 |

## 示例响应解析

### 成功响应
```json
{
  "success": true,
  "data": {
    "id": 2,
    "username": "john_doe",
    "email": "john@example.com",
    "roles": ["user", "editor"],
    "is_active": true,
    "created_at": 1710000000,
    "updated_at": 1710500000,
    "last_login": 1710600000
  }
}
```

### 权限不足响应
```json
{
  "success": false,
  "error": "Only admin users can view other users' details",
  "code": "FORBIDDEN"
}
```

### 用户不存在响应
```json
{
  "success": false,
  "error": "User 999 not found",
  "code": "NOT_FOUND"
}
```

## 实现细节

### 路由注册
```rust
GET /api/v1/users/{id} -> handlers::users_detail::get_user
```

### 权限检查流程
1. 提取并验证 JWT Token
2. 从 Token 中获取当前用户 ID（sub 字段）
3. 检查当前用户是否为 admin：
   - 如果是 admin：可以查看任意用户
   - 如果不是 admin：只允许查看自己（ID 匹配）
4. 从数据库获取用户信息
5. 返回结果（不含密码字段）

## 安全特性

1. **JWT 认证**：必须提供有效的 JWT Token
2. **权限控制**：
   - Admin 可以查看任意用户详情
   - 普通用户只能查看自己的详情
   - 违反规则返回 403 Forbidden
3. **用户存在性验证**：不存在的用户返回 404 Not Found
4. **密码保护**：响应中不包含密码信息
5. **数据库错误处理**：返回 500 Internal Server Error

## 相关接口

- `GET /api/v1/users` - 用户列表 (Phase 225)
- `POST /api/v1/users` - 创建用户 (Phase 224)
- `PUT /api/v1/users/{id}` - 更新用户 (Phase 223)
- `DELETE /api/v1/users/{id}` - 删除用户 (Phase 222)

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-28 | Phase 226 | 初始实现：用户详情 API |
| 2026-03-26 | Phase 101 | 早期版本：模拟数据实现 |

---

**兵部尚书 签发**
2026-03-28 08:00 UTC
