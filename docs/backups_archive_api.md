# Phase 194: 备份归档 API

## 概述

- **接口**: `POST /api/v1/backups/{id}/archive`
- **功能**: 归档备份任务（将状态从 `active` 或 `completed` 更新为 `archived`）
- **权限**: 仅 admin 用户可访问
- **Phase**: 194

## 请求参数

| 参数名 | 位置 | 类型 | 必填 | 说明 |
|--------|------|------|------|------|
| id | path | integer | 是 | 备份任务 ID |

## 请求示例

```bash
curl -X POST http://localhost:8080/api/v1/backups/1/archive \
  -H "Authorization: Bearer <your_jwt_token>"
```

## 响应

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Backup 'Daily Backup' archived successfully",
  "data": {
    "id": 1,
    "name": "Daily Backup",
    "description": "Daily backup of critical data",
    "backup_type": "daily",
    "source_path": "/srv/data",
    "destination_path": "/srv/backups/daily",
    "schedule": "0 2 * * *",
    "status": "archived",
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
  "error": "Only admin users can archive backups",
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
  "error": "Backup is already archived",
  "code": "ALREADY_ARCHIVED"
}
```

#### 400 Bad Request

```json
{
  "success": false,
  "error": "Backup status is 'running'. Only active or completed backups can be archived",
  "code": "INVALID_STATUS"
}
```

## 业务规则

1. **允许归档的状态**：`active` 或 `completed` 状态的备份可以归档
2. **禁止归档的状态**：
   - `running`：正在运行中，返回 400 Bad Request
   - `archived`：已归档，返回 409 Conflict
   - `pending`、`failed` 等其他状态，返回 400 Bad Request
3. **admin 权限**：仅 admin 用户可执行归档操作
4. **状态更新**：成功归档后，备份状态从 `active` 或 `completed` 变更为 `archived`

## 数据库变更

- 更新备份表 `backups` 的 `status` 字段为 `'archived'`
- 更新 `updated_at` 时间戳

## 实现历史

- **Phase 194**: 备份归档 API (POST /api/v1/backups/{id}/archive) - 2026-03-28
