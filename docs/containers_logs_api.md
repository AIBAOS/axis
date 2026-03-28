# 容器日志 API

## Phase 244

## 接口说明

获取指定容器的日志，仅限 admin 角色访问。

## 请求

`GET /api/v1/containers/{id}/logs`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 容器 ID |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| tail | integer | 否 | 100 | 返回最近 N 行日志（最大 1000） |
| since | integer | 否 | - | 从此时间戳开始（Unix 时间戳） |
| follow | boolean | 否 | false | 是否实时跟踪（当前不支持） |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "container_id": 1,
    "logs": [
      "2026-03-27T06:00:00Z [INFO] Container nginx-web started",
      "2026-03-27T06:00:01Z [INFO] Initializing application...",
      "2026-03-27T06:00:02Z [INFO] Application ready"
    ],
    "lines_count": 3
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | object | 日志数据 |
| data.container_id | integer | 容器 ID |
| data.logs | array | 日志行数组（字符串） |
| data.lines_count | integer | 日志行数 |

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
  "error": "Only admin users can view container logs",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 容器不存在

```json
{
  "success": false,
  "error": "Container 999 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to get container logs: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取容器日志（最近 100 行）

```bash
curl -X GET "http://localhost:8080/api/v1/containers/1/logs" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "container_id": 1,
    "logs": [
      "2026-03-27T06:00:00Z [INFO] Container nginx-web started",
      "2026-03-27T06:00:01Z [INFO] Initializing application...",
      "2026-03-27T06:00:02Z [INFO] Application ready"
    ],
    "lines_count": 3
  }
}
```

### 获取最近 50 行日志

```bash
curl -X GET "http://localhost:8080/api/v1/containers/1/logs?tail=50" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 获取不存在的容器日志

```bash
curl -X GET "http://localhost:8080/api/v1/containers/999/logs" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Container 999 not found",
  "code": "NOT_FOUND"
}
```

### 非 admin 用户获取日志

```bash
curl -X GET "http://localhost:8080/api/v1/containers/1/logs" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view container logs",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析容器 ID 路径参数
4. 解析查询参数（tail/since/follow）
5. 查询容器详情
6. 容器不存在返回 404 Not Found
7. 调用 Docker/LXC API 获取容器日志
8. 返回日志数据

## 查询参数说明

| 参数 | 说明 | 默认值 | 最大值 |
|------|------|--------|--------|
| tail | 返回最近 N 行日志 | 100 | 1000 |
| since | 从此时间戳开始的日志 | - | - |
| follow | 是否实时跟踪日志 | false | - |

## 安全说明

- 此接口仅限 admin 用户调用
- 容器日志可能包含敏感信息，建议添加访问审计
- 建议限制 tail 参数最大值防止资源耗尽

## 版本历史

- **Phase 149** (2026-03-27): 容器模块 - 容器日志 API 初始实现
- **Phase 244** (2026-03-28): 容器模块 - 容器日志 API 增强版
