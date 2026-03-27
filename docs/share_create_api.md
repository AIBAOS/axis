# 创建共享文件夹 API (Phase 89)

## 接口说明

实现创建共享文件夹的接口，用于配置 SMB/NFS 等网络共享。仅 admin 角色可访问。

## 接口定义

```
POST /api/v1/shares
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |
| Content-Type | application/json | 是 | 请求体格式 |

## 请求体

```json
{
  "name": "Public",
  "volume_id": 2,
  "path": "/public",
  "description": "Public shared folder",
  "protocols": ["smb", "nfs"],
  "read_only": false,
  "guest_access": true
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 是 | 共享名称（最大 100 字符） |
| volume_id | integer | 是 | 所属存储卷 ID |
| path | string | 是 | 文件夹路径（相对于卷根目录，必须以 / 开头） |
| description | string | 否 | 共享描述 |
| protocols | array | 是 | 共享协议列表（至少一个：smb/nfs/afp/ftp/webdav） |
| read_only | boolean | 否 | 是否只读（默认 false） |
| guest_access | boolean | 否 | 是否允许访客访问（默认 false） |

## 响应格式

### 成功响应 (201 Created)

```json
{
  "success": true,
  "message": "Share created successfully",
  "data": {
    "id": 100,
    "name": "Public",
    "volume_id": 2,
    "volume_name": "Data Volume 1",
    "path": "/public",
    "description": "Public shared folder",
    "protocols": ["smb", "nfs"],
    "read_only": false,
    "guest_access": true,
    "enabled": true,
    "created_at": 1711500000,
    "created_by": "admin"
  }
}
```

### 未授权 (401 Unauthorized)

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 禁止访问 (403 Forbidden)

```json
{
  "success": false,
  "error": "Only admin users can create shares",
  "code": "FORBIDDEN"
}
```

### 存储卷不存在 (404 Not Found)

```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "NOT_FOUND"
}
```

### 共享名称已存在 (409 Conflict)

```json
{
  "success": false,
  "error": "Share name 'Public' already exists",
  "code": "CONFLICT"
}
```

### 路径格式错误 (400 Bad Request)

```json
{
  "success": false,
  "error": "Invalid path: Path must start with /",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Invalid path: Path cannot contain '..'",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Invalid path: Path cannot be root directory",
  "code": "INVALID_PARAMS"
}
```

### 协议无效 (400 Bad Request)

```json
{
  "success": false,
  "error": "At least one protocol is required",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Invalid protocol 'invalid'. Valid protocols: smb, nfs, afp, ftp, webdav",
  "code": "INVALID_PARAMS"
}
```

### 参数错误 (400 Bad Request)

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
  "error": "name must be less than 100 characters",
  "code": "INVALID_PARAMS"
}
```

## 使用示例

### cURL 示例

```bash
# 创建共享文件夹
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Public",
    "volume_id": 2,
    "path": "/public",
    "description": "Public shared folder",
    "protocols": ["smb", "nfs"],
    "read_only": false,
    "guest_access": true
  }'

# 创建只读共享
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Media",
    "volume_id": 2,
    "path": "/media",
    "protocols": ["smb"],
    "read_only": true
  }'

# 创建多个协议的共享
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Home",
    "volume_id": 1,
    "path": "/home",
    "protocols": ["smb", "nfs", "afp"],
    "guest_access": false
  }'

# 路径格式错误（返回 400）
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Invalid",
    "volume_id": 2,
    "path": "invalid/path",
    "protocols": ["smb"]
  }'

# 包含 .. 的路径（返回 400）
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Invalid",
    "volume_id": 2,
    "path": "/../etc",
    "protocols": ["smb"]
  }'

# 无效协议（返回 400）
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Invalid",
    "volume_id": 2,
    "path": "/invalid",
    "protocols": ["invalid_protocol"]
  }'

# 名称冲突（返回 409）
curl -X POST "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Public",
    "volume_id": 2,
    "path": "/public2",
    "protocols": ["smb"]
  }'
```

### JavaScript 示例

```javascript
// 创建共享文件夹
async function createShare(name, volumeId, path, protocols, description = '', readOnly = false, guestAccess = false) {
  const response = await fetch(
    'http://localhost:8080/api/v1/shares',
    {
      method: 'POST',
      headers: {
        'Authorization': 'Bearer ' + adminToken,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        name,
        volume_id: volumeId,
        path,
        description,
        protocols,
        read_only: readOnly,
        guest_access: guestAccess
      })
    }
  );
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error);
  }
  
  const data = await response.json();
  console.log('Share created:', data.data);
  return data.data;
}

// 使用示例
try {
  const share = await createShare(
    'Public',
    2,
    '/public',
    ['smb', 'nfs'],
    'Public shared folder',
    false,
    true
  );
  console.log(`Created share ${share.name} on ${share.volume_name}`);
  console.log(`Protocols: ${share.protocols.join(', ')}`);
} catch (e) {
  console.error('Creation failed:', e.message);
}
```

## 响应字段说明

### ShareInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 共享文件夹 ID |
| name | string | 共享名称 |
| volume_id | integer | 所属存储卷 ID |
| volume_name | string | 所属存储卷名称 |
| path | string | 文件夹路径（相对于卷根目录） |
| description | string | 共享描述 |
| protocols | array | 共享协议列表（smb/nfs/afp/ftp/webdav） |
| read_only | boolean | 是否只读 |
| guest_access | boolean | 是否允许访客访问 |
| enabled | boolean | 是否启用 |
| created_at | integer | 创建时间（Unix 时间戳） |
| created_by | string | 创建者用户名 |

## 验证规则

### 名称验证
- 不能为空
- 最大长度 100 字符
- 必须全局唯一

### 路径验证
- 必须以 / 开头
- 不能包含 ..
- 不能是根目录 /

### 协议验证
- 至少一个协议
- 有效协议：smb, nfs, afp, ftp, webdav

### 存储卷验证
- 存储卷必须存在

### 权限验证
- 仅 admin 角色可创建共享

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **路径安全**: 防止路径遍历攻击（禁止 ..）
4. **名称唯一性**: 共享名称必须全局唯一
5. **协议验证**: 只允许已知安全的协议

## 实现文件

- `src/handlers/shares_create.rs` - 创建共享处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟数据，后续将连接系统 API
2. 路径相对于存储卷根目录
3. 名称必须唯一，重复返回 409 Conflict
4. 时间戳使用 Unix 时间戳（秒级）
5. 路径不能是根目录 /

## 相关接口

- `GET /api/v1/shares` - 共享列表（待实现）
- `GET /api/v1/shares/{id}` - 共享详情（待实现）
- `PUT /api/v1/shares/{id}` - 更新共享（待实现）
- `DELETE /api/v1/shares/{id}` - 删除共享（待实现）
- `GET /api/v1/storage/volumes` - 存储卷列表（Phase 78）

## 协议说明

| 协议 | 说明 | 适用场景 |
|------|------|----------|
| smb | Windows 文件共享 | Windows 网络 |
| nfs | Linux/Unix 文件共享 | Linux/Unix 网络 |
| afp | Apple 文件共享 | macOS 网络 |
| ftp | FTP 文件传输 | 通用文件传输 |
| webdav | WebDAV 文件共享 | Web 浏览器访问 |
