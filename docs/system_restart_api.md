# 系统重启 API

## Phase 229

## 接口说明

触发系统重启操作，支持延迟重启。

## 请求

`POST /api/v1/system/restart`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 否 | application/json |

### 请求体

```json
{
  "delay_seconds": 0
}
```

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| delay_seconds | integer | 否 | 0 | 延迟秒数（0-300） |

### 请求体示例

**立即重启：**
```json
{}
```

**延迟 60 秒重启：**
```json
{
  "delay_seconds": 60
}
```

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "status": "scheduled",
  "message": "System restart scheduled in 60 seconds",
  "restart_at": 1711584060
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 操作是否成功 |
| status | string | 状态（scheduled） |
| message | string | 重启消息 |
| restart_at | integer | 计划重启时间戳（Unix 时间戳） |

### 错误响应

#### 400 Bad Request - 延迟参数错误

```json
{
  "success": false,
  "error": "Delay seconds must be between 0 and 300",
  "code": "INVALID_DELAY"
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
  "error": "Only admin users can restart the system",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 系统错误

```json
{
  "success": false,
  "error": "Failed to get current time",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 立即重启系统

```bash
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{}'
```

响应（200 OK）：
```json
{
  "success": true,
  "status": "scheduled",
  "message": "System restart scheduled in 0 seconds",
  "restart_at": 1711584000
}
```

### 延迟 60 秒重启

```bash
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"delay_seconds": 60}'
```

响应（200 OK）：
```json
{
  "success": true,
  "status": "scheduled",
  "message": "System restart scheduled in 60 seconds",
  "restart_at": 1711584060
}
```

### 延迟参数超出范围

```bash
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"delay_seconds": 500}'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Delay seconds must be between 0 and 300",
  "code": "INVALID_DELAY"
}
```

### 非 admin 用户请求

```bash
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{}'
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can restart the system",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析 delay_seconds 参数（默认 0）
4. 验证延迟参数范围（0-300 秒）
5. 计算重启时间戳
6. 触发系统重启（模拟实现）
7. 返回重启计划信息

## 安全说明

- 此接口仅限 admin 用户调用
- 延迟重启功能允许用户在系统重启前保存工作
- 实际生产环境中应添加二次确认机制
- 建议添加操作审计日志

## 版本历史

- **Phase 229** (2026-03-28): 系统管理模块 - 系统重启 API
