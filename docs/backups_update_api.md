# 备份更新 API

## Phase 165

## 接口说明

更新指定备份任务的配置，支持部分字段更新。

## 请求

`PUT /api/v1/backups/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 备份 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

所有字段均为可选，支持部分更新：

```json
{
  "name": "Updated Backup Name",
  "type": "weekly",
  "size": 2147483648,
  "status": "completed",
  "source_path": "/srv/newdata",
  "destination_path": "/srv/backups/new",
  "compression": true,
  "encryption": false
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 否 | 备份名称（1-128 字符） |
| type | string | 否 | 备份类型（daily/weekly/monthly/manual） |
| size | u64 | 否 | 备份大小（字节） |
| status | string | 否 | 状态（pending/running/completed/failed） |
| source_path | string | 否 | 源路径（必须以/开头，最大 512 字符） |
| destination_path | string | 否 | 目标路径（必须以/开头，最大 512 字符） |
| compression | boolean | 否 | 是否压缩 |
| encryption | boolean | 否 | 是否加密 |

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "Backup updated successfully",
  "data": {
    "id": 1,
    "name": "Updated Backup Name",
    "type": "weekly",
    "size": 2147483648,
    "status": "completed",
    "source_path": "/srv/newdata",
    "destination_path": "/srv/backups/new",
    "compression": true,
    "encryption": false,
    "created_at": "2026-03-27T00:00:00Z",
    "completed_at": "2026-03-27T01:30:00Z"
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid backup name. Must be 1-128 chars",
  "code": "INVALID_NAME"
}
```

```json
{
  "success": false,
  "error": "Invalid backup type. Valid types: daily, weekly, monthly, manual",
  "code": "INVALID_TYPE"
}
```

```json
{
  "success": false,
  "error": "Invalid source path. Must start with / and be <= 512 chars",
  "code": "INVALID_SOURCE_PATH"
}
```

```json
{
  "success": false,
  "error": "Invalid status. Valid statuses: pending, running, completed, failed",
  "code": "INVALID_STATUS"
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
  "error": "Only admin users can update backups",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 备份不存在

```json
{
  "success": false,
  "error": "Backup 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 更新备份名称

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Daily Backup"
  }'
```

### 更新多个字段

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Backup",
    "status": "completed",
    "size": 2147483648,
    "compression": true
  }'
```

### 尝试更新不存在的备份

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/999" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Name"
  }'
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Backup 999 not found",
  "code": "NOT_FOUND"
}
```

### 尝试使用无效的备份类型

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "type": "invalid"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid backup type. Valid types: daily, weekly, monthly, manual",
  "code": "INVALID_TYPE"
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
| data | object | 更新后的备份信息 |

### 备份信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 备份 ID |
| name | string | 备份名称 |
| type | string | 备份类型（daily/weekly/monthly/manual） |
| size | u64 | 备份大小（字节） |
| status | string | 状态（pending/running/completed/failed） |
| source_path | string | 源路径 |
| destination_path | string | 目标路径 |
| compression | boolean | 是否压缩 |
| encryption | boolean | 是否加密 |
| created_at | string | 创建时间（ISO 8601 格式） |
| completed_at | string\|null | 完成时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证备份 ID 存在性（404 Not Found）
4. 验证名称格式（如果提供）
5. 验证备份类型（如果提供）
6. 验证路径格式（如果提供）
7. 验证状态（如果提供）
8. 部分更新备份配置
9. 返回 200 OK + 更新后的备份详情

## 版本历史

- **Phase 165** (2026-03-27): 备份管理模块 - 备份更新 API
