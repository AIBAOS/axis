# 磁盘详情 API

**Phase 74** - 存储管理 API 之获取单个磁盘详情接口

---

## 接口信息

- **端点:** `GET /api/v1/storage/disks/{id}`
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
| `id` | integer | 是 | 磁盘 ID |

### 请求示例

```bash
# 获取磁盘详情
curl -X GET "http://localhost:8080/api/v1/storage/disks/1" \
  -H "Authorization: Bearer <JWT_TOKEN>"
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "Disk 1",
    "path": "/dev/sda",
    "model": "WD Red Pro 4TB",
    "serial_number": "WD-WCC12345678",
    "disk_type": "hdd",
    "size_bytes": 4398046511104,
    "used_bytes": 2199023255552,
    "available_bytes": 2199023255552,
    "usage_percent": 50.0,
    "smart_status": "healthy",
    "speed_rpm": 7200,
    "health_status": "good",
    "in_storage_pool": true,
    "pool_name": "System Pool",
    "created_at": 1710489600,
    "updated_at": 1711440000
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

### 404 Not Found - 磁盘不存在

```json
{
  "success": false,
  "error": "Disk 999 not found",
  "code": "NOT_FOUND"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 任意登录用户可访问（无需 admin 权限）

---

## 响应字段说明

### DiskDetailResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `data` | object | 磁盘详情数据 |

### DiskInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 磁盘 ID |
| `name` | string | 磁盘名称（如 Disk 1） |
| `path` | string | 设备路径（如 /dev/sda） |
| `model` | string | 型号 |
| `serial_number` | string | 序列号 |
| `disk_type` | string | 类型：hdd/ssd/nvme |
| `size_bytes` | integer | 总容量（字节） |
| `used_bytes` | integer | 已用容量（字节） |
| `available_bytes` | integer | 可用容量（字节） |
| `usage_percent` | number | 使用率百分比 |
| `smart_status` | string | S.M.A.R.T. 状态：healthy/warning/failed |
| `speed_rpm` | integer\|null | 转速（RPM），SSD/NVMe 为 null |
| `health_status` | string | 健康状态：good/warning/bad |
| `in_storage_pool` | boolean | 是否已加入存储池 |
| `pool_name` | string\|null | 所属存储池名称（未加入为 null） |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

---

## 实现细节

- **文件位置:** `src/handlers/storage_disks_detail.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/storage/disks/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `GET /api/v1/storage/disks` - 磁盘列表（Phase 73）
- `GET /api/v1/storage/pools` - 存储池列表
- `GET /api/v1/storage/volumes` - 存储卷列表

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 74 初始实现 |
