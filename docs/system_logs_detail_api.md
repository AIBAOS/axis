# 系统日志详情 API

## Phase 172

## 接口说明

获取指定系统日志的详细信息。

## 请求

`GET /api/v1/system/logs/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 日志 ID |

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
    "id": 1,
    "timestamp": "2026-03-27T11:00:00Z",
    "level": "INFO",
    "source": "system",
    "message": "System started successfully",
    "details": null,
    "context": "System boot sequence completed",
    "user": "system"
  }
}
```

### 错误响应

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
  "error": "Only admin users can view system log details",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 日志不存在

```json
{
  "success": false,
  "error": "System log 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 获取系统日志详情

```bash
curl "http://localhost:8080/api/v1/system/logs/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的日志

```bash
curl "http://localhost:8080/api/v1/system/logs/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "System log 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 日志详情字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 日志 ID |
| timestamp | string | 时间戳（ISO 8601 格式） |
| level | string | 日志级别（INFO/WARN/ERROR） |
| source | string | 日志来源（system/docker/network/等） |
| message | string | 日志消息 |
| details | string\|null | 详细信息（可选） |
| context | string\|null | 上下文信息（可选） |
| user | string\|null | 相关用户（可选） |

## 日志级别说明

| 级别 | 说明 | 示例 |
| ---- | ---- | ---- |
| INFO | 信息性消息 | 系统启动、用户登录、配置重载 |
| WARN | 警告消息 | 资源使用率高、性能问题 |
| ERROR | 错误消息 | 连接失败、认证失败、系统异常 |

## 日志来源说明

| 来源 | 说明 |
| ---- | ---- |
| system | 系统级事件 |
| docker | Docker 容器相关事件 |
| network | 网络配置和连接事件 |
| storage | 存储和磁盘相关事件 |
| backup | 备份任务相关事件 |
| auth | 认证和授权事件 |
| container | 容器管理事件 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据日志 ID 查找日志
4. 日志不存在返回 404 Not Found
5. 返回 200 OK + 日志详情

## 版本历史

- **Phase 172** (2026-03-27): 系统日志模块 - 系统日志详情 API
