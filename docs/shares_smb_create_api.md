# SMB 共享创建 API

## Phase 153

## 接口说明

创建新的 SMB 共享。

## 请求

`POST /api/v1/shares/smb`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

```json
{
  "name": "NewShare",
  "path": "/srv/samba/newshare",
  "comment": "New shared folder",
  "read_only": false,
  "guest_access": false,
  "browseable": true,
  "valid_users": ["user1", "user2"],
  "invalid_users": []
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 是 | 共享名称（1-64 字符，字母数字 -_.） |
| path | string | 是 | 共享路径（必须以/开头，最大 256 字符） |
| comment | string | 否 | 备注描述 |
| read_only | boolean | 否 | 是否只读（默认 false） |
| guest_access | boolean | 否 | 是否允许访客访问（默认 false） |
| browseable | boolean | 否 | 是否可浏览（默认 true） |
| valid_users | string[] | 否 | 允许用户列表 |
| invalid_users | string[] | 否 | 禁止用户列表 |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "SMB share created successfully",
  "data": {
    "id": 5,
    "name": "NewShare",
    "path": "/srv/samba/newshare",
    "comment": "New shared folder",
    "read_only": false,
    "guest_access": false,
    "browseable": true,
    "valid_users": ["user1", "user2"],
    "invalid_users": [],
    "enabled": true,
    "status": "active",
    "created_at": "2026-03-27T07:00:00Z",
    "updated_at": "2026-03-27T07:00:00Z"
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid share name. Must be 1-64 chars, alphanumeric with -_. allowed",
  "code": "INVALID_NAME"
}
```

```json
{
  "success": false,
  "error": "Invalid share path. Must start with / and be <= 256 chars",
  "code": "INVALID_PATH"
}
```

#### 401 Unauthorized - 未认证或 Token 无效

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
  "error": "Only admin users can create SMB shares",
  "code": "FORBIDDEN"
}
```

#### 409 Conflict - 名称冲突

```json
{
  "success": false,
  "error": "SMB share name 'Public' already exists",
  "code": "NAME_CONFLICT"
}
```

## 示例

### 创建 SMB 共享

```bash
curl -X POST "http://localhost:8080/api/v1/shares/smb" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "NewShare",
    "path": "/srv/samba/newshare",
    "comment": "New shared folder",
    "read_only": false,
    "guest_access": false,
    "browseable": true,
    "valid_users": ["user1", "user2"],
    "invalid_users": []
  }'
```

### 创建只读共享

```bash
curl -X POST "http://localhost:8080/api/v1/shares/smb" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ReadOnly",
    "path": "/srv/samba/readonly",
    "comment": "Read-only shared folder",
    "read_only": true,
    "guest_access": true,
    "browseable": true
  }'
```

### 尝试创建已存在的共享

```bash
curl -X POST "http://localhost:8080/api/v1/shares/smb" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Public",
    "path": "/srv/samba/public2"
  }'
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "SMB share name 'Public' already exists",
  "code": "NAME_CONFLICT"
}
```

### 尝试使用无效名称

```bash
curl -X POST "http://localhost:8080/api/v1/shares/smb" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Invalid@Name!",
    "path": "/srv/samba/invalid"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid share name. Must be 1-64 chars, alphanumeric with -_. allowed",
  "code": "INVALID_NAME"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 创建结果字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| success | boolean | 是否成功 |
| message | string | 响应消息 |
| data | object | 创建的共享信息 |

### 共享信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径 |
| comment | string | 备注描述 |
| read_only | boolean | 是否只读 |
| guest_access | boolean | 是否允许访客访问 |
| browseable | boolean | 是否可浏览 |
| valid_users | string[] | 允许用户列表 |
| invalid_users | string[] | 禁止用户列表 |
| enabled | boolean | 是否启用 |
| status | string | 状态（active/inactive） |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证共享名称格式（1-64 字符，字母数字 -_.）
4. 验证共享路径格式（必须以/开头，最大 256 字符）
5. 验证名称唯一性
6. 创建 SMB 共享配置
7. 返回 201 Created + 共享详情

## 版本历史

- **Phase 153** (2026-03-27): 初始版本
