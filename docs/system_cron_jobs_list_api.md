# 系统定时任务列表 API 文档

## 概述

本文档描述 Axis NAS 系统中获取系统定时任务列表 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/system/cron-jobs`
- **版本**: v1
- **Phase**: 254

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Query 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `limit` | number | 否 | 50 | 每页数量（最大 200） |
| `offset` | number | 否 | 0 | 偏移量 |
| `status` | string | 否 | - | 按状态筛选：active/inactive |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "Daily Backup",
      "schedule": "0 2 * * *",
      "command": "/usr/local/bin/backup.sh",
      "status": "active",
      "last_run": 1711500000,
      "next_run": 1711600000,
      "created_at": 1711000000,
      "updated_at": 1711500000
    }
  ],
  "pagination": {
    "limit": 50,
    "offset": 0,
    "total": 10
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
  "error": "Only admin users can view cron jobs",
  "code": "FORBIDDEN"
}
```

#### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "Invalid status. Valid statuses: active, inactive",
  "code": "INVALID_STATUS"
}
```

#### 500 Internal Server Error - 系统错误

```json
{
  "success": false,
  "error": "Failed to get current time",
  "code": "INTERNAL_ERROR"
}
```

## 数据模型

### CronJobStatus

定时任务状态枚举值：
- `active` - 激活状态
- `inactive` - 非激活状态

### CronJobInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 任务 ID |
| `name` | string | 任务名称 |
| `schedule` | string | Cron 表达式或间隔描述 |
| `command` | string | 执行的命令/处理程序 |
| `status` | CronJobStatus | 任务状态 |
| `last_run` | number? | 上次执行时间戳（Unix 时间戳） |
| `next_run` | number? | 下次计划执行时间戳 |
| `created_at` | number | 创建时间戳 |
| `updated_at` | number | 更新时间戳 |

### PaginationInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `limit` | number | 每页数量 |
| `offset` | number | 偏移量 |
| `total` | number | 总任务数 |

### CronJobsResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | CronJobInfo[] | 定时任务列表 |
| `pagination` | PaginationInfo | 分页信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试访问 |
| `INVALID_STATUS` | 400 | 无效的状态筛选值 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求（默认参数）

```bash
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "Daily Backup",
      "schedule": "0 2 * * *",
      "command": "/usr/local/bin/backup.sh",
      "status": "active",
      "last_run": 1711500000,
      "next_run": 1711600000,
      "created_at": 1711000000,
      "updated_at": 1711500000
    },
    {
      "id": 2,
      "name": "Weekly Cleanup",
      "schedule": "0 3 * * 0",
      "command": "/usr/local/bin/cleanup.sh",
      "status": "active",
      "last_run": 1711000000,
      "next_run": 1711800000,
      "created_at": 1710500000,
      "updated_at": 1711000000
    }
  ],
  "pagination": {
    "limit": 50,
    "offset": 0,
    "total": 10
  }
}
```

### 请求（筛选激活状态的任务）

```bash
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs?status=active" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 请求（分页查询）

```bash
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs?limit=20&offset=20" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

## 权限说明

- **Admin 用户**: 可访问定时任务列表
- **普通用户**: 无权访问（返回 403 Forbidden）

## 实现细节

### Cron 表达式说明
- `0 2 * * *` - 每天凌晨 2 点执行
- `0 3 * * 0` - 每周日凌晨 3 点执行
- `0 4 1 * *` - 每月 1 日凌晨 4 点执行
- `0 * * * *` - 每小时执行

### 状态说明
- **active**: 任务处于激活状态，会按计划执行
- **inactive**: 任务处于非激活状态，不会执行

### 数据来源
- 当前为模拟实现，返回固定任务数据
- 实际实现可：
  - 读取系统 crontab 文件
  - 查询数据库中的定时任务配置
  - 集成系统调度服务

## 相关接口

- `POST /api/v1/system/cron-jobs` - 创建定时任务
- `GET /api/v1/system/cron-jobs/{id}` - 获取定时任务详情
- `PUT /api/v1/system/cron-jobs/{id}` - 更新定时任务
- `DELETE /api/v1/system/cron-jobs/{id}` - 删除定时任务

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取定时任务列表
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 定时任务列表

# 测试状态筛选
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs?status=active" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 激活状态的任务列表

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/system/cron-jobs"

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 254** (2026-03-28): 初始实现，模拟定时任务数据
