# 备份详情 API

## Phase 164

## 接口说明

获取指定备份任务的详细信息。

## 请求

`GET /api/v1/backups/{id}`

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
  "data": {
    "id": 1,
    "name": "Daily Backup 2026-03-27",
    "type": "daily",
    "size": 1073741824,
    "status": "completed",
    "source_path": "/srv/data",
    "destination_path": "/srv/backups/daily",
    "compression": true,
    "encryption": false,
    "created_at": "2026-03-27T00:00:00Z",
    "completed_at": "2026-03-27T01:30:00Z"
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
  "error": "Only admin users can view backup details",
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

## 示例

### 获取备份详情

```bash
curl "http://localhost:8080/api/v1/backups/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的备份

```bash
curl "http://localhost:8080/api/v1/backups/999" \
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
- 仅限 admin 角色访问

## 响应字段说明

### 备份详情字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 备份 ID |
| name | string | 备份名称 |
| type | string | 备份类型（daily/weekly/monthly/manual） |
| size | u64 | 备份大小（字节） |
| status | string | 状态（pending/running/completed/failed） |
| source_path | string | 源路径 |
| destination_path | string | 目标路径 |
| compression | boolean | 是否压缩 |
| encryption | boolean | 是否加密 |
| created_at | string | 创建时间（ISO 8601 格式） |
| completed_at | string\|null | 完成时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据备份 ID 查找备份
4. 备份不存在返回 404 Not Found
5. 返回 200 OK + 备份详情

## 版本历史

- **Phase 164** (2026-03-27): 备份管理模块 - 备份详情 API
