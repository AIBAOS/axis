# 终止进程 API 文档

## 概述

本文档描述 Axis NAS 系统中终止指定进程 API 的实现细节。

## API 端点

- **路径**: `POST /api/v1/system/processes/{pid}/terminate`
- **版本**: v1
- **Phase**: 252

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `pid` | number | 是 | 进程 ID |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Process 1234 terminated successfully",
  "pid": 1234,
  "terminated_at": 1711600000
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

#### 403 Forbidden - 权限不足或关键进程

```json
{
  "success": false,
  "error": "Only admin users can terminate processes",
  "code": "FORBIDDEN"
}
```

或（关键进程保护）：

```json
{
  "success": false,
  "error": "Cannot terminate critical system process (PID: 1)",
  "code": "CRITICAL_PROCESS"
}
```

#### 404 Not Found - 进程不存在

```json
{
  "success": false,
  "error": "Process with PID 99999 not found",
  "code": "NOT_FOUND"
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

### TerminateProcessResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `pid` | number | 被终止的进程 ID |
| `terminated_at` | number | 终止时间戳（Unix 时间戳） |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试访问 |
| `CRITICAL_PROCESS` | 403 | 尝试终止系统关键进程 |
| `NOT_FOUND` | 404 | 指定的进程不存在 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求（终止普通进程）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/terminate" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 响应（成功）

```json
{
  "success": true,
  "message": "Process 1234 terminated successfully",
  "pid": 1234,
  "terminated_at": 1711600000
}
```

### 请求（尝试终止关键进程）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1/terminate" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 响应（关键进程保护）

```json
{
  "success": false,
  "error": "Cannot terminate critical system process (PID: 1)",
  "code": "CRITICAL_PROCESS"
}
```

### 请求（进程不存在）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/99999/terminate" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 响应（进程不存在）

```json
{
  "success": false,
  "error": "Process with PID 99999 not found",
  "code": "NOT_FOUND"
}
```

## 权限说明

- **Admin 用户**: 可终止非关键进程
- **普通用户**: 无权访问（返回 403 Forbidden）
- **关键进程保护**: 系统关键进程（PID 1-5）不可终止

## 实现细节

### 关键进程保护
为保护系统稳定性，以下进程不可终止：
- **PID 1**: init/systemd（系统初始化进程）
- **PID 2-5**: 其他系统关键进程

### 安全考虑
- 仅 admin 用户可访问
- 关键进程受保护，防止误操作导致系统崩溃
- 实际实现中应记录终止操作日志

### 数据来源
- 当前为模拟实现，返回固定响应
- 实际实现可：
  - 调用 kill() 系统调用终止进程
  - 读取 /proc/[pid] 验证进程存在性
  - 记录操作日志到系统审计日志

## 相关接口

- `GET /api/v1/system/processes` - 获取系统进程列表
- `GET /api/v1/system/resources` - 获取系统资源使用情况

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试终止进程
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/terminate" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 终止成功响应

# 测试关键进程保护
curl -X POST "http://localhost:8080/api/v1/system/processes/1/terminate" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：403 Forbidden + 关键进程保护错误

# 测试未认证访问
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/terminate"

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 252** (2026-03-28): 初始实现，模拟终止进程功能
