# 存储卷创建 API (Phase 80)

## 接口说明

实现创建存储卷的接口。仅 admin 角色可访问。

## 接口定义

```
POST /api/v1/storage/volumes
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |
| Content-Type | application/json | 是 | 请求体格式 |

## 请求体

```json
{
  "name": "New Volume",
  "description": "New storage volume",
  "pool_id": 2,
  "size_bytes": 1099511627776,
  "filesystem": "ext4",
  "mount_point": "/mnt/newvolume"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 是 | 存储卷名称（最大 100 字符） |
| description | string | 否 | 存储卷描述 |
| pool_id | integer | 是 | 所属存储池 ID |
| size_bytes | integer | 是 | 卷容量（字节） |
| filesystem | string | 否 | 文件系统类型（ext4/xfs/btrfs/zfs，默认 ext4） |
| mount_point | string | 否 | 挂载点（默认 /mnt/{name}） |

## 响应格式

### 成功响应 (201 Created)

```json
{
  "success": true,
  "message": "Storage volume created successfully",
  "data": {
    "id": 100,
    "name": "New Volume",
    "description": "New storage volume",
    "pool_id": 2,
    "pool_name": "Data Pool",
    "total_bytes": 1099511627776,
    "used_bytes": 0,
    "available_bytes": 1099511627776,
    "usage_percent": 0.0,
    "status": "active",
    "filesystem": "ext4",
    "mount_point": "/mnt/newvolume",
    "created_at": 1711500000,
    "updated_at": 1711500000
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
  "error": "Only admin users can create storage volumes",
  "code": "FORBIDDEN"
}
```

### 存储池不存在 (404 Not Found)

```json
{
  "success": false,
  "error": "Storage pool 999 not found",
  "code": "NOT_FOUND"
}
```

### 名称已存在 (409 Conflict)

```json
{
  "success": false,
  "error": "Storage volume 'Data Volume 1' already exists",
  "code": "CONFLICT"
}
```

### 容量超限 (400 Bad Request)

```json
{
  "success": false,
  "error": "Requested size exceeds pool available space (2400000000000 bytes)",
  "code": "INVALID_PARAMS"
}
```

### 文件系统类型无效 (400 Bad Request)

```json
{
  "success": false,
  "error": "Invalid filesystem. Valid values: ext4, xfs, btrfs, zfs",
  "code": "INVALID_PARAMS"
}
```

## 使用示例

### cURL 示例

```bash
# 创建存储卷（默认文件系统）
curl -X POST "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Volume",
    "pool_id": 2,
    "size_bytes": 1099511627776
  }'

# 创建 xfs 文件系统卷
curl -X POST "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "XFS Volume",
    "pool_id": 2,
    "size_bytes": 549755813888,
    "filesystem": "xfs",
    "mount_point": "/mnt/xfs_volume"
  }'

# 容量超限（返回 400）
curl -X POST "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Huge Volume",
    "pool_id": 2,
    "size_bytes": 9999999999999999
  }'
```

### JavaScript 示例

```javascript
// 创建存储卷
async function createVolume(name, poolId, sizeBytes, filesystem = 'ext4') {
  const response = await fetch(
    'http://localhost:8080/api/v1/storage/volumes',
    {
      method: 'POST',
      headers: {
        'Authorization': 'Bearer ' + adminToken,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        name,
        pool_id: poolId,
        size_bytes: sizeBytes,
        filesystem
      })
    }
  );
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error);
  }
  
  const data = await response.json();
  console.log('Volume created:', data.data);
  return data.data;
}

// 使用示例
try {
  const volume = await createVolume('New Volume', 2, 1099511627776);
  console.log(`Created ${volume.name} at ${volume.mount_point}`);
} catch (e) {
  console.error('Creation failed:', e.message);
}
```

## 响应字段说明

### VolumeInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 存储卷 ID |
| name | string | 存储卷名称 |
| description | string | 存储卷描述 |
| pool_id | integer | 所属存储池 ID |
| pool_name | string | 所属存储池名称 |
| total_bytes | integer | 总容量（字节） |
| used_bytes | integer | 已使用容量（字节） |
| available_bytes | integer | 可用容量（字节） |
| usage_percent | float | 使用率（百分比） |
| status | string | 状态（active/inactive） |
| filesystem | string | 文件系统类型 |
| mount_point | string | 挂载点 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |

## 验证规则

### 名称验证
- 不能为空
- 最大长度 100 字符
- 必须全局唯一

### 容量验证
- 必须大于 0
- 不能超过存储池可用空间

### 文件系统类型
- 有效值：ext4, xfs, btrfs, zfs
- 默认值：ext4

### 存储池验证
- 必须存在
- 必须有足够可用空间

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **参数验证**: 严格的输入验证
4. **名称唯一性**: 存储卷名称必须全局唯一
5. **容量保护**: 禁止创建超过池可用空间的卷

## 实现文件

- `src/handlers/storage_volumes_create.rs` - 存储卷创建处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟数据，后续将连接系统 API
2. 名称最大长度为 100 字符
3. 容量以字节为单位
4. 挂载点默认为 /mnt/{name}
5. 名称必须唯一，重复返回 409 Conflict

## 相关接口

- `GET /api/v1/storage/volumes` - 存储卷列表（Phase 78）
- `GET /api/v1/storage/volumes/{id}` - 存储卷详情（Phase 79）
- `PUT /api/v1/storage/volumes/{id}` - 更新存储卷（Phase 68）
- `DELETE /api/v1/storage/volumes/{id}` - 删除存储卷（Phase 69）
- `GET /api/v1/storage/pools` - 存储池列表（Phase 75）
