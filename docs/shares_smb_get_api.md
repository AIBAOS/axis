# SMB 共享详情 API

## Phase 203

## 接口说明

获取单个 SMB 共享的详细信息。

## 请求

`GET /api/v1/shares/smb/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | SMB 共享 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
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

### 返回字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | integer | 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径 |
| description | string | 共享描述（可选） |
| allowed_users | string | 允许访问的用户名（逗号分隔，可选） |
| allowed_groups | string | 允许访问的组名（逗号分隔，可选） |
| guest_ok | boolean | 是否允许访客访问 |
| read_only | boolean | 是否只读 |
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

#### 404 Not Found - SMB 共享不存在

```json
{
  "success": false,
  "error": "SMB share 1 not found",
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

### 获取 SMB 共享详情

```bash
curl -X GET "http://localhost:8080/api/v1/shares/smb/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 响应（成功）

```json
{
  "success": true,
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
- **登录用户可访问**

## 业务逻辑

1. 验证 JWT Token 有效性
2. 查询 SMB 共享是否存在（404 Not Found）
3. 验证协议类型（仅 SMB）
4. 返回完整共享信息

## SMB 共享字段说明

| 字段 | 说明 |
| ---- | ---- |
| `allowed_users` | 允许访问的 SMB 用户名列表，逗号分隔 |
| `allowed_groups` | 允许访问的 SMB 组名列表，逗号分隔 |
| `guest_ok` | 是否允许访客（无密码）访问 |
| `read_only` | 是否只读 |

## 版本历史

- **Phase 203** (2026-03-28): SMB 共享详情 API - SqliteShareRepository 真实查询
- **Phase 203** (2026-03-28 02:05): SMB 共享详情 API 初始实现
