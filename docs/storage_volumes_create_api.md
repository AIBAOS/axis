# 存储卷创建 API

## Phase 189

## 接口说明

创建新的存储卷。

## 请求

`POST /api/v1/storage/volumes`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

```json
{
  "name": "New Volume",
  "size_bytes": 1099511627776,
  "filesystem_type": "ext4",
  "mount_point": "/mnt/new_volume",
  "description": "New storage volume"
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 是 | 卷名称（唯一） |
| size_bytes | number | 是 | 卷大小（字节） |
| filesystem_type | string | 否 | 文件系统类型（ext4/xfs/btrfs/zfs，默认 ext4） |
| mount_point | string | 否 | 挂载点（默认 /mnt/{name}） |
| description | string | 否 | 描述信息 |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "Storage volume created successfully",
  "data": {
    "id": 4,
    "name": "New Volume",
    "total_bytes": 1099511627776,
    "used_bytes": 0,
    "available_bytes": 1099511627776,
    "usage_percent": 0.0,
    "status": "active",
    "filesystem_type": "ext4",
    "mount_point": "/mnt/new_volume",
    "created_at": "2026-03-27T17:00:00Z",
    "updated_at": "2026-03-27T17:00:00Z"
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid filesystem type. Valid types: ext4, xfs, btrfs, zfs",
  "code": "INVALID_FILESYSTEM"
}
```

```json
{
  "success": false,
  "error": "Invalid mount point format. Must start with '/' and be <= 256 chars",
  "code": "INVALID_MOUNT_POINT"
}
```

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can create storage volumes",
  "code": "FORBIDDEN"
}
```

#### 409 Conflict - 卷名称已存在

```json
{
  "success": false,
  "error": "Storage volume 'Data Volume' already exists",
  "code": "VOLUME_EXISTS"
}
```

## 示例

### 创建存储卷

```bash
curl -X POST "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Volume",
    "size_bytes": 1099511627776,
    "filesystem_type": "ext4",
    "mount_point": "/mnt/new_volume",
    "description": "New storage volume"
  }'
```

### 创建已存在的卷

```bash
curl -X POST "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Data Volume",
    "size_bytes": 1099511627776
  }'
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "Storage volume 'Data Volume' already exists",
  "code": "VOLUME_EXISTS"
}
```

### 无效的文件系统类型

```bash
curl -X POST "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Volume",
    "size_bytes": 1099511627776,
    "filesystem_type": "ntfs"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid filesystem type. Valid types: ext4, xfs, btrfs, zfs",
  "code": "INVALID_FILESYSTEM"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 存储卷字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 卷 ID |
| name | string | 卷名称 |
| total_bytes | u64 | 总容量（字节） |
| used_bytes | u64 | 已用容量（字节） |
| available_bytes | u64 | 可用容量（字节） |
| usage_percent | number | 使用率百分比 |
| status | string | 状态（active/inactive） |
| filesystem_type | string | 文件系统类型（ext4/xfs/btrfs/zfs） |
| mount_point | string | 挂载点 |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证文件系统类型（ext4/xfs/btrfs/zfs）
4. 验证挂载点格式（必须以/开头，最大 256 字符）
5. 验证卷名称唯一性
6. 创建新卷
7. 返回 201 Created + 卷详情

## 版本历史

- **Phase 189** (2026-03-27): 存储管理模块 - 存储卷创建 API
