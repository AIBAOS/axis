# Phase 76 - 存储池详情 API 文档

**接口:** `GET /api/v1/storage/pools/{id}`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

获取单个存储池的详细信息，包含关联的磁盘和存储卷列表。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 任意登录用户可访问

**功能特性:**
- 返回存储池完整信息
- 包含关联的磁盘列表（disks 数组）
- 包含关联的存储卷列表（volumes 数组）
- 验证存储池 ID 存在
- 存储池不存在返回 404 Not Found

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

### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | number | 是 | 存储池 ID |

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "primary",
    "description": "Primary storage pool for system data",
    "total_bytes": 3298534883328,
    "used_bytes": 1649267441664,
    "available_bytes": 1649267441664,
    "usage_percent": 50.0,
    "disk_count": 2,
    "disks": [
      {
        "disk_id": 1,
        "name": "System Disk",
        "device_path": "/dev/sda1",
        "capacity_bytes": 1649267441664,
        "status": "online"
      },
      {
        "disk_id": 2,
        "name": "Data Disk 1",
        "device_path": "/dev/sdb1",
        "capacity_bytes": 1649267441664,
        "status": "online"
      }
    ],
    "volume_count": 2,
    "volumes": [
      {
        "volume_id": 1,
        "name": "root",
        "size_bytes": 1099511627776,
        "used_bytes": 549755813888,
        "status": "online"
      },
      {
        "volume_id": 2,
        "name": "data",
        "size_bytes": 2199023255552,
        "used_bytes": 1099511627776,
        "status": "online"
      }
    ],
    "status": "online",
    "created_at": 1710000000,
    "updated_at": 1774345600
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | object | 存储池详情 |
| `data.id` | number | 存储池 ID |
| `data.name` | string | 存储池名称 |
| `data.description` | string/null | 描述信息 |
| `data.total_bytes` | number | 总容量（字节） |
| `data.used_bytes` | number | 已用容量（字节） |
| `data.available_bytes` | number | 可用容量（字节） |
| `data.usage_percent` | number | 使用率（%） |
| `data.disk_count` | number | 磁盘数量 |
| `data.disks` | array | 磁盘列表 |
| `data.disks[].disk_id` | number | 磁盘 ID |
| `data.disks[].name` | string | 磁盘名称 |
| `data.disks[].device_path` | string | 设备路径 |
| `data.disks[].capacity_bytes` | number | 磁盘容量（字节） |
| `data.disks[].status` | string | 磁盘状态 |
| `data.volume_count` | number | 存储卷数量 |
| `data.volumes` | array | 存储卷列表 |
| `data.volumes[].volume_id` | number | 存储卷 ID |
| `data.volumes[].name` | string | 存储卷名称 |
| `data.volumes[].size_bytes` | number | 存储卷容量（字节） |
| `data.volumes[].used_bytes` | number | 存储卷已用容量（字节） |
| `data.volumes[].status` | string | 存储卷状态 |
| `data.status` | string | 存储池状态：`online` / `degraded` / `offline` |
| `data.created_at` | number | 创建时间（Unix 时间戳） |
| `data.updated_at` | number | 更新时间（Unix 时间戳） |

---

## ❌ 错误响应

### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 404 Not Found

```json
{
  "success": false,
  "error": "Storage pool 999 not found",
  "code": "NOT_FOUND"
}
```

---

## 🧪 使用示例

```bash
# 获取存储池 ID 为 1 的详情
curl -X GET "http://localhost:8080/api/v1/storage/pools/1" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 获取不存在的存储池（返回 404）
curl -X GET "http://localhost:8080/api/v1/storage/pools/999" \
  -H "Authorization: Bearer <access_token>"
```

---

## 📝 注意事项

1. **权限要求**: 任意登录用户可访问，无需 admin 权限
2. **存储池不存在**: 返回 404 Not Found
3. **容量单位**: 所有容量字段单位为字节（bytes）
4. **disks 数组**: 包含该存储池关联的所有磁盘信息
5. **volumes 数组**: 包含该存储池关联的所有存储卷信息

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/storage/pools` | 存储池列表 (Phase 75) |
| `POST /api/v1/storage/pools` | 创建存储池 |
| `PUT /api/v1/storage/pools/{id}` | 更新存储池 |
| `DELETE /api/v1/storage/pools/{id}` | 删除存储池 |
| `GET /api/v1/storage/pools/{id}/volumes` | 存储池下的卷列表 (Phase 71) |
| `GET /api/v1/storage/disks` | 磁盘列表 (Phase 73) |
| `GET /api/v1/storage/volumes` | 存储卷列表 (Phase 70) |

---

*文档维护：兵部尚书*
