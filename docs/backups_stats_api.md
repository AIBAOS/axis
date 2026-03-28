# 备份统计 API

## Phase 196

## 接口说明

获取备份任务和执行历史的统计信息，用于仪表板展示。

## 请求

`GET /api/v1/backups/stats`

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
    "total_backups": 15,
    "active_backups": 8,
    "archived_backups": 5,
    "total_executions": 128,
    "successful_executions": 120,
    "failed_executions": 6,
    "running_executions": 2,
    "last_execution_at": 1711584000,
    "next_scheduled_execution": 1711670400,
    "storage_used_bytes": 10737418240
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| total_backups | integer | 备份任务总数 |
| active_backups | integer | 活跃备份数（status = 'active'/'running'/'idle'/'completed'） |
| archived_backups | integer | 已归档备份数（status = 'archived'） |
| total_executions | integer | 总执行次数 |
| successful_executions | integer | 成功执行次数（status = 'completed'） |
| failed_executions | integer | 失败执行次数（status = 'failed'） |
| running_executions | integer | 正在执行的次数（status = 'running'） |
| last_execution_at | integer | 最近执行时间（Unix 时间戳，null 表示无执行记录） |
| next_scheduled_execution | integer | 下次计划执行时间（Unix 时间戳，null 表示无计划任务） |
| storage_used_bytes | integer | 备份占用存储空间（字节） |

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
  "error": "Only admin users can view backup statistics",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "查询统计信息失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 获取备份统计信息

```bash
curl -X GET "http://localhost:8080/api/v1/backups/stats" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "total_backups": 15,
    "active_backups": 8,
    "archived_backups": 5,
    "total_executions": 128,
    "successful_executions": 120,
    "failed_executions": 6,
    "running_executions": 2
  }
}
```

### 未认证请求

```bash
curl -X GET "http://localhost:8080/api/v1/backups/stats"
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 非 admin 用户请求

```bash
curl -X GET "http://localhost:8080/api/v1/backups/stats" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view backup statistics",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 查询备份任务统计（总数、活跃数、归档数）
4. 查询执行历史统计（总数、成功数、失败数、运行中）
5. 返回统计信息

## 统计说明

### 备份任务状态

| 状态 | 说明 |
| ---- | ---- |
| active | 活跃备份，可正常执行 |
| archived | 已归档备份，已释放资源 |
| idle | 空闲状态 |
| running | 正在执行 |

### 执行历史状态

| 状态 | 说明 |
| ---- | ---- |
| running | 执行中 |
| completed | 执行成功 |
| failed | 执行失败 |
| cancelled | 已取消 |

## 版本历史

- **Phase 196** (2026-03-28): 备份管理模块 - 备份统计 API
