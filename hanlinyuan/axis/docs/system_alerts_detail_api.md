# 系统告警详情 API

## Phase 175

## 接口说明

获取指定系统告警的详细信息。

## 请求

`GET /api/v1/system/alerts/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 告警 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "title": "High CPU Usage",
    "message": "CPU usage exceeded 90% for more than 5 minutes",
    "severity": "critical",
    "status": "active",
    "source": "system",
    "created_at": "2026-03-27T12:30:00Z",
    "acknowledged_at": null,
    "acknowledged_by": null,
    "resolved_at": null,
    "resolved_by": null,
    "metadata": "{\"cpu_usage\": 92.5, \"threshold\": 90, \"duration_seconds\": 300}"
  }
}
```

### 错误响应

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
  "error": "Only admin users can view system alert details",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 告警不存在

```json
{
  "success": false,
  "error": "System alert 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 获取系统告警详情

```bash
curl "http://localhost:8080/api/v1/system/alerts/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的告警

```bash
curl "http://localhost:8080/api/v1/system/alerts/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "System alert 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 告警详情字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 告警 ID |
| title | string | 告警标题 |
| message | string | 告警消息 |
| severity | string | 告警级别（critical/warning/info） |
| status | string | 告警状态（active/resolved/acknowledged） |
| source | string | 告警来源（system/storage/network/等） |
| created_at | string | 创建时间（ISO 8601 格式） |
| acknowledged_at | string\|null | 确认时间（ISO 8601 格式） |
| acknowledged_by | string\|null | 确认人 |
| resolved_at | string\|null | 解决时间（ISO 8601 格式） |
| resolved_by | string\|null | 解决人 |
| metadata | string\|null | 元数据（JSON 格式） |

## 告警级别说明

| 级别 | 说明 | 示例 |
| ---- | ---- | ---- |
| critical | 严重告警，需立即处理 | CPU 使用率>90%、磁盘故障 |
| warning | 警告，需关注 | 磁盘使用率>80%、内存使用率高 |
| info | 信息性通知 | 备份完成、配置更新 |

## 告警状态说明

| 状态 | 说明 |
| ---- | ---- |
| active | 活跃状态，未处理 |
| acknowledged | 已确认，处理中 |
| resolved | 已解决 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据告警 ID 查找告警
4. 告警不存在返回 404 Not Found
5. 返回 200 OK + 告警详情

## 版本历史

- **Phase 175** (2026-03-27): 系统告警模块 - 系统告警详情 API
