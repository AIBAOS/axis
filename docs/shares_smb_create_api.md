# SMB 共享创建 API

## Phase 210

## 接口说明

创建 SMB 共享配置。用于将本地目录通过 SMB 协议共享给网络用户。

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
  "name": "shared_folder",
  "path": "/data/shared",
  "description": "共享文件夹",
  "allowed_users": "user1,user2",
  "allowed_groups": "group1",
  "guest_ok": false,
  "read_only": false
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 是 | 共享名称（1-64 字符，字母数字及 -_.） |
| path | string | 是 | 共享路径（必须以 / 开头，最长 256 字符） |
| description | string | 否 | 共享描述 |
| allowed_users | string | 否 | 允许访问的用户名（逗号分隔） |
| allowed_groups | string | 否 | 允许访问的组名（逗号分隔） |
| guest_ok | boolean | 否 | 是否允许访客访问（默认 false） |
| read_only | boolean | 否 | 是否只读（默认 false） |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "SMB share created successfully",
  "data": {
    "id": 1,
    "name": "shared_folder",
    "path": "/data/shared",
    "description": "共享文件夹",
    "allowed_users": "user1,user2",
    "allowed_groups": "group1",
    "guest_ok": false,
    "read_only": false,
    "status": "active",
    "created_at": 1711584000,
    "updated_at": 1711584000
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
  "error": "Only admin users can create SMB shares",
  "code": "FORBIDDEN"
}
```

#### 409 Conflict - 名称冲突

```json
{
  "success": false,
  "error": "SMB share name 'shared_folder' already exists",
  "code": "NAME_CONFLICT"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "创建共享失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 创建 SMB 共享（无访客访问）

```bash
curl -X POST "http://localhost:8080/api/v1/shares/smb" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "shared_folder",
    "path": "/data/shared",
    "description": "共享文件夹",
    "allowed_users": "user1,user2",
    "allowed_groups": "group1",
    "guest_ok": false,
    "read_only": false
  }'
```

### 创建 SMB 共享（允许访客只读）

```bash
curl -X POST "http://localhost:8080/api/v1/shares/smb" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "public_share",
    "path": "/data/public",
    "guest_ok": true,
    "read_only": true
  }'
```

### 响应（成功）

```json
{
  "success": true,
  "message": "SMB share created successfully",
  "data": {
    "id": 1,
    "name": "shared_folder",
    "path": "/data/shared",
    "description": "共享文件夹",
    "allowed_users": "user1,user2",
    "allowed_groups": "group1",
    "guest_ok": false,
    "read_only": false,
    "status": "active",
    "created_at": 1711584000,
    "updated_at": 1711584000
  }
}
```

## 权限要求

- 需要 JWT 认证
- **仅 admin 用户可创建 SMB 共享**

## 业务逻辑

1. 验证 JWT Token 有效性
2. 验证用户角色是否为 admin
3. 验证共享名称格式（1-64 字符，字母数字及 -_.）
4. 验证共享路径格式（以 / 开头，最长 256 字符）
5. 验证路径是否存在
6. 检查名称唯一性（409 Conflict）
7. 创建 SMB 共享记录
8. 返回创建的共享详情

## SMB 共享字段说明

| 字段 | 说明 |
| ---- | ---- |
| `allowed_users` | 允许访问的 SMB 用户名列表，逗号分隔 |
| `allowed_groups` | 允许访问的 SMB 组名列表，逗号分隔 |
| `guest_ok` | 是否允许访客（无密码）访问 |
| `read_only` | 是否只读，false 表示可读写 |

## 版本历史

- **Phase 210** (2026-03-28): SMB 共享创建 API - 支持 allowed_users/allowed_groups/guest_ok/read_only
- **Phase 201** (2026-03-28): SMB 共享创建 API 初始实现（使用 `public` 字段）
