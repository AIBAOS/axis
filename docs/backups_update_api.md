# 备份任务更新 API

## Phase 191

## 接口说明

更新现有备份任务的配置。

## 请求

`PUT /api/v1/backups/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 备份任务 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

所有字段均为可选，支持部分更新：

```json
{
  "name": "Updated Daily Backup",
  "schedule": "0 3 * * *",
  "enabled": true,
  "retention_days": 14,
  "source_paths": ["/data", "/home"],
  "destination": "/backup/daily"
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 否 | 任务名称（1-128 字符） |
| schedule | string | 否 | 调度表达式（cron 或预定义：daily/weekly/monthly/hourly） |
| enabled | boolean | 否 | 是否启用 |
| retention_days | number | 否 | 保留天数 |
| source_paths | string[] | 否 | 源路径列表 |
| destination | string | 否 | 目标路径 |

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "Backup task updated successfully",
  "data": {
    "id": 1,
    "name": "Updated Daily Backup",
    "schedule": "0 3 * * *",
    "enabled": true,
    "retention_days": 14,
    "source_paths": ["/data", "/home"],
    "destination": "/backup/daily",
    "status": "completed",
    "created_at": "2026-03-01T00:00:00Z",
    "updated_at": "2026-03-27T19:00:00Z"
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid schedule format. Valid values: daily, weekly, monthly, hourly, or cron expression",
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

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can update backup tasks",
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

### 更新备份任务名称和调度

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Daily Backup",
    "schedule": "0 3 * * *"
  }'
```

### 更新保留策略

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "retention_days": 30
  }'
```

### 禁用备份任务

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": false
  }'
```

### 更新不存在的任务

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/999" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test"
  }'
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Backup task 999 not found",
  "code": "NOT_FOUND"
}
```

### 无效的调度格式

```bash
curl -X PUT "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "schedule": "invalid"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid schedule format. Valid values: daily, weekly, monthly, hourly, or cron expression",
  "code": "INVALID_SCHEDULE"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 备份任务字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 任务 ID |
| name | string | 任务名称 |
| schedule | string | 调度表达式 |
| enabled | boolean | 是否启用 |
| retention_days | number | 保留天数 |
| source_paths | string[] | 源路径列表 |
| destination | string | 目标路径 |
| status | string | 状态（active/completed/failed） |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据备份 ID 查找任务
4. 任务不存在返回 404 Not Found
5. 验证 schedule 格式（如果提供）
6. 部分更新任务配置
7. 更新时间戳
8. 返回 200 OK + 更新后的任务详情

## 版本历史

- **Phase 191** (2026-03-27): 备份管理模块 - 备份任务更新 API
