# 存储卷快照详情 API

**Phase 84** - 存储管理 API 之获取存储卷快照详情接口

---

## 接口信息

- **端点:** `GET /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}`
- **认证:** 需要 JWT Bearer Token（登录用户）
- **权限:** 所有已认证用户可访问
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
| `volume_id` | integer | 是 | 存储卷 ID |
| `snapshot_id` | integer | 是 | 快照 ID |

### 请求示例

```bash
# 获取快照详情
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <JWT_TOKEN>"
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "System Volume-snapshot-1",
    "description": "Initial backup",
    "volume_id": 1,
    "volume_name": "System Volume",
    "size_bytes": 107374182400,
    "created_at": 1710489600,
    "created_by": "admin",
    "is_protected": false,
    "status": "completed"
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

或

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
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

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 登录用户可访问（无需 admin 权限）

---

## 响应字段说明

### SnapshotDetailResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `data` | object | 快照详情数据 |

### SnapshotInfo

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
| `status` | string | 状态：creating/completed/failed/deleting |

---

## 实现细节

- **文件位置:** `src/handlers/storage_volume_snapshot_detail.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `GET /api/v1/storage/volumes/{id}/snapshots` - 快照列表（Phase 83）
- `POST /api/v1/storage/volumes/{id}/snapshots` - 创建快照（Phase 82）
- `DELETE /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}` - 删除快照
- `POST /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}/restore` - 恢复快照

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 84 初始实现 |
