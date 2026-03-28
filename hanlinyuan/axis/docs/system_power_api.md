# 系统电源管理 API 文档

## 概述

本文档描述 Axis NAS 系统中获取电源状态 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/system/power`
- **版本**: v1
- **Phase**: 248

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

无

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "power_status": "on",
    "power_consumption_watts": 45.5,
    "ups_connected": true,
    "ups_battery_percent": 95.0,
    "ups_runtime_minutes": 120,
    "last_power_event": {
      "event_type": "power_on",
      "timestamp": 1711500000
    }
  }
}
```

### 错误响应

#### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can access power status",
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

## 数据模型

### PowerStatus

电源状态枚举值：
- `on` - 开机
- `off` - 关机
- `sleep` - 睡眠
- `shutdown` - 关闭中

### PowerEventType

电源事件类型枚举值：
- `power_on` - 开机
- `power_off` - 关机
- `sleep_entered` - 进入睡眠
- `sleep_exited` - 退出睡眠
- `shutdown_initiated` - 开始关闭
- `ups_battery_low` - UPS 电量低
- `ups_power_restored` - UPS 电源恢复

### PowerEvent

| 字段 | 类型 | 描述 |
|------|------|------|
| `event_type` | PowerEventType | 事件类型 |
| `timestamp` | number | 事件发生时间戳（Unix 时间戳） |

### PowerStatusInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `power_status` | PowerStatus | 当前电源状态 |
| `power_consumption_watts` | number | 当前功耗（瓦特） |
| `ups_connected` | boolean | 是否连接 UPS |
| `ups_battery_percent` | number? | UPS 电量百分比（0-100），无 UPS 则 null |
| `ups_runtime_minutes` | number? | UPS 剩余运行时间（分钟），无 UPS 则 null |
| `last_power_event` | PowerEvent? | 最近电源事件 |

### PowerStatusResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | PowerStatusInfo | 电源状态信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试访问 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求

```bash
curl -X GET "http://localhost:8080/api/v1/system/power" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 响应（有 UPS）

```json
{
  "success": true,
  "data": {
    "power_status": "on",
    "power_consumption_watts": 45.5,
    "ups_connected": true,
    "ups_battery_percent": 95.0,
    "ups_runtime_minutes": 120,
    "last_power_event": {
      "event_type": "power_on",
      "timestamp": 1711500000
    }
  }
}
```

### 响应（无 UPS）

```json
{
  "success": true,
  "data": {
    "power_status": "on",
    "power_consumption_watts": 32.0,
    "ups_connected": false,
    "ups_battery_percent": null,
    "ups_runtime_minutes": null,
    "last_power_event": {
      "event_type": "power_on",
      "timestamp": 1711500000
    }
  }
}
```

## 权限说明

- **Admin 用户**: 可访问电源状态
- **普通用户**: 无权访问（返回 403 Forbidden）

## 实现细节

### 电源状态说明
- **power_status**: 当前系统电源状态
- **power_consumption_watts**: 实时功耗（瓦特）
- **ups_connected**: UPS 连接状态
- **ups_battery_percent**: UPS 电池电量百分比（0-100）
- **ups_runtime_minutes**: UPS 剩余供电时间（分钟）
- **last_power_event**: 最近一次电源事件

### 数据来源
- 当前为模拟实现，返回固定数据
- 实际实现可：
  - 读取系统电源管理接口（如 ACPI）
  - 查询 UPS 设备状态（如通过 USB）
  - 使用传感器读取实时功耗

## 相关接口

- `GET /api/v1/system/info` - 获取系统信息
- `GET /api/v1/system/settings` - 获取系统设置
- `PUT /api/v1/system/settings` - 更新系统设置
- `POST /api/v1/system/power` - 执行电源操作

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取电源状态
curl -X GET "http://localhost:8080/api/v1/system/power" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 电源状态信息

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/system/power"

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 248** (2026-03-28): 初始实现，模拟电源状态数据
