# 删除存储卷快照 API 文档 (Phase 85)

## 概述

删除存储卷快照 API 允许管理员删除指定存储卷的指定快照。

## 接口详情

### DELETE /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}

删除指定存储卷的指定快照。

#### 认证要求

需要有效的 JWT Token，且用户必须具有 `admin` 角色。

**请求头：**
```
Authorization: Bearer <admin_jwt_token>
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `volume_id` | integer | 是 | 存储卷 ID |
| `snapshot_id` | integer | 是 | 快照 ID |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "message": "Snapshot deleted successfully",
  "data": {
    "id": 1,
    "name": "snapshot-2024-01-01",
    "deleted_at": 1711468800
  }
}
```

**错误响应 (400 Bad Request) - 快照受保护**

```json
{
  "success": false,
  "error": "Cannot delete protected snapshot",
  "code": "SNAPSHOT_PROTECTED"
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
  "error": "Only admin users can delete snapshots",
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

### 示例 1：删除存储卷快照

```bash
curl -X DELETE "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

**响应：**
```json
{
  "success": true,
  "message": "Snapshot deleted successfully",
  "data": {
    "id": 1,
    "name": "snapshot-2024-01-01",
    "deleted_at": 1711468800
  }
}
```

### 示例 2：删除受保护的快照（400）

```bash
curl -X DELETE "http://localhost:8080/api/v1/storage/volumes/1/snapshots/2" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Cannot delete protected snapshot",
  "code": "SNAPSHOT_PROTECTED"
}
```

### 示例 3：删除不存在的存储卷快照（404）

```bash
curl -X DELETE "http://localhost:8080/api/v1/storage/volumes/999/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "VOLUME_NOT_FOUND"
}
```

### 示例 4：删除不存在的快照（404）

```bash
curl -X DELETE "http://localhost:8080/api/v1/storage/volumes/1/snapshots/999" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Snapshot 999 not found",
  "code": "SNAPSHOT_NOT_FOUND"
}
```

### 示例 5：非 admin 用户访问（403）

```bash
curl -X DELETE "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <user_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Only admin users can delete snapshots",
  "code": "FORBIDDEN"
}
```

### 示例 6：未认证访问（401）

```bash
curl -X DELETE "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1"
```

**响应：**
```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可删除快照
- 非 admin 角色返回 403 Forbidden

### 3. 快照保护

- 受保护的快照（`is_protected=true`）不能被删除
- 尝试删除受保护的快照返回 400 Bad Request

### 4. 双重验证

- **存储卷存在性验证**：存储卷不存在返回 404
- **快照存在性验证**：快照不存在返回 404

## 实现文件

- `src/handlers/storage_volume_snapshot_delete.rs` - 存储卷快照删除处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **权限控制**：仅 admin 角色可删除快照
2. **保护机制**：受保护的快照不能被删除
3. **不可恢复**：删除操作是永久性的，快照无法恢复
4. **双重 404**：存储卷不存在和快照不存在都返回 404，但错误代码不同

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `VOLUME_NOT_FOUND` | 404 | 存储卷不存在 |
| `SNAPSHOT_NOT_FOUND` | 404 | 快照不存在 |
| `SNAPSHOT_PROTECTED` | 400 | 快照受保护，无法删除 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/storage/volumes/{volume_id}/snapshots** - 获取快照列表 (Phase 83)
- **POST /api/v1/storage/volumes/{volume_id}/snapshots** - 创建快照 (Phase 82)
- **GET /api/v1/storage/volumes/{volume_id}/snapshots/{id}** - 获取快照详情 (Phase 84)
- **GET /api/v1/storage/volumes** - 获取存储卷列表

## 响应示例（完整）

### 成功删除

```json
{
  "success": true,
  "message": "Snapshot deleted successfully",
  "data": {
    "id": 1,
    "name": "snapshot-2024-01-01",
    "deleted_at": 1711468800
  }
}
```

### 快照受保护（400）

```json
{
  "success": false,
  "error": "Cannot delete protected snapshot",
  "code": "SNAPSHOT_PROTECTED"
}
```

### 存储卷不存在（404）

```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "VOLUME_NOT_FOUND"
}
```

### 快照不存在（404）

```json
{
  "success": false,
  "error": "Snapshot 999 not found",
  "code": "SNAPSHOT_NOT_FOUND"
}
```

### 权限不足（403）

```json
{
  "success": false,
  "error": "Only admin users can delete snapshots",
  "code": "FORBIDDEN"
}
```

### 未认证（401）

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 快照保护说明

| is_protected | 说明 | 是否可删除 |
|--------------|------|-----------|
| `true` | 受保护快照 | ❌ 不可删除 |
| `false` | 普通快照 | ✅ 可删除 |

### 保护机制

- 创建快照时可设置 `is_protected=true` 来防止误删除
- 重要快照（如系统升级前、数据迁移前）建议设置为受保护
- 如需删除受保护快照，需先通过更新接口取消保护状态

## 最佳实践

### 1. 删除前确认

建议在删除前先用 GET 接口获取快照详情，确认快照信息：

```bash
# 1. 获取快照详情
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 2. 确认不是受保护快照后，再执行删除
curl -X DELETE "http://localhost:8080/api/v1/storage/volumes/1/snapshots/1" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 2. 批量删除注意事项

批量删除快照时：
- 先检查每个快照的保护状态
- 跳过受保护的快照
- 记录删除失败的快照及原因

### 3. 审计日志

所有删除操作都应该记录到审计日志中，包括：
- 删除时间
- 执行删除的管理员 ID
- 被删除的快照 ID 和名称
- 所属存储卷 ID
