# 创建共享文件夹 API 文档 (Phase 91)

## 概述

创建共享文件夹 API 允许管理员创建新的网络共享文件夹，支持 SMB/NFS/AFP/FTP 等协议。

## 接口详情

### POST /api/v1/shares

创建新的共享文件夹。

#### 认证要求

需要有效的 JWT Token，且用户必须具有 `admin` 角色。

**请求头：**
```
Authorization: Bearer <admin_jwt_token>
Content-Type: application/json
```

#### 请求体

```json
{
  "name": "media",
  "volume_id": 2,
  "path": "/mnt/volumes/data/media",
  "description": "Media files shared folder",
  "protocols": ["smb", "nfs"],
  "read_only": false,
  "guest_access": false
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 共享文件夹名称（唯一） |
| `volume_id` | integer | 是 | 所属存储卷 ID |
| `path` | string | 是 | 文件夹路径（相对于卷根目录） |
| `description` | string | 否 | 共享文件夹描述 |
| `protocols` | array | 是 | 支持的协议列表（smb/nfs/afp/ftp） |
| `read_only` | boolean | 否 | 是否只读访问（默认 false） |
| `guest_access` | boolean | 否 | 是否允许访客访问（默认 false） |

#### 响应格式

**成功响应 (201 Created)**

```json
{
  "success": true,
  "message": "Shared folder created successfully",
  "data": {
    "id": 1,
    "name": "media",
    "volume_id": 2,
    "volume_name": "data",
    "path": "/mnt/volumes/data/media",
    "description": "Media files shared folder",
    "protocols": ["smb", "nfs"],
    "read_only": false,
    "guest_access": false,
    "status": "active",
    "created_at": 1711468800,
    "created_by": "admin"
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

**错误响应 (400 Bad Request) - 协议无效**

```json
{
  "success": false,
  "error": "Invalid protocol 'http'. Valid protocols: smb, nfs, afp, ftp",
  "code": "INVALID_PROTOCOL"
}
```

**错误响应 (400 Bad Request) - 路径格式无效**

```json
{
  "success": false,
  "error": "Path must start with /",
  "code": "INVALID_PATH"
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

**错误响应 (409 Conflict) - 名称已存在**

```json
{
  "success": false,
  "error": "Shared folder name already exists",
  "code": "SHARE_EXISTS"
}
```

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "error": "Only admin users can create shares",
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

### 示例 1：创建 SMB/NFS 共享文件夹

```bash
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "media",
    "volume_id": 2,
    "path": "/mnt/volumes/data/media",
    "description": "Media files shared folder",
    "protocols": ["smb", "nfs"],
    "read_only": false,
    "guest_access": false
  }'
```

### 示例 2：创建公共 FTP 共享

```bash
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "public",
    "volume_id": 2,
    "path": "/mnt/volumes/data/public",
    "protocols": ["ftp"],
    "read_only": true,
    "guest_access": true
  }'
```

### 示例 3：创建只读共享

```bash
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "archive",
    "volume_id": 3,
    "path": "/mnt/volumes/backup/archive",
    "description": "Archive read-only share",
    "protocols": ["smb"],
    "read_only": true,
    "guest_access": false
  }'
```

### 示例 4：名称已存在（409）

```bash
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "public",
    "volume_id": 2,
    "path": "/mnt/test",
    "protocols": ["smb"]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Shared folder name already exists",
  "code": "SHARE_EXISTS"
}
```

### 示例 5：无效协议（400）

```bash
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test",
    "volume_id": 2,
    "path": "/mnt/test",
    "protocols": ["http"]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Invalid protocol 'http'. Valid protocols: smb, nfs, afp, ftp",
  "code": "INVALID_PROTOCOL"
}
```

### 示例 6：路径格式无效（400）

```bash
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test",
    "volume_id": 2,
    "path": "mnt/test",
    "protocols": ["smb"]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Path must start with /",
  "code": "INVALID_PATH"
}
```

### 示例 7：存储卷不存在（404）

```bash
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test",
    "volume_id": 999,
    "path": "/mnt/test",
    "protocols": ["smb"]
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

