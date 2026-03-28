# SMB 共享更新 API

## Phase 211

## 接口说明

更新 SMB 共享配置。支持部分更新，仅更新提供的字段。

## 请求

`PUT /api/v1/shares/smb/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | SMB 共享 ID |

### 请求体

```json
{
  "name": "new_shared_name",
  "description": "更新后的描述",
  "allowed_users": "user1,user2,user3",
  "guest_ok": true,
  "read_only": true
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 否 | 共享名称（1-64 字符，字母数字及 -_.） |
| path | string | 否 | 共享路径（必须以 / 开头，最长 256 字符） |
| description | string | 否 | 共享描述 |
| allowed_users | string | 否 | 允许访问的 SMB 用户名（逗号分隔） |
| allowed_groups | string | 否 | 允许访问的 SMB 组名（逗号分隔） |
| guest_ok | boolean | 否 | 是否允许访客访问 |
| read_only | boolean | 否 | 是否只读 |

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "SMB share updated successfully",
  "data": {
    "id": 1,
    "name": "new_shared_name",
    "path": "/data/shared",
    "description": "更新后的描述",
    "allowed_users": "user1,user2,user3",
    "allowed_groups": null,
    "guest_ok": true,
    "read_only": true,
    "status": "active",
    "created_at": 1711584000,
    "updated_at": 1711587600
  }
}
```

### 错误响应

#### 400 Bad Request - 请求参数错误

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
  "error": "Path '/nonexistent' does not exist",
  "code": "PATH_NOT_FOUND"
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

#### 403 Forbidden - 非 admin 用户

```json
{
  "success": false,
  "error": "Only admin users can update SMB shares",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - SMB 共享不存在

```json
{
  "success": false,
  "error": "SMB share 1 not found",
  "code": "NOT_FOUND"
}
```

#### 409 Conflict - 名称冲突

```json
{
  "success": false,
  "error": "Share name 'new_name' already exists",
  "code": "NAME_CONFLICT"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "更新共享失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 更新 SMB 共享（部分字段）

```bash
curl -X PUT "http://localhost:8080/api/v1/shares/smb/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "new_shared_name",
    "description": "更新后的描述",
    "allowed_users": "user1,user2,user3",
    "guest_ok": true,
    "read_only": true
  }'
```

### 响应（成功）

```json
{
  "success": true,
  "message": "SMB share updated successfully",
  "data": {
    "id": 1,
    "name": "new_shared_name",
    "path": "/data/shared",
    "description": "更新后的描述",
    "allowed_users": "user1,user2,user3",
    "allowed_groups": null,
    "guest_ok": true,
    "read_only": true,
    "status": "active",
    "created_at": 1711584000,
    "updated_at": 1711587600
  }
}
```

## 权限要求

- 需要 JWT 认证
- **仅 admin 用户可更新 SMB 共享**

## 业务逻辑

1. 验证 JWT Token 有效性
2. 验证用户角色是否为 admin
3. 查询 SMB 共享是否存在（404 Not Found）
4. 验证名称格式（如果提供）
5. 验证路径格式（如果提供）
6. 验证路径是否存在（如果提供）
7. 验证名称唯一性（如果提供，排除自身）
8. 更新 SMB 共享记录
9. 返回更新后的共享详情

## SMB 共享字段说明

| 字段 | 说明 |
| ---- | ---- |
| `allowed_users` | 允许访问的 SMB 用户名列表，逗号分隔，null 表示不限 |
| `allowed_groups` | 允许访问的 SMB 组名列表，逗号分隔，null 表示不限 |
| `guest_ok` | 是否允许访客（无密码）访问 |
| `read_only` | 是否只读 |

## 版本历史

- **Phase 211** (2026-03-28): SMB 共享更新 API - 支持 allowed_users/allowed_groups/guest_ok/read_only
- **Phase 211** (2026-03-28): SMB 共享更新 API 初始实现（仅 guest_ok/read_only）
