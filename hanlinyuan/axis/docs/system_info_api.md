# 系统信息 API

## Phase 245

## 接口说明

获取 NAS 系统基本信息，供 WebUI 展示，仅限 admin 角色访问。

## 请求

`GET /api/v1/system/info`

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
    "os_version": "Linux 6.6.87",
    "kernel_version": "6.6.87.2-microsoft-standard-WSL2",
    "cpu_model": "Intel(R) Core(TM) i7-9700K",
    "cpu_cores": 8,
    "total_memory_gb": 32,
    "uptime_seconds": 86400,
    "boot_time": 1711497600
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | object | 系统信息 |
| data.hostname | string | 主机名 |
| data.os_version | string | 操作系统版本 |
| data.kernel_version | string | 内核版本 |
| data.cpu_model | string | CPU 型号 |
| data.cpu_cores | integer | CPU 核心数 |
| data.total_memory_gb | integer | 总内存（GB） |
| data.uptime_seconds | integer | 运行时间（秒） |
| data.boot_time | integer | 启动时间（Unix 时间戳） |

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
  "error": "Only admin users can view system info",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to get system info: io error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取系统信息

```bash
curl -X GET "http://localhost:8080/api/v1/system/info" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "hostname": "nas-server",
    "os_version": "Linux 6.6.87",
    "kernel_version": "6.6.87.2-microsoft-standard-WSL2",
    "cpu_model": "Intel(R) Core(TM) i7-9700K",
    "cpu_cores": 8,
    "total_memory_gb": 32,
    "uptime_seconds": 86400,
    "boot_time": 1711497600
  }
}
```

### 非 admin 用户获取系统信息

```bash
curl -X GET "http://localhost:8080/api/v1/system/info" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view system info",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X GET "http://localhost:8080/api/v1/system/info"
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
3. 收集系统信息（主机名、OS、内核、CPU、内存等）
4. 返回系统信息

## 安全说明

- 此接口仅限 admin 用户调用
- 系统信息可能包含敏感信息，建议添加访问审计
- 建议限制调用频率防止资源消耗

## 版本历史

- **Phase 245** (2026-03-28): 系统模块 - 系统信息 API