### 示例 8：非 admin 用户访问（403）

```bash
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test",
    "volume_id": 2,
    "path": "/mnt/test",
    "protocols": ["smb"]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Only admin users can create shares",
  "code": "FORBIDDEN"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可创建共享文件夹
- 非 admin 角色返回 403 Forbidden

### 3. 输入验证

- **名称验证**：不能为空，必须唯一
- **路径验证**：不能为空，必须以 / 开头，不能包含 ..
- **协议验证**：必须是 smb/nfs/afp/ftp 之一
- **存储卷验证**：存储卷必须存在

## 协议说明

| 协议 | 说明 | 适用场景 |
|------|------|---------|
| `smb` | Server Message Block | Windows 文件共享 |
| `nfs` | Network File System | Linux/Unix 文件共享 |
| `afp` | Apple Filing Protocol | macOS 文件共享 |
| `ftp` | File Transfer Protocol | 通用文件传输 |

## 实现文件

- `src/handlers/shares_create.rs` - 共享文件夹创建处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **权限控制**：仅 admin 角色可创建共享文件夹
2. **名称唯一性**：共享文件夹名称必须唯一
3. **路径格式**：必须以 / 开头，不能包含 ..
4. **协议选择**：至少选择一个支持的协议
5. **只读访问**：`read_only=true` 限制为只读访问
6. **访客访问**：`guest_access=true` 允许匿名访问

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `VOLUME_NOT_FOUND` | 404 | 存储卷不存在 |
| `SHARE_EXISTS` | 409 | 共享文件夹名称已存在 |
| `INVALID_PROTOCOL` | 400 | 协议无效 |
| `INVALID_PATH` | 400 | 路径格式无效 |
| `INVALID_PARAMS` | 400 | 参数无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/shares** - 获取共享文件夹列表 (Phase 90)
- **GET /api/v1/shares/{id}** - 获取共享文件夹详情
- **PUT /api/v1/shares/{id}** - 更新共享文件夹
- **DELETE /api/v1/shares/{id}** - 删除共享文件夹
- **POST /api/v1/shared-folders** - 创建共享文件夹 (Phase 89)

## 响应示例（完整）

### 成功创建

```json
{
  "success": true,
  "message": "Shared folder created successfully",
  "data": {
    "id": 1,
    "name": "media",
    "volume_id": 2,
    "volume_name": "data",
    "path": "/mnt/volumes/data/media",
    "description": "Media files shared folder",
    "protocols": ["smb", "nfs"],
    "read_only": false,
    "guest_access": false,
    "status": "active",
    "created_at": 1711468800,
    "created_by": "admin"
  }
}
```

### 名称已存在（409）

```json
{
  "success": false,
  "error": "Shared folder name already exists",
  "code": "SHARE_EXISTS"
}
```

### 权限不足（403）

```json
{
  "success": false,
  "error": "Only admin users can create shares",
  "code": "FORBIDDEN"
}
```

## 最佳实践

### 1. 命名规范

使用有意义的命名规范：
- 小写字母
- 使用连字符或下划线分隔单词
- 避免特殊字符

例如：
- `media-files`
- `public_documents`
- `backup_data`

### 2. 路径规划

建议路径结构：
```
/mnt/volumes/{volume_name}/{folder_name}
```

例如：
- `/mnt/volumes/data/media`
- `/mnt/volumes/data/public`
- `/mnt/volumes/backup/archives`

### 3. 协议选择

根据使用场景选择协议：
- **Windows 环境**：选择 `smb`
- **Linux 环境**：选择 `nfs`
- **macOS 环境**：选择 `afp` 或 `smb`
- **跨平台**：选择多个协议

### 4. 权限设置

- **只读共享**：`read_only=true`，适合归档数据
- **访客访问**：`guest_access=true`，适合公共数据
- **私有共享**：`read_only=false` + `guest_access=false`，适合敏感数据

### 5. 审计日志

所有创建操作都应该记录到审计日志中，包括：
- 创建时间
- 执行创建的管理员 ID
- 共享文件夹名称和路径
- 选择的协议列表
- 权限设置（read_only/guest_access）
