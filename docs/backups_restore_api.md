# 备份恢复 API

## Phase 193

## 接口说明

恢复已归档的备份任务，将备份状态从 `archived` 变更为 `active`。

## 请求

`POST /api/v1/backups/{id}/restore`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 备份 ID |

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
  "message": "备份任务 '每日备份' 已恢复",
  "backup": {
    "id": 1,
    "name": "每日备份",
    "description": "每日凌晨备份",
    "backup_type": "full",
    "source_path": "/data/source",
    "destination_path": "/backup/daily",
    "schedule": "0 2 * * *",
    "status": "active",
    "last_run_at": 1711584000,
    "last_run_status": "completed",
    "last_run_size_bytes": 1073741824,
    "created_at": 1711497600,
    "updated_at": 1711670400
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
  "error": "Only admin users can restore backups",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 备份不存在

```json
{
  "success": false,
  "error": "Backup 999 not found",
  "code": "NOT_FOUND"
}
```

#### 409 Conflict - 状态冲突

```json
{
  "success": false,
  "error": "备份任务 '每日备份' 状态为 'idle'，仅 archived 状态的备份可恢复",
  "code": "CONFLICT"
}
```

#### 409 Conflict - 活跃备份冲突

```json
{
  "success": false,
  "error": "已有活跃备份任务，无法同时存在多个 active 状态的备份",
  "code": "CONFLICT"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "恢复备份任务失败：Update failed: database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 恢复归档的备份

```bash
curl -X POST "http://localhost:8080/api/v1/backups/1/restore" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "message": "备份任务 '每日备份' 已恢复",
  "backup": {
    "id": 1,
    "name": "每日备份",
    "description": "每日凌晨备份",
    "backup_type": "full",
    "source_path": "/data/source",
    "destination_path": "/backup/daily",
    "schedule": "0 2 * * *",
    "status": "active",
    "last_run_at": 1711584000,
    "last_run_status": "completed",
    "last_run_size_bytes": 1073741824,
    "created_at": 1711497600,
    "updated_at": 1711670400
  }
}
```

### 恢复不存在的备份

```bash
curl -X POST "http://localhost:8080/api/v1/backups/999/restore" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Backup 999 not found",
  "code": "NOT_FOUND"
}
```

### 恢复非 archived 状态的备份

```bash
curl -X POST "http://localhost:8080/api/v1/backups/1/restore" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "备份任务 '每日备份' 状态为 'idle'，仅 archived 状态的备份可恢复",
  "code": "CONFLICT"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证备份 ID 存在性（404 Not Found）
4. 检查备份状态是否为 `archived`（409 Conflict）
5. 检查是否已有活跃备份（409 Conflict）
6. 将备份状态从 `archived` 更新为 `active`
7. 返回恢复后的备份完整信息

## 状态流转

```
archived ──restore──> active
```

## 版本历史

- **Phase 193** (2026-03-27): 备份管理模块 - 备份恢复 API
