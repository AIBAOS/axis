# FTP 共享列表 API 文档

## 概述

本文档描述 Axis NAS 系统中获取 FTP 共享列表 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/shares/ftp`
- **版本**: v1
- **Phase**: 220

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Query 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `page` | number | 否 | 1 | 页码（从 1 开始） |
| `per_page` | number | 否 | 20 | 每页数量（最大 100） |
| `status` | string | 否 | - | 状态筛选：`active` / `inactive` |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "FTP Documents",
      "path": "/srv/ftp/documents",
      "description": "FTP 文档共享",
      "public": false,
      "status": "active",
      "created_at": 1711500000,
      "updated_at": 1711500000
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

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can list FTP shares",
  "code": "FORBIDDEN"
}
```

#### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "查询 FTP 共享列表失败：数据库错误",
  "code": "DATABASE_ERROR"
}
```

## 数据模型

### FtpShareInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 共享 ID |
| `name` | string | 共享名称 |
| `path` | string | 共享路径 |
| `description` | string? | 共享描述 |
| `public` | boolean | 是否公开访问 |
| `status` | string | 状态：`active` / `inactive` |
| `created_at` | number | 创建时间（Unix 时间戳） |
| `updated_at` | number | 更新时间（Unix 时间戳） |

### PaginationInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `page` | number | 当前页码 |
| `per_page` | number | 每页数量 |
| `total` | number | 总记录数 |
| `total_pages` | number | 总页数 |

### FtpShareListResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | FtpShareInfo[] | 共享列表 |
| `pagination` | PaginationInfo | 分页信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试访问 |
| `DATABASE_ERROR` | 500 | 数据库查询失败 |

## 示例

### 请求

```bash
curl -X GET "http://localhost:8080/api/v1/shares/ftp?page=1&per_page=20&status=active" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "FTP Documents",
      "path": "/srv/ftp/documents",
      "description": "FTP 文档共享",
      "public": false,
      "status": "active",
      "created_at": 1711500000,
      "updated_at": 1711500000
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

## 权限说明

- **Admin 用户**: 可查看所有 FTP 共享
- **普通用户**: 无权访问（返回 403 Forbidden）

## 实现细节

### 数据库查询

- **查询**: `get_shares(page, per_page, protocol, status)`
- **协议筛选**: `protocol = 'ftp'`
- **排序**: `created_at DESC`
- **数据库**: SQLite
- **框架**: Actix-web
- **仓库**: SqliteShareRepository

## 数据库表结构

```sql
CREATE TABLE IF NOT EXISTS shares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    protocol TEXT NOT NULL CHECK(protocol IN ('smb', 'nfs', 'webdav', 'ftp')),
    status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'inactive')),
    description TEXT,
    allowed_users TEXT,
    allowed_groups TEXT,
    guest_ok INTEGER NOT NULL DEFAULT 0,
    read_only INTEGER NOT NULL DEFAULT 0,
    comment TEXT,
    no_subtree_check INTEGER NOT NULL DEFAULT 0,
    sync INTEGER NOT NULL DEFAULT 0,
    clients TEXT,
    enabled INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
```

## 相关接口

- `GET /api/v1/shares/ftp/{id}` - 获取 FTP 共享详情
- `POST /api/v1/shares/ftp` - 创建 FTP 共享
- `PUT /api/v1/shares/ftp/{id}` - 更新 FTP 共享
- `DELETE /api/v1/shares/ftp/{id}` - 删除 FTP 共享

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取 FTP 共享列表
curl -X GET "http://localhost:8080/api/v1/shares/ftp?page=1&per_page=20" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 共享列表

# 测试状态筛选
curl -X GET "http://localhost:8080/api/v1/shares/ftp?status=active" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 仅 active 状态的共享
```

## 版本历史

- **Phase 220** (2026-03-28): 初始实现，SQLite 持久化
