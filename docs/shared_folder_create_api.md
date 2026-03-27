# Phase 89 - 创建共享文件夹 API 文档

**接口:** `POST /api/v1/shared-folders`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

创建新的共享文件夹，用于配置 SMB/NFS 等网络共享。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 仅 `admin` 角色可调用此接口

**功能特性:**
- 创建共享文件夹
- 支持多协议配置（SMB/NFS/AFP/FTP）
- 验证存储卷存在性
- 检查共享文件夹名称唯一性
- 返回创建的共享文件夹信息

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

### 请求体 (Body)

```json
{
  "name": "public",
  "path": "/public",
  "volume_id": 1,
  "description": "Public shared folder",
  "protocols": ["smb", "nfs"],
  "is_public": true
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 共享文件夹名称（必须唯一） |
| `path` | string | 是 | 共享路径（相对于存储卷根目录） |
| `volume_id` | number | 是 | 所属存储卷 ID |
| `description` | string | 否 | 共享文件夹描述 |
| `protocols` | array | 是 | 支持的协议列表：`smb` / `nfs` / `afp` / `ftp` |
| `is_public` | boolean | 否 | 是否公开访问（默认 false） |

---

## 📥 响应结果

### 201 Created

```json
{
  "success": true,
  "message": "Shared folder created successfully",
  "data": {
    "id": 101,
    "name": "public",
    "path": "/public",
    "volume_id": 1,
    "volume_name": "data",
    "description": "Public shared folder",
    "protocols": ["smb", "nfs"],
    "is_public": true,
    "status": "active",
    "created_at": 1774432000,
    "created_by": "admin"
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data.id` | number | 共享文件夹 ID（系统分配） |
| `data.name` | string | 共享文件夹名称 |
| `data.path` | string | 共享路径 |
| `data.volume_id` | number | 所属存储卷 ID |
| `data.volume_name` | string | 所属存储卷名称 |
| `data.description` | string/null | 共享文件夹描述 |
| `data.protocols` | array | 支持的协议列表 |
| `data.is_public` | boolean | 是否公开访问 |
| `data.status` | string | 状态：`active` / `inactive` |
| `data.created_at` | number | 创建时间（Unix 时间戳） |
| `data.created_by` | string | 创建者用户名 |

---

## ❌ 错误响应

### 400 Bad Request

```json
{
  "success": false,
  "error": "name is required",
  "code": "INVALID_PARAMS"
}
```

```json
{
  "success": false,
  "error": "path is required",
  "code": "INVALID_PARAMS"
}
```

```json
{
  "success": false,
  "error": "At least one protocol must be specified",
  "code": "INVALID_PARAMS"
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
  "error": "Only admin users can create shared folders",
  "code": "FORBIDDEN"
}
```

### 404 Not Found (存储卷不存在)

```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "VOLUME_NOT_FOUND"
}
```

### 409 Conflict (名称已存在)

```json
{
  "success": false,
  "error": "Shared folder name 'public' already exists",
  "code": "SHARED_FOLDER_EXISTS"
}
```

---

## 🧪 使用示例

```bash
# 创建 SMB/NFS 共享文件夹
curl -X POST "http://localhost:8080/api/v1/shared-folders" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "public",
    "path": "/public",
    "volume_id": 1,
    "description": "Public shared folder",
    "protocols": ["smb", "nfs"],
    "is_public": true
  }'
```

```bash
# 创建仅 SMB 共享
curl -X POST "http://localhost:8080/api/v1/shared-folders" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "homes",
    "path": "/homes",
    "volume_id": 2,
    "protocols": ["smb"]
  }'
```

```bash
# 创建多协议共享
curl -X POST "http://localhost:8080/api/v1/shared-folders" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "media",
    "path": "/media",
    "volume_id": 1,
    "description": "Media files shared folder",
    "protocols": ["smb", "nfs", "afp"],
    "is_public": false
  }'
```

```bash
# 无效的协议（400）
curl -X POST "http://localhost:8080/api/v1/shared-folders" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test",
    "path": "/test",
    "volume_id": 1,
    "protocols": ["http"]
  }'
# 响应：400 Bad Request - Invalid protocol 'http'
```

```bash
# 不存在的存储卷（404）
curl -X POST "http://localhost:8080/api/v1/shared-folders" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test",
    "path": "/test",
    "volume_id": 999,
    "protocols": ["smb"]
  }'
# 响应：404 Not Found - Storage volume not found
```

```bash
# 重复名称（409）
curl -X POST "http://localhost:8080/api/v1/shared-folders" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "public",
    "path": "/public2",
    "volume_id": 1,
    "protocols": ["smb"]
  }'
# 响应：409 Conflict - Shared folder name already exists
```

```bash
# 非 admin 用户（403）
curl -X POST "http://localhost:8080/api/v1/shared-folders" \
  -H "Authorization: Bearer <user_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test",
    "path": "/test",
    "volume_id": 1,
    "protocols": ["smb"]
  }'
# 响应：403 Forbidden - Only admin users can create shared folders
```

---

## 📝 注意事项

1. **权限要求**: 仅 admin 角色可创建共享文件夹，普通用户返回 403
2. **名称唯一性**: 共享文件夹名称全局唯一
3. **路径格式**: 路径应相对于存储卷根目录，以 `/` 开头
4. **协议支持**:
   - `smb`: SMB/CIFS 协议（Windows 共享）
   - `nfs`: NFS 协议（Linux/Unix 共享）
   - `afp`: AFP 协议（macOS 共享）
   - `ftp`: FTP 协议
5. **公开访问**: `is_public=true` 允许匿名访问
6. **存储卷**: 共享文件夹必须关联到已存在的存储卷

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/shared-folders` | 共享文件夹列表（待实现） |
| `GET /api/v1/shared-folders/{id}` | 共享文件夹详情（待实现） |
| `PUT /api/v1/shared-folders/{id}` | 更新共享文件夹（待实现） |
| `DELETE /api/v1/shared-folders/{id}` | 删除共享文件夹（待实现） |
| `POST /api/v1/storage/volumes` | 创建存储卷 (Phase 80) |

---

*文档维护：兵部尚书*
