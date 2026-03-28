# 系统定时任务更新 API

## Phase 258

## 接口说明

更新系统定时任务（cron jobs）配置，用于修改已有定时任务的参数，仅限 admin 角色访问。

## 请求

`PUT /api/v1/system/cron-jobs/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | application/json |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 定时任务 ID |

### 请求体

```json
{
  "name": "daily-backup-updated",
  "schedule": "0 3 * * *",
  "command": "/usr/local/bin/backup-v2.sh",
  "description": "Updated daily backup",
  "enabled": true
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 否 | 任务名称（1-128 字符，字母数字 -_.） |
| schedule | string | 否 | cron 表达式或预定义（@hourly/@daily/@weekly/@monthly） |
| command | string | 否 | 执行的命令（1-512 字符） |
| description | string | 否 | 任务描述（0-256 字符） |
| enabled | boolean | 否 | 是否启用 |

**注意：** 所有字段均为可选，支持部分更新。

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "Cron job updated successfully",
  "data": {
    "id": 1,
    "name": "daily-backup-updated",
    "schedule": "0 3 * * *",
    "command": "/usr/local/bin/backup-v2.sh",
    "status": "active",
    "enabled": true,
    "description": "Updated daily backup",
    "created_at": 1711497600,
    "updated_at": 1711634400
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| message | string | 响应消息 |
| data | object | 更新后的任务信息 |
| data.id | integer | 任务 ID |
| data.name | string | 任务名称 |
| data.schedule | string | cron 表达式 |
| data.command | string | 执行的命令 |
| data.status | string | 任务状态（active/inactive） |
| data.enabled | boolean | 是否启用 |
| data.description | string | 任务描述 |
| data.created_at | integer | 创建时间（Unix 时间戳） |
| data.updated_at | integer | 更新时间（Unix 时间戳） |

### 错误响应

#### 400 Bad Request - 参数格式错误

```json
{
  "success": false,
  "error": "Invalid name. Must be 1-128 chars, alphanumeric with -_. allowed",
  "code": "INVALID_NAME"
}
```

或（无效 schedule）：

```json
{
  "success": false,
  "error": "Invalid schedule. Must be valid cron expression (5 parts) or predefined (@hourly/@daily/@weekly/@monthly)",
  "code": "INVALID_SCHEDULE"
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

#### 403 Forbidden - 权限不足（非 admin）

```json
{
  "success": false,
  "error": "Only admin users can update cron jobs",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 任务不存在

```json
{
  "success": false,
  "error": "Cron job 999 not found",
  "code": "NOT_FOUND"
}
```

#### 409 Conflict - 任务名称重复

```json
{
  "success": false,
  "error": "Cron job with name 'daily-backup' already exists",
  "code": "NAME_CONFLICT"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to update cron job: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 更新定时任务（全量更新）

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "daily-backup-updated",
    "schedule": "0 3 * * *",
    "command": "/usr/local/bin/backup-v2.sh",
    "description": "Updated daily backup",
    "enabled": true
  }'
```

响应（200 OK）：
```json
{
  "success": true,
  "message": "Cron job updated successfully",
  "data": {
    "id": 1,
    "name": "daily-backup-updated",
    "schedule": "0 3 * * *",
    "command": "/usr/local/bin/backup-v2.sh",
    "status": "active",
    "enabled": true,
    "description": "Updated daily backup",
    "created_at": 1711497600,
    "updated_at": 1711634400
  }
}
```

### 部分更新（仅更新 enabled）

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": false
  }'
```

### 部分更新（仅更新 schedule）

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "schedule": "@daily"
  }'
```

### 更新不存在的任务

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/999" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "new-name"
  }'
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Cron job 999 not found",
  "code": "NOT_FOUND"
}
```

### 无效的 schedule 格式

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "schedule": "invalid"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid schedule. Must be valid cron expression (5 parts) or predefined (@hourly/@daily/@weekly/@monthly)",
  "code": "INVALID_SCHEDULE"
}
```

### 非 admin 用户更新任务

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": false
  }'
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can update cron jobs",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": false
  }'
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析任务 ID 路径参数
4. 解析并验证请求体参数
5. 验证任务是否存在（404 Not Found）
6. 验证 name 格式和唯一性
7. 验证 schedule 格式（cron 表达式或预定义）
8. 验证 command 格式
9. 更新定时任务
10. 返回更新后的任务信息

## 安全说明

- 此接口仅限 admin 用户调用
- 定时任务可能执行系统命令，建议添加操作审计日志
- 建议对 command 进行白名单验证，防止恶意命令执行
- 更新操作应记录到审计日志

## 版本历史

- **Phase 258** (2026-03-28): 系统模块 - 系统定时任务更新 API
