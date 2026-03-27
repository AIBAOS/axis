# 备份创建 API

## Phase 163

## 接口说明

创建新的备份任务。

## 请求

`POST /api/v1/backups`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

```json
{
  "name": "Manual Backup 2026-03-27",
  "type": "manual",
  "source_path": "/srv/data",
  "destination_path": "/srv/backups",
  "compression": true,
  "encryption": false
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 是 | 备份名称（1-128 字符） |
| type | string | 是 | 备份类型（daily/weekly/monthly/manual） |
| source_path | string | 是 | 源路径（必须以/开头，最大 512 字符） |
| destination_path | string | 是 | 目标路径（必须以/开头，最大 512 字符） |
| compression | boolean | 否 | 是否压缩（默认 true） |
| encryption | boolean | 否 | 是否加密（默认 false） |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "Backup task created successfully",
  "data": {
    "id": 6,
    "name": "Manual Backup 2026-03-27",
    "type": "manual",
    "size": 0,
    "status": "pending",
    "source_path": "/srv/data",
    "destination_path": "/srv/backups",
    "compression": true,
    "encryption": false,
    "created_at": "2026-03-27T09:00:00Z",
    "completed_at": null
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
  "error": "Invalid destination path. Must start with / and be <= 512 chars",
  "code": "INVALID_DESTINATION_PATH"
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
  "error": "Only admin users can create backups",
  "code": "FORBIDDEN"
}
```

## 示例

### 创建手动备份

```bash
curl -X POST "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Manual Backup 2026-03-27",
    "type": "manual",
    "source_path": "/srv/data",
    "destination_path": "/srv/backups",
    "compression": true,
    "encryption": false
  }'
```

### 创建加密备份

```bash
curl -X POST "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Encrypted Backup",
    "type": "manual",
    "source_path": "/srv/sensitive",
    "destination_path": "/srv/backups/secure",
    "compression": true,
    "encryption": true
  }'
```

### 尝试使用无效的备份类型

```bash
curl -X POST "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Backup",
    "type": "invalid",
    "source_path": "/srv/data",
    "destination_path": "/srv/backups"
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

### 创建结果字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| success | boolean | 是否成功 |
| message | string | 响应消息 |
| data | object | 创建的备份信息 |

### 备份信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 备份 ID |
| name | string | 备份名称 |
| type | string | 备份类型（daily/weekly/monthly/manual） |
| size | u64 | 备份大小（字节，初始为 0） |
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
3. 验证备份名称格式（1-128 字符）
4. 验证备份类型（daily/weekly/monthly/manual）
5. 验证源路径格式（以/开头，最大 512 字符）
6. 验证目标路径格式（以/开头，最大 512 字符）
7. 创建备份任务
8. 返回 201 Created + 备份详情

## 版本历史

- **Phase 163** (2026-03-27): 备份管理模块 - 备份创建 API
