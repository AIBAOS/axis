# 备份任务列表 API

## Phase 189

## 接口说明

获取所有备份任务列表。

## 请求

`GET /api/v1/backups`

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
  "data": [
    {
      "id": 1,
      "name": "Daily Backup",
      "description": "Daily backup of system data",
      "source_path": "/data",
      "destination_path": "/backup/daily",
      "schedule": "0 2 * * *",
      "status": "active",
      "last_run": "2026-03-27T02:00:00Z",
      "next_run": "2026-03-28T02:00:00Z",
      "created_at": "2026-03-01T00:00:00Z",
      "updated_at": "2026-03-27T02:00:00Z"
    },
    {
      "id": 2,
      "name": "Weekly Backup",
      "description": "Weekly full backup",
      "source_path": "/",
      "destination_path": "/backup/weekly",
      "schedule": "0 3 * * 0",
      "status": "active",
      "last_run": "2026-03-24T03:00:00Z",
      "next_run": "2026-03-31T03:00:00Z",
      "created_at": "2026-03-01T00:00:00Z",
      "updated_at": "2026-03-24T03:00:00Z"
    }
  ]
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

## 示例

### 获取备份任务列表

```bash
curl "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <jwt_token>"
```

### 无权限访问

```bash
curl "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <user_token>"
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

## 权限要求

- 需要 JWT 认证
- 登录用户可访问

## 响应字段说明

### 备份任务字段

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
| created_at | string | 创建时间 |
| updated_at | string | 更新时间 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 获取所有备份任务
3. 返回 200 OK + 任务列表

## 版本历史

- **Phase 189** (2026-03-27): 备份管理模块 - 备份任务列表 API
