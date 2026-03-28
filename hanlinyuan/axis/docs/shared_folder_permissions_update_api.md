# 更新共享文件夹权限 API

**Phase 97** - 共享文件夹权限管理 API 之更新权限接口

---

## 接口信息

- **端点:** `PUT /api/v1/shared-folders/{id}/permissions/{permission_id}`
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
| `permission_id` | integer | 是 | 权限配置 ID |

### 请求体

```json
{
  "permissions": ["read", "write"]
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `permissions` | array | 是 | 权限列表（全量替换）：["read", "write", "admin"] |

### 请求示例

```bash
# 更新权限为只读
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1/permissions/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "permissions": ["read"]
  }'

# 更新权限为读写
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1/permissions/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "permissions": ["read", "write"]
  }'

# 更新权限为管理员
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1/permissions/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "permissions": ["read", "write", "admin"]
  }'
```

---

## 响应

### 200 OK - 更新成功

```json
{
  "success": true,
  "message": "Permission updated successfully",
  "data": {
    "id": 1,
    "shared_folder_id": 1,
    "user_id": 1,
    "group_id": null,
    "permissions": ["read", "write"],
    "created_at": 1710489600,
    "updated_at": 1711526400
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "permissions array cannot be empty",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Invalid permission 'delete'. Valid permissions: read, write, admin",
  "code": "INVALID_PERMISSION"
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
  "error": "Only admin users can manage permissions",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 资源不存在

```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "NOT_FOUND"
}
```

或

```json
{
  "success": false,
  "error": "Permission 999 not found",
  "code": "NOT_FOUND"
}
```

或

```json
{
  "success": false,
  "error": "Permission 1 not found for shared folder 2",
  "code": "NOT_FOUND"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以更新权限
3. **权限验证**: 只允许 read、write、admin 三种权限
4. **归属验证**: 权限配置必须属于指定的共享文件夹

---

## 响应字段说明

### UpdatePermissionResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 权限信息 |

### PermissionInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 权限 ID |
| `shared_folder_id` | integer | 共享文件夹 ID |
| `user_id` | integer\|null | 用户 ID（如果是用户权限） |
| `group_id` | integer\|null | 用户组 ID（如果是组权限） |
| `permissions` | array | 权限列表：["read", "write", "admin"] |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

---

## 权限说明

| 权限 | 说明 |
|------|------|
| `read` | 读取权限（浏览、下载文件） |
| `write` | 写入权限（上传、修改、删除文件） |
| `admin` | 管理权限（修改共享设置、管理权限） |

---

## 实现细节

- **文件位置:** `src/handlers/shared_folder_permissions_update.rs`
- **路由注册:** `src/main.rs` - `PUT /api/v1/shared-folders/{id}/permissions/{permission_id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/shared-folders/{id}/permissions` - 权限列表（Phase 95）
- `POST /api/v1/shared-folders/{id}/permissions` - 添加权限（Phase 96）
- `DELETE /api/v1/shared-folders/{id}/permissions/{permission_id}` - 删除权限

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 97 初始实现 |
