# 容器日志 API

## Phase 149

## 接口说明

查看指定容器的日志输出。

## 请求

`GET /api/v1/containers/{id}/logs`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 容器 ID |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ---- | ---- |
| tail | u32 | 否 | 100 | 返回最后 N 行日志（最大 1000） |
| since | u64 | 否 | - | 时间戳过滤，返回此时间之后的日志 |
| follow | boolean | 否 | false | 流式输出（本次实现暂不支持，始终返回完整日志） |

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
    "container_id": 1,
    "logs": [
      "2026-03-27T06:00:00Z [INFO] Container nginx-web started",
      "2026-03-27T06:00:01Z [INFO] Initializing application...",
      "2026-03-27T06:00:02Z [INFO] Loading configuration...",
      "2026-03-27T06:00:03Z [INFO] Configuration loaded successfully",
      "2026-03-27T06:00:04Z [INFO] Starting server...",
      "2026-03-27T06:00:05Z [INFO] Server listening on port 80",
      "2026-03-27T06:00:10Z [INFO] Received request GET /",
      "2026-03-27T06:00:10Z [INFO] Response sent 200 OK",
      "2026-03-27T06:00:15Z [WARN] High memory usage detected",
      "2026-03-27T06:00:20Z [INFO] Garbage collection completed"
    ],
    "lines_count": 10
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

## 示例

### 获取容器最后 100 行日志（默认）

```bash
curl "http://localhost:8080/api/v1/containers/1/logs" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取容器最后 50 行日志

```bash
curl "http://localhost:8080/api/v1/containers/1/logs?tail=50" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取指定时间之后的日志

```bash
curl "http://localhost:8080/api/v1/containers/1/logs?since=1711500000" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的容器日志

```bash
curl "http://localhost:8080/api/v1/containers/999/logs" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Container 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证容器 ID 存在性（404 Not Found）
4. 解析查询参数（tail/since/follow）
5. 调用 Docker API 获取容器日志
6. 应用 tail 参数过滤（返回最后 N 行）
7. 应用 since 参数过滤（时间戳过滤）
8. 返回 200 OK + 日志数据

## 响应字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| container_id | u64 | 容器 ID |
| logs | string[] | 日志行数组，每行一个字符串 |
| lines_count | u32 | 返回的日志行数 |

## 版本历史

- **Phase 149** (2026-03-27): 初始版本（非流式）
