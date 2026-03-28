# 创建存储池 API 文档 (Phase 64)

## 概述

创建存储池 API 允许管理员创建新的存储池，支持多种 RAID 类型。

## 接口详情

### POST /api/v1/storage/pools

创建新的存储池。

#### 认证要求

需要有效的 JWT Token，且用户必须具有 `admin` 角色。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
Content-Type: application/json
```

#### 请求体

```json
{
  "name": "media_pool",
  "type": "raid5",
  "disk_ids": [1, 2, 3, 4],
  "filesystem": "ext4"
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 存储池名称 |
| `type` | string | 是 | 池类型：basic/raid0/raid1/raid5/raid6/raid10 |
| `disk_ids` | array | 是 | 磁盘 ID 列表 |
| `filesystem` | string | 否 | 文件系统类型（默认 ext4） |

**类型与磁盘数量要求：**

| 类型 | 最少磁盘数 | 说明 |
|------|-----------|------|
| `basic` | 1 | 基本卷（单磁盘） |
| `raid0` | 2 | RAID 0（条带化，无冗余） |
| `raid1` | 2 | RAID 1（镜像） |
| `raid5` | 3 | RAID 5（分布式奇偶校验） |
| `raid6` | 4 | RAID 6（双奇偶校验） |
| `raid10` | 4 | RAID 10（镜像 + 条带化） |

#### 响应格式

**成功响应 (201 Created)**

```json
{
  "success": true,
  "message": "Storage pool 'media_pool' created",
  "data": {
    "id": 3,
    "name": "media_pool",
    "type": "raid5",
    "status": "online",
    "total_bytes": 8796093022208,
    "used_bytes": 0,
    "available_bytes": 8796093022208,
    "usage_percent": 0.0,
    "disk_count": 4,
    "disks": [
      {
        "disk_id": 1,
        "name": "Disk 1",
        "capacity_bytes": 2199023255552,
        "status": "online"
      },
      {
        "disk_id": 2,
        "name": "Disk 2",
        "capacity_bytes": 2199023255552,
        "status": "online"
      },
      {
        "disk_id": 3,
        "name": "Disk 3",
        "capacity_bytes": 2199023255552,
        "status": "online"
      },
      {
        "disk_id": 4,
        "name": "Disk 4",
        "capacity_bytes": 2199023255552,
        "status": "online"
      }
    ],
    "filesystem": "ext4",
    "created_at": 1711468800,
    "updated_at": 1711468800
  }
}
```

**错误响应 (400 Bad Request) - 参数无效**

```json
{
  "success": false,
  "error": "Invalid pool type. Valid types: basic, raid0, raid1, raid5, raid6, raid10",
  "code": "INVALID_TYPE"
}
```

**错误响应 (400 Bad Request) - 磁盘数量不足**

```json
{
  "success": false,
  "error": "RAID5 requires at least 3 disks",
  "code": "INSUFFICIENT_DISKS"
}
```

**错误响应 (400 Bad Request) - 磁盘已被使用**

```json
{
  "success": false,
  "error": "Disk 1 is already in use by another pool",
  "code": "DISK_IN_USE"
}
```

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "error": "Only admin users can create storage pools",
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

### 示例 1：创建 RAID1 存储池

```bash
curl -X POST "http://localhost:8080/api/v1/storage/pools" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "backup_pool",
    "type": "raid1",
    "disk_ids": [1, 2],
    "filesystem": "ext4"
  }'
```

**响应：**
```json
{
  "success": true,
  "message": "Storage pool 'backup_pool' created",
  "data": {
    "id": 3,
    "name": "backup_pool",
    "type": "raid1",
    "status": "online",
    "total_bytes": 2199023255552,
    "used_bytes": 0,
    "available_bytes": 2199023255552,
    "usage_percent": 0.0,
    "disk_count": 2,
    "disks": [
      {
        "disk_id": 1,
        "name": "Disk 1",
        "capacity_bytes": 2199023255552,
        "status": "online"
      },
      {
        "disk_id": 2,
        "name": "Disk 2",
        "capacity_bytes": 2199023255552,
        "status": "online"
      }
    ],
    "filesystem": "ext4",
    "created_at": 1711468800,
    "updated_at": 1711468800
  }
}
```

### 示例 2：创建 RAID5 存储池

```bash
curl -X POST "http://localhost:8080/api/v1/storage/pools" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "media_pool",
    "type": "raid5",
    "disk_ids": [3, 4, 5, 6],
    "filesystem": "btrfs"
  }'
```

### 示例 3：创建基本存储池

```bash
curl -X POST "http://localhost:8080/api/v1/storage/pools" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "single_disk",
    "type": "basic",
    "disk_ids": [7],
    "filesystem": "ext4"
  }'
```

### 示例 4：无效类型（400）

```bash
curl -X POST "http://localhost:8080/api/v1/storage/pools" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "invalid_pool",
    "type": "invalid_type",
    "disk_ids": [1, 2]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Invalid pool type. Valid types: basic, raid0, raid1, raid5, raid6, raid10",
  "code": "INVALID_TYPE"
}
```

### 示例 5：磁盘数量不足（400）

```bash
curl -X POST "http://localhost:8080/api/v1/storage/pools" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "raid5_pool",
    "type": "raid5",
    "disk_ids": [1, 2]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "RAID5 requires at least 3 disks",
  "code": "INSUFFICIENT_DISKS"
}
```

### 示例 6：非 admin 用户访问（403）

```bash
curl -X POST "http://localhost:8080/api/v1/storage/pools" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "pool",
    "type": "basic",
    "disk_ids": [1]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Only admin users can create storage pools",
  "code": "FORBIDDEN"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可创建存储池
- 非 admin 角色返回 403 Forbidden

### 3. 输入验证

- **类型验证**：必须是有效的 RAID 类型
- **磁盘数量验证**：必须满足 RAID 类型的最低磁盘数要求
- **磁盘可用性验证**：磁盘不能被其他池使用

## 文件系统类型说明

| 类型 | 说明 |
|------|------|
| `ext4` | 第四代扩展文件系统（默认） |
| `btrfs` | B-Tree 文件系统（支持快照） |
| `xfs` | 高性能日志文件系统 |
| `ntfs` | Windows NT 文件系统 |
| `exfat` | 扩展 FAT（适合大容量存储） |

## 实现文件

- `src/handlers/storage.rs` - 存储池创建处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **权限控制**：仅 admin 角色可创建存储池
2. **RAID 类型限制**：不同类型有不同磁盘数量要求
3. **磁盘独占**：一个磁盘只能属于一个存储池
4. **容量计算**：RAID 会占用部分容量用于冗余（RAID5/6）

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `INVALID_TYPE` | 400 | 存储池类型无效 |
| `INSUFFICIENT_DISKS` | 400 | 磁盘数量不足 |
| `DISK_IN_USE` | 400 | 磁盘已被使用 |
| `INVALID_FILESYSTEM` | 400 | 文件系统类型无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/storage/pools** - 获取存储池列表 (Phase 62)
- **GET /api/v1/storage/pools/{id}** - 获取存储池详情 (Phase 63)
- **DELETE /api/v1/storage/pools/{id}** - 删除存储池
- **GET /api/v1/storage/disks** - 获取磁盘列表

## 响应示例（完整）

```json
{
  "success": true,
  "message": "Storage pool 'media_pool' created",
  "data": {
    "id": 3,
    "name": "media_pool",
    "type": "raid5",
    "status": "online",
    "total_bytes": 8796093022208,
    "used_bytes": 0,
    "available_bytes": 8796093022208,
    "usage_percent": 0.0,
    "disk_count": 4,
    "disks": [
      {
        "disk_id": 1,
        "name": "Disk 1",
        "capacity_bytes": 2199023255552,
        "status": "online"
      },
      {
        "disk_id": 2,
        "name": "Disk 2",
        "capacity_bytes": 2199023255552,
        "status": "online"
      },
      {
        "disk_id": 3,
        "name": "Disk 3",
        "capacity_bytes": 2199023255552,
        "status": "online"
      },
      {
        "disk_id": 4,
        "name": "Disk 4",
        "capacity_bytes": 2199023255552,
        "status": "online"
      }
    ],
    "filesystem": "ext4",
    "created_at": 1711468800,
    "updated_at": 1711468800
  }
}
```

## RAID 容量计算

| RAID 类型 | 可用容量公式 | 示例（4x4TB 磁盘） |
|----------|------------|------------------|
| `basic` | N × 单盘容量 | 1 × 4TB = 4TB |
| `raid0` | N × 单盘容量 | 4 × 4TB = 16TB |
| `raid1` | (N/2) × 单盘容量 | (4/2) × 4TB = 8TB |
| `raid5` | (N-1) × 单盘容量 | (4-1) × 4TB = 12TB |
| `raid6` | (N-2) × 单盘容量 | (4-2) × 4TB = 8TB |
| `raid10` | (N/2) × 单盘容量 | (4/2) × 4TB = 8TB |

**N = 磁盘数量**
