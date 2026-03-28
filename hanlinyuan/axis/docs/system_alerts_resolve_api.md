# 系统告警解决 API

## Phase 177

## 接口说明

解决指定的系统告警，将其状态从 active/acknowledged 更改为 resolved。

## 请求

`POST /api/v1/system/alerts/{id}/resolve`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 告警 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体（可选）

```json
{
  "notes": "已解决，问题已修复"
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| notes | string | 否 | 解决备注 |

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "System alert resolved successfully",
  "data": {
    "id": 1,
    "title": "High CPU Usage",
    "message": "CPU usage exceeded 90% for more than 5 minutes",
    "severity": "critical",
    "status": "resolved",
    "source": "system",
    "created_at": "2026-03-27T12:30:00Z",
    "acknowledged_at": "2026-03-27T13:00:00Z",
    "acknowledged_by": "admin",
    "resolved_at": "2026-03-27T13:30:00Z",
    "resolved_by": "admin"
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Alert 3 is already resolved",
  "code": "ALREADY_RESOLVED"
}
```

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
  "error": "Only admin users can resolve system alerts",
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

### 解决系统告警

```bash
curl -X POST "http://localhost:8080/api/v1/system/alerts/1/resolve" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "notes": "已解决，问题已修复"
  }'
```

### 解决已确认的告警

```bash
curl -X POST "http://localhost:8080/api/v1/system/alerts/2/resolve" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json"
```

### 解决已解决的告警

```bash
curl -X POST "http://localhost:8080/api/v1/system/alerts/3/resolve" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json"
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Alert 3 is already resolved",
  "code": "ALREADY_RESOLVED"
}
```

### 解决不存在的告警

```bash
curl -X POST "http://localhost:8080/api/v1/system/alerts/999/resolve" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "System alert 999 not found",
  "code": "NOT_FOUND"
}
```

### 无权限访问

```bash
curl -X POST "http://localhost:8080/api/v1/system/alerts/1/resolve" \
  -H "Authorization: Bearer <user_token>" \
  -H "Content-Type: application/json"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can resolve system alerts",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 解决结果字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| success | boolean | 是否成功 |
| message | string | 响应消息 |
| data | object | 更新后的告警信息 |

### 告警信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 告警 ID |
| title | string | 告警标题 |
| message | string | 告警消息 |
| severity | string | 告警级别（critical/warning/info） |
| status | string | 告警状态（active/acknowledged/resolved） |
| source | string | 告警来源（system/storage/network/等） |
| created_at | string | 创建时间（ISO 8601 格式） |
| acknowledged_at | string\|null | 确认时间（ISO 8601 格式） |
| acknowledged_by | string\|null | 确认人 |
| resolved_at | string\|null | 解决时间（ISO 8601 格式） |
| resolved_by | string\|null | 解决人 |

## 告警状态说明

| 状态 | 说明 | 可解决 |
| ---- | ---- | ------ |
| active | 活跃状态，未处理 | ✅ 是 |
| acknowledged | 已确认，处理中 | ✅ 是 |
| resolved | 已解决 | ❌ 否 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据告警 ID 查找告警
4. 告警不存在返回 404 Not Found
5. 验证告警状态（仅 active/acknowledged 状态可解决）
6. 更新告警状态为 resolved
7. 记录解决时间和解决人
8. 返回 200 OK + 更新后的告警详情

## 版本历史

- **Phase 177** (2026-03-27): 系统告警模块 - 系统告警解决 API
