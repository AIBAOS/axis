# Phase 193: 备份恢复 API

## 概述

- **接口**: `POST /api/v1/backups/{id}/restore`
- **功能**: 恢复已归档的备份任务（将状态从 `archived` 更新为 `active`）
- **权限**: 仅 admin 用户可访问
- **Phase**: 193

## 请求参数

| 参数名 | 位置 | 类型 | 必填 | 说明 |
|--------|------|------|------|------|
| id | path | integer | 是 | 备份任务 ID |

## 请求示例

```bash
curl -X POST http://localhost:8080/api/v1/backups/1/restore \
  -H "Authorization: Bearer <your_jwt_token>"
```

## 响应

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Backup 'Daily Backup' restored successfully",
  "data": {
    "id": 1,
    "name": "Daily Backup",
    "description": "Daily backup of critical data",
    "backup_type": "daily",
    "source_path": "/srv/data",
    "destination_path": "/srv/backups/daily",
    "schedule": "0 2 * * *",
    "status": "active",
    "last_run_at": 1711526400,
    "last_run_status": "completed",
    "last_run_size_bytes": 1073741824,
    "created_at": 1711500000,
    "updated_at": 1711530000
  }
}
```

### 错误响应

#### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden

```json
{
  "success": false,
  "error": "Only admin users can restore backups",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found

```json
{
  "success": false,
  "error": "Backup not found",
  "code": "NOT_FOUND"
}
```

#### 409 Conflict

```json
{
  "success": false,
  "error": "A backup with name 'Daily Backup' is already active",
  "code": "CONFLICT"
}
```

#### 400 Bad Request

```json
{
  "success": false,
  "error": "Backup status is 'completed'. Only archived backups can be restored",
  "code": "INVALID_STATUS"
}
```

## 业务规则

1. **仅可恢复已归档备份**：只有状态为 `archived` 的备份可以恢复
2. **同名冲突检查**：恢复时不允许存在同名的 `active` 备份（409 Conflict）
3. **admin 权限**：仅 admin 用户可执行恢复操作
4. **状态更新**：成功恢复后，备份状态从 `archived` 变更为 `active`

## 数据库变更

- 更新备份表 `backups` 的 `status` 字段为 `'active'`
- 更新 `updated_at` 时间戳

## 实现历史

- **Phase 193**: 备份恢复 API (POST /api/v1/backups/{id}/restore) - 2026-03-27
