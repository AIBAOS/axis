# 备份列表 API

## Phase 162

## 接口说明

获取备份任务列表，支持分页查询。

## 请求

`GET /api/v1/backups`

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ---- | ---- |
| page | u32 | 否 | 1 | 页码（从 1 开始） |
| limit | u32 | 否 | 20 | 每页数量（最大 100） |

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
      "name": "Daily Backup 2026-03-27",
      "type": "daily",
      "size": 1073741824,
      "status": "completed",
      "created_at": "2026-03-27T00:00:00Z",
      "completed_at": "2026-03-27T01:30:00Z"
    },
    {
      "id": 2,
      "name": "Weekly Backup 2026-03-24",
      "type": "weekly",
      "size": 5368709120,
      "status": "completed",
      "created_at": "2026-03-24T00:00:00Z",
      "completed_at": "2026-03-24T03:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 5,
    "total_pages": 1
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
  "error": "Only admin users can list backups",
  "code": "FORBIDDEN"
}
```

## 示例

### 获取备份列表（默认分页）

```bash
curl "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取第 2 页，每页 10 条

```bash
curl "http://localhost:8080/api/v1/backups?page=2&limit=10" \
  -H "Authorization: Bearer <jwt_token>"
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 备份列表字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 备份 ID |
| name | string | 备份名称 |
| type | string | 备份类型（daily/weekly/manual） |
| size | u64 | 备份大小（字节） |
| status | string | 状态（completed/failed/running） |
| created_at | string | 创建时间（ISO 8601 格式） |
| completed_at | string\|null | 完成时间（ISO 8601 格式） |

### 分页字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| page | u32 | 当前页码 |
| limit | u32 | 每页数量 |
| total | u64 | 总记录数 |
| total_pages | u32 | 总页数 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析查询参数（page/limit）
4. 获取备份列表
5. 应用分页
6. 返回 200 OK + 备份列表 + 分页信息

## 版本历史

- **Phase 162** (2026-03-27): 备份管理模块 - 备份列表 API
