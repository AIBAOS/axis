# 系统日志导出 API

## Phase 173

## 接口说明

导出系统日志为 CSV 或 JSON 格式文件，支持按时间范围、日志级别、来源筛选。

## 请求

`POST /api/v1/system/logs/export`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

```json
{
  "format": "csv",
  "startTime": "2026-03-27T00:00:00Z",
  "endTime": "2026-03-27T23:59:59Z",
  "level": "ERROR",
  "source": "network",
  "page": 1,
  "limit": 100
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| format | string | 是 | 导出格式（csv/json） |
| startTime | string | 否 | 起始时间（ISO 8601 格式） |
| endTime | string | 否 | 结束时间（ISO 8601 格式） |
| level | string | 否 | 日志级别（INFO/WARN/ERROR） |
| source | string | 否 | 日志来源（system/docker/network/等） |
| page | number | 否 | 页码（默认 1） |
| limit | number | 否 | 每页数量（默认 100，最大 1000） |

## 响应

### 成功响应（200 OK）

**CSV 格式**:
```
Content-Type: text/csv
Content-Disposition: attachment; filename="system_logs_20260327_120000.csv"

id,timestamp,level,source,message,details
1,2026-03-27T11:00:00Z,INFO,system,"System started successfully",""
2,2026-03-27T10:55:00Z,WARN,docker,"Container nginx-web high CPU usage detected","CPU usage: 85%"
3,2026-03-27T10:50:00Z,ERROR,network,"Failed to connect to external API","Connection timeout after 30s"
```

**JSON 格式**:
```json
Content-Type: application/json
Content-Disposition: attachment; filename="system_logs_20260327_120000.json"

[
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
  }
]
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid format. Valid formats: csv, json",
  "code": "INVALID_FORMAT"
}
```

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
  "error": "Only admin users can export system logs",
  "code": "FORBIDDEN"
}
```

## 示例

### 导出所有日志为 CSV

```bash
curl -X POST "http://localhost:8080/api/v1/system/logs/export" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "csv"
  }' \
  -o system_logs.csv
```

### 导出 ERROR 级别日志为 JSON

```bash
curl -X POST "http://localhost:8080/api/v1/system/logs/export" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "json",
    "level": "ERROR"
  }' \
  -o error_logs.json
```

### 导出指定时间范围的日志

```bash
curl -X POST "http://localhost:8080/api/v1/system/logs/export" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "csv",
    "startTime": "2026-03-27T10:00:00Z",
    "endTime": "2026-03-27T12:00:00Z",
    "level": "WARN"
  }' \
  -o warn_logs.csv
```

### 导出特定来源的日志

```bash
curl -X POST "http://localhost:8080/api/v1/system/logs/export" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "json",
    "source": "docker",
    "limit": 50
  }' \
  -o docker_logs.json
```

### 尝试使用无效格式

```bash
curl -X POST "http://localhost:8080/api/v1/system/logs/export" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "xml"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid format. Valid formats: csv, json",
  "code": "INVALID_FORMAT"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 日志字段（CSV/JSON）

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 日志 ID |
| timestamp | string | 时间戳（ISO 8601 格式） |
| level | string | 日志级别（INFO/WARN/ERROR） |
| source | string | 日志来源（system/docker/network/等） |
| message | string | 日志消息 |
| details | string\|null | 详细信息（可选） |

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
3. 验证导出格式（csv/json）
4. 验证日志级别（如果提供）
5. 获取系统日志
6. 应用时间范围、级别、来源过滤
7. 应用分页
8. 生成 CSV 或 JSON 文件
9. 返回文件下载（带 Content-Disposition 头）

## 性能考虑

- **分页限制**: 每页最大 1000 条记录，避免内存溢出
- **流式处理**: 大日志量时建议使用分页导出
- **时间范围**: 建议指定时间范围以减少数据量

## 版本历史

- **Phase 173** (2026-03-27): 系统日志模块 - 系统日志导出 API
