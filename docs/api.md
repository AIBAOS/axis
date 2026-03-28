# Axis API 文档

**版本：** 0.1.0 (草案)  
**基础路径：** `/api/v1`  
**认证方式：** Bearer Token (v0.2+)

---

## 📋 概述

Axis API 采用 RESTful 设计风格，所有接口返回 JSON 格式数据。

### 通用响应格式

**成功响应：**
```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-03-14T06:35:00Z"
  }
}
```

**错误响应：**
```json
{
  "success": false,
  "error": {
    "code": "FILE_NOT_FOUND",
    "message": "文件未找到：/path/to/file",
    "details": { ... }
  },
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-03-14T06:35:00Z"
  }
}
```

### HTTP 状态码

| 状态码 | 含义 | 说明 |
|--------|------|------|
| 200 OK | 成功 | 请求成功处理 |
| 201 Created | 已创建 | 资源创建成功 |
| 204 No Content | 无内容 | 删除成功，无返回体 |
| 400 Bad Request | 错误请求 | 请求参数无效 |
| 401 Unauthorized | 未授权 | 认证失败 |
| 403 Forbidden | 禁止访问 | 权限不足 |
| 404 Not Found | 未找到 | 资源不存在 |
| 409 Conflict | 冲突 | 资源已存在 |
| 500 Internal Server Error | 内部错误 | 服务器异常 |

---

## 🔍 健康检查

### `GET /health`

服务健康状态检查。

**请求：**
```http
GET /health HTTP/1.1
Host: localhost:8080
```

**响应 (200 OK)：**
```json
{
  "status": "ok",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "checks": {
    "storage": "healthy",
    "database": "healthy"
  }
}
```

**字段说明：**

| 字段 | 类型 | 描述 |
|------|------|------|
| `status` | string | 状态：`ok` / `degraded` / `unhealthy` |
| `version` | string | 服务版本号 |
| `uptime_seconds` | number | 运行时长（秒） |
| `checks` | object | 子系统健康状态 |

---

## 📁 文件操作

### `GET /files/*path`

获取文件内容或目录列表。

**请求：**
```http
GET /files/documents/report.pdf HTTP/1.1
Host: localhost:8080
Authorization: Bearer <token>
```

**响应 (200 OK) - 文件：**
```http
HTTP/1.1 200 OK
Content-Type: application/pdf
Content-Length: 1024
Content-Disposition: attachment; filename="report.pdf"

[binary data]
```

**响应 (200 OK) - 目录：**
```json
{
  "success": true,
  "data": {
    "type": "directory",
    "path": "/documents",
    "items": [
      {
        "name": "report.pdf",
        "type": "file",
        "size": 1024,
        "modified_at": "2026-03-14T06:35:00Z"
      },
      {
        "name": "photos",
        "type": "directory",
        "modified_at": "2026-03-14T06:30:00Z"
      }
    ]
  }
}
```

**查询参数：**

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `include_hidden` | boolean | `false` | 是否包含隐藏文件 |
| `sort_by` | string | `name` | 排序字段：`name` / `size` / `modified_at` |
| `order` | string | `asc` | 排序方向：`asc` / `desc` |

---

### `PUT /files/*path`

上传文件。

**请求：**
```http
PUT /files/documents/report.pdf HTTP/1.1
Host: localhost:8080
Authorization: Bearer <token>
Content-Type: application/pdf
Content-Length: 1024

[binary data]
```

**响应 (201 Created)：**
```json
{
  "success": true,
  "data": {
    "path": "/documents/report.pdf",
    "size": 1024,
    "checksum": "sha256:abc123...",
    "created_at": "2026-03-14T06:35:00Z"
  }
}
```

**请求头：**

| 头 | 必需 | 描述 |
|----|------|------|
| `Content-Type` | 否 | 文件 MIME 类型 |
| `Content-Length` | 是 | 文件大小（字节） |
| `Content-MD5` | 否 | 文件 MD5 校验和 |

---

### `DELETE /files/*path`

删除文件或目录。

**请求：**
```http
DELETE /files/documents/report.pdf HTTP/1.1
Host: localhost:8080
Authorization: Bearer <token>
```

**响应 (204 No Content)：**
```http
HTTP/1.1 204 No Content
```

**查询参数：**

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `recursive` | boolean | `false` | 是否递归删除目录 |

---

### `POST /files/*path`

创建目录。

**请求：**
```http
POST /files/documents/archive HTTP/1.1
Host: localhost:8080
Authorization: Bearer <token>
Content-Type: application/json

{}
```

