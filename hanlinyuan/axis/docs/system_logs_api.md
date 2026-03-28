# 系统日志 API

## Phase 170

## 接口说明

获取系统日志列表，支持分页和过滤。

## 请求

`GET /api/v1/system/logs`

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ---- | ---- |
| page | u32 | 否 | 1 | 页码（从 1 开始） |
| limit | u32 | 否 | 50 | 每页数量（最大 200） |
| level | string | 否 | - | 日志级别（INFO/WARN/ERROR） |
| source | string | 否 | - | 日志来源（system/docker/network/等） |
| since | string | 否 | - | 起始时间（ISO 8601 格式） |

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
  "data": [
    {
      "id": 1,
      "timestamp": "2026-03-27T11:00:00Z",
      "level": "INFO",
      "source": "system",
      "message": "System started successfully",
      "details": null
    },
    {
      "id": 2,
      "timestamp": "2026-03-27T10:55:00Z",
      "level": "WARN",
      "source": "docker",
      "message": "Container nginx-web high CPU usage detected",
      "details": "CPU usage: 85%"
    },
    {
      "id": 3,
      "timestamp": "2026-03-27T10:50:00Z",
      "level": "ERROR",
      "source": "network",
      "message": "Failed to connect to external API",
      "details": "Connection timeout after 30s"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 10,
    "total_pages": 1
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid log level. Valid levels: INFO, WARN, ERROR",
  "code": "INVALID_LEVEL"
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

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can view system logs",
  "code": "FORBIDDEN"
}
```

## 示例

### 获取系统日志（默认分页）

```bash
curl "http://localhost:8080/api/v1/system/logs" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取 ERROR 级别日志

```bash
curl "http://localhost:8080/api/v1/system/logs?level=ERROR" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取特定来源的日志

```bash
curl "http://localhost:8080/api/v1/system/logs?source=docker" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取指定时间之后的日志

```bash
curl "http://localhost:8080/api/v1/system/logs?since=2026-03-27T10:00:00Z" \
  -H "Authorization: Bearer <jwt_token>"
```

### 组合过滤

```bash
curl "http://localhost:8080/api/v1/system/logs?level=WARN&source=storage&limit=20" \
  -H "Authorization: Bearer <jwt_token>"
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 日志列表字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 日志 ID |
| timestamp | string | 时间戳（ISO 8601 格式） |
| level | string | 日志级别（INFO/WARN/ERROR） |
| source | string | 日志来源（system/docker/network/storage/等） |
| message | string | 日志消息 |
| details | string\|null | 详细信息（可选） |

### 分页字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| page | u32 | 当前页码 |
| limit | u32 | 每页数量 |
| total | u64 | 总记录数 |
| total_pages | u32 | 总页数 |

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
3. 验证日志级别（如果提供）
4. 获取系统日志
5. 应用级别/来源/时间过滤
6. 应用分页
7. 返回 200 OK + 日志列表（时间倒序）+ 分页信息

## 版本历史

- **Phase 170** (2026-03-27): 系统日志模块 - 系统日志列表 API
