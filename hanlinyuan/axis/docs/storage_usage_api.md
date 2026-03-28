# 存储使用统计 API

**Phase 77** - 存储管理 API 之全局存储使用统计接口

---

## 接口信息

- **端点:** `GET /api/v1/storage/usage`
- **认证:** 需要 JWT Bearer Token（任意登录用户）
- **权限:** 所有已认证用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |

### 请求示例

```bash
curl -X GET "http://localhost:8080/api/v1/storage/usage" \
  -H "Authorization: Bearer <JWT_TOKEN>"
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "data": {
    "total_bytes": 36500000000000,
    "used_bytes": 16650000000000,
    "available_bytes": 19850000000000,
    "usage_percent": 45.6,
    "pool_count": 4,
    "volume_count": 4,
    "disk_count": 13,
    "health_status": "degraded"
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

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 任意登录用户可访问（无需 admin 权限）

---

## 响应字段说明

### StorageUsageResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `data` | object | 存储使用统计数据 |

### StorageUsage

| 字段 | 类型 | 说明 |
|------|------|------|
| `total_bytes` | integer | 总容量（字节） |
| `used_bytes` | integer | 已用容量（字节） |
| `available_bytes` | integer | 可用容量（字节） |
| `usage_percent` | number | 使用率百分比 |
| `pool_count` | integer | 存储池数量 |
| `volume_count` | integer | 存储卷数量 |
| `disk_count` | integer | 磁盘数量 |
| `health_status` | string | 整体健康状态：healthy/degraded/critical |

---

## 健康状态说明

| 状态 | 说明 |
|------|------|
| `healthy` | 所有存储池健康 |
| `degraded` | 部分存储池降级（如 RAID 中有磁盘故障） |
| `critical` | 有存储池严重故障（如 RAID 失效） |

---

## 实现细节

- **文件位置:** `src/handlers/storage_usage.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/storage/usage`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `GET /api/v1/storage/disks` - 磁盘列表
- `GET /api/v1/storage/pools` - 存储池列表
- `GET /api/v1/storage/volumes` - 存储卷列表

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 77 初始实现 |
