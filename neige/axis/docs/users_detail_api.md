# 用户详情 API - Phase 226

## 接口规范

### 基本信息

- **端点**: `GET /api/v1/users/{id}`
- **认证**: JWT Bearer Token（必需）
- **权限**: 登录用户可访问
  - Admin 角色：可查看任意用户
  - 普通用户：仅可查看自己的信息
- **方法**: GET

---

## 请求

### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|-----|------|-----|------|
| id | string | 是 | 用户 ID |

### 请求示例

**查看自己的信息**:
```
GET /api/v1/users/user_123
Authorization: Bearer <jwt_token_of_user_123>
```

**Admin 查看任意用户**:
```
GET /api/v1/users/user_456
Authorization: Bearer <admin_jwt_token>
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "message": "获取用户详情成功",
  "data": {
    "id": "user_123456",
    "username": "zhangsan",
    "email": "zhangsan@example.com",
    "roles": ["user", "moderator"],
    "is_active": true,
    "created_at": 1711468800,
    "updated_at": 1711555200,
    "last_login": 1711641600
  }
}
```

### 响应字段说明

| 字段 | 类型 | 说明 |
|-----|------|------|
| id | string | 用户 ID |
| username | string | 用户名 |
| email | string | 邮箱地址 |
| roles | array | 角色列表 |
| is_active | bool | 是否激活 |
| created_at | i64 | 创建时间（Unix 时间戳） |
| updated_at | i64 | 更新时间（Unix 时间戳） |
| last_login | i64\|null | 最后登录时间（Unix 时间戳） |

**注意**: 响应中**不包含密码字段**

### 403 Forbidden - 权限不足

普通用户尝试查看其他用户信息：
```json
{
  "success": false,
  "message": "普通用户仅可查看自己的信息",
  "error_code": "FORBIDDEN"
}
```

### 404 Not Found - 用户不存在

```json
{
  "success": false,
  "message": "用户不存在：user_999",
  "error_code": "NOT_FOUND"
}
```

### 401 Unauthorized - 未登录

```json
{
  "success": false,
  "message": "未授权访问",
  "error_code": "UNAUTHORIZED"
}
```

---

## 使用示例

### cURL

**查看自己的信息**:
```bash
curl -X GET "http://localhost:8080/api/v1/users/user_123" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**Admin 查看用户**:
```bash
curl -X GET "http://localhost:8080/api/v1/users/user_456" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### JavaScript (fetch)

```javascript
async function getUserDetail(userId, token) {
  const response = await fetch(`http://localhost:8080/api/v1/users/${userId}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  
  const data = await response.json();
  
  if (data.success) {
    console.log(`用户名：${data.data.username}`);
    console.log(`邮箱：${data.data.email}`);
    console.log(`角色：${data.data.roles.join(', ')}`);
    console.log(`激活状态：${data.data.is_active ? '已激活' : '未激活'}`);
    
    if (data.data.last_login) {
      const lastLogin = new Date(data.data.last_login * 1000);
      console.log(`最后登录：${lastLogin.toLocaleString()}`);
    }
  } else {
    console.error(`错误：${data.message}`);
  }
  
  return data;
}

// 使用示例
getUserDetail('user_123', jwtToken);
```

### Python (requests)

```python
import requests
from datetime import datetime

def get_user_detail(user_id, token):
    url = f'http://localhost:8080/api/v1/users/{user_id}'
    headers = {
        'Authorization': f'Bearer {token}',
    }
    
    response = requests.get(url, headers=headers)
    data = response.json()
    
    if data['success']:
        user = data['data']
        print(f"用户名：{user['username']}")
        print(f"邮箱：{user['email']}")
        print(f"角色：{', '.join(user['roles'])}")
        print(f"激活状态：{'已激活' if user['is_active'] else '未激活'}")
        
        if user['last_login']:
            last_login = datetime.fromtimestamp(user['last_login'])
            print(f"最后登录：{last_login}")
    else:
        print(f"错误：{data['message']}")
    
    return data

# 使用示例
get_user_detail('user_123', jwt_token)
```

### TypeScript

```typescript
interface UserInfo {
  id: string;
  username: string;
  email: string;
  roles: string[];
  is_active: boolean;
  created_at: number;
  updated_at: number;
  last_login: number | null;
}

interface UserDetailResponse {
  success: boolean;
  message: string;
  data: UserInfo | null;
}

async function getUserDetail(userId: string, token: string): Promise<UserDetailResponse> {
  const response = await fetch(`/api/v1/users/${userId}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  
  const data: UserDetailResponse = await response.json();
  return data;
}

// 使用示例
const userDetail = await getUserDetail('user_123', jwtToken);
if (userDetail.success && userDetail.data) {
  console.log(`用户：${userDetail.data.username}`);
}
```

---

## 权限说明

### Admin 角色
- ✅ 可查看任意用户的详情
- ✅ 可查看所有字段（不含密码）

### 普通用户
- ✅ 可查看自己的详情
- ❌ 不可查看其他用户的详情（返回 403）

### 权限校验逻辑

```rust
let is_admin = claims.roles.contains(&"admin".to_string());
if !is_admin && current_user_id != target_user_id {
    return HttpResponse::Forbidden();
}
```

---

## 安全说明

1. **密码保护**: 响应中**绝不包含密码字段**
2. **权限隔离**: 普通用户无法越权查看他人信息
3. **JWT 认证**: 需要有效的 JWT Token
4. **审计日志**: 建议记录敏感信息访问日志

---

## 相关文件

- 实现：`src/handlers/users_get.rs`
- 路由注册：`src/main.rs`
- 数据库：`src/database/user_store.rs`

---

**版本**: v1.0.0  
**最后更新**: 2026-03-28 08:05 UTC  
**Phase**: 226
