# 恢复存储卷快照 API

**Phase 87** - 存储管理 API 之恢复存储卷快照接口

---

## 接口信息

- **端点:** `POST /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/restore`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <ADMIN_JWT_TOKEN>` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `volume_id` | integer | 是 | 存储卷 ID（恢复目标） |
| `snapshot_id` | integer | 是 | 快照 ID（恢复源） |

### 请求示例

```bash
# 恢复快照到存储卷
curl -X POST "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1/restore" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"
```

---

## 响应

### 200 OK - 恢复成功

```json
{
  "success": true,
  "message": "Snapshot restored successfully",
  "data": {
    "source_snapshot_id": 1,
    "target_volume_id": 1,
    "target_volume_name": "System Volume",
    "restored_at": 1711526400,
    "status": "completed"
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "Cannot restore snapshot: volume 'Data Volume' is currently mounted/in use",
  "code": "VOLUME_IN_USE"
}
```

或

```json
{
  "success": false,
  "error": "Cannot restore snapshot: volume status is 'offline'",
  "code": "VOLUME_NOT_READY"
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
  "error": "Cannot restore snapshot: snapshot status is 'creating'",
  "code": "SNAPSHOT_NOT_READY"
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
  "error": "Only admin users can restore snapshots",
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

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以恢复快照
3. **存储卷状态**: 存储卷必须在线且未挂载才能恢复
4. **快照状态**: 只有 completed 状态的快照可以恢复
5. **归属验证**: 快照必须属于目标存储卷

---

## 响应字段说明

### RestoreSnapshotResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 恢复信息 |

### RestoreInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `source_snapshot_id` | integer | 源快照 ID |
| `target_volume_id` | integer | 目标存储卷 ID |
| `target_volume_name` | string | 目标存储卷名称 |
| `restored_at` | integer | 恢复时间（Unix 时间戳） |
| `status` | string | 恢复状态：completed/failed |

---

## 实现细节

- **文件位置:** `src/handlers/storage_volume_snapshot_restore.rs`
- **路由注册:** `src/main.rs` - `POST /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/restore`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/storage/volumes/{id}/snapshots` - 快照列表（Phase 83）
- `POST /api/v1/storage/volumes/{id}/snapshots` - 创建快照（Phase 82）
- `GET /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}` - 快照详情（Phase 84）
- `DELETE /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}` - 删除快照（Phase 85）
- `PUT /api/v1/storage/volumes/{id}/snapshots/{snapshot_id}` - 更新快照（Phase 86）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 87 初始实现 |
