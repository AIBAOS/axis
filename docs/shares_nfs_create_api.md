# NFS 共享创建 API

## Phase 154

## 接口说明

创建新的 NFS 共享。

## 请求

`POST /api/v1/shares/nfs`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

```json
{
  "name": "Data",
  "path": "/srv/nfs/data",
  "comment": "Data shared folder",
  "read_only": false,
  "no_subtree_check": true,
  "sync": true,
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
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 是 | 共享名称（1-64 字符，字母数字 -_.） |
| path | string | 是 | 共享路径（必须以/开头，最大 256 字符） |
| comment | string | 否 | 备注描述 |
| read_only | boolean | 否 | 是否只读（默认 false） |
| no_subtree_check | boolean | 否 | 是否禁用子树检查（默认 true） |
| sync | boolean | 否 | 是否同步写入（默认 true） |
| clients | ClientConfig[] | 是 | 客户端配置列表（至少 1 个） |

### ClientConfig 对象

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| network | string | 是 | 客户端网络（CIDR 格式，如 192.168.1.0/24） |
| access | string | 是 | 访问权限（ro/rw） |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "NFS share created successfully",
  "data": {
    "id": 4,
    "name": "Data",
    "path": "/srv/nfs/data",
    "comment": "Data shared folder",
    "read_only": false,
    "no_subtree_check": true,
    "sync": true,
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      },
      {
        "network": "10.0.0.0/8",
        "access": "ro"
      }
    ],
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
  "error": "Only admin users can create NFS shares",
  "code": "FORBIDDEN"
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

### 创建 NFS 共享

```bash
curl -X POST "http://localhost:8080/api/v1/shares/nfs" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Data",
    "path": "/srv/nfs/data",
    "comment": "Data shared folder",
    "read_only": false,
    "no_subtree_check": true,
    "sync": true,
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      }
    ]
  }'
```

### 创建只读 NFS 共享

```bash
curl -X POST "http://localhost:8080/api/v1/shares/nfs" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ReadOnly",
    "path": "/srv/nfs/readonly",
    "comment": "Read-only shared folder",
    "read_only": true,
    "no_subtree_check": true,
    "sync": false,
    "clients": [
      {
        "network": "192.168.0.0/16",
        "access": "ro"
      }
    ]
  }'
```

### 尝试创建已存在的共享

```bash
curl -X POST "http://localhost:8080/api/v1/shares/nfs" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Data",
    "path": "/srv/nfs/data2",
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      }
    ]
  }'
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "NFS share name 'Data' already exists",
  "code": "NAME_CONFLICT"
}
```

### 尝试使用无效网络格式

```bash
curl -X POST "http://localhost:8080/api/v1/shares/nfs" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test",
    "path": "/srv/nfs/test",
    "clients": [
      {
        "network": "192.168.1.1",
        "access": "rw"
      }
    ]
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid client network '192.168.1.1'. Must be CIDR format (e.g., 192.168.1.0/24)",
  "code": "INVALID_NETWORK"
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
3. 验证共享名称格式（1-64 字符，字母数字 -_.）
4. 验证共享路径格式（必须以/开头，最大 256 字符）
5. 验证客户端配置（至少 1 个，CIDR 格式，access 为 ro/rw）
6. 验证名称唯一性
7. 创建 NFS 共享配置
8. 返回 201 Created + 共享详情

## 版本历史

- **Phase 154** (2026-03-27): 初始版本
