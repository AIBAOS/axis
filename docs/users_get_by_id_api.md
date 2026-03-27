# Phase 101 - 用户详情 API 文档

**接口:** `GET /api/v1/users/{id}`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

获取指定用户的详细信息。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 登录用户可访问

**功能特性:**
- 返回用户完整信息
- 验证用户 ID 存在
- 用户不存在返回 404 Not Found

---

## 🔐 认证方式

```
Authorization: Bearer <access_token>
```

---

## 📤 请求参数

### 请求头

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `Authorization` | string | 是 | JWT Bearer Token |

### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | number | 是 | 用户 ID |

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "data": {
    "id": 2,
    "username": "user1",
    "email": "user1@example.com",
    "role": "user",
    "created_at": 1774345600,
    "updated_at": 1774345600
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | object | 用户详情 |
| `data.id` | number | 用户 ID |
| `data.username` | string | 用户名 |
| `data.email` | string | 邮箱 |
| `data.role` | string | 角色：`admin` / `user` / `guest` |
| `data.created_at` | number | 创建时间（Unix 时间戳） |
| `data.updated_at` | number | 更新时间（Unix 时间戳） |

---

## ❌ 错误响应

### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 404 Not Found

```json
{
  "success": false,
  "error": "User 999 not found",
  "code": "NOT_FOUND"
}
```

---

## 🧪 使用示例

```bash
# 获取用户 ID 为 2 的详情
curl -X GET "http://localhost:8080/api/v1/users/2" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 获取不存在的用户（返回 404）
curl -X GET "http://localhost:8080/api/v1/users/999" \
  -H "Authorization: Bearer <access_token>"
# 响应：404 Not Found - User not found
```

```bash
# 未认证（401）
curl -X GET "http://localhost:8080/api/v1/users/2"
# 响应：401 Unauthorized - Missing or invalid Authorization header
```

---

## 📝 注意事项

1. **权限要求**: 登录用户可访问，无需 admin 权限
2. **用户不存在**: 返回 404 Not Found
3. **敏感信息**: 响应中不包含密码等敏感信息

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/users` | 用户列表 (Phase 99) |
| `POST /api/v1/users` | 创建用户 |
| `PUT /api/v1/users/{id}` | 更新用户 |
| `DELETE /api/v1/users/{id}` | 删除用户 |

---

*文档维护：兵部尚书*
