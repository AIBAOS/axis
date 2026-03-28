# 容器停止 API

## Phase 242

## 接口说明

停止指定的容器，仅限 admin 角色访问。

## 请求

`POST /api/v1/containers/{id}/stop`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 容器 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "Container 'nginx-web' stopped successfully",
  "data": {
    "id": 1,
    "container_id": 1,
    "status": "stopped"
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| message | string | 响应消息 |
| data | object | 容器状态信息 |
| data.container_id | integer | 容器 ID |
| data.status | string | 容器状态（stopped） |

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
  "error": "Only admin users can stop containers",
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

#### 409 Conflict - 容器已停止

```json
{
  "success": false,
  "error": "Container 'nginx-web' is already stopped",
  "code": "ALREADY_STOPPED"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to stop container: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 停止容器

```bash
curl -X POST "http://localhost:8080/api/v1/containers/1/stop" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "message": "Container 'nginx-web' stopped successfully",
  "data": {
    "id": 1,
    "container_id": 1,
    "status": "stopped"
  }
}
```

### 停止已停止的容器

```bash
curl -X POST "http://localhost:8080/api/v1/containers/2/stop" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "Container 'postgres-db' is already stopped",
  "code": "ALREADY_STOPPED"
}
```

### 停止不存在的容器

```bash
curl -X POST "http://localhost:8080/api/v1/containers/999/stop" \
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

### 非 admin 用户停止容器

```bash
curl -X POST "http://localhost:8080/api/v1/containers/1/stop" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can stop containers",
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
4. 查询容器详情及状态
5. 容器不存在返回 404 Not Found
6. 容器已停止返回 409 Conflict
7. 调用 Docker/LXC API 停止容器
8. 返回停止成功响应

## 安全说明

- 此接口仅限 admin 用户调用
- 停止容器可能导致服务中断，建议添加警告提示
- 建议添加操作审计日志

## 版本历史

- **Phase 146** (2026-03-27): 容器模块 - 容器停止 API 初始实现
- **Phase 242** (2026-03-28): 容器模块 - 容器停止 API 增强版
