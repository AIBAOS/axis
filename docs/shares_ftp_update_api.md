# FTP 共享更新 API 文档

## 概述

本文档描述 Axis NAS 系统中更新 FTP 共享 API 的实现细节。

## API 端点

- **路径**: `PUT /api/v1/shares/ftp/{id}`
- **版本**: v1
- **Phase**: 223

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `id` | number | 是 | FTP 共享 ID |

### Request Body

```json
{
  "name": "string (optional)",
  "path": "string (optional)",
  "description": "string (optional)",
  "public": "boolean (optional)",
  "status": "string (optional)"
}
```

### 字段说明

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `name` | string | 否 | 共享名称（1-64 字符） |
| `path` | string | 否 | 共享路径（必须以 / 开头） |
| `description` | string | 否 | 共享描述 |
| `public` | boolean | 否 | 是否公开访问 |
| `status` | string | 否 | 状态：`active` / `inactive` |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "FTP share updated successfully",
  "data": {
    "id": 1,
    "name": "FTP Documents",
    "path": "/srv/ftp/documents",
    "description": "FTP 文档共享",
    "public": false,
    "status": "active",
    "created_at": 1711500000,
    "updated_at": 1711600000
  }
}
```

### 错误响应

#### 404 Not Found - 共享不存在

```json
{
  "success": false,
  "error": "FTP share 999 not found",
  "code": "NOT_FOUND"
}
```

#### 400 Bad Request - 名称格式无效

```json
{
  "success": false,
  "error": "Invalid share name. Must be 1-64 chars, alphanumeric with -_. allowed",
  "code": "INVALID_NAME"
}
```

#### 400 Bad Request - 路径格式无效

```json
{
  "success": false,
  "error": "Invalid share path. Must start with / and be <= 256 chars",
  "code": "INVALID_PATH"
}
```

#### 400 Bad Request - 路径不存在

```json
{
  "success": false,
  "error": "Path '/nonexistent' does not exist",
  "code": "PATH_NOT_FOUND"
}
```

#### 409 Conflict - 名称已存在

```json
{
  "success": false,
  "error": "Share name 'FTP Documents' already exists",
  "code": "NAME_CONFLICT"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can update FTP shares",
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

## 数据模型

### UpdateFtpShareRequest

| 字段 | 类型 | 描述 |
|------|------|------|
| `name` | string? | 共享名称 |
| `path` | string? | 共享路径 |
| `description` | string? | 共享描述 |
| `public` | boolean? | 是否公开 |
| `status` | string? | 状态 |

### FtpShareInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 共享 ID |
| `name` | string | 共享名称 |
| `path` | string | 共享路径 |
| `description` | string? | 共享描述 |
| `public` | boolean | 是否公开 |
| `status` | string | 状态：`active` / `inactive` |
| `created_at` | number | 创建时间（Unix 时间戳） |
| `updated_at` | number | 更新时间（Unix 时间戳） |

### UpdateFtpShareResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data` | FtpShareInfo | 更新后的共享信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试更新 |
| `NOT_FOUND` | 404 | 共享不存在或非 FTP 协议 |
| `INVALID_NAME` | 400 | 共享名称格式无效 |
| `INVALID_PATH` | 400 | 共享路径格式无效 |
| `PATH_NOT_FOUND` | 400 | 指定路径不存在 |
| `NAME_CONFLICT` | 409 | 共享名称已存在 |
| `DATABASE_ERROR` | 500 | 数据库操作失败 |

## 示例

### 请求

```bash
curl -X PUT "http://localhost:8080/api/v1/shares/ftp/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "FTP Documents",
    "description": "更新后的 FTP 文档共享",
    "public": true
  }'
```

### 响应

```json
{
  "success": true,
  "message": "FTP share updated successfully",
  "data": {
    "id": 1,
    "name": "FTP Documents",
    "path": "/srv/ftp/documents",
    "description": "更新后的 FTP 文档共享",
    "public": true,
    "status": "active",
    "created_at": 1711500000,
    "updated_at": 1711600000
  }
}
```

## 权限说明

- **Admin 用户**: 可更新任意 FTP 共享
- **普通用户**: 无权访问（返回 403 Forbidden）

## 验证规则

### 名称验证
- 长度：1-64 字符
- 允许字符：字母、数字、`-`、`_`、`.`
- 必须唯一（排除自身）

### 路径验证
- 必须以 `/` 开头
- 最大长度：256 字符
- 路径必须存在（使用 `std::path::Path::exists()` 验证）

## 实现细节

### 部分更新
- 仅更新提供的字段
- 未提供的字段保持原值

### 协议验证
- 仅允许更新 `protocol = 'ftp'` 的共享
- 非 FTP 协议返回 404 Not Found

### 数据库操作
- **查询**: `get_share_by_id(id)`
- **更新**: `update_share(id, name, path, protocol, status)`
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

- `GET /api/v1/shares/ftp` - 获取 FTP 共享列表
- `GET /api/v1/shares/ftp/{id}` - 获取 FTP 共享详情
- `POST /api/v1/shares/ftp` - 创建 FTP 共享
- `DELETE /api/v1/shares/ftp/{id}` - 删除 FTP 共享

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试更新 FTP 共享
curl -X PUT "http://localhost:8080/api/v1/shares/ftp/1" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "FTP Documents",
    "description": "更新后的描述"
  }'

# 预期：200 OK + 更新后的共享信息

# 测试更新不存在的共享
curl -X PUT "http://localhost:8080/api/v1/shares/ftp/999" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "NewName"}'

# 预期：404 Not Found
```

## 版本历史

- **Phase 223** (2026-03-28): 初始实现，SQLite 持久化
