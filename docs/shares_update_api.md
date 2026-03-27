# 更新共享文件夹 API

**Phase 93** - 共享文件夹管理 API 之更新共享文件夹接口

---

## 接口信息

- **端点:** `PUT /api/v1/shares/{id}`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <ADMIN_JWT_TOKEN>` |
| `Content-Type` | 是 | `application/json` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 共享文件夹 ID |

### 请求体

```json
{
  "name": "string (可选)",
  "description": "string (可选)",
  "protocols": ["smb", "nfs"] (可选)",
  "read_only": "boolean (可选)",
  "guest_access": "boolean (可选)",
  "enabled": "boolean (可选)"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 共享名称（必须唯一） |
| `description` | string | 否 | 共享描述 |
| `protocols` | array | 否 | 共享协议列表：["smb", "nfs", "afp"] |
| `read_only` | boolean | 否 | 是否只读 |
| `guest_access` | boolean | 否 | 是否允许访客访问 |
| `enabled` | boolean | 否 | 是否启用 |

### 请求示例

```bash
# 更新共享名称
curl -X PUT "http://localhost:8080/api/v1/shares/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Public Files"
  }'

# 更新协议和权限
curl -X PUT "http://localhost:8080/api/v1/shares/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "protocols": ["smb", "nfs"],
    "read_only": true,
    "guest_access": false
  }'

# 禁用共享
curl -X PUT "http://localhost:8080/api/v1/shares/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": false
  }'
```

---

## 响应

### 200 OK - 更新成功

```json
{
  "success": true,
  "message": "Share updated successfully",
  "data": {
    "id": 1,
    "name": "Public Files",
    "volume_id": 2,
    "volume_name": "Data Volume",
    "path": "/public",
    "description": "Public shared folder",
    "protocols": ["smb", "nfs"],
    "read_only": true,
    "guest_access": false,
    "enabled": true,
    "status": "active",
    "created_at": 1710489600,
    "updated_at": 1711526400,
    "created_by": "admin"
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "Invalid protocol 'ftp'. Valid protocols: smb, nfs, afp",
  "code": "INVALID_PROTOCOL"
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
  "error": "Only admin users can update shares",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 共享不存在

```json
{
  "success": false,
  "error": "Share 999 not found",
  "code": "NOT_FOUND"
}
```

### 409 Conflict - 名称已存在

```json
{
  "success": false,
  "error": "Share 'Media' already exists",
  "code": "NAME_CONFLICT"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以更新共享文件夹
3. **名称唯一性**: 共享名称必须全局唯一（排除自身）
4. **协议验证**: 只支持 smb、nfs、afp 三种协议

---

## 响应字段说明

### UpdateShareResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 共享文件夹信息 |

### ShareInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 共享 ID |
| `name` | string | 共享名称 |
| `volume_id` | integer | 所属存储卷 ID |
| `volume_name` | string | 所属存储卷名称 |
| `path` | string | 文件夹路径 |
| `description` | string\|null | 共享描述 |
| `protocols` | array | 共享协议列表：["smb", "nfs", "afp"] |
| `read_only` | boolean | 是否只读 |
| `guest_access` | boolean | 是否允许访客访问 |
| `enabled` | boolean | 是否启用 |
| `status` | string | 状态：active/inactive |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |
| `created_by` | string | 创建者用户名 |

---

## 实现细节

- **文件位置:** `src/handlers/shares_update.rs`
- **路由注册:** `src/main.rs` - `PUT /api/v1/shares/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/shares` - 共享文件夹列表（Phase 90）
- `POST /api/v1/shares` - 创建共享文件夹（Phase 89/91）
- `GET /api/v1/shares/{id}` - 共享文件夹详情（Phase 92）
- `DELETE /api/v1/shares/{id}` - 删除共享文件夹

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 93 初始实现 |
