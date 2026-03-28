# NFS 共享详情 API

## Phase 214

## 接口说明

获取单个 NFS 共享的详细信息。

## 请求

`GET /api/v1/shares/nfs/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | NFS 共享 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "Data",
    "path": "/srv/nfs/data",
    "comment": "Data shared folder",
    "read_only": false,
    "no_subtree_check": true,
    "sync": true,
    "clients": "[{\"network\":\"192.168.1.0/24\",\"access\":\"rw\"}]",
    "enabled": true,
    "status": "active",
    "created_at": 1711584000,
    "updated_at": 1711584000
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | integer | 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径 |
| comment | string | 共享描述（可选） |
| read_only | boolean | 是否只读 |
| no_subtree_check | boolean | 是否禁用子树检查 |
| sync | boolean | 是否同步写入 |
| clients | string | 客户端配置（JSON 字符串，可选） |
| enabled | boolean | 是否启用 |
| status | string | 状态（active/inactive） |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |

### 错误响应

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
  "error": "Only admin users can view NFS share details",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - NFS 共享不存在

```json
{
  "success": false,
  "error": "NFS share 1 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "查询共享失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 获取 NFS 共享详情

```bash
curl -X GET "http://localhost:8080/api/v1/shares/nfs/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 响应（成功）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "Data",
    "path": "/srv/nfs/data",
    "comment": "Data shared folder",
    "read_only": false,
    "no_subtree_check": true,
    "sync": true,
    "clients": "[{\"network\":\"192.168.1.0/24\",\"access\":\"rw\"}]",
    "enabled": true,
    "status": "active",
    "created_at": 1711584000,
    "updated_at": 1711584000
  }
}
```

## 权限要求

- 需要 JWT 认证
- **仅 admin 用户可访问**

## 业务逻辑

1. 验证 JWT Token 有效性
2. 验证用户角色是否为 admin
3. 查询 NFS 共享是否存在（404 Not Found）
4. 验证协议类型（仅 NFS）
5. 返回完整共享信息

## 客户端配置格式

`clients` 字段为 JSON 格式的客户端配置数组：

```json
[
  {
    "network": "192.168.1.0/24",
    "access": "rw"
  }
]
```

## 版本历史

- **Phase 214** (2026-03-28): NFS 共享详情 API - SqliteShareRepository 真实查询
- **Phase 204** (2026-03-28): NFS 共享详情 API 初始实现
