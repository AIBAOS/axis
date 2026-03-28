# WebDAV 共享创建 API 文档

## 概述

本文档描述 Axis NAS 系统中创建 WebDAV 共享 API 的实现细节。

## API 端点

- **路径**: `POST /api/v1/shares/webdav`
- **版本**: v1
- **Phase**: 217

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Request Body

```json
{
  "name": "string",
  "path": "string",
  "description": "string (optional)",
  "public": "boolean (optional)"
}
```

### 字段说明

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `name` | string | 是 | 共享名称（1-64 字符） |
| `path` | string | 是 | 共享路径（必须以 / 开头） |
| `description` | string | 否 | 共享描述 |
| `public` | boolean | 否 | 是否公开访问 |

## 响应格式

### 成功响应 (201 Created)

```json
{
  "success": true,
  "message": "WebDAV share created successfully",
  "data": {
    "id": 1,
    "name": "Documents",
    "path": "/srv/webdav/documents",
    "description": "文档共享文件夹",
    "public": false,
    "status": "active",
    "created_at": 1711500000,
    "updated_at": 1711500000
  }
}
```

### 错误响应

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
  "error": "Share name 'Documents' already exists",
  "code": "NAME_CONFLICT"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can create WebDAV shares",
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

### CreateWebdavShareRequest

| 字段 | 类型 | 描述 |
|------|------|------|
| `name` | string | 共享名称 |
| `path` | string | 共享路径 |
| `description` | string? | 共享描述 |
| `public` | boolean? | 是否公开 |

### CreatedWebdavShare

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

### CreateWebdavShareResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data` | CreatedWebdavShare | 创建的共享信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试创建 |
| `INVALID_NAME` | 400 | 共享名称格式无效 |
| `INVALID_PATH` | 400 | 共享路径格式无效 |
| `PATH_NOT_FOUND` | 400 | 指定路径不存在 |
| `NAME_CONFLICT` | 409 | 共享名称已存在 |
| `DATABASE_ERROR` | 500 | 数据库操作失败 |

## 示例

### 请求

```bash
curl -X POST "http://localhost:8080/api/v1/shares/webdav" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Documents",
    "path": "/srv/webdav/documents",
    "description": "文档共享文件夹",
    "public": false
  }'
```

### 响应

```json
{
  "success": true,
  "message": "WebDAV share created successfully",
  "data": {
    "id": 1,
    "name": "Documents",
    "path": "/srv/webdav/documents",
    "description": "文档共享文件夹",
    "public": false,
    "status": "active",
    "created_at": 1711500000,
    "updated_at": 1711500000
  }
}
```

## 权限说明

- **Admin 用户**: 可创建 WebDAV 共享
- **普通用户**: 无权访问（返回 403 Forbidden）

## 验证规则

### 名称验证
- 长度：1-64 字符
- 允许字符：字母、数字、`-`、`_`、`.`
- 必须唯一

### 路径验证
- 必须以 `/` 开头
- 最大长度：256 字符
- 路径必须存在（使用 `std::path::Path::exists()` 验证）

## 实现细节

### 数据库操作

- **创建**: `create_share(name, path, protocol, description, ...)`
- **协议**: 固定为 `webdav`
- **状态**: 创建后默认为 `active`
- **数据库**: SQLite
- **框架**: Actix-web
- **仓库**: SqliteShareRepository

## 数据库表结构

```sql
CREATE TABLE IF NOT EXISTS shares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    protocol TEXT NOT NULL CHECK(protocol IN ('smb', 'nfs', 'webdav')),
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

- `GET /api/v1/shares/webdav` - 获取 WebDAV 共享列表
- `GET /api/v1/shares/webdav/{id}` - 获取 WebDAV 共享详情
- `PUT /api/v1/shares/webdav/{id}` - 更新 WebDAV 共享
- `DELETE /api/v1/shares/webdav/{id}` - 删除 WebDAV 共享

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试创建 WebDAV 共享
curl -X POST "http://localhost:8080/api/v1/shares/webdav" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Documents",
    "path": "/srv/webdav/documents",
    "description": "文档共享文件夹"
  }'

# 预期：201 Created + 创建的共享信息

# 测试创建重名共享
curl -X POST "http://localhost:8080/api/v1/shares/webdav" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Documents",
    "path": "/srv/webdav/new-documents"
  }'

# 预期：409 Conflict
```

## 版本历史

- **Phase 217** (2026-03-28): 初始实现，SQLite 持久化
