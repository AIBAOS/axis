# 系统定时任务更新 API 文档

## 概述

本文档描述 Axis NAS 系统中更新定时任务 API 的实现细节。

## API 端点

- **路径**: `PUT /api/v1/system/cron-jobs/{id}`
- **版本**: v1
- **Phase**: 258

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `id` | number | 是 | 定时任务 ID |

### Request Body（所有字段可选，支持部分更新）

```json
{
  "name": "Daily Backup",
  "schedule": "0 3 * * *",
  "command": "/usr/local/bin/backup.sh",
  "description": "Daily system backup",
  "enabled": true
}
```

#### 字段说明

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `name` | string | 否 | 任务名称（1-128 字符，字母数字 -_.） |
| `schedule` | string | 否 | Cron 表达式或预定义：@hourly/@daily/@weekly/@monthly |
| `command` | string | 否 | 执行的命令（1-512 字符） |
| `description` | string | 否 | 任务描述（0-256 字符） |
| `enabled` | boolean | 否 | 是否启用任务 |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Cron job updated successfully",
  "data": {
    "id": 1,
    "name": "Daily Backup",
    "schedule": "0 3 * * *",
    "command": "/usr/local/bin/backup.sh",
    "description": "Daily system backup",
    "status": "active",
    "enabled": true,
    "created_at": 1711600000,
    "updated_at": 1711700000
  }
}
```

### 错误响应

#### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can update cron jobs",
  "code": "FORBIDDEN"
}
```

#### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "Invalid name format. Must be 1-128 chars, alphanumeric with -_. allowed",
  "code": "INVALID_NAME"
}
```

或（无效 schedule）：

```json
{
  "success": false,
  "error": "Invalid schedule format. Must be cron expression or @hourly/@daily/@weekly/@monthly",
  "code": "INVALID_SCHEDULE"
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

#### 409 Conflict - 名称已存在

```json
{
  "success": false,
  "error": "Cron job with name 'Daily Backup' already exists",
  "code": "NAME_CONFLICT"
}
```

#### 500 Internal Server Error - 系统错误

```json
{
  "success": false,
  "error": "Failed to update cron job: Database error",
  "code": "DATABASE_ERROR"
}
```

## 数据模型

### UpdateCronJobRequest

| 字段 | 类型 | 描述 |
|------|------|------|
| `name` | string? | 任务名称 |
| `schedule` | string? | Cron 表达式 |
| `command` | string? | 执行的命令 |
| `description` | string? | 任务描述 |
| `enabled` | boolean? | 是否启用 |

### CronJobInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 任务 ID |
| `name` | string | 任务名称 |
| `schedule` | string | Cron 表达式 |
| `command` | string | 执行的命令 |
| `description` | string? | 任务描述 |
| `status` | string | 任务状态：active/inactive |
| `enabled` | boolean | 是否启用 |
| `created_at` | number | 创建时间戳 |
| `updated_at` | number | 更新时间戳 |

### UpdateCronJobResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data` | CronJobInfo | 更新后的任务信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试访问 |
| `INVALID_NAME` | 400 | 无效的任务名称格式 |
| `INVALID_SCHEDULE` | 400 | 无效的 Cron 表达式格式 |
| `NOT_FOUND` | 404 | 指定的任务不存在 |
| `NAME_CONFLICT` | 409 | 任务名称已存在 |
| `DATABASE_ERROR` | 500 | 数据库错误 |

## 示例

### 请求（更新任务名称和描述）

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Daily Backup V2",
    "description": "Updated daily system backup"
  }'
```

### 响应（成功）

```json
{
  "success": true,
  "message": "Cron job updated successfully",
  "data": {
    "id": 1,
    "name": "Daily Backup V2",
    "schedule": "0 2 * * *",
    "command": "/usr/local/bin/backup.sh",
    "description": "Updated daily system backup",
    "status": "active",
    "enabled": true,
    "created_at": 1711600000,
    "updated_at": 1711700000
  }
}
```

### 请求（更新 Cron 表达式）

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "schedule": "0 3 * * *"
  }'
```

### 请求（启用/禁用任务）

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": false
  }'
```

### 请求（名称冲突）

```bash
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Existing Job"
  }'
```

### 响应（名称冲突）

```json
{
  "success": false,
  "error": "Cron job with name 'Existing Job' already exists",
  "code": "NAME_CONFLICT"
}
```

## 权限说明

- **Admin 用户**: 可更新定时任务
- **普通用户**: 无权访问（返回 403 Forbidden）

## 实现细节

### 部分更新支持

所有请求体字段均为可选，支持部分更新：
- 仅提供需要更新的字段
- 未提供的字段保持原值

### Cron 表达式格式

支持标准 5 字段 Cron 表达式和预定义表达式：
- `0 2 * * *` - 每天凌晨 2 点
- `@hourly` - 每小时
- `@daily` - 每天
- `@weekly` - 每周
- `@monthly` - 每月

### 验证规则
- **name**: 1-128 字符，允许字母、数字、`-`、`_`、`.`，必须唯一
- **schedule**: Cron 表达式或预定义值
- **command**: 1-512 字符
- **description**: 0-256 字符（可选）
- **enabled**: boolean

### 数据来源
- 使用 SqliteCronJobRepository 实现真实数据库持久化
- 任务名称必须唯一（数据库 UNIQUE 约束）

## 相关接口

- `GET /api/v1/system/cron-jobs` - 获取定时任务列表
- `POST /api/v1/system/cron-jobs` - 创建定时任务
- `GET /api/v1/system/cron-jobs/{id}` - 获取定时任务详情
- `DELETE /api/v1/system/cron-jobs/{id}` - 删除定时任务

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试更新定时任务
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Job",
    "description": "Updated description"
  }'

# 预期：200 OK + 更新后的任务信息

# 测试名称冲突
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Existing Job"
  }'

# 预期：409 Conflict

# 测试未认证访问
curl -X PUT "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Job"
  }'

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 258** (2026-03-28): 初始实现，使用 SqliteCronJobRepository 持久化，支持部分更新
