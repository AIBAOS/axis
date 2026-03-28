# 更新用户 API - Phase 103

## 接口规范

### 基本信息

- **端点**: `PUT /api/v1/users/{id}`
- **认证**: JWT Bearer Token（必需）
- **权限**: 仅 `admin` 角色可访问
- **Content-Type**: `application/json`

---

## 请求

### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|-----|------|-----|------|
| id | string | 是 | 用户 ID |

### 请求体

```json
{
  "email": "newemail@example.com",
  "role": "admin"
}
```

| 字段 | 类型 | 必填 | 说明 |
|-----|------|-----|------|
| email | string | 否 | 新邮箱地址（需符合邮箱格式） |
| role | string | 否 | 新角色（admin/user/moderator/guest） |

**注意**: 至少需要提供 `email` 或 `role` 中的一个字段。

---

## 响应

### 200 OK - 更新成功

```json
{
  "success": true,
  "message": "用户信息更新成功",
  "data": {
    "id": "usr_123456",
    "username": "zhangsan",
    "email": "newemail@example.com",
    "role": "admin",
    "created_at": 1711468800,
    "updated_at": 1711555200
  }
}
```

### 400 Bad Request - 请求参数错误

**邮箱格式无效**:
```json
{
  "success": false,
  "message": "邮箱格式无效",
  "error_code": "BAD_REQUEST"
}
```

**角色无效**:
```json
{
  "success": false,
  "message": "无效的角色：superadmin",
  "error_code": "BAD_REQUEST"
}
```

**缺少更新字段**:
```json
{
  "success": false,
  "message": "至少需要提供一个更新字段（email 或 role）",
  "error_code": "BAD_REQUEST"
}
```

### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "message": "仅 admin 角色可更新用户信息",
  "error_code": "FORBIDDEN"
}
```

### 404 Not Found - 用户不存在

```json
{
  "success": false,
  "message": "用户不存在：usr_999",
  "error_code": "NOT_FOUND"
}
```

### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "message": "数据库更新失败",
  "error_code": "INTERNAL_ERROR"
}
```

---

## 使用示例

### cURL

```bash
curl -X PUT http://localhost:8080/api/v1/users/usr_123456 \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newemail@example.com",
    "role": "moderator"
  }'
```

### JavaScript (fetch)

```javascript
const response = await fetch('http://localhost:8080/api/v1/users/usr_123456', {
  method: 'PUT',
  headers: {
    'Authorization': 'Bearer <admin_jwt_token>',
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    email: 'newemail@example.com',
    role: 'moderator',
  }),
});

const data = await response.json();
console.log(data);
```

### Python (requests)

```python
import requests

url = 'http://localhost:8080/api/v1/users/usr_123456'
headers = {
    'Authorization': 'Bearer <admin_jwt_token>',
    'Content-Type': 'application/json',
}
data = {
    'email': 'newemail@example.com',
    'role': 'moderator',
}

response = requests.put(url, headers=headers, json=data)
print(response.json())
```

---

## 安全说明

1. **权限控制**: 仅 `admin` 角色可调用此接口
2. **邮箱验证**: 使用正则表达式校验邮箱格式
3. **角色白名单**: 仅允许预定义角色（admin/user/moderator/guest）
4. **审计日志**: 所有更新操作记录日志（操作者、目标用户、变更内容）

---

## 相关文件

- 实现：`src/handlers/users_update.rs`
- 路由注册：`src/main.rs`
- 数据库：`src/database/user_store.rs`

---

**版本**: v1.0.0  
**最后更新**: 2026-03-26 17:15 UTC  
**Phase**: 103
