# WebDAV 共享列表 API

## Phase 215

## 接口说明

获取 WebDAV 共享列表，支持分页和筛选。要求 admin 角色访问。

## 请求

`GET /api/v1/shares/webdav`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ------- | ---- |
| page | integer | 否 | 1 | 页码（从 1 开始） |
| per_page | integer | 否 | 20 | 每页数量（最大 100） |
| status | string | 否 | - | 状态筛选（active/inactive） |

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
      "name": "Public",
      "path": "/data/webdav/public",
      "description": "公开 WebDAV 共享",
      "enabled": true,
      "created_at": 1711584000,
      "updated_at": 1711584000
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 1,
    "total_pages": 1
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | integer | 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径 |
| description | string | 共享描述（可选） |
| enabled | boolean | 是否启用 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 非 admin 用户

```json
{
  "success": false,
  "error": "Only admin users can list WebDAV shares",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "Database error",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 获取 WebDAV 共享列表（第一页）

```bash
curl -X GET "http://localhost:8080/api/v1/shares/webdav?page=1&per_page=20" \
  -H "Authorization: Bearer <jwt_token>"
```

### 按状态筛选

```bash
curl -X GET "http://localhost:8080/api/v1/shares/webdav?status=active" \
  -H "Authorization: Bearer <jwt_token>"
```

## 权限要求

- 需要 JWT 认证
- **仅 admin 用户可访问**

## 业务逻辑

1. 验证 JWT Token 有效性
2. 验证用户角色是否为 admin
3. 从数据库查询 WebDAV 共享列表（protocol = 'webdav'）
4. 应用分页和筛选
5. 返回 WebDAV 共享列表 + 分页信息

## 版本历史

- **Phase 215** (2026-03-28): WebDAV 共享列表 API - SqliteShareRepository 真实查询，支持分页和筛选
