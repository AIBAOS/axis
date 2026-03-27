# 文件复制 API

**Phase 110** - 文件管理 API 之文件复制接口

---

## 接口信息

- **端点:** `POST /api/v1/files/{id}/copy`
- **认证:** 需要 JWT Bearer Token（登录用户）
- **权限:** 登录用户可访问（仅可复制自己的文件，admin 可复制任意文件）
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |
| `Content-Type` | 是 | `application/json` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 源文件 ID |

### 请求体

```json
{
  "destination_path": "/Documents"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `destination_path` | string | 是 | 目标目录路径（必须是已存在的目录） |

### 请求示例

```bash
# 复制文件到 Documents 目录
curl -X POST "http://localhost:8080/api/v1/files/1/copy" \
  -H "Authorization: Bearer <JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "destination_path": "/Documents"
  }'

# 复制文件到 Pictures 目录
curl -X POST "http://localhost:8080/api/v1/files/1/copy" \
  -H "Authorization: Bearer <JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "destination_path": "/Pictures"
  }'
```

---

## 响应

### 201 Created - 复制成功

```json
{
  "success": true,
  "message": "File copied successfully",
  "data": {
    "id": 101,
    "name": "readme.txt",
    "path": "/Documents/readme.txt",
    "size_bytes": 1024,
    "mime_type": "text/plain",
    "created_at": 1711526400,
    "owner_id": 1
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "destination_path is required",
  "code": "INVALID_PARAMS"
}
```

### 401 Unauthorized - 未认证

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden - 无权限

```json
{
  "success": false,
  "error": "You can only copy your own files",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 资源不存在

```json
{
  "success": false,
  "error": "File 999 not found",
  "code": "NOT_FOUND"
}
```

或

```json
{
  "success": false,
  "error": "Destination path '/NonExistent' not found",
  "code": "NOT_FOUND"
}
```

### 409 Conflict - 文件已存在

```json
{
  "success": false,
  "error": "File 'readme.txt' already exists in destination",
  "code": "FILE_EXISTS"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **归属权验证**: 用户只能复制自己的文件
3. **Admin 权限**: admin 角色可以复制任意文件
4. **目标路径验证**: 目标目录必须存在
5. **文件名冲突**: 目标位置同名文件已存在时返回 409

---

## 响应字段说明

### CopyFileResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 新文件信息 |

### FileInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 新文件 ID |
| `name` | string | 文件名称 |
| `path` | string | 新文件完整路径 |
| `size_bytes` | integer | 文件大小（字节） |
| `mime_type` | string | MIME 类型 |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `owner_id` | integer | 所有者用户 ID（复制后属于当前用户） |

---

## 实现细节

- **文件位置:** `src/handlers/files_copy.rs`
- **路由注册:** `src/main.rs` - `POST /api/v1/files/{id}/copy`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `GET /api/v1/files/browse` - 文件浏览（Phase 104）
- `POST /api/v1/files/upload` - 文件上传（Phase 105）
- `DELETE /api/v1/files/{id}` - 文件删除（Phase 106）
- `PUT /api/v1/files/{id}` - 文件更新（Phase 108）
- `GET /api/v1/files/{id}/download` - 文件下载（Phase 109）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 110 初始实现 |
