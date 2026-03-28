# 获取用户详情 API (Phase 49)
# 别名：GET /api/v1/users/{id}

## 接口说明

实现获取单个用户详情的接口，返回用户详细信息（不含密码）。仅 admin 角色可访问。

## 接口定义

```
GET /api/v1/users/{id}
```

## 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | integer | 是 | 用户 ID |

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "id": 1,
  "username": "admin",
  "email": "admin@axis.local",
  "roles": ["admin"],
  "status": "active",
  "created_at": 1710500000,
  "updated_at": 1711400000,
  "last_login": 1711411200
}
```

### 用户不存在 (404 Not Found)

```json
{
  "error": "User 123 not found"
}
```

### 禁止访问 (403 Forbidden)

```json
{
  "error": "Only admin users can access user details"
}
```

### 未授权 (401 Unauthorized)

```json
{
  "error": "Current user not found"
}
```

## 使用示例

### cURL 示例

```bash
# 获取用户详情
curl -X GET "http://localhost:8080/api/v1/users/1" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 获取不存在的用户（返回 404）
curl -X GET "http://localhost:8080/api/v1/users/999" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### JavaScript 示例

```javascript
// 获取用户详情
async function getUserDetail(userId) {
  const response = await fetch(
    `http://localhost:8080/api/v1/users/${userId}`,
    {
      method: 'GET',
      headers: {
        'Authorization': 'Bearer ' + adminToken
      }
    }
  );
  
  if (response.status === 404) {
    console.log('User not found');
    return null;
  }
  
  const data = await response.json();
  console.log('User detail:', data);
  return data;
}

// 使用示例
const user = await getUserDetail(1);
```

## 响应字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 用户 ID |
| username | string | 用户名 |
| email | string | 邮箱地址 |
| roles | array | 角色列表 |
| status | string | 状态（active/inactive） |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |
| last_login | integer/null | 最后登录时间（Unix 时间戳） |

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **密码保护**: 响应中不包含密码相关字段
4. **路径参数校验**: 用户 ID 必须为正整数

## 实现文件

- `src/handlers/users_detail.rs` - 用户详情处理器
- `src/handlers/users.rs` - 用户管理模块
- `src/main.rs` - 路由注册
- `src/database/user_store.rs` - 用户存储
- `src/models/user.rs` - 用户模型

## 注意事项

1. 非 admin 用户访问返回 403 Forbidden
2. 用户不存在返回 404 Not Found
3. 时间戳使用 Unix 时间戳（秒级）
4. last_login 为 null 表示用户从未登录
5. 响应中不包含密码哈希等敏感信息

## 相关接口

- `GET /api/v1/users` - 用户列表（Phase 48）
- `POST /api/v1/users` - 创建用户（Phase 47）
- `PUT /api/v1/users/{id}` - 更新用户
- `DELETE /api/v1/users/{id}` - 删除用户
