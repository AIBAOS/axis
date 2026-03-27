# 存储池更新 API

**Phase 65** - 存储管理 API 之更新存储池接口

---

## 接口信息

- **端点:** `PUT /api/v1/storage/pools/{id}`
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
| `id` | integer | 是 | 存储池 ID |

### 请求体

```json
{
  "name": "string (可选)",
  "type": "basic|raid0|raid1|raid5|raid6|raid10 (可选)",
  "status": "online|degraded|offline (可选)"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 存储池新名称（必须唯一） |
| `type` | string | 否 | RAID 类型变更（谨慎使用） |
| `status` | string | 否 | 状态：online/degraded/offline |

### 请求示例

```bash
# 更新存储池名称
curl -X PUT "http://localhost:8080/api/v1/storage/pools/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated System Pool"
  }'

# 更新存储池状态
curl -X PUT "http://localhost:8080/api/v1/storage/pools/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "degraded"
  }'

# 更新存储池类型（无卷使用时）
curl -X PUT "http://localhost:8080/api/v1/storage/pools/3" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "type": "raid6"
  }'
```

---

## 响应

### 200 OK - 更新成功

```json
{
  "success": true,
  "message": "Storage pool updated successfully",
  "data": {
    "id": 1,
    "name": "Updated System Pool",
    "type": "basic",
    "status": "online",
    "total_bytes": 536870912000,
    "used_bytes": 268435456000,
    "available_bytes": 268435456000,
    "usage_percent": 50.0,
    "disk_count": 1,
    "created_at": 1710489600,
    "updated_at": 1711440000
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "Invalid type. Valid types: basic, raid0, raid1, raid5, raid6, raid10",
  "code": "INVALID_TYPE"
}
```

或

```json
{
  "success": false,
  "error": "Cannot change pool type while volumes are using this pool",
  "code": "POOL_IN_USE"
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
  "error": "Only admin users can update storage pools",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 存储池不存在

```json
{
  "success": false,
  "error": "Storage pool 123 not found",
  "code": "NOT_FOUND"
}
```

### 409 Conflict - 名称已存在

```json
{
  "success": false,
  "error": "Storage pool 'Data Pool' already exists",
  "code": "NAME_CONFLICT"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以更新存储池
3. **名称唯一性**: 存储池名称必须全局唯一（排除自身）
4. **类型变更限制**: 有卷使用的存储池不允许变更类型

---

## 响应字段说明

### UpdateStoragePoolResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 存储池信息 |

### StoragePoolResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 存储池 ID |
| `name` | string | 存储池名称 |
| `type` | string | RAID 类型 |
| `status` | string | 状态：online/degraded/offline |
| `total_bytes` | integer | 总容量（字节） |
| `used_bytes` | integer | 已用容量（字节） |
| `available_bytes` | integer | 可用容量（字节） |
| `usage_percent` | number | 使用率百分比 |
| `disk_count` | integer | 磁盘数量 |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

---

## 实现细节

- **文件位置:** `src/handlers/storage_pools_update.rs`
- **路由注册:** `src/main.rs` - `PUT /api/v1/storage/pools/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/storage/pools` - 存储池列表（Phase 62）
- `GET /api/v1/storage/pools/{id}` - 存储池详情（Phase 63）
- `POST /api/v1/storage/pools` - 创建存储池（Phase 64）
- `DELETE /api/v1/storage/pools/{id}` - 删除存储池

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 65 初始实现 |
