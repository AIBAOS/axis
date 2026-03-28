# 系统资源监控 API 文档

## 概述

本文档描述 Axis NAS 系统中获取系统资源使用情况 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/system/resources`
- **版本**: v1
- **Phase**: 250

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
    "cpu": {
      "usage_percent": 25.5,
      "load_1m": 1.25,
      "load_5m": 1.10,
      "load_15m": 0.95,
      "core_count": 8
    },
    "memory": {
      "total_bytes": 34359738368,
      "used_bytes": 12884901888,
      "available_bytes": 21474836480,
      "usage_percent": 37.5
    },
    "disk_io": {
      "read_bytes_sec": 1048576,
      "write_bytes_sec": 524288,
      "read_ops_sec": 150,
      "write_ops_sec": 75
    },
    "network_io": {
      "rx_bytes_sec": 2097152,
      "tx_bytes_sec": 1048576,
      "rx_packets_sec": 1500,
      "tx_packets_sec": 1000
    },
    "timestamp": 1711600000
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
  "error": "Only admin users can access system resources",
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

### CpuInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `usage_percent` | number | CPU 使用率百分比（0-100） |
| `load_1m` | number | 1 分钟平均负载 |
| `load_5m` | number | 5 分钟平均负载 |
| `load_15m` | number | 15 分钟平均负载 |
| `core_count` | number | CPU 核心数 |

### MemoryInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `total_bytes` | number | 总内存（字节） |
| `used_bytes` | number | 已使用内存（字节） |
| `available_bytes` | number | 可用内存（字节） |
| `usage_percent` | number | 内存使用率百分比（0-100） |

### DiskIoInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `read_bytes_sec` | number | 每秒读取字节数 |
| `write_bytes_sec` | number | 每秒写入字节数 |
| `read_ops_sec` | number | 每秒读取操作数 |
| `write_ops_sec` | number | 每秒写入操作数 |

### NetworkIoInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `rx_bytes_sec` | number | 每秒接收字节数 |
| `tx_bytes_sec` | number | 每秒发送字节数 |
| `rx_packets_sec` | number | 每秒接收数据包数 |
| `tx_packets_sec` | number | 每秒发送数据包数 |

### SystemResources

| 字段 | 类型 | 描述 |
|------|------|------|
| `cpu` | CpuInfo | CPU 信息 |
| `memory` | MemoryInfo | 内存信息 |
| `disk_io` | DiskIoInfo | 磁盘 IO 信息 |
| `network_io` | NetworkIoInfo | 网络 IO 信息 |
| `timestamp` | number | 数据采集时间戳（Unix 时间戳） |

### SystemResourcesResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | SystemResources | 系统资源信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试访问 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求

```bash
curl -X GET "http://localhost:8080/api/v1/system/resources" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": {
    "cpu": {
      "usage_percent": 25.5,
      "load_1m": 1.25,
      "load_5m": 1.10,
      "load_15m": 0.95,
      "core_count": 8
    },
    "memory": {
      "total_bytes": 34359738368,
      "used_bytes": 12884901888,
      "available_bytes": 21474836480,
      "usage_percent": 37.5
    },
    "disk_io": {
      "read_bytes_sec": 1048576,
      "write_bytes_sec": 524288,
      "read_ops_sec": 150,
      "write_ops_sec": 75
    },
    "network_io": {
      "rx_bytes_sec": 2097152,
      "tx_bytes_sec": 1048576,
      "rx_packets_sec": 1500,
      "tx_packets_sec": 1000
    },
    "timestamp": 1711600000
  }
}
```

## 权限说明

- **Admin 用户**: 可访问系统资源信息
- **普通用户**: 无权访问（返回 403 Forbidden）

## 实现细节

### 数据来源
- 当前为模拟实现，返回固定数据
- 实际实现可：
  - 读取 /proc/stat 获取 CPU 使用率
  - 读取 /proc/meminfo 获取内存信息
  - 读取 /proc/diskstats 获取磁盘 IO
  - 读取 /proc/net/dev 获取网络 IO

### 数据采集频率
- 建议前端每 5-10 秒调用一次
- 避免频繁调用造成系统负担

## 相关接口

- `GET /api/v1/system/info` - 获取系统信息
- `GET /api/v1/system/power` - 获取电源状态
- `GET /api/v1/system/logs` - 获取系统日志

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取系统资源
curl -X GET "http://localhost:8080/api/v1/system/resources" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 系统资源信息

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/system/resources"

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 250** (2026-03-28): 初始实现，模拟系统资源数据
