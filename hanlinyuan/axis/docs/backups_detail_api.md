# 备份任务详情 API

## Phase 190

## 接口说明

获取指定备份任务的详细信息。

## 请求

`GET /api/v1/backups/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 备份任务 ID |

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
    "name": "Daily Backup",
    "description": "Daily backup of system data",
    "source_path": "/data",
    "destination_path": "/backup/daily",
    "schedule": "0 2 * * *",
    "status": "active",
    "last_run": "2026-03-27T02:00:00Z",
    "next_run": "2026-03-28T02:00:00Z",
    "last_duration": 3600,
    "last_status": "success",
    "retention_policy": "7d",
    "created_at": "2026-03-01T00:00:00Z",
    "updated_at": "2026-03-27T02:00:00Z"
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
  "error": "Only admin users can view backup task details",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 任务不存在

```json
{
  "success": false,
  "error": "Backup task 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 获取备份任务详情

```bash
curl "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的任务

```bash
curl "http://localhost:8080/api/v1/backups/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Backup task 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 登录用户可访问

## 响应字段说明

### 备份任务详情字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 任务 ID |
| name | string | 任务名称 |
| description | string | 任务描述 |
| source_path | string | 源路径 |
| destination_path | string | 目标路径 |
| schedule | string | Cron 表达式 |
| status | string | 状态（active/inactive） |
| last_run | string\|null | 最后执行时间 |
| next_run | string\|null | 下次执行时间 |
| last_duration | number\|null | 最后执行耗时（秒） |
| last_status | string\|null | 最后执行状态 |
| retention_policy | string\|null | 保留策略 |
| created_at | string | 创建时间 |
| updated_at | string | 更新时间 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 根据任务 ID 查找任务
3. 任务不存在返回 404 Not Found
4. 返回 200 OK + 任务详情

## 版本历史

- **Phase 190** (2026-03-27): 备份管理模块 - 备份任务详情 API
