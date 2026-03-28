# 系统告警删除 API

## Phase 178

## 接口说明

删除指定的系统告警，完成告警生命周期管理。

## 请求

`DELETE /api/v1/system/alerts/{id}`

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
  "note": "告警已处理，可以删除"
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| note | string | 否 | 删除备注 |

## 响应

### 成功响应（204 No Content）

删除成功，无响应体。

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Alert 1 must be acknowledged or resolved before deletion",
  "code": "INVALID_STATUS"
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
  "error": "Only admin users can delete system alerts",
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

### 删除已确认的告警

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/alerts/2" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "note": "告警已处理，可以删除"
  }'
```

响应：`204 No Content`

### 删除已解决的告警

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/alerts/3" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json"
```

响应：`204 No Content`

### 删除活跃的告警（需先确认）

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/alerts/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json"
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Alert 1 must be acknowledged or resolved before deletion",
  "code": "INVALID_STATUS"
}
```

### 删除不存在的告警

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/alerts/999" \
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
curl -X DELETE "http://localhost:8080/api/v1/system/alerts/2" \
  -H "Authorization: Bearer <user_token>" \
  -H "Content-Type: application/json"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can delete system alerts",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 告警状态说明

| 状态 | 说明 | 可删除 |
| ---- | ---- | ------ |
| active | 活跃状态，未处理 | ❌ 否（需先确认） |
| acknowledged | 已确认，处理中 | ✅ 是 |
| resolved | 已解决 | ✅ 是 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据告警 ID 查找告警
4. 告警不存在返回 404 Not Found
5. 验证告警状态（仅 acknowledged/resolved 状态可删除）
6. 删除告警
7. 返回 204 No Content

## 版本历史

- **Phase 178** (2026-03-27): 系统告警模块 - 系统告警删除 API
