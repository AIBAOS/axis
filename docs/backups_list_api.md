# 备份任务列表 API

## Phase 189/260

## 接口说明

获取备份任务列表，支持分页和状态过滤，用于 Web UI 备份管理展示，仅限 admin 角色访问。

## 请求

`GET /api/v1/backups`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| page | integer | 否 | 1 | 页码 |
| page_size | integer | 否 | 20 | 每页数量（最大 100） |
| status | string | 否 | - | 状态过滤（active/inactive/all） |

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
    }
  ],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total": 3,
    "total_pages": 1
  }
}
```

### 返回字段说明

#### 备份任务

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 任务 ID |
| name | string | 任务名称 |
| description | string | 任务描述 |
| source_path | string | 源路径 |
| destination_path | string | 目标路径 |
| schedule | string | Cron 表达式 |
| status | string | 状态（active/inactive） |
| last_run | string | 最后执行时间 |
| next_run | string | 下次执行时间 |
| created_at | string | 创建时间 |
| updated_at | string | 更新时间 |

#### 分页信息

| 字段 | 类型 | 说明 |
|------|------|------|
| page | integer | 当前页码 |
| page_size | integer | 每页数量 |
| total | integer | 总记录数 |
| total_pages | integer | 总页数 |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足（非 admin）

```json
{
  "success": false,
  "error": "Only admin users can view backup tasks",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to get backup tasks: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取备份任务列表（默认参数）

```bash
curl "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 分页查询（第 2 页，每页 10 条）

```bash
curl "http://localhost:8080/api/v1/backups?page=2&page_size=10" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 状态过滤（仅 active）

```bash
curl "http://localhost:8080/api/v1/backups?status=active" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 非 admin 用户访问

```bash
curl "http://localhost:8080/api/v1/backups" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view backup tasks",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl "http://localhost:8080/api/v1/backups"
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
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析查询参数（page/page_size/status）
4. 从数据库读取备份任务列表
5. 应用状态过滤
6. 应用分页
7. 返回任务列表 + 分页信息

## 版本历史

- **Phase 189** (2026-03-27): 备份管理模块 - 备份任务列表 API 初始实现
- **Phase 260** (2026-03-28): 备份管理模块 - 备份任务列表 API 增强版（分页/过滤）
