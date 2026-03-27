# 存储池卷列表 API

**Phase 71** - 存储管理 API 之获取存储池下的卷列表接口

---

## 接口信息

- **端点:** `GET /api/v1/storage/pools/{id}/volumes`
- **认证:** 需要 JWT Bearer Token（任意登录用户）
- **权限:** 所有已认证用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 存储池 ID |

### 查询参数

| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `page` | integer | 否 | 1 | 页码（从 1 开始） |
| `limit` | integer | 否 | 20 | 每页数量（最大 100） |

### 请求示例

```bash
# 获取存储池下的卷列表
curl -X GET "http://localhost:8080/api/v1/storage/pools/2/volumes" \
  -H "Authorization: Bearer <JWT_TOKEN>"

# 带分页参数
curl -X GET "http://localhost:8080/api/v1/storage/pools/2/volumes?page=1&limit=10" \
  -H "Authorization: Bearer <JWT_TOKEN>"
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "pool_id": 2,
  "pool_name": "Data Pool",
  "data": [
    {
      "id": 2,
      "name": "Data Volume",
      "pool_id": 2,
      "pool_name": "Data Pool",
      "size_bytes": 1099511627776,
      "used_bytes": 549755813888,
      "available_bytes": 549755813888,
      "usage_percent": 50.0,
      "filesystem_type": "ext4",
      "status": "online",
      "mount_point": "/mnt/data_volume",
      "created_at": 1710489600,
      "updated_at": 1711440000
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 1,
    "total_pages": 1
  }
}
```

### 401 Unauthorized - 未认证

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

或

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

### 404 Not Found - 存储池不存在

```json
{
  "success": false,
  "error": "Storage pool 999 not found",
  "code": "NOT_FOUND"
}
```

### 200 OK - 空列表（存储池下无卷）

```json
{
  "success": true,
  "pool_id": 3,
  "pool_name": "Backup Pool",
  "data": [],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 0,
    "total_pages": 0
  }
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 任意登录用户可访问（无需 admin 权限）

---

## 响应字段说明

### StoragePoolVolumesResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `pool_id` | integer | 存储池 ID |
| `pool_name` | string | 存储池名称 |
| `data` | array | 存储卷列表 |
| `pagination` | object | 分页信息 |

### StorageVolumeInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 存储卷 ID |
| `name` | string | 存储卷名称 |
| `pool_id` | integer | 所属存储池 ID |
| `pool_name` | string | 所属存储池名称 |
| `size_bytes` | integer | 卷大小（字节） |
| `used_bytes` | integer | 已用容量（字节） |
| `available_bytes` | integer | 可用容量（字节） |
| `usage_percent` | number | 使用率百分比 |
| `filesystem_type` | string | 文件系统类型 |
| `status` | string | 状态：online/offline |
| `mount_point` | string | 挂载点 |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

### PaginationMeta

| 字段 | 类型 | 说明 |
|------|------|------|
| `page` | integer | 当前页码 |
| `limit` | integer | 每页数量 |
| `total` | integer | 总记录数 |
| `total_pages` | integer | 总页数 |

---

## 实现细节

- **文件位置:** `src/handlers/storage_pools_volumes_list.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/storage/pools/{id}/volumes`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `GET /api/v1/storage/pools` - 存储池列表（Phase 62）
- `GET /api/v1/storage/pools/{id}` - 存储池详情（Phase 63）
- `GET /api/v1/storage/volumes` - 存储卷列表（Phase 70）
- `GET /api/v1/storage/volumes/{id}` - 存储卷详情（Phase 61）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 71 初始实现 |
