# 系统日志列表 API

## Phase 257

## 接口说明

获取系统日志列表，用于 Web UI 系统监控展示，仅限 admin 角色访问。

## 请求

`GET /api/v1/system/logs`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| page | integer | 否 | 1 | 页码 |
| page_size | integer | 否 | 20 | 每页数量（最大 100） |
| level | string | 否 | - | 日志级别过滤（debug/info/warn/error） |

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
      "level": "info",
      "message": "System started successfully",
      "source": "system",
      "created_at": 1711634400
    },
    {
      "id": 2,
      "level": "warn",
      "message": "High memory usage detected: 85%",
      "source": "monitor",
      "created_at": 1711634100
    }
  ],
  "total": 150,
  "page": 1,
  "page_size": 20
}
```

### 返回字段说明

#### 日志条目

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 日志 ID |
| level | string | 日志级别（debug/info/warn/error） |
| message | string | 日志消息 |
| source | string | 日志来源（模块名） |
| created_at | integer | 创建时间（Unix 时间戳） |

#### 列表信息

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | array | 日志列表 |
| total | integer | 总日志数 |
| page | integer | 当前页码 |
| page_size | integer | 每页数量 |

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
  "error": "Only admin users can view system logs",
  "code": "FORBIDDEN"
}
```

#### 400 Bad Request - 参数格式错误

```json
{
  "success": false,
  "error": "Invalid level. Must be debug, info, warn, or error",
  "code": "INVALID_LEVEL"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to get system logs: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取系统日志列表（默认参数）

```bash
curl -X GET "http://localhost:8080/api/v1/system/logs" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "level": "info",
      "message": "System started successfully",
      "source": "system",
      "created_at": 1711634400
    }
  ],
  "total": 150,
  "page": 1,
  "page_size": 20
}
```

### 按级别筛选日志

```bash
curl -X GET "http://localhost:8080/api/v1/system/logs?level=error" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 分页查询（第 2 页，每页 50 条）

```bash
curl -X GET "http://localhost:8080/api/v1/system/logs?page=2&page_size=50" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 组合筛选

```bash
curl -X GET "http://localhost:8080/api/v1/system/logs?level=warn&page=1&page_size=100" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 非 admin 用户获取日志

```bash
curl -X GET "http://localhost:8080/api/v1/system/logs" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view system logs",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X GET "http://localhost:8080/api/v1/system/logs"
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 无效的日志级别

```bash
curl -X GET "http://localhost:8080/api/v1/system/logs?level=invalid" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid level. Must be debug, info, warn, or error",
  "code": "INVALID_LEVEL"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析并验证查询参数
4. 从数据库读取系统日志
5. 应用级别过滤
6. 应用分页
7. 返回日志列表（时间倒序）

## 日志级别说明

| 级别 | 说明 | 使用场景 |
|------|------|----------|
| debug | 调试 | 开发调试信息 |
| info | 信息 | 正常系统运行信息 |
| warn | 警告 | 需要注意但不影响运行的问题 |
| error | 错误 | 影响功能的错误 |

## 安全说明

- 此接口仅限 admin 用户调用
- 系统日志包含敏感信息，建议添加访问审计
- 建议限制调用频率防止资源消耗

## 版本历史

- **Phase 170** (2026-03-27): 系统模块 - 系统日志列表 API 初始实现
- **Phase 257** (2026-03-28): 系统模块 - 系统日志列表 API 增强版
