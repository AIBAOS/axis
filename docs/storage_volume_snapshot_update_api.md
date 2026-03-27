# 更新存储卷快照 API 文档 (Phase 86)

## 概述

更新存储卷快照 API 允许管理员修改指定存储卷的指定快照信息。

## 接口详情

### PUT /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}

更新指定存储卷的指定快照。

#### 认证要求

需要有效的 JWT Token，且用户必须具有 `admin` 角色。

**请求头：**
```
Authorization: Bearer <admin_jwt_token>
Content-Type: application/json
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `volume_id` | integer | 是 | 存储卷 ID |
| `snapshot_id` | integer | 是 | 快照 ID |

#### 请求体

```json
{
  "name": "snapshot-2024-03-01-updated",
  "description": "Updated description",
  "is_protected": true
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 快照名称（唯一） |
| `description` | string | 否 | 快照描述 |
| `is_protected` | boolean | 否 | 是否受保护（防止删除） |

**注意：** 所有字段均为可选，仅提供需要更新的字段。

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "message": "Snapshot updated successfully",
  "data": {
    "id": 1,
    "name": "snapshot-2024-03-01-updated",
    "description": "Updated description",
    "volume_id": 1,
    "volume_name": "System Volume",
    "size_bytes": 1099511627776,
    "created_at": 1704067200,
    "updated_at": 1711468800,
    "created_by": "admin",
    "is_protected": true,
    "status": "completed"
  }
}
```

**错误响应 (400 Bad Request) - 参数无效**

```json
{
  "success": false,
  "error": "name cannot be empty",
  "code": "INVALID_PARAMS"
}
```

**错误响应 (409 Conflict) - 快照名称已存在**

```json
{
  "success": false,
  "error": "Snapshot name already exists",
  "code": "SNAPSHOT_EXISTS"
}
```

**错误响应 (404 Not Found) - 存储卷不存在**

```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "VOLUME_NOT_FOUND"
}
```

**错误响应 (404 Not Found) - 快照不存在**

```json
{
  "success": false,
  "error": "Snapshot 999 not found",
  "code": "SNAPSHOT_NOT_FOUND"
}
```

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "error": "Only admin users can update snapshots",
  "code": "FORBIDDEN"
}
```

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：更新快照描述

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "description": "Updated description"
  }'
```

**响应：**
```json
{
  "success": true,
  "message": "Snapshot updated successfully",
  "data": {
    "id": 1,
    "name": "snapshot-2024-01-01",
    "description": "Updated description",
    "volume_id": 1,
    "volume_name": "System Volume",
    "size_bytes": 1099511627776,
    "created_at": 1704067200,
    "updated_at": 1711468800,
    "created_by": "admin",
    "is_protected": false,
    "status": "completed"
  }
}
```

### 示例 2：设置快照保护状态

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "is_protected": true
  }'
```

### 示例 3：更新快照名称

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "snapshot-2024-01-01-renamed"
  }'
```

### 示例 4：更新多个字段

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "snapshot-2024-01-01-updated",
    "description": "Critical backup",
    "is_protected": true
  }'
```

### 示例 5：名称已存在（409）

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "snapshot-2024-02-01"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Snapshot name already exists",
  "code": "SNAPSHOT_EXISTS"
}
```

### 示例 6：存储卷不存在（404）

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/999/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "description": "Test"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "VOLUME_NOT_FOUND"
}
```

### 示例 7：快照不存在（404）

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/999" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "description": "Test"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Snapshot 999 not found",
  "code": "SNAPSHOT_NOT_FOUND"
}
```

### 示例 8：非 admin 用户访问（403）

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "description": "Test"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Only admin users can update snapshots",
  "code": "FORBIDDEN"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可更新快照
- 非 admin 角色返回 403 Forbidden

### 3. 输入验证

- **名称验证**：不能为空，必须唯一
- **存储卷验证**：存储卷必须存在
- **快照验证**：快照必须存在

### 4. 双重验证

- **存储卷存在性验证**：存储卷不存在返回 404
- **快照存在性验证**：快照不存在返回 404

## 实现文件

- `src/handlers/storage_volume_snapshot_update.rs` - 存储卷快照更新处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **权限控制**：仅 admin 角色可更新快照
2. **名称唯一性**：快照名称必须唯一（修改名称时检查）
3. **部分更新**：仅提供需要更新的字段，未提供字段保持原值
4. **不可变字段**：`volume_id`、`size_bytes`、`created_at`、`created_by` 不可修改

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `VOLUME_NOT_FOUND` | 404 | 存储卷不存在 |
| `SNAPSHOT_NOT_FOUND` | 404 | 快照不存在 |
| `SNAPSHOT_EXISTS` | 409 | 快照名称已存在 |
| `INVALID_PARAMS` | 400 | 参数无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/storage/volumes/{volume_id}/snapshots** - 获取快照列表 (Phase 83)
- **POST /api/v1/storage/volumes/{volume_id}/snapshots** - 创建快照 (Phase 82)
- **GET /api/v1/storage/volumes/{volume_id}/snapshots/{id}** - 获取快照详情 (Phase 84)
- **DELETE /api/v1/storage/volumes/{volume_id}/snapshots/{id}** - 删除快照 (Phase 85)

## 响应示例（完整）

### 成功更新

```json
{
  "success": true,
  "message": "Snapshot updated successfully",
  "data": {
    "id": 1,
    "name": "snapshot-2024-01-01-updated",
    "description": "Critical backup",
    "volume_id": 1,
    "volume_name": "System Volume",
    "size_bytes": 1099511627776,
    "created_at": 1704067200,
    "updated_at": 1711468800,
    "created_by": "admin",
    "is_protected": true,
    "status": "completed"
  }
}
```

### 名称已存在（409）

```json
{
  "success": false,
  "error": "Snapshot name already exists",
  "code": "SNAPSHOT_EXISTS"
}
```

### 权限不足（403）

```json
{
  "success": false,
  "error": "Only admin users can update snapshots",
  "code": "FORBIDDEN"
}
```

## 可更新字段说明

| 字段 | 说明 | 是否可空 |
|------|------|---------|
| `name` | 快照名称（唯一） | ❌ 不能为空字符串 |
| `description` | 快照描述 | ✅ 可为 null |
| `is_protected` | 是否受保护 | ✅ 可为 null |

## 不可更新字段

| 字段 | 说明 |
|------|------|
| `id` | 快照 ID（不可变） |
| `volume_id` | 所属存储卷 ID（不可变） |
| `volume_name` | 所属存储卷名称（只读） |
| `size_bytes` | 快照大小（不可变） |
| `created_at` | 创建时间（不可变） |
| `created_by` | 创建者（不可变） |
| `status` | 快照状态（不可变） |

## 最佳实践

### 1. 更新前确认

建议在更新前先用 GET 接口获取快照详情，确认快照信息：

```bash
# 1. 获取快照详情
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 2. 确认后再执行更新
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "is_protected": true
  }'
```

### 2. 保护重要快照

对于重要快照（如系统升级前、数据迁移前），建议设置 `is_protected=true`：

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "is_protected": true,
    "description": "Critical backup - do not delete"
  }'
```

### 3. 审计日志

所有更新操作都应该记录到审计日志中，包括：
- 更新时间
- 执行更新的管理员 ID
- 被更新的快照 ID
- 更新的字段及旧值/新值
