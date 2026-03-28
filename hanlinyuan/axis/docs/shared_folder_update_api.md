# Phase 93 - 更新共享文件夹 API 文档

**接口:** `PUT /api/v1/shared-folders/{id}`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

更新共享文件夹配置信息。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 仅 `admin` 角色可调用此接口

**功能特性:**
- 更新共享文件夹名称、描述、协议等
- 验证共享文件夹 ID 存在
- 检查名称唯一性（排除自身）
- 返回更新后的共享文件夹信息

---

## 🔐 认证方式

```
Authorization: Bearer <access_token>
```

---

## 📤 请求参数

### 请求头

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `Authorization` | string | 是 | JWT Bearer Token |
| `Content-Type` | string | 是 | `application/json` |

### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | number | 是 | 共享文件夹 ID |

### 请求体 (Body)

```json
{
  "name": "public-updated",
  "description": "Updated public shared folder",
  "protocols": ["smb", "nfs", "afp"],
  "read_only": false,
  "guest_access": true,
  "enabled": true
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 新共享文件夹名称（必须唯一） |
| `description` | string | 否 | 新描述信息 |
| `protocols` | array | 否 | 支持的协议列表：`smb` / `nfs` / `afp` / `ftp` |
| `read_only` | boolean | 否 | 是否只读 |
| `guest_access` | boolean | 否 | 是否允许访客访问 |
| `enabled` | boolean | 否 | 是否启用（true=active, false=inactive） |

**注意:** 至少提供一个字段，所有字段均为可选。

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "message": "Shared folder updated successfully",
  "data": {
    "id": 1,
    "name": "public-updated",
    "path": "/public",
    "volume_id": 1,
    "volume_name": "data",
    "description": "Updated public shared folder",
    "protocols": ["smb", "nfs", "afp"],
    "is_public": true,
    "read_only": false,
    "guest_access": true,
    "status": "active",
    "created_at": 1774259200,
    "updated_at": 1774432000,
    "created_by": "admin"
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data.id` | number | 共享文件夹 ID |
| `data.name` | string | 共享文件夹名称 |
| `data.path` | string | 共享路径 |
| `data.volume_id` | number | 所属存储卷 ID |
| `data.volume_name` | string | 所属存储卷名称 |
| `data.description` | string/null | 共享文件夹描述 |
| `data.protocols` | array | 支持的协议列表 |
| `data.is_public` | boolean | 是否公开访问 |
| `data.read_only` | boolean | 是否只读 |
| `data.guest_access` | boolean | 是否允许访客访问 |
| `data.status` | string | 状态：`active` / `inactive` |
| `data.created_at` | number | 创建时间（Unix 时间戳） |
| `data.updated_at` | number | 更新时间（Unix 时间戳） |
| `data.created_by` | string | 创建者用户名 |

---

## ❌ 错误响应

### 400 Bad Request

```json
{
  "success": false,
  "error": "At least one field must be provided",
  "code": "INVALID_PARAMS"
}
```

```json
{
  "success": false,
  "error": "At least one protocol must be specified",
  "code": "INVALID_PROTOCOL"
}
```

```json
{
  "success": false,
  "error": "Invalid protocol 'http'. Valid protocols: smb, nfs, afp, ftp",
  "code": "INVALID_PROTOCOL"
}
```

### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden

```json
{
  "success": false,
  "error": "Only admin users can update shared folders",
  "code": "FORBIDDEN"
}
```

### 404 Not Found

```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "NOT_FOUND"
}
```

### 409 Conflict (名称已存在)

```json
{
  "success": false,
  "error": "Shared folder name 'homes' already exists",
  "code": "SHARED_FOLDER_EXISTS"
}
```

---

## 🧪 使用示例

```bash
# 更新共享文件夹名称
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "public-updated"
  }'
```

```bash
# 更新协议列表
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "protocols": ["smb", "nfs", "afp"]
  }'
```

```bash
# 更新多个字段
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "public-updated",
    "description": "Updated public shared folder",
    "protocols": ["smb", "nfs", "afp"],
    "read_only": false,
    "guest_access": true,
    "enabled": true
  }'
```

```bash
# 禁用共享文件夹
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": false
  }'
```

```bash
# 不存在的共享文件夹（404）
curl -X PUT "http://localhost:8080/api/v1/shared-folders/999" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "updated-name"
  }'
# 响应：404 Not Found - Shared folder not found
```

```bash
# 重复名称（409）
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "homes"
  }'
# 响应：409 Conflict - Shared folder name already exists
```

```bash
# 非 admin 用户（403）
curl -X PUT "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <user_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "updated-name"
  }'
# 响应：403 Forbidden - Only admin users can update shared folders
```

---

## 📝 注意事项

1. **权限要求**: 仅 admin 角色可更新共享文件夹，普通用户返回 403
2. **参数要求**: 至少提供一个字段（name/description/protocols/read_only/guest_access/enabled）
3. **共享文件夹不存在**: 返回 404 Not Found
4. **名称唯一性**: 共享文件夹名称全局唯一（排除自身）
5. **协议验证**: 仅支持 `smb` / `nfs` / `afp` / `ftp`
6. **启用/禁用**: `enabled=false` 将状态设置为 `inactive`

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/shared-folders` | 共享文件夹列表 (Phase 90) |
| `GET /api/v1/shared-folders/{id}` | 共享文件夹详情 (Phase 92) |
| `POST /api/v1/shared-folders` | 创建共享文件夹 (Phase 89) |
| `DELETE /api/v1/shared-folders/{id}` | 删除共享文件夹（待实现） |

---

*文档维护：兵部尚书*
