# 系统重启 API 文档

## 概述

本文档描述 Axis NAS 系统中系统重启 API 的实现细节。

## API 端点

- **路径**: `POST /api/v1/system/restart`
- **版本**: v1
- **Phase**: 229

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Request Body (可选)

```json
{
  "delay_seconds": 0
}
```

### 字段说明

| 字段 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `delay_seconds` | number | 否 | 0 | 延迟秒数（0-300 秒） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "status": "scheduled",
  "message": "System restart scheduled in 10 seconds",
  "restart_at": 1711600000
}
```

### 错误响应

#### 400 Bad Request - 延迟参数无效

```json
{
  "success": false,
  "error": "Delay seconds must be between 0 and 300",
  "code": "INVALID_DELAY"
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

#### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
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

## 数据模型

### SystemRestartRequest

| 字段 | 类型 | 描述 |
|------|------|------|
| `delay_seconds` | number? | 延迟秒数（0-300） |

### SystemRestartResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `status` | string | 状态：`scheduled` |
| `message` | string | 响应消息 |
| `restart_at` | number | 重启时间戳（Unix 时间戳） |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试重启 |
| `INVALID_DELAY` | 400 | 延迟参数超出范围（0-300 秒） |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求（立即重启）

```bash
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{}'
```

### 响应

```json
{
  "success": true,
  "status": "scheduled",
  "message": "System restart scheduled in 0 seconds",
  "restart_at": 1711600000
}
```

### 请求（延迟 60 秒重启）

```bash
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "delay_seconds": 60
  }'
```

### 响应

```json
{
  "success": true,
  "status": "scheduled",
  "message": "System restart scheduled in 60 seconds",
  "restart_at": 1711600060
}
```

### 错误请求（延迟超过 300 秒）

```bash
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "delay_seconds": 500
  }'
```

### 响应

```json
{
  "success": false,
  "error": "Delay seconds must be between 0 and 300",
  "code": "INVALID_DELAY"
}
```

## 权限说明

- **Admin 用户**: 可重启系统
- **普通用户**: 无权访问（返回 403 Forbidden）

## 实现细节

### 延迟参数验证
- 最小值：0 秒（立即重启）
- 最大值：300 秒（5 分钟）
- 超出范围返回 400 Bad Request

### 时间戳计算
- 使用 `std::time::SystemTime` 获取当前时间
- 计算重启时间戳：`now + delay_seconds`
- 返回 Unix 时间戳（秒）

### 系统重启实现
- 当前为模拟实现，仅返回计划信息
- 实际实现可调用系统命令：
  - Linux: `shutdown -r +<minutes>` 或 `systemctl reboot`
  - 需要适当的系统权限

## 安全考虑

### 权限控制
- 仅 admin 用户可访问
- 需要有效的 JWT Token

### 延迟限制
- 最大延迟 300 秒，防止误操作
- 可考虑添加二次确认机制

### 审计日志
- 建议记录所有重启操作
- 包括操作者、时间、延迟等信息

## 相关接口

- `GET /api/v1/system/info` - 获取系统信息
- `POST /api/v1/system/shutdown` - 系统关机
- `GET /api/v1/system/health` - 系统健康检查

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试立即重启
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{}'

# 预期：200 OK + 重启计划信息

# 测试延迟重启
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"delay_seconds": 60}'

# 预期：200 OK + 重启计划信息

# 测试无效延迟
curl -X POST "http://localhost:8080/api/v1/system/restart" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"delay_seconds": 500}'

# 预期：400 Bad Request
```

## 版本历史

- **Phase 229** (2026-03-28): 初始实现，模拟重启计划
