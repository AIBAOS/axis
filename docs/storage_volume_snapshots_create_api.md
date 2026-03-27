# 创建存储卷快照 API 文档 (Phase 82)

## 概述

创建存储卷快照 API 允许管理员为指定存储卷创建快照。

## 接口详情

### POST /api/v1/storage/volumes/{volume_id}/snapshots

创建存储卷快照。

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

#### 请求体

```json
{
  "name": "snapshot-2024-03-01",
  "description": "Before system upgrade",
  "is_protected": false
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 快照名称（唯一） |
| `description` | string | 否 | 快照描述 |
| `is_protected` | boolean | 否 | 是否受保护（防止删除，默认 false） |

#### 响应格式

**成功响应 (201 Created)**

```json
{
  "success": true,
  "message": "Snapshot created successfully",
  "data": {
    "id": 4,
    "name": "snapshot-2024-03-01",
    "description": "Before system upgrade",
    "volume_id": 1,
    "volume_name": "root",
    "size_bytes": 1099511627776,
    "created_at": 1709251200,
    "created_by": "admin",
    "is_protected": false,
    "status": "completed"
  }
}
```

**错误响应 (400 Bad Request) - 参数无效**

```json
{
  "success": false,
  "error": "name is required",
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

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "error": "Only admin users can create snapshots",
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

### 示例 1：创建存储卷快照

```bash
curl -X POST "http://localhost:8080/api/v1/storage/volumes/1/snapshots" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "snapshot-2024-03-01",
    "description": "Before system upgrade",
    "is_protected": false
  }'
```

**响应：**
```json
{
  "success": true,
  "message": "Snapshot created successfully",
  "data": {
    "id": 4,
    "name": "snapshot-2024-03-01",
    "description": "Before system upgrade",
    "volume_id": 1,
    "volume_name": "root",
    "size_bytes": 1099511627776,
    "created_at": 1709251200,
    "created_by": "admin",
    "is_protected": false,
    "status": "completed"
  }
}
```

### 示例 2：创建受保护的快照

```bash
curl -X POST "http://localhost:8080/api/v1/storage/volumes/1/snapshots" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "snapshot-critical",
    "description": "Critical backup - do not delete",
    "is_protected": true
  }'
```

### 示例 3：名称已存在（409）

```bash
curl -X POST "http://localhost:8080/api/v1/storage/volumes/1/snapshots" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "snapshot-2024-01-01",
    "description": "Duplicate name"
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

### 示例 4：存储卷不存在（404）

```bash
curl -X POST "http://localhost:8080/api/v1/storage/volumes/999/snapshots" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "snapshot-test",
    "description": "Test snapshot"
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

### 示例 5：非 admin 用户访问（403）

```bash
curl -X POST "http://localhost:8080/api/v1/storage/volumes/1/snapshots" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "snapshot-test",
    "description": "Test snapshot"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Only admin users can create snapshots",
  "code": "FORBIDDEN"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可创建快照
- 非 admin 角色返回 403 Forbidden

### 3. 输入验证

- **名称验证**：不能为空，必须唯一
- **存储卷验证**：存储卷必须存在

## 快照状态说明

| 状态 | 说明 |
|------|------|
| `creating` | 创建中 |
| `completed` | 已完成 |
| `failed` | 失败 |
| `deleting` | 删除中 |

## 实现文件

- `src/handlers/storage_volume_snapshots_create.rs` - 存储卷快照创建处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **权限控制**：仅 admin 角色可创建快照
2. **名称唯一性**：快照名称必须唯一
3. **存储卷依赖**：必须指定有效的存储卷 ID
4. **受保护快照**：`is_protected=true` 的快照不能被删除

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `VOLUME_NOT_FOUND` | 404 | 存储卷不存在 |
| `SNAPSHOT_EXISTS` | 409 | 快照名称已存在 |
| `INVALID_PARAMS` | 400 | 参数无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/storage/volumes/{volume_id}/snapshots** - 获取快照列表 (Phase 81)
- **GET /api/v1/storage/volumes/{volume_id}/snapshots/{id}** - 获取快照详情
- **DELETE /api/v1/storage/volumes/{volume_id}/snapshots/{id}** - 删除快照
- **GET /api/v1/storage/volumes** - 获取存储卷列表

## 响应示例（完整）

### 成功创建

```json
{
  "success": true,
  "message": "Snapshot created successfully",
  "data": {
    "id": 4,
    "name": "snapshot-2024-03-01",
    "description": "Before system upgrade",
    "volume_id": 1,
    "volume_name": "root",
    "size_bytes": 1099511627776,
    "created_at": 1709251200,
    "created_by": "admin",
    "is_protected": false,
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
  "error": "Only admin users can create snapshots",
  "code": "FORBIDDEN"
}
```

## 快照命名建议

建议格式：
```
snapshot-YYYY-MM-DD[-description]
```

例如：
- `snapshot-2024-01-01`
- `snapshot-2024-02-01-monthly`
- `snapshot-2024-03-01-before-upgrade`

## 容量换算示例

| 字节 | 换算 |
|------|------|
| 1099511627776 | 1 TB |
| 2199023255552 | 2 TB |

**换算公式：**
- GB = bytes / 1073741824
- TB = bytes / 1099511627776
