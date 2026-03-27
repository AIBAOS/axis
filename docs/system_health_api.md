# Phase 59 - 系统健康检查 API 文档

**接口:** `GET /api/v1/system/health`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

获取系统健康状态，供 Web UI 和监控系统使用。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 任意登录用户可访问

**功能特性:**
- 返回 CPU、内存、磁盘使用率
- 返回系统运行时间
- 返回核心服务状态（database/cache/storage/network）
- 健康状态评估（healthy/degraded/critical）

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

---

## 📥 响应结果

### 200 OK

```json
{
  "status": "healthy",
  "checked_at": "2026-03-26T06:30:00Z",
  "uptime_seconds": 86400,
  "cpu_usage_percent": 25.5,
  "memory_usage_percent": 37.5,
  "disk_usage_percent": 50.0,
  "services": [
    {
      "name": "database",
      "status": "healthy",
      "message": null
    },
    {
      "name": "cache",
      "status": "healthy",
      "message": null
    },
    {
      "name": "storage",
      "status": "healthy",
      "message": null
    },
    {
      "name": "network",
      "status": "healthy",
      "message": null
    }
  ],
  "alerts": []
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `status` | string | 健康状态：`healthy` / `degraded` / `critical` |
| `checked_at` | string | 检查时间（ISO 8601） |
| `uptime_seconds` | number | 系统运行时间（秒） |
| `cpu_usage_percent` | number | CPU 使用率（%） |
| `memory_usage_percent` | number | 内存使用率（%） |
| `disk_usage_percent` | number | 磁盘使用率（%） |
| `services` | array | 服务状态列表 |
| `services[].name` | string | 服务名称 |
| `services[].status` | string | 服务状态：`healthy` / `unhealthy` |
| `services[].message` | string/null | 附加信息 |
| `alerts` | array | 告警列表 |

---

## ❌ 错误响应

### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header"
}
```

---

## 🧪 使用示例

```bash
curl -X GET "http://localhost:8080/api/v1/system/health" \
  -H "Authorization: Bearer <access_token>"
```

---

## 🔒 健康状态判定

| 状态 | 条件 |
|------|------|
| `healthy` | CPU < 80% 且 内存 < 75% 且 磁盘 < 75% |
| `degraded` | CPU > 80% 或 内存 > 75% 或 磁盘 > 75% |
| `critical` | CPU > 95% 或 内存 > 90% 或 磁盘 > 90% |

---

## 📝 注意事项

1. **权限要求**: 任意登录用户可访问，无需 admin 权限
2. **实时监控**: 建议 Web UI 每 30-60 秒轮询一次
3. **告警处理**: 当状态为 `degraded` 或 `critical` 时，`alerts` 数组包含告警信息

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/system/info` | 系统详细信息 |
| `GET /api/v1/system/resources` | 实时资源监控 |
| `GET /api/v1/system/resources/history` | 资源历史（24 小时） |

---

*文档维护：兵部尚书*
