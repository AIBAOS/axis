# 容器列表 API

## Phase 168

## 接口说明

获取容器列表，支持分页查询。

## 请求

`GET /api/v1/containers`

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ---- | ---- |
| page | u32 | 否 | 1 | 页码（从 1 开始） |
| per_page | u32 | 否 | 20 | 每页数量（最大 100） |

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
      "name": "nginx-web",
      "image": "nginx:latest",
      "status": "running",
      "created_at": "2026-03-27T06:00:00Z"
    },
    {
      "id": 2,
      "name": "postgres-db",
      "image": "postgres:15",
      "status": "running",
      "created_at": "2026-03-27T06:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
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
  "error": "Only admin users can list containers",
  "code": "FORBIDDEN"
}
```

## 示例

### 获取容器列表（默认分页）

```bash
curl "http://localhost:8080/api/v1/containers" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取第 2 页，每页 10 条

```bash
curl "http://localhost:8080/api/v1/containers?page=2&per_page=10" \
  -H "Authorization: Bearer <jwt_token>"
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 容器列表字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 容器 ID |
| name | string | 容器名称 |
| image | string | 镜像名称 |
| status | string | 状态（running/stopped/paused） |
| created_at | string | 创建时间（ISO 8601 格式） |

### 分页字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| page | u32 | 当前页码 |
| per_page | u32 | 每页数量 |
| total | u64 | 总记录数 |
| total_pages | u32 | 总页数 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析查询参数（page/per_page）
4. 获取容器列表
5. 应用分页
6. 返回 200 OK + 容器列表 + 分页信息

## 版本历史

- **Phase 168** (2026-03-27): 容器管理模块 - 容器列表 API
