# 备份任务执行 API

## Phase 190

## 接口说明

手动触发备份任务执行。

## 请求

`POST /api/v1/backups/{id}/execute`

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
  "message": "Backup task execution started successfully",
  "data": {
    "backup_id": 1,
    "status": "running",
    "message": "Backup task 'Daily Backup' is now executing",
    "started_at": "2026-03-27T18:00:00Z"
  }
}
```

### 错误响应

#### 400 Bad Request - 任务状态不允许执行

```json
{
  "success": false,
  "error": "Backup task is currently active. Only completed or failed tasks can be re-executed",
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
  "error": "Only admin users can execute backup tasks",
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

### 触发备份任务执行

```bash
curl -X POST "http://localhost:8080/api/v1/backups/1/execute" \
  -H "Authorization: Bearer <jwt_token>"
```

### 触发正在运行的任务

```bash
curl -X POST "http://localhost:8080/api/v1/backups/2/execute" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Backup task is currently active. Only completed or failed tasks can be re-executed",
  "code": "INVALID_STATUS"
}
```

### 触发不存在的任务

```bash
curl -X POST "http://localhost:8080/api/v1/backups/999/execute" \
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
- 仅限 admin 角色访问

## 响应字段说明

### 执行状态字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| backup_id | u64 | 备份任务 ID |
| status | string | 执行状态（running） |
| message | string | 执行消息 |
| started_at | string | 开始时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据备份 ID 查找任务
4. 任务不存在返回 404 Not Found
5. 验证任务状态（仅 completed/failed 状态可重新执行）
6. 触发备份任务执行
7. 返回 200 OK + 执行状态

## 版本历史

- **Phase 190** (2026-03-27): 备份管理模块 - 备份任务执行 API
