# 进程信号发送 API 文档

## 概述

本文档描述 Axis NAS 系统中向进程发送信号 API 的实现细节。

## API 端点

- **路径**: `POST /api/v1/system/processes/{pid}/signal`
- **版本**: v1
- **Phase**: 253

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `pid` | number | 是 | 进程 ID |

### Request Body

```json
{
  "signal": "HUP"
}
```

#### 支持的信号类型

| 信号 | 说明 |
|------|------|
| `HUP` | 挂起信号，通常用于重新加载配置 |
| `INT` | 中断信号，相当于 Ctrl+C |
| `QUIT` | 退出信号，优雅退出 |
| `TERM` | 终止信号，请求进程终止 |
| `USR1` | 用户自定义信号 1 |
| `USR2` | 用户自定义信号 2 |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Signal HUP sent to process 1234 successfully",
  "pid": 1234,
  "signal": "HUP",
  "sent_at": 1711600000
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

#### 400 Bad Request - 无效信号类型

```json
{
  "success": false,
  "error": "Invalid signal type",
  "code": "INVALID_SIGNAL"
}
```

#### 403 Forbidden - 权限不足或关键进程

```json
{
  "success": false,
  "error": "Only admin users can send signals to processes",
  "code": "FORBIDDEN"
}
```

或（关键进程保护）：

```json
{
  "success": false,
  "error": "Cannot send signal to critical system process (PID: 1)",
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

### SendSignalRequest

| 字段 | 类型 | 描述 |
|------|------|------|
| `signal` | SignalType | 信号类型：HUP/INT/QUIT/TERM/USR1/USR2 |

### SendSignalResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `pid` | number | 目标进程 ID |
| `signal` | string | 发送的信号类型 |
| `sent_at` | number | 发送时间戳（Unix 时间戳） |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `INVALID_SIGNAL` | 400 | 无效的信号类型 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试访问 |
| `CRITICAL_PROCESS` | 403 | 尝试向系统关键进程发送信号 |
| `NOT_FOUND` | 404 | 指定的进程不存在 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求（发送 HUP 信号重新加载配置）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/signal" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signal": "HUP"}'
```

### 响应（成功）

```json
{
  "success": true,
  "message": "Signal HUP sent to process 1234 successfully",
  "pid": 1234,
  "signal": "HUP",
  "sent_at": 1711600000
}
```

### 请求（发送 TERM 信号终止进程）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/signal" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signal": "TERM"}'
```

### 请求（尝试向关键进程发送信号）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1/signal" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signal": "TERM"}'
```

### 响应（关键进程保护）

```json
{
  "success": false,
  "error": "Cannot send signal to critical system process (PID: 1)",
  "code": "CRITICAL_PROCESS"
}
```

## 权限说明

- **Admin 用户**: 可向非关键进程发送信号
- **普通用户**: 无权访问（返回 403 Forbidden）
- **关键进程保护**: 系统关键进程（PID 1-5）不可发送信号

## 实现细节

### 关键进程保护
为保护系统稳定性，以下进程不可发送信号：
- **PID 1-5**: 系统关键进程

### 信号说明
- **HUP**: 通常用于通知进程重新加载配置文件
- **INT**: 中断信号，相当于用户在终端按下 Ctrl+C
- **QUIT**: 退出信号，进程应优雅退出
- **TERM**: 终止信号，请求进程终止（默认信号）
- **USR1/USR2**: 用户自定义信号，供应用程序自定义用途

### 安全考虑
- 仅 admin 用户可访问
- 关键进程受保护，防止误操作导致系统崩溃
- 实际实现中应记录信号发送操作日志

### 数据来源
- 当前为模拟实现，返回固定响应
- 实际实现可：
  - 调用 kill(pid, signal) 系统调用发送信号
  - 读取 /proc/[pid] 验证进程存在性
  - 记录操作日志到系统审计日志

## 相关接口

- `GET /api/v1/system/processes` - 获取系统进程列表
- `POST /api/v1/system/processes/{pid}/terminate` - 终止进程

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试发送信号
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/signal" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signal": "HUP"}'

# 预期：200 OK + 信号发送成功响应

# 测试关键进程保护
curl -X POST "http://localhost:8080/api/v1/system/processes/1/signal" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signal": "TERM"}'

# 预期：403 Forbidden + 关键进程保护错误

# 测试未认证访问
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/signal" \
  -H "Content-Type: application/json" \
  -d '{"signal": "HUP"}'

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 253** (2026-03-28): 初始实现，模拟信号发送功能
