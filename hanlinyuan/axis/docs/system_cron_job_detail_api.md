# 系统定时任务详情 API

## Phase 256

## 接口说明

获取单个系统定时任务（cron job）的详细信息，用于 Web UI 系统管理展示，仅限 admin 角色访问。

## 请求

`GET /api/v1/system/cron-jobs/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 定时任务 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "daily-backup",
    "schedule": "0 2 * * *",
    "command": "/usr/local/bin/backup.sh",
    "status": "active",
    "last_run": 1711584000,
    "next_run": 1711670400,
    "enabled": true,
    "description": "Daily system backup",
    "created_at": 1711497600,
    "updated_at": 1711584000
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | object | 定时任务详情 |
| data.id | integer | 任务 ID |
| data.name | string | 任务名称 |
| data.schedule | string | cron 表达式 |
| data.command | string | 执行的命令 |
| data.status | string | 任务状态（active/inactive） |
| data.last_run | integer | 最后运行时间（Unix 时间戳，可选） |
| data.next_run | integer | 下次运行时间（Unix 时间戳，可选） |
| data.enabled | boolean | 是否启用 |
| data.description | string | 任务描述 |
| data.created_at | integer | 创建时间（Unix 时间戳） |
| data.updated_at | integer | 更新时间（Unix 时间戳） |

### 错误响应

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
  "error": "Only admin users can view cron job details",
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

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to get cron job: io error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取定时任务详情

```bash
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "daily-backup",
    "schedule": "0 2 * * *",
    "command": "/usr/local/bin/backup.sh",
    "status": "active",
    "last_run": 1711584000,
    "next_run": 1711670400,
    "enabled": true,
    "description": "Daily system backup",
    "created_at": 1711497600,
    "updated_at": 1711584000
  }
}
```

### 获取不存在的任务

```bash
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs/999" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Cron job 999 not found",
  "code": "NOT_FOUND"
}
```

### 非 admin 用户获取任务详情

```bash
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view cron job details",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs/1"
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
4. 从数据库读取定时任务详情
5. 任务不存在返回 404 Not Found
6. 返回定时任务详情

## 任务状态说明

| 状态 | 说明 |
|------|------|
| active | 已激活（已启用且等待执行） |
| inactive | 未激活（已禁用） |

## 安全说明

- 此接口仅限 admin 用户调用
- 定时任务详情包含系统配置信息，建议添加访问审计
- 建议限制调用频率防止资源消耗

## 版本历史

- **Phase 256** (2026-03-28): 系统模块 - 系统定时任务详情 API
