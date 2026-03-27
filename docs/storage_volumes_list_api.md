# 存储卷列表 API 文档 (Phase 78)

## 概述

存储卷列表 API 提供系统中所有存储卷的信息列表。

## 接口详情

### GET /api/v1/storage/volumes

获取存储卷列表。

#### 认证要求

需要有效的 JWT Token，任意登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `page` | integer | 否 | 1 | 页码（从 1 开始） |
| `per_page` | integer | 否 | 20 | 每页数量（最大 100） |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "System Volume",
      "description": "System root volume",
      "pool_id": 1,
      "pool_name": "primary",
      "total_bytes": 268435456000,
      "used_bytes": 134217728000,
      "available_bytes": 134217728000,
      "usage_percent": 50.0,
      "status": "online",
      "filesystem": "ext4",
      "mount_point": "/",
      "created_at": 1710000000,
      "updated_at": 1711468800
    },
    {
      "id": 2,
      "name": "Data Volume",
      "description": "Data storage volume",
      "pool_id": 1,
      "pool_name": "primary",
      "total_bytes": 1099511627776,
      "used_bytes": 549755813888,
      "available_bytes": 549755813888,
      "usage_percent": 50.0,
      "status": "online",
      "filesystem": "ext4",
      "mount_point": "/data",
      "created_at": 1710000000,
      "updated_at": 1711468800
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 2,
    "total_pages": 1
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `data`: 存储卷列表
  - `id`: 卷 ID
  - `name`: 卷名称
  - `description`: 卷描述
  - `pool_id`: 所属存储池 ID
  - `pool_name`: 所属存储池名称
  - `total_bytes`: 总容量（字节）
  - `used_bytes`: 已用容量（字节）
  - `available_bytes`: 可用容量（字节）
  - `usage_percent`: 使用率（百分比）
  - `status`: 状态（online/offline/degraded）
  - `filesystem`: 文件系统类型（ext4/btrfs/xfs 等）
  - `mount_point`: 挂载点路径
  - `created_at`: 创建时间（Unix 时间戳）
  - `updated_at`: 更新时间（Unix 时间戳）
- `pagination`: 分页信息
  - `page`: 当前页码
  - `per_page`: 每页数量
  - `total`: 总记录数
  - `total_pages`: 总页数

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：获取存储卷列表（默认分页）

```bash
curl -X GET "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 2：分页查询

```bash
# 第 2 页，每页 10 条
curl -X GET "http://localhost:8080/api/v1/storage/volumes?page=2&per_page=10" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 3：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/storage/volumes"
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
- 任意登录用户可访问，不需要 admin 权限

## 存储卷状态说明

| 状态 | 说明 |
|------|------|
| `online` | 在线，正常运行 |
| `offline` | 离线，无法访问 |
| `degraded` | 降级，性能下降 |

## 文件系统类型说明

| 类型 | 说明 |
|------|------|
| `ext4` | 第四代扩展文件系统（默认） |
| `btrfs` | B-Tree 文件系统（支持快照） |
| `xfs` | 高性能日志文件系统 |
| `ntfs` | Windows NT 文件系统 |
| `exfat` | 扩展 FAT（适合大容量存储） |

## 实现文件

- `src/handlers/storage_volumes_list.rs` - 存储卷列表处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：任意登录用户可访问，不需要 admin 权限
2. **容量单位**：所有容量字段以字节为单位
3. **使用率计算**：`usage_percent = (used_bytes / total_bytes) * 100`
4. **分页限制**：per_page 最大值为 100
5. **空结果**：无数据时返回空数组

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **POST /api/v1/storage/volumes** - 创建存储卷 (Phase 67)
- **GET /api/v1/storage/volumes/{id}** - 获取存储卷详情 (Phase 61)
- **PUT /api/v1/storage/volumes/{id}** - 更新存储卷信息 (Phase 68)
- **DELETE /api/v1/storage/volumes/{id}** - 删除存储卷 (Phase 69)
- **GET /api/v1/storage/pools/{id}/volumes** - 获取存储池下的卷列表 (Phase 71)

## 响应示例（完整）

### 成功响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "System Volume",
      "description": "System root volume",
      "pool_id": 1,
      "pool_name": "primary",
      "total_bytes": 268435456000,
      "used_bytes": 134217728000,
      "available_bytes": 134217728000,
      "usage_percent": 50.0,
      "status": "online",
      "filesystem": "ext4",
      "mount_point": "/",
      "created_at": 1710000000,
      "updated_at": 1711468800
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 1,
    "total_pages": 1
  }
}
```

### 空结果

```json
{
  "success": true,
  "data": [],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 0,
    "total_pages": 0
  }
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

## 分页说明

- **页码从 1 开始**（page=1 表示第一页）
- **每页最大 100 条**（per_page 超过 100 会自动限制为 100）
- **总页数计算**：`total_pages = ceil(total / per_page)`
- **空结果**：当 total=0 时，total_pages=0

## 容量换算示例

| 字节 | 换算 |
|------|------|
| 268435456000 | 250 GB |
| 1099511627776 | 1 TB |
| 2199023255552 | 2 TB |

**换算公式：**
- GB = bytes / 1073741824
- TB = bytes / 1099511627776

## 挂载点说明

| 挂载点 | 说明 |
|--------|------|
| `/` | 根目录，系统卷 |
| `/data` | 数据目录 |
| `/backup` | 备份目录 |
| `/mnt/xxx` | 外部挂载点 |
