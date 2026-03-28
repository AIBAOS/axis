# 系统定时任务创建 API

## Phase 255

## 接口说明

创建系统定时任务（cron jobs），用于自动化系统管理任务，仅限 admin 角色访问。

## 请求

`POST /api/v1/system/cron-jobs`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | application/json |

### 请求体

```json
{
  "name": "daily-backup",
  "schedule": "0 2 * * *",
  "command": "/usr/local/bin/backup.sh",
  "description": "Daily system backup",
  "enabled": true
}
```

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| name | string | 是 | - | 任务名称（1-128 字符，字母数字 -_.） |
| schedule | string | 是 | - | cron 表达式或预定义（@hourly/@daily/@weekly/@monthly） |
| command | string | 是 | - | 执行的命令（1-512 字符） |
| description | string | 否 | - | 任务描述（0-256 字符） |
| enabled | boolean | 否 | true | 是否启用 |

### 支持的预定义 schedule

| 预定义 | 等价 cron 表达式 | 说明 |
|--------|-----------------|------|
| @hourly | 0 * * * * | 每小时执行 |
| @daily | 0 0 * * * | 每天凌晨执行 |
| @weekly | 0 0 * * 0 | 每周日执行 |
| @monthly | 0 0 1 * * | 每月 1 号执行 |
| @yearly | 0 0 1 1 * | 每年 1 月 1 号执行 |
| @reboot | - | 系统启动时执行 |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "Cron job created successfully",
  "data": {
    "id": 12345,
    "name": "daily-backup",
    "schedule": "0 2 * * *",
    "command": "/usr/local/bin/backup.sh",
    "status": "active",
    "enabled": true,
    "description": "Daily system backup",
    "created_at": 1711634400,
    "updated_at": 1711634400
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| message | string | 响应消息 |
| data | object | 创建的任务信息 |
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

或（无效命令）：

```json
{
  "success": false,
  "error": "Invalid command. Must be 1-512 chars",
  "code": "INVALID_COMMAND"
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
  "error": "Only admin users can create cron jobs",
  "code": "FORBIDDEN"
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
  "error": "Failed to create cron job: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 创建每日备份任务

```bash
curl -X POST "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "daily-backup",
    "schedule": "0 2 * * *",
    "command": "/usr/local/bin/backup.sh",
    "description": "Daily system backup",
    "enabled": true
  }'
```

响应（201 Created）：
```json
{
  "success": true,
  "message": "Cron job created successfully",
  "data": {
    "id": 12345,
    "name": "daily-backup",
    "schedule": "0 2 * * *",
    "command": "/usr/local/bin/backup.sh",
    "status": "active",
    "enabled": true,
    "description": "Daily system backup",
    "created_at": 1711634400,
    "updated_at": 1711634400
  }
}
```

### 使用预定义 schedule 创建任务

```bash
curl -X POST "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "hourly-sync",
    "schedule": "@hourly",
    "command": "/usr/local/bin/sync.sh",
    "description": "Hourly data sync",
    "enabled": true
  }'
```

### 创建禁用的任务

```bash
curl -X POST "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "weekly-report",
    "schedule": "@weekly",
    "command": "/usr/local/bin/report.sh",
    "description": "Weekly system report",
    "enabled": false
  }'
```

### 创建重复名称的任务

```bash
curl -X POST "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "daily-backup",
    "schedule": "0 3 * * *",
    "command": "/usr/local/bin/backup2.sh"
  }'
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "Cron job with name 'daily-backup' already exists",
  "code": "NAME_CONFLICT"
}
```

### 无效的 schedule 格式

```bash
curl -X POST "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "invalid-task",
    "schedule": "invalid",
    "command": "/usr/local/bin/test.sh"
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

### 非 admin 用户创建任务

```bash
curl -X POST "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test-task",
    "schedule": "@daily",
    "command": "/usr/local/bin/test.sh"
  }'
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can create cron jobs",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X POST "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test-task",
    "schedule": "@daily",
    "command": "/usr/local/bin/test.sh"
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
3. 解析并验证请求体参数
4. 验证 name 格式和唯一性
5. 验证 schedule 格式（cron 表达式或预定义）
6. 验证 command 格式
7. 创建定时任务
8. 返回创建的任务信息

## 安全说明

- 此接口仅限 admin 用户调用
- 定时任务可能执行系统命令，建议添加操作审计日志
- 建议对 command 进行白名单验证，防止恶意命令执行

## Cron 表达式格式

标准 5 部分 cron 表达式：
```
分 时 日 月 星期
```

字段说明：
- 分：0-59
- 时：0-23
- 日：1-31
- 月：1-12
- 星期：0-6（0=周日）

特殊字符：
- `*` - 匹配所有值
- `,` - 分隔多个值（如 `1,3,5`）
- `-` - 指定范围（如 `1-5`）
- `/` - 指定步长（如 `*/5` 表示每 5 个单位）

## 版本历史

- **Phase 255** (2026-03-28): 系统模块 - 系统定时任务创建 API
