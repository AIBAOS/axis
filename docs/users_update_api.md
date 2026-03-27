# 更新用户 API (Phase 103)

## 接口说明

更新用户信息接口，支持修改邮箱和角色。

## 接口定义

```
PUT /api/v1/users/{id}
```

## 请求参数

### 路径参数

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| id | integer | 是 | 用户 ID |

### 请求体 (JSON)

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| email | string | 否 | 新邮箱地址 |
| role | string | 否 | 新角色（admin/user/guest） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "User updated successfully",
  "data": {
    "id": 2,
    "username": "user1",
    "email": "newemail@axis.local",
    "role": "admin",
    "role_id": 1,
    "created_at": 1710500000,
    "updated_at": 1711500000
  }
}
```

### 邮箱格式错误 (400 Bad Request)

```json
{
  "success": false,
  "error": "Invalid email format",
  "code": "INVALID_EMAIL"
}
```

### 角色无效 (400 Bad Request)

```json
{
  "success": false,
  "error": "Invalid role 'superuser'. Valid roles: admin, user, guest",
  "code": "INVALID_ROLE"
}
```

### 用户不存在 (404 Not Found)

```json
{
  "success": false,
  "error": "User 999 not found",
  "code": "NOT_FOUND"
}
```

### 非管理员访问 (403 Forbidden)

```json
{
  "success": false,
  "error": "Only admin users can update users",
  "code": "FORBIDDEN"
}
```

## 使用示例

### 1. 更新用户邮箱

```bash
curl -X PUT "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newemail@axis.local"
  }'
```

### 2. 更新用户角色

```bash
# 提升为管理员
curl -X PUT "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "role": "admin"
  }'

# 降级为普通用户
curl -X PUT "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "role": "user"
  }'
```

### 3. 同时更新邮箱和角色

```bash
curl -X PUT "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "updated@axis.local",
    "role": "admin"
  }'
```

### 4. 错误示例

```bash
# 无效邮箱格式（返回 400）
curl -X PUT "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "invalid-email"
  }'

# 无效角色（返回 400）
curl -X PUT "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "role": "superuser"
  }'

# 更新不存在的用户（返回 404）
curl -X PUT "http://localhost:8080/api/v1/users/999" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@axis.local"
  }'

# 非管理员尝试更新（返回 403）
curl -X PUT "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@axis.local"
  }'
```

## 功能特性

- ✅ JWT 认证，仅 admin 角色可访问
- ✅ 可更新字段：email, role
- ✅ 验证邮箱格式（必须包含 @ 和 .）
- ✅ 验证角色有效性（admin/user/guest）
- ✅ 用户不存在 → 404 Not Found
- ✅ 非 admin 访问 → 403 Forbidden
- ✅ 更新成功返回 200 OK + 完整用户信息

## 角色说明

| 角色 | role_id | 说明 |
|------|---------|------|
| admin | 1 | 管理员，拥有全部权限 |
| user | 2 | 普通用户，默认角色 |
| guest | 3 | 访客，只读权限 |

## 注意事项

1. 密码不可通过此接口修改，请使用 `/api/v1/users/{id}/password` 接口
2. 用户名不可修改
3. 部分字段更新：只传递需要更新的字段即可
4. 邮箱格式验证：必须包含 @ 和 . 字符

## 实现文件

- `src/handlers/users_update.rs` - 更新用户处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册