**响应 (201 Created)：**
```json
{
  "success": true,
  "data": {
    "path": "/documents/archive",
    "type": "directory",
    "created_at": "2026-03-14T06:35:00Z"
  }
}
```

---

### `PATCH /files/*path`

移动/重命名文件。

**请求：**
```http
PATCH /files/documents/report.pdf HTTP/1.1
Host: localhost:8080
Authorization: Bearer <token>
Content-Type: application/json

{
  "destination": "/documents/archive/report.pdf"
}
```

**响应 (200 OK)：**
```json
{
  "success": true,
  "data": {
    "old_path": "/documents/report.pdf",
    "new_path": "/documents/archive/report.pdf",
    "moved_at": "2026-03-14T06:35:00Z"
  }
}
```

**请求体字段：**

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `destination` | string | 是 | 目标路径 |
| `overwrite` | boolean | 否 | 是否覆盖已存在文件（默认 `false`） |

---

## 📊 统计信息

### `GET /stats`

获取存储统计信息。

**请求：**
```http
GET /stats HTTP/1.1
Host: localhost:8080
Authorization: Bearer <token>
```

**响应 (200 OK)：**
```json
{
  "success": true,
  "data": {
    "total_bytes": 107374182400,
    "used_bytes": 53687091200,
    "available_bytes": 53687091200,
    "file_count": 1024,
    "directory_count": 128,
    "largest_file": {
      "path": "/backups/full-2026-03-14.tar",
      "size": 10737418240
    }
  }
}
```

---

## 🔐 认证（v0.2+）

### `POST /auth/login`

用户登录。

**请求：**
```http
POST /auth/login HTTP/1.1
Host: localhost:8080
Content-Type: application/json

{
  "username": "admin",
  "password": "password123"
}
```

**响应 (200 OK)：**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIs...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "refresh_token": "dGhpcyBpcyBhIHJlZnJl..."
  }
}
```

---

### `POST /auth/refresh`

刷新访问令牌。

**请求：**
```http
POST /auth/refresh HTTP/1.1
Host: localhost:8080
Content-Type: application/json

{
  "refresh_token": "dGhpcyBpcyBhIHJlZnJl..."
}
```

**响应 (200 OK)：**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIs...",
    "token_type": "Bearer",
    "expires_in": 3600
  }
}
```

---

## ❌ 错误码列表

| 错误码 | HTTP 状态码 | 描述 |
|--------|------------|------|
| `FILE_NOT_FOUND` | 404 | 文件不存在 |
| `DIRECTORY_NOT_EMPTY` | 409 | 目录非空，无法删除 |
| `INSUFFICIENT_SPACE` | 507 | 磁盘空间不足 |
| `PERMISSION_DENIED` | 403 | 权限不足 |
| `INVALID_PATH` | 400 | 路径格式无效 |
| `FILE_ALREADY_EXISTS` | 409 | 文件已存在 |
| `AUTHENTICATION_REQUIRED` | 401 | 需要认证 |
| `INVALID_TOKEN` | 401 | 令牌无效或过期 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

---

## 📝 使用示例

### cURL 示例

**上传文件：**
```bash
curl -X PUT http://localhost:8080/files/documents/test.txt \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: text/plain" \
  --data-binary @test.txt
```

**下载文件：**
```bash
curl -X GET http://localhost:8080/files/documents/test.txt \
  -H "Authorization: Bearer <token>" \
  -o downloaded.txt
```

**列出目录：**
```bash
curl -X GET "http://localhost:8080/files/documents?sort_by=modified_at&order=desc" \
  -H "Authorization: Bearer <token>"
```

### JavaScript 示例

```javascript
// 上传文件
async function uploadFile(path, file) {
  const response = await fetch(`/files${path}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': file.type,
    },
    body: file,
  });
  
  if (!response.ok) {
    throw new Error(`Upload failed: ${response.status}`);
  }
  
  return await response.json();
}

// 下载文件
async function downloadFile(path) {
  const response = await fetch(`/files${path}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  
  if (!response.ok) {
    throw new Error(`Download failed: ${response.status}`);
  }
  
  return await response.blob();
}
```

---

## 🔄 变更日志

| 版本 | 日期 | 变更 |
|------|------|------|
| 0.1.0 | 2026-03-14 | 初始版本：基础文件操作 API |
| 0.2.0 | TBD | 认证系统、WebDAV 支持 |
| 0.3.0 | TBD | 批量操作、SMB 协议 |

---

*本文档由翰林院编写，随 API 迭代同步更新。*
