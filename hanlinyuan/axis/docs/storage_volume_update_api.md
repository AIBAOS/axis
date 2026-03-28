# 更新存储卷 API

**Phase 68** - 存储管理 API 之更新存储卷接口

---

## 接口信息

- **端点:** `PUT /api/v1/storage/volumes/{id}`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
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
| `id` | integer | 是 | 存储卷 ID |

### 请求体

```json
{
  "name": "string (可选)",
  "size_bytes": "integer (可选)",
  "filesystem_type": "string (可选，但不可变更)"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 存储卷新名称（必须唯一） |
| `size_bytes` | integer | 否 | 新容量（字节），不能小于已用空间 |
| `filesystem_type` | string | 否 | 文件系统类型（不可变更，提交将返回 400） |

### 请求示例

```bash
# 更新存储卷名称
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated System Volume"
  }'

# 扩容存储卷
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "size_bytes": 536870912000
  }'

# 尝试变更文件系统类型（返回 400）
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "filesystem_type": "btrfs"
  }'
```

---

## 响应

### 200 OK - 更新成功

```json
{
  "success": true,
  "message": "Storage volume updated successfully",
  "data": {
    "id": 1,
    "name": "Updated System Volume",
    "pool_id": 1,
    "pool_name": "System Pool",
    "size_bytes": 536870912000,
    "used_bytes": 125000000000,
    "available_bytes": 411870912000,
    "filesystem_type": "ext4",
    "mount_point": "/mnt/updated_system_volume",
    "status": "online",
    "created_at": 1710489600,
    "updated_at": 1711440000
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "filesystem_type cannot be changed after volume creation",
  "code": "FILESYSTEM_IMMUTABLE"
}
```

或

```json
{
  "success": false,
  "error": "Cannot shrink volume below used space (125000000000 bytes)",
  "code": "SIZE_TOO_SMALL"
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
  "error": "Only admin users can update storage volumes",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 存储卷不存在

```json
{
  "success": false,
  "error": "Storage volume 123 not found",
  "code": "NOT_FOUND"
}
```

### 409 Conflict - 名称已存在

```json
{
  "success": false,
  "error": "Storage volume 'Data Volume' already exists",
  "code": "NAME_CONFLICT"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以更新存储卷
3. **名称唯一性**: 存储卷名称必须全局唯一（排除自身）
4. **容量限制**: 不能缩小到小于已用空间
5. **文件系统不可变**: 创建后 filesystem_type 不可变更

---

## 响应字段说明

### UpdateStorageVolumeResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 存储卷信息 |

### StorageVolumeResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 存储卷 ID |
| `name` | string | 存储卷名称 |
| `pool_id` | integer | 所属存储池 ID |
| `pool_name` | string | 所属存储池名称 |
| `size_bytes` | integer | 卷大小（字节） |
| `used_bytes` | integer | 已用容量（字节） |
| `available_bytes` | integer | 可用容量（字节） |
| `filesystem_type` | string | 文件系统类型 |
| `mount_point` | string | 挂载点 |
| `status` | string | 状态：online/offline |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

---

## 实现细节

- **文件位置:** `src/handlers/storage_volumes_update.rs`
- **路由注册:** `src/main.rs` - `PUT /api/v1/storage/volumes/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/storage/volumes` - 存储卷列表（Phase 60）
- `GET /api/v1/storage/volumes/{id}` - 存储卷详情（Phase 61）
- `POST /api/v1/storage/volumes` - 创建存储卷（Phase 67）
- `DELETE /api/v1/storage/volumes/{id}` - 删除存储卷

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 68 初始实现 |
