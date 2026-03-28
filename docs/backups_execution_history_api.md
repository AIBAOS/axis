# 备份执行历史 API

## Phase 195

## 接口说明

获取指定备份任务的执行历史记录列表，支持分页查询。

## 请求

`GET /api/v1/backups/{id}/execution-history`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 备份任务 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ------- | ---- |
| page | integer | 否 | 1 | 页码（从 1 开始） |
| per_page | integer | 否 | 20 | 每页数量（最大 100） |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "backup_id": 1,
    "executions": [
      {
        "execution_id": 15,
        "backup_id": 1,
        "status": "completed",
        "started_at": 1711584000,
        "completed_at": 1711584180,
        "duration_seconds": 180,
        "error_message": null
      },
      {
        "execution_id": 14,
        "backup_id": 1,
        "status": "completed",
        "started_at": 1711497600,
        "completed_at": 1711497720,
        "duration_seconds": 120,
        "error_message": null
      },
      {
        "execution_id": 13,
        "backup_id": 1,
        "status": "failed",
        "started_at": 1711411200,
        "completed_at": 1711411260,
        "duration_seconds": 60,
        "error_message": "Source directory not found"
      }
    ],
    "total": 15,
    "page": 1,
    "per_page": 20
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| execution_id | integer | 执行记录 ID |
| backup_id | integer | 备份任务 ID |
| status | string | 执行状态（running/completed/failed/cancelled） |
| started_at | integer | 开始时间（Unix 时间戳） |
| completed_at | integer | 完成时间（Unix 时间戳，null 表示未完成） |
| duration_seconds | integer | 执行耗时（秒） |
| error_message | string | 错误信息（失败时） |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
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

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "查询执行历史失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 获取备份执行历史（第一页）

```bash
curl -X GET "http://localhost:8080/api/v1/backups/1/execution-history?page=1&per_page=20" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取备份执行历史（第二页，每页 10 条）

```bash
curl -X GET "http://localhost:8080/api/v1/backups/1/execution-history?page=2&per_page=10" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的备份执行历史

```bash
curl -X GET "http://localhost:8080/api/v1/backups/999/execution-history" \
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

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问自己的备份

## 业务逻辑

1. 验证 JWT Token 有效性
2. 验证备份 ID 存在性（404 Not Found）
3. 查询执行历史记录
4. 按 started_at 降序排列（最新的在前）
5. 应用分页
6. 返回执行历史列表

## 执行状态

| 状态 | 说明 |
| ---- | ---- |
| running | 执行中 |
| completed | 执行成功 |
| failed | 执行失败 |
| cancelled | 已取消 |

## 版本历史

- **Phase 195** (2026-03-28): 备份管理模块 - 备份执行历史 API
