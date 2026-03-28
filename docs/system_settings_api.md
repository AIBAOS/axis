# 系统设置获取 API

## Phase 246

## 接口说明

获取系统设置信息，供 Web UI 展示系统配置，仅限 admin 角色访问。

## 请求

`GET /api/v1/system/settings`

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
    "hostname": "nas-server",
    "timezone": "Asia/Shanghai",
    "language": "zh-CN",
    "update_channel": "stable",
    "auto_update_enabled": true,
    "notification_enabled": true,
    "power_schedule": "none"
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | object | 系统设置信息 |
| data.hostname | string | 主机名 |
| data.timezone | string | 时区（如 Asia/Shanghai） |
| data.language | string | 系统语言（如 zh-CN） |
| data.update_channel | string | 更新通道（stable/beta/nightly） |
| data.auto_update_enabled | boolean | 是否启用自动更新 |
| data.notification_enabled | boolean | 是否启用通知 |
| data.power_schedule | string | 电源计划（可选，如 none/schedule） |

### 错误响应

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
  "error": "Only admin users can view system settings",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to get system settings: io error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取系统设置

```bash
curl -X GET "http://localhost:8080/api/v1/system/settings" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "hostname": "nas-server",
    "timezone": "Asia/Shanghai",
    "language": "zh-CN",
    "update_channel": "stable",
    "auto_update_enabled": true,
    "notification_enabled": true,
    "power_schedule": "none"
  }
}
```

### 非 admin 用户获取系统设置

```bash
curl -X GET "http://localhost:8080/api/v1/system/settings" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view system settings",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X GET "http://localhost:8080/api/v1/system/settings"
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 从数据库或配置文件读取系统设置
4. 返回系统设置信息

## 设置说明

| 设置项 | 说明 | 可选值 |
|--------|------|--------|
| update_channel | 更新通道 | stable, beta, nightly |
| auto_update_enabled | 自动更新 | true, false |
| notification_enabled | 通知开关 | true, false |
| power_schedule | 电源计划 | none, schedule, custom |

## 安全说明

- 此接口仅限 admin 用户调用
- 系统设置包含敏感配置信息，建议添加访问审计
- 建议限制调用频率防止资源消耗

## 版本历史

- **Phase 246** (2026-03-28): 系统模块 - 系统设置获取 API
