# 创建存储卷快照 API

**Phase 82** - 存储管理 API 之创建存储卷快照接口

---

## 接口信息

- **端点:** `POST /api/v1/storage/volumes/{id}/snapshots`
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
  "name": "string",
  "description": "string (可选)",
  "is_protected": "boolean (可选，默认 false)"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `name` | string | 是 | - | 快照名称（必须唯一） |
| `description` | string | 否 | null | 快照描述 |
| `is_protected` | boolean | 否 | false | 是否受保护（防止删除） |

### 请求示例

```bash
# 创建快照
curl -X POST "http://localhost:8080/api/v1/storage/volumes/1/snapshots" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "System Volume-snapshot-3",
    "description": "Before system update",
    "is_protected": false
  }'

# 创建受保护的快照
curl -X POST "http://localhost:8080/api/v1/storage/volumes/1/snapshots" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "System Volume-snapshot-4",
    "description": "Critical backup",
    "is_protected": true
  }'
```

---

## 响应

### 201 Created - 创建成功

```json
{
  "success": true,
  "message": "Snapshot created successfully",
  "data": {
    "id": 100,
    "name": "System Volume-snapshot-3",
    "description": "Before system update",
    "volume_id": 1,
    "volume_name": "System Volume",
    "size_bytes": 107374182400,
    "created_at": 1711526400,
    "created_by": "admin",
    "is_protected": false
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "name is required",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Cannot create snapshot: volume status is 'offline'",
  "code": "VOLUME_NOT_READY"
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
  "error": "Only admin users can create snapshots",
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

### 409 Conflict - 快照名称已存在

```json
{
  "success": false,
  "error": "Snapshot 'System Volume-snapshot-1' already exists",
  "code": "NAME_CONFLICT"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以创建快照
3. **名称唯一性**: 快照名称在同一存储卷下必须唯一
4. **存储卷状态**: 仅 online 状态的存储卷可以创建快照

---

## 响应字段说明

### CreateSnapshotResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 快照信息 |

### SnapshotResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 快照 ID |
| `name` | string | 快照名称 |
| `description` | string\|null | 快照描述 |
| `volume_id` | integer | 所属存储卷 ID |
| `volume_name` | string | 所属存储卷名称 |
| `size_bytes` | integer | 快照大小（字节） |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `created_by` | string | 创建者用户名 |
| `is_protected` | boolean | 是否受保护（防止删除） |

---

## 实现细节

- **文件位置:** `src/handlers/storage_volume_snapshot_create.rs`
- **路由注册:** `src/main.rs` - `POST /api/v1/storage/volumes/{id}/snapshots`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/storage/volumes/{id}/snapshots` - 快照列表（Phase 81）
- `GET /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}` - 快照详情
- `DELETE /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}` - 删除快照
- `POST /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}/restore` - 恢复快照

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 82 初始实现 |
