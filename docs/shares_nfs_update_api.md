# NFS 共享更新 API

## Phase 158

## 接口说明

更新指定 NFS 共享的配置，支持部分字段更新。

## 请求

`PUT /api/v1/shares/nfs/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 共享 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

所有字段均为可选，支持部分更新：

```json
{
  "name": "NewName",
  "path": "/srv/nfs/newpath",
  "comment": "Updated comment",
  "read_only": true,
  "no_subtree_check": true,
  "sync": true,
  "clients": [
    {
      "network": "192.168.1.0/24",
      "access": "rw"
    }
  ]
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 否 | 共享名称（1-64 字符，字母数字 -_.） |
| path | string | 否 | 共享路径（必须以/开头，最大 256 字符） |
| comment | string | 否 | 备注描述 |
| read_only | boolean | 否 | 是否只读 |
| no_subtree_check | boolean | 否 | 是否禁用子树检查 |
| sync | boolean | 否 | 是否同步写入 |
| clients | ClientConfig[] | 否 | 客户端配置列表 |

### ClientConfig 对象

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| network | string | 是 | 客户端网络（CIDR 格式，如 192.168.1.0/24） |
| access | string | 是 | 访问权限（ro/rw） |

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "NFS share updated successfully",
  "data": {
    "id": 1,
    "name": "NewName",
    "path": "/srv/nfs/newpath",
    "comment": "Updated comment",
    "read_only": true,
    "no_subtree_check": true,
    "sync": true,
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      }
    ],
    "enabled": true,
    "status": "active",
    "created_at": "2026-03-27T06:00:00Z",
    "updated_at": "2026-03-27T08:00:00Z"
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

```json
{
  "success": false,
  "error": "At least one client configuration is required",
  "code": "INVALID_CLIENTS"
}
```

```json
{
  "success": false,
  "error": "Invalid client network '192.168.1.1'. Must be CIDR format (e.g., 192.168.1.0/24)",
  "code": "INVALID_NETWORK"
}
```

```json
{
  "success": false,
  "error": "Invalid client access 'read'. Must be 'ro' or 'rw'",
  "code": "INVALID_ACCESS"
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
  "error": "Only admin users can update NFS shares",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 共享不存在

```json
{
  "success": false,
  "error": "NFS share 999 not found",
  "code": "NOT_FOUND"
}
```

#### 409 Conflict - 名称冲突

```json
{
  "success": false,
  "error": "NFS share name 'Data' already exists",
  "code": "NAME_CONFLICT"
}
```

## 示例

### 更新 NFS 共享名称

```bash
curl -X PUT "http://localhost:8080/api/v1/shares/nfs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "NewData"
  }'
```

### 更新多个字段

```bash
curl -X PUT "http://localhost:8080/api/v1/shares/nfs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "NewData",
    "comment": "Updated data folder",
    "read_only": true,
    "sync": false
  }'
```

### 更新客户端配置

```bash
curl -X PUT "http://localhost:8080/api/v1/shares/nfs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      },
      {
        "network": "10.0.0.0/8",
        "access": "ro"
      }
    ]
  }'
```

### 尝试更新不存在的共享

```bash
curl -X PUT "http://localhost:8080/api/v1/shares/nfs/999" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "NewName"
  }'
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "NFS share 999 not found",
  "code": "NOT_FOUND"
}
```

### 尝试使用已存在的名称

```bash
curl -X PUT "http://localhost:8080/api/v1/shares/nfs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Backup"
  }'
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "NFS share name 'Backup' already exists",
  "code": "NAME_CONFLICT"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 更新结果字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| success | boolean | 是否成功 |
| message | string | 响应消息 |
| data | object | 更新后的共享信息 |

### 共享信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径 |
| comment | string | 备注描述 |
| read_only | boolean | 是否只读 |
| no_subtree_check | boolean | 是否禁用子树检查 |
| sync | boolean | 是否同步写入 |
| clients | ClientConfig[] | 客户端配置列表 |
| enabled | boolean | 是否启用 |
| status | string | 状态（active/inactive） |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

### 客户端配置字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| network | string | 客户端网络（CIDR 格式） |
| access | string | 访问权限（ro/rw） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证共享 ID 存在性（404 Not Found）
4. 验证名称格式（如果提供）
5. 验证路径格式（如果提供）
6. 验证客户端配置（如果提供）
7. 验证名称唯一性（排除自身）
8. 部分更新共享配置
9. 更新时间戳
10. 返回 200 OK + 更新后的共享详情

## 版本历史

- **Phase 158** (2026-03-27): 初始版本
