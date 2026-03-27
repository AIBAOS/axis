# Phase 80 - 创建存储卷 API 文档

**接口:** `POST /api/v1/storage/volumes`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

创建新的存储卷。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 仅 `admin` 角色可调用此接口

**功能特性:**
- 创建新的存储卷
- 验证存储池存在性和容量
- 检查名称唯一性
- 验证文件系统类型

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
  "name": "new_volume",
  "description": "New volume for user data",
  "pool_id": 1,
  "size_bytes": 1099511627776,
  "filesystem": "ext4"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 存储卷名称（必须唯一） |
| `description` | string | 否 | 描述信息 |
| `pool_id` | number | 是 | 所属存储池 ID |
| `size_bytes` | number | 是 | 卷大小（字节） |
| `filesystem` | string | 是 | 文件系统类型：`ext4` / `btrfs` / `xfs` / `zfs` |

---

## 📥 响应结果

### 201 Created

```json
{
  "success": true,
  "message": "Storage volume created successfully",
  "data": {
    "id": 101,
    "name": "new_volume",
    "description": "New volume for user data",
    "pool_id": 1,
    "total_bytes": 1099511627776,
    "used_bytes": 0,
    "available_bytes": 1099511627776,
    "usage_percent": 0.0,
    "status": "online",
    "filesystem": "ext4",
    "mount_point": "/mnt/volumes/new_volume",
    "created_at": 1774345600,
    "updated_at": 1774345600
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data.id` | number | 存储卷 ID（系统分配） |
| `data.name` | string | 存储卷名称 |
| `data.description` | string/null | 描述信息 |
| `data.pool_id` | number | 所属存储池 ID |
| `data.total_bytes` | number | 总容量（字节） |
| `data.used_bytes` | number | 已用容量（字节） |
| `data.available_bytes` | number | 可用容量（字节） |
| `data.usage_percent` | number | 使用率（%） |
| `data.status` | string | 状态：`online` / `offline` |
| `data.filesystem` | string | 文件系统类型 |
| `data.mount_point` | string | 挂载点 |
| `data.created_at` | number | 创建时间（Unix 时间戳） |
| `data.updated_at` | number | 更新时间（Unix 时间戳） |

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

**常见错误:**
- `name is required` - 缺少名称
- `size_bytes must be greater than 0` - 大小必须大于 0
- `Invalid filesystem 'xxx'. Valid types: ext4, btrfs, xfs, zfs` - 无效文件系统类型
- `Insufficient space in storage pool. Available: X bytes, Requested: Y bytes` - 存储池容量不足

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
  "error": "Only admin users can create storage volumes",
  "code": "FORBIDDEN"
}
```

### 404 Not Found

```json
{
  "success": false,
  "error": "Storage pool 999 not found",
  "code": "POOL_NOT_FOUND"
}
```

### 409 Conflict

```json
{
  "success": false,
  "error": "Storage volume name 'data' already exists",
  "code": "CONFLICT"
}
```

---

## 🧪 使用示例

```bash
# 创建 1TB ext4 存储卷
curl -X POST "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "new_volume",
    "description": "New volume for user data",
    "pool_id": 1,
    "size_bytes": 1099511627776,
    "filesystem": "ext4"
  }'
```

```bash
# 创建 2TB btrfs 存储卷
curl -X POST "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "media",
    "pool_id": 2,
    "size_bytes": 2199023255552,
    "filesystem": "btrfs"
  }'
```

---

## 🔒 安全说明

### 权限控制

- 仅 `admin` 角色可创建存储卷
- 非 admin 用户调用返回 403 Forbidden

### 输入验证

| 字段 | 验证规则 |
|------|----------|
| `name` | 必填，非空，全局唯一 |
| `pool_id` | 必填，存储池必须存在 |
| `size_bytes` | 必填，必须大于 0，不能超过存储池可用容量 |
| `filesystem` | 必填，仅限 `ext4` / `btrfs` / `xfs` / `zfs` |

### 名称唯一性

- 存储卷名称全局唯一
- 重复名称返回 409 Conflict

### 容量检查

- 创建前检查存储池可用容量
- 容量不足返回 400 Bad Request，错误码 `INSUFFICIENT_SPACE`

---

## 📝 注意事项

1. **权限要求**: 仅 admin 角色可创建，普通用户返回 403
2. **容量规划**: 确保存储池有足够的可用空间
3. **挂载点**: 系统自动生成挂载点 `/mnt/volumes/{name}`
4. **文件系统**: 支持 ext4/btrfs/xfs/zfs

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/storage/volumes` | 存储卷列表 (Phase 78) |
| `GET /api/v1/storage/volumes/{id}` | 存储卷详情 (Phase 79) |
| `PUT /api/v1/storage/volumes/{id}` | 更新存储卷 |
| `DELETE /api/v1/storage/volumes/{id}` | 删除存储卷 |
| `GET /api/v1/storage/pools` | 存储池列表 (Phase 75) |

---

*文档维护：兵部尚书*
