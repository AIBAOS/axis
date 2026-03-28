# 进程信号发送 API

## Phase 253

## 接口说明

向指定进程发送信号，用于进程管理（终止、重启等），仅限 admin 角色访问。

## 请求

`POST /api/v1/system/processes/{pid}/signal`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 否 | application/json |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| pid | integer | 是 | 进程 ID |

### 请求体

```json
{
  "signal": "SIGTERM"
}
```

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| signal | string | 否 | SIGTERM | 信号类型（SIGTERM/SIGHUP/SIGINT/SIGKILL/SIGUSR1/SIGUSR2） |

### 支持的信号类型

| 信号 | 编号 | 说明 |
|------|------|------|
| SIGTERM | 15 | 终止信号（默认，优雅退出） |
| SIGHUP | 1 | 挂起信号（常用于重启服务） |
| SIGINT | 2 | 中断信号（Ctrl+C） |
| SIGKILL | 9 | 强制终止（不可捕获） |
| SIGUSR1 | 10 | 用户定义信号 1 |
| SIGUSR2 | 12 | 用户定义信号 2 |

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "Signal SIGTERM sent to process 1234",
  "pid": 1234,
  "signal": "SIGTERM",
  "sent_at": 1711634400
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| message | string | 响应消息 |
| pid | integer | 目标进程 ID |
| signal | string | 发送的信号类型 |
| sent_at | integer | 发送时间（Unix 时间戳） |

### 错误响应

#### 400 Bad Request - 参数格式错误

```json
{
  "success": false,
  "error": "Invalid signal: INVALID. Must be SIGTERM, SIGHUP, SIGINT, SIGKILL, SIGUSR1, or SIGUSR2",
  "code": "INVALID_SIGNAL"
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

#### 403 Forbidden - 权限不足或关键进程保护

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
  "error": "Cannot send signal to critical system process (PID 1)",
  "code": "CRITICAL_PROCESS"
}
```

#### 404 Not Found - 进程不存在

```json
{
  "success": false,
  "error": "Process 99999 not found",
  "code": "PROCESS_NOT_FOUND"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to send signal: permission denied",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 发送 SIGTERM 信号（优雅终止）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/signal" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"signal": "SIGTERM"}'
```

响应（200 OK）：
```json
{
  "success": true,
  "message": "Signal SIGTERM sent to process 1234",
  "pid": 1234,
  "signal": "SIGTERM",
  "sent_at": 1711634400
}
```

### 发送 SIGHUP 信号（重启服务）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/5678/signal" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"signal": "SIGHUP"}'
```

### 发送 SIGKILL 信号（强制终止）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/9999/signal" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"signal": "SIGKILL"}'
```

### 使用默认信号（SIGTERM）

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/signal" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{}'
```

### 尝试终止系统关键进程

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1/signal" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"signal": "SIGTERM"}'
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Cannot send signal to critical system process (PID 1)",
  "code": "CRITICAL_PROCESS"
}
```

### 无效的 PID

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/0/signal" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"signal": "SIGTERM"}'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid PID: 0",
  "code": "INVALID_PID"
}
```

### 非 admin 用户发送信号

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/signal" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"signal": "SIGTERM"}'
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can send signals to processes",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X POST "http://localhost:8080/api/v1/system/processes/1234/signal" \
  -H "Content-Type: application/json" \
  -d '{"signal": "SIGTERM"}'
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
3. 验证 PID 合法性
4. 检查是否为系统关键进程（保护机制）
5. 解析信号类型
6. 向进程发送信号
7. 返回发送结果

## 安全说明

- 此接口仅限 admin 用户调用
- 系统关键进程（PID 1-100）受保护，不可终止
- 建议添加操作审计日志
- SIGKILL 信号应谨慎使用（进程无法捕获或忽略）

## 信号使用场景

| 场景 | 推荐信号 | 说明 |
|------|----------|------|
| 优雅终止服务 | SIGTERM | 允许进程清理资源后退出 |
| 重启服务 | SIGHUP | 通知进程重新加载配置 |
| 强制终止 | SIGKILL | 无法捕获，立即终止 |
| 自定义行为 | SIGUSR1/SIGUSR2 | 应用程序自定义处理 |

## 版本历史

- **Phase 253** (2026-03-28): 系统模块 - 进程信号发送 API
