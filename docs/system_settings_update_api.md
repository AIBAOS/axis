# 系统设置更新 API

## Phase 247

## 接口说明

更新系统设置信息，仅限 admin 角色访问。支持部分更新。

## 请求

`PUT /api/v1/system/settings`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | application/json |

### 请求体

```json
{
  "timezone": "Asia/Shanghai",
  "language": "zh-CN",
  "update_channel": "stable",
  "auto_update_enabled": true,
  "notification_enabled": true,
  "power_schedule": "none"
}
```

所有字段均为可选，支持部分更新。

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| timezone | string | 否 | 时区（如 Asia/Shanghai） |
| language | string | 否 | 系统语言（如 zh-CN, en-US） |
| update_channel | string | 否 | 更新通道（stable/beta/nightly） |
| auto_update_enabled | boolean | 否 | 是否启用自动更新 |
| notification_enabled | boolean | 否 | 是否启用通知 |
| power_schedule | string | 否 | 电源计划（none/schedule） |

### 请求示例

**更新单个设置项：**
```json
{
  "timezone": "America/New_York"
}
```

**更新多个设置项：**
```json
{
  "language": "en-US",
  "update_channel": "beta",
  "auto_update_enabled": false
}
```

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "System settings updated successfully",
  "data": {
    "hostname": "nas-server",
    "timezone": "America/New_York",
    "language": "en-US",
    "update_channel": "beta",
    "auto_update_enabled": false,
    "notification_enabled": true,
    "power_schedule": "none"
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| message | string | 响应消息 |
| data | object | 更新后的系统设置 |
| data.hostname | string | 主机名（只读） |
| data.timezone | string | 时区 |
| data.language | string | 系统语言 |
| data.update_channel | string | 更新通道 |
| data.auto_update_enabled | boolean | 是否启用自动更新 |
| data.notification_enabled | boolean | 是否启用通知 |
| data.power_schedule | string | 电源计划 |

### 错误响应

#### 400 Bad Request - 参数格式错误

```json
{
  "success": false,
  "error": "Invalid update channel. Must be stable, beta, or nightly",
  "code": "INVALID_UPDATE_CHANNEL"
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

#### 403 Forbidden - 权限不足（非 admin）

```json
{
  "success": false,
  "error": "Only admin users can update system settings",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to update system settings: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 更新系统设置

```bash
curl -X PUT "http://localhost:8080/api/v1/system/settings" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "timezone": "America/New_York",
    "language": "en-US"
  }'
```

响应（200 OK）：
```json
{
  "success": true,
  "message": "System settings updated successfully",
  "data": {
    "hostname": "nas-server",
    "timezone": "America/New_York",
    "language": "en-US",
    "update_channel": "stable",
    "auto_update_enabled": true,
    "notification_enabled": true,
    "power_schedule": "none"
  }
}
```

### 更新非法的更新通道

```bash
curl -X PUT "http://localhost:8080/api/v1/system/settings" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "update_channel": "invalid"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid update channel. Must be stable, beta, or nightly",
  "code": "INVALID_UPDATE_CHANNEL"
}
```

### 非 admin 用户更新设置

```bash
curl -X PUT "http://localhost:8080/api/v1/system/settings" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "timezone": "Asia/Shanghai"
  }'
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can update system settings",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析并验证请求体参数
4. 验证设置项合法性（时区格式、语言格式、更新通道等）
5. 更新系统设置
6. 返回更新后的设置

## 设置项说明

| 设置项 | 说明 | 可选值/格式 |
|--------|------|-------------|
| timezone | 时区 | IANA 时区格式（如 Asia/Shanghai） |
| language | 系统语言 | 语言代码（如 zh-CN, en-US） |
| update_channel | 更新通道 | stable, beta, nightly |
| auto_update_enabled | 自动更新 | true, false |
| notification_enabled | 通知开关 | true, false |
| power_schedule | 电源计划 | none, schedule, custom |

## 安全说明

- 此接口仅限 admin 用户调用
- 系统设置包含关键配置，建议添加操作审计日志
- 建议限制调用频率防止恶意修改

## 版本历史

- **Phase 247** (2026-03-28): 系统模块 - 系统设置更新 API
