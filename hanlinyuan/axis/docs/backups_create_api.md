# 备份任务创建 API

## Phase 261

## 接口说明

创建新的备份任务，支持全量备份 (full) 和增量备份 (incremental)。

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
  "name": "Daily Full Backup",
  "description": "每日全量备份系统数据",
  "source_path": "/srv/data",
  "destination": "/srv/backups",
  "backup_type": "full",
  "schedule": "0 2 * * *"
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 是 | 备份任务名称（1-128 字符，允许字母数字 -_ ） |
| description | string | 否 | 备份任务描述 |
| source_path | string | 是 | 源路径（必须以/开头，最大 512 字符） |
| destination | string | 是 | 目标路径（必须以/开头，最大 512 字符） |
| backup_type | string | 是 | 备份类型：full（全量）或 incremental（增量） |
| schedule | string | 否 | Cron 表达式，可选，用于计划任务 |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "Backup task created successfully",
  "data": {
    "id": 1,
    "name": "Daily Full Backup",
    "description": "每日全量备份系统数据",
    "backup_type": "full",
    "source_path": "/srv/data",
    "destination_path": "/srv/backups",
    "schedule": "0 2 * * *",
    "status": "idle",
    "created_at": 1711641600,
    "updated_at": 1711641600
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid backup name. Must be 1-128 chars, alphanumeric with -_  allowed",
  "code": "INVALID_NAME"
}
```

```json
{
  "success": false,
  "error": "Invalid backup type. Valid types: full, incremental",
  "code": "INVALID_BACKUP_TYPE"
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
  "code": "INVALID_DESTINATION"
}
```

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

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
  "error": "Only admin users can create backup tasks",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "创建备份任务失败：数据库操作失败",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 创建全量备份任务

```bash
curl -X POST "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Daily Full Backup",
    "description": "每日全量备份系统数据",
    "source_path": "/srv/data",
    "destination": "/srv/backups",
    "backup_type": "full",
    "schedule": "0 2 * * *"
  }'
```

### 创建增量备份任务

```bash
curl -X POST "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Hourly Incremental Backup",
    "source_path": "/srv/data",
    "destination": "/srv/backups/incremental",
    "backup_type": "incremental"
  }'
```

### 尝试使用无效的备份类型

```bash
curl -X POST "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Backup",
    "source_path": "/srv/data",
    "destination": "/srv/backups",
    "backup_type": "daily"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid backup type. Valid types: full, incremental",
  "code": "INVALID_BACKUP_TYPE"
}
```

### 非 admin 用户尝试创建

```bash
curl -X POST "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Backup",
    "source_path": "/srv/data",
    "destination": "/srv/backups",
    "backup_type": "full"
  }'
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can create backup tasks",
  "code": "FORBIDDEN"
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
| data | object | 创建的备份任务信息 |

### 备份任务信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | i64 | 备份任务 ID（自增） |
| name | string | 备份任务名称 |
| description | string | 备份任务描述 |
| backup_type | string | 备份类型（full/incremental） |
| source_path | string | 源路径 |
| destination_path | string | 目标路径 |
| schedule | string\|null | Cron 表达式（计划任务） |
| status | string | 状态（idle/running/completed/failed/archived） |
| created_at | i64 | 创建时间（Unix 时间戳） |
| updated_at | i64 | 更新时间（Unix 时间戳） |

## 业务逻辑

1. 从 Authorization 头提取 JWT Token
2. 验证 JWT Token 有效性
3. 检查用户角色是否包含 admin
4. 验证备份任务名称格式（1-128 字符，允许字母数字 -_ ）
5. 验证备份类型（full 或 incremental）
6. 验证源路径格式（以/开头，最大 512 字符）
7. 验证目标路径格式（以/开头，最大 512 字符）
8. 使用 SqliteBackupRepository 创建备份任务并持久化
9. 返回 201 Created + 备份任务详情

## 版本历史

- **Phase 261** (2026-03-28): 备份任务创建 API - 使用 SqliteBackupRepository 持久化，支持 full/incremental 类型
- **Phase 163** (2026-03-27): 备份管理模块 - 备份创建 API（旧版，已废弃）
