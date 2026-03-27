# Phase 108 - 文件更新/重命名 API 文档

**接口:** `PUT /api/v1/files/{id}`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

更新文件信息，支持重命名和移动文件。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 登录用户可访问
- 用户只能操作自己的文件
- Admin 可以操作任意文件

**功能特性:**
- 验证文件 ID 存在性
- 验证文件归属权
- Admin 权限支持
- 支持重命名（name 字段）
- 支持移动（path 字段）
- 检查同名文件冲突

---

## 🔐 认证方式

```
Authorization: Bearer <access_token>
```

---

## 📤 请求参数

### 请求头

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `Authorization` | string | 是 | JWT Bearer Token |
| `Content-Type` | string | 是 | `application/json` |

### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | string | 是 | 文件 ID |

### 请求体 (Body)

```json
{
  "name": "new_filename.pdf",
  "path": "/Documents/new_filename.pdf"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 新文件名（1-255 字符，不能包含 / 或 \） |
| `path` | string | 否 | 新路径/移动目标目录 |

**注意:** 至少提供一个字段（name 或 path）

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "message": "File updated successfully",
  "data": {
    "id": "file_001",
    "name": "new_filename.pdf",
    "path": "/Documents/new_filename.pdf",
    "size_bytes": 524288,
    "mime_type": "application/pdf",
    "volume_id": 1,
    "owner_id": 1,
    "updated_at": 1774432000
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data.id` | string | 文件 ID |
| `data.name` | string | 文件名称 |
| `data.path` | string | 文件完整路径 |
| `data.size_bytes` | number | 文件大小（字节） |
| `data.mime_type` | string | 文件 MIME 类型 |
| `data.volume_id` | number | 所属存储卷 ID |
| `data.owner_id` | number | 所有者用户 ID |
| `data.updated_at` | number | 更新时间（Unix 时间戳） |

---

## ❌ 错误响应

### 400 Bad Request

```json
{
  "success": false,
  "error": "At least one field (name or path) must be provided",
  "code": "INVALID_PARAMS"
}
```

```json
{
  "success": false,
  "error": "Invalid filename. Must be 1-255 characters and cannot contain / or \\",
  "code": "INVALID_FILENAME"
}
```

### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden

```json
{
  "success": false,
  "error": "No permission to modify this file",
  "code": "FORBIDDEN"
}
```

### 404 Not Found

```json
{
  "success": false,
  "error": "File 'file_999' not found",
  "code": "NOT_FOUND"
}
```

### 409 Conflict (文件名冲突)

```json
{
  "success": false,
  "error": "File 'document.pdf' already exists in this directory",
  "code": "FILE_EXISTS"
}
```

---

## 🧪 使用示例

### 重命名文件

```bash
# 重命名文件
curl -X PUT "http://localhost:8080/api/v1/files/file_001" \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "renamed_document.pdf"
  }'
```

### 移动文件

```bash
# 移动文件到新目录
curl -X PUT "http://localhost:8080/api/v1/files/file_001" \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "path": "/Archives/document.pdf"
  }'
```

### 重命名并移动

```bash
# 同时重命名和移动
curl -X PUT "http://localhost:8080/api/v1/files/file_001" \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "archived_document.pdf",
    "path": "/Archives/archived_document.pdf"
  }'
```

### Admin 操作任意文件

```bash
# Admin 可以操作任意文件
curl -X PUT "http://localhost:8080/api/v1/files/file_003" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "admin_renamed.mp4"
  }'
```

### 错误场景

```bash
# 未提供字段（400）
curl -X PUT "http://localhost:8080/api/v1/files/file_001" \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{}'
# 响应：400 Bad Request - At least one field must be provided

# 无效文件名（400）
curl -X PUT "http://localhost:8080/api/v1/files/file_001" \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "invalid/name.pdf"}'
# 响应：400 Bad Request - Invalid filename

# 不存在的文件（404）
curl -X PUT "http://localhost:8080/api/v1/files/file_999" \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "new_name.pdf"}'
# 响应：404 Not Found - File not found

# 无权限（403）
curl -X PUT "http://localhost:8080/api/v1/files/file_003" \
  -H "Authorization: Bearer <user_token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "new_name.mp4"}'
# 响应：403 Forbidden - No permission to modify this file

# 文件名冲突（409）
curl -X PUT "http://localhost:8080/api/v1/files/file_001" \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "existing_file.pdf"}'
# 响应：409 Conflict - File already exists in this directory
```

---

## 📝 注意事项

1. **权限要求**: 登录用户可访问，但只能操作自己的文件
2. **Admin 权限**: Admin 用户可以操作任意文件
3. **文件不存在**: 返回 404 Not Found
4. **无权限**: 尝试操作他人文件返回 403 Forbidden
5. **文件名规则**: 1-255 字符，不能包含 `/` 或 `\`
6. **同名检查**: 同一目录下不能有同名文件
7. **至少一个字段**: name 或 path 至少提供一个

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/files/{id}` | 文件详情 (Phase 107) |
| `GET /api/v1/files/browse` | 文件浏览 (Phase 104) |
| `POST /api/v1/files/upload` | 文件上传 (Phase 105) |
| `DELETE /api/v1/files/{id}` | 文件删除 (Phase 106) |

---

*文档维护：兵部尚书*
