# 存储卷快照列表 API

**Phase 83** - 存储管理 API 之存储卷快照列表接口（增强版）

---

## 接口信息

- **端点:** `GET /api/v1/storage/volumes/{volume_id}/snapshots`
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

### 查询参数

| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `limit` | integer | 否 | 20 | 每页数量（最大 100） |
| `offset` | integer | 否 | 0 | 偏移量 |
| `status` | string | 否 | - | 按状态筛选：creating/completed/failed/deleting |
| `is_protected` | boolean | 否 | - | 按是否受保护筛选：true/false |

### 请求示例

```bash
# 获取存储卷的快照列表
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots" \
  -H "Authorization: Bearer <JWT_TOKEN>"

# 带分页参数
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots?limit=10&offset=0" \
  -H "Authorization: Bearer <JWT_TOKEN>"

# 按状态筛选
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots?status=completed" \
  -H "Authorization: Bearer <JWT_TOKEN>"

# 筛选受保护的快照
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots?is_protected=true" \
  -H "Authorization: Bearer <JWT_TOKEN>"
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "data": [
    {
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
    },
    {
      "id": 2,
      "name": "System Volume-snapshot-2",
      "description": "Before update",
      "volume_id": 1,
      "volume_name": "System Volume",
      "size_bytes": 107374182400,
      "created_at": 1711440000,
      "created_by": "admin",
      "is_protected": true,
      "status": "completed"
    }
  ],
  "pagination": {
    "total_count": 4,
    "limit": 20,
    "offset": 0
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

### 200 OK - 空列表（无快照）

```json
{
  "success": true,
  "data": [],
  "pagination": {
    "total_count": 0,
    "limit": 20,
    "offset": 0
  }
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 登录用户可访问（无需 admin 权限）

---

## 响应字段说明

### SnapshotListResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `data` | array | 快照列表 |
| `pagination` | object | 分页信息 |

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

### PaginationMeta

| 字段 | 类型 | 说明 |
|------|------|------|
| `total_count` | integer | 总记录数 |
| `limit` | integer | 每页数量 |
| `offset` | integer | 偏移量 |

---

## 快照状态说明

| 状态 | 说明 |
|------|------|
| `creating` | 创建中 |
| `completed` | 已完成 |
| `failed` | 创建失败 |
| `deleting` | 删除中 |

---

## 实现细节

- **文件位置:** `src/handlers/storage_volume_snapshots_list.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/storage/volumes/{volume_id}/snapshots`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `POST /api/v1/storage/volumes/{id}/snapshots` - 创建快照（Phase 82）
- `GET /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}` - 快照详情
- `DELETE /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}` - 删除快照
- `POST /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}/restore` - 恢复快照

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 81 初始实现 |
| 2026-03-26 | 1.1 | Phase 83 增强版：添加 status/is_protected 筛选，description/status 字段 |
