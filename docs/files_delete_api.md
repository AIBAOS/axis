# 文件删除 API

**Phase 106** - 文件管理 API 之文件删除接口

---

## 接口信息

- **端点:** `DELETE /api/v1/files/{id}`
- **认证:** 需要 JWT Bearer Token（登录用户）
- **权限:** 登录用户可访问（仅可删除自己的文件，admin 可删除任意文件）
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 文件 ID |

### 请求示例

```bash
# 删除自己的文件
curl -X DELETE "http://localhost:8080/api/v1/files/1" \
  -H "Authorization: Bearer <JWT_TOKEN>"

# Admin 删除任意文件
curl -X DELETE "http://localhost:8080/api/v1/files/3" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"
```

---

## 响应

### 200 OK - 删除成功

```json
{
  "success": true,
  "message": "File deleted successfully",
  "deleted_file": {
    "file_id": 1,
    "name": "readme.txt",
    "path": "/readme.txt",
    "size_bytes": 1024
  }
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
  "error": "You can only delete your own files",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 文件不存在

```json
{
  "success": false,
  "error": "File 999 not found",
  "code": "NOT_FOUND"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **归属权验证**: 用户只能删除自己的文件
3. **Admin 权限**: admin 角色可以删除任意文件

---

## 响应字段说明

### FileDeleteResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `deleted_file` | object | 已删除文件信息 |

### DeletedFileInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `file_id` | integer | 文件 ID |
| `name` | string | 文件名称 |
| `path` | string | 文件完整路径 |
| `size_bytes` | integer | 文件大小（字节） |

---

## 实现细节

- **文件位置:** `src/handlers/files_delete.rs`
- **路由注册:** `src/main.rs` - `DELETE /api/v1/files/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `GET /api/v1/files/browse` - 文件浏览（Phase 104）
- `POST /api/v1/files/upload` - 文件上传（Phase 105）
- `GET /api/v1/files/download/{path}` - 文件下载
- `GET /api/v1/files/{path}` - 文件详情

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 106 初始实现 |
