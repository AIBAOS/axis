# 更新存储池 API 文档 (Phase 65)

## 概述

更新存储池 API 允许管理员修改存储池的配置信息。

## 接口详情

### PUT /api/v1/storage/pools/{id}

更新指定存储池的信息。

#### 认证要求

需要有效的 JWT Token，且用户必须具有 `admin` 角色。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
Content-Type: application/json
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 存储池 ID |

#### 请求体

```json
{
  "name": "backup_pool_updated",
  "type": "raid1",
  "status": "online"
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 存储池名称（最大 100 字符） |
| `type` | string | 否 | 池类型：basic/raid0/raid1/raid5/raid6/raid10 |
| `status` | string | 否 | 池状态：online/degraded/offline |

**注意：** 所有字段均为可选，仅提供需要更新的字段。

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "message": "存储池信息更新成功",
  "data": {
    "id": 1,
    "name": "backup_pool_updated",
    "type": "raid1",
    "status": "online",
    "total_bytes": 2199023255552,
    "used_bytes": 1099511627776,
    "available_bytes": 1099511627776,
    "usage_percent": 50.0,
    "disk_count": 2,
    "updated_at": 1711468800
  }
}
```

**错误响应 (404 Not Found) - 池不存在**

```json
{
  "success": false,
  "message": "存储池 999 不存在",
  "code": "NOT_FOUND"
}
```

**错误响应 (400 Bad Request) - 无效类型**

```json
{
  "success": false,
  "message": "无效的存储池类型。有效类型：basic, raid0, raid1, raid5, raid6, raid10",
  "code": "INVALID_TYPE"
}
```

**错误响应 (400 Bad Request) - 无效状态**

```json
{
  "success": false,
  "message": "无效的存储池状态。有效状态：online, degraded, offline",
  "code": "INVALID_STATUS"
}
```

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "message": "仅管理员可更新存储池信息",
  "code": "FORBIDDEN"
}
```

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "message": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：更新存储池名称

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/pools/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "primary_pool_updated"
  }'
```

**响应：**
```json
{
  "success": true,
  "message": "存储池信息更新成功",
  "data": {
    "id": 1,
    "name": "primary_pool_updated",
    "type": "raid1",
    "status": "online",
    "total_bytes": 3298534883328,
    "used_bytes": 1649267441664,
    "available_bytes": 1649267441664,
    "usage_percent": 50.0,
    "disk_count": 2,
    "updated_at": 1711468800
  }
}
```

### 示例 2：更新存储池状态

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/pools/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "degraded"
  }'
```

### 示例 3：更新多个字段

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/pools/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "backup_pool",
    "type": "raid1",
    "status": "online"
  }'
```

### 示例 4：更新不存在的存储池（404）

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/pools/999" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "invalid_pool"
  }'
```

**响应：**
```json
{
  "success": false,
  "message": "存储池 999 不存在",
  "code": "NOT_FOUND"
}
```

### 示例 5：无效类型（400）

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/pools/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "type": "invalid_type"
  }'
```

**响应：**
```json
{
  "success": false,
  "message": "无效的存储池类型。有效类型：basic, raid0, raid1, raid5, raid6, raid10",
  "code": "INVALID_TYPE"
}
```

### 示例 6：非 admin 用户访问（403）

```bash
curl -X PUT "http://localhost:8080/api/v1/storage/pools/1" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "updated_name"
  }'
```

**响应：**
```json
{
  "success": false,
  "message": "仅管理员可更新存储池信息",
  "code": "FORBIDDEN"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可更新存储池信息
- 非 admin 角色返回 403 Forbidden

### 3. 输入验证

- **名称验证**：最大 100 字符
- **类型验证**：必须是有效的 RAID 类型
- **状态验证**：必须是 online/degraded/offline

## 存储池类型说明

| 类型 | 说明 |
|------|------|
| `basic` | 基本卷（单磁盘） |
| `raid0` | RAID 0（条带化，无冗余） |
| `raid1` | RAID 1（镜像） |
| `raid5` | RAID 5（分布式奇偶校验） |
| `raid6` | RAID 6（双奇偶校验） |
| `raid10` | RAID 10（镜像 + 条带化） |

## 存储池状态说明

| 状态 | 说明 |
|------|------|
| `online` | 在线，正常运行 |
| `degraded` | 降级，部分磁盘异常 |
| `offline` | 离线，无法访问 |

## 实现文件

- `src/handlers/storage_pools_update.rs` - 存储池更新处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **权限控制**：仅 admin 角色可更新存储池信息
2. **部分更新**：仅提供需要更新的字段，未提供字段保持原值
3. **ID 验证**：存储池 ID 必须存在，否则返回 404
4. **类型/状态验证**：必须是预定义的有效值
5. **类型变更警告**：更改 RAID 类型可能涉及数据迁移，生产环境需谨慎

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `NOT_FOUND` | 404 | 存储池不存在 |
| `INVALID_NAME` | 400 | 名称格式无效 |
| `INVALID_TYPE` | 400 | 存储池类型无效 |
| `INVALID_STATUS` | 400 | 存储池状态无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/storage/pools** - 获取存储池列表 (Phase 62)
- **POST /api/v1/storage/pools** - 创建存储池 (Phase 64)
- **GET /api/v1/storage/pools/{id}** - 获取存储池详情 (Phase 63)
- **DELETE /api/v1/storage/pools/{id}** - 删除存储池

## 响应示例（完整）

```json
{
  "success": true,
  "message": "存储池信息更新成功",
  "data": {
    "id": 1,
    "name": "backup_pool_updated",
    "type": "raid1",
    "status": "online",
    "total_bytes": 2199023255552,
    "used_bytes": 1099511627776,
    "available_bytes": 1099511627776,
    "usage_percent": 50.0,
    "disk_count": 2,
    "updated_at": 1711468800
  }
}
```
