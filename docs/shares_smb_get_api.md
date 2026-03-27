# SMB 共享详情 API

## Phase 155

## 接口说明

获取指定 SMB 共享的详细信息。

## 请求

`GET /api/v1/shares/smb/{id}`

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
    "name": "Public",
    "path": "/srv/samba/public",
    "comment": "Public shared folder",
    "read_only": false,
    "guest_access": true,
    "browseable": true,
    "valid_users": [],
    "invalid_users": [],
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
  "error": "Only admin users can view SMB share details",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 共享不存在

```json
{
  "success": false,
  "error": "SMB share 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 获取 SMB 共享详情

```bash
curl "http://localhost:8080/api/v1/shares/smb/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的共享

```bash
curl "http://localhost:8080/api/v1/shares/smb/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "SMB share 999 not found",
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
3. 根据共享 ID 查找共享
4. 共享不存在返回 404 Not Found
5. 返回 200 OK + 共享详情

## 版本历史

- **Phase 155** (2026-03-27): 初始版本
