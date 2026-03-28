# 克隆存储卷快照 API

**Phase 88** - 存储管理 API 之克隆存储卷快照接口

---

## 接口信息

- **端点:** `POST /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/clone`
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
| `volume_id` | integer | 是 | 源存储卷 ID |
| `snapshot_id` | integer | 是 | 快照 ID |

### 请求体

```json
{
  "new_volume_name": "string",
  "description": "string (可选)",
  "pool_id": "integer (可选)"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `new_volume_name` | string | 是 | 新存储卷名称（必须唯一） |
| `description` | string | 否 | 新存储卷描述 |
| `pool_id` | integer | 否 | 目标存储池 ID（默认使用源卷所在池） |

### 请求示例

```bash
# 克隆快照到新卷
curl -X POST "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1/clone" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "new_volume_name": "Cloned Volume",
    "description": "Cloned from snapshot 1",
    "pool_id": 2
  }'
```

---

## 响应

### 201 Created - 克隆成功

```json
{
  "success": true,
  "message": "Snapshot cloned successfully",
  "data": {
    "id": 100,
    "name": "Cloned Volume",
    "description": "Cloned from snapshot 1",
    "pool_id": 2,
    "pool_name": "Data Pool",
    "size_bytes": 107374182400,
    "used_bytes": 107374182400,
    "available_bytes": 0,
    "usage_percent": 100.0,
    "filesystem": "ext4",
    "status": "online",
    "mount_point": "/mnt/cloned_volume",
    "created_at": 1711526400,
    "created_from_snapshot": 1
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "new_volume_name is required",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Snapshot 3 does not belong to volume 1",
  "code": "SNAPSHOT_MISMATCH"
}
```

或

```json
{
  "success": false,
  "error": "Cannot clone snapshot: snapshot status is 'creating'",
  "code": "SNAPSHOT_NOT_READY"
}
```

或

```json
{
  "success": false,
  "error": "Insufficient pool capacity: requires 107374182400 bytes, available 53687091200 bytes",
  "code": "INSUFFICIENT_CAPACITY"
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
  "error": "Only admin users can clone snapshots",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 存储卷不存在

```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "NOT_FOUND"
}
```

### 404 Not Found - 快照不存在

```json
{
  "success": false,
  "error": "Snapshot 999 not found",
  "code": "NOT_FOUND"
}
```

### 404 Not Found - 存储池不存在

```json
{
  "success": false,
  "error": "Storage pool 999 not found",
  "code": "POOL_NOT_FOUND"
}
```

### 409 Conflict - 名称已存在

```json
{
  "success": false,
  "error": "Volume 'Data Volume' already exists",
  "code": "NAME_CONFLICT"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以克隆快照
3. **快照状态**: 只有 completed 状态的快照可以克隆
4. **名称唯一性**: 新卷名称必须全局唯一
5. **存储池容量**: 目标存储池必须有足够容量

---

## 响应字段说明

### CloneSnapshotResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 新卷信息 |

### NewVolumeInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 新卷 ID |
| `name` | string | 新卷名称 |
| `description` | string\|null | 新卷描述 |
| `pool_id` | integer | 所属存储池 ID |
| `pool_name` | string | 所属存储池名称 |
| `size_bytes` | integer | 卷大小（字节） |
| `used_bytes` | integer | 已用容量（字节） |
| `available_bytes` | integer | 可用容量（字节） |
| `usage_percent` | number | 使用率百分比 |
| `filesystem` | string | 文件系统类型 |
| `status` | string | 状态：online/offline |
| `mount_point` | string | 挂载点 |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `created_from_snapshot` | integer | 源快照 ID |

---

## 实现细节

- **文件位置:** `src/handlers/storage_volume_snapshot_clone.rs`
- **路由注册:** `src/main.rs` - `POST /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/clone`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/storage/volumes/{id}/snapshots` - 快照列表（Phase 83）
- `POST /api/v1/storage/volumes/{id}/snapshots` - 创建快照（Phase 82）
- `POST /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}/restore` - 恢复快照（Phase 87）
- `POST /api/v1/storage/volumes` - 创建存储卷（Phase 67）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 88 初始实现 |
