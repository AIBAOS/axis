# NFS 共享详情 API

## Phase 156

## 接口说明

获取指定 NFS 共享的详细信息。

## 请求

`GET /api/v1/shares/nfs/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 共享 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

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
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      }
    ],
    "enabled": true,
    "status": "active",
    "created_at": "2026-03-27T06:00:00Z",
    "updated_at": "2026-03-27T06:00:00Z"
  }
}
```

### 错误响应

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
  "error": "Only admin users can view NFS share details",
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

## 示例

### 获取 NFS 共享详情

```bash
curl "http://localhost:8080/api/v1/shares/nfs/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的共享

```bash
curl "http://localhost:8080/api/v1/shares/nfs/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "NFS share 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

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
3. 根据共享 ID 查找共享
4. 共享不存在返回 404 Not Found
5. 返回 200 OK + 共享详情

## 版本历史

- **Phase 156** (2026-03-27): 初始版本
