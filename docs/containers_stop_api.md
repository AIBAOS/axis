# 容器停止 API

## Phase 146

## 接口说明

停止指定容器。

## 请求

`POST /api/v1/containers/{id}/stop`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 容器 ID |

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
  "message": "Container stopped successfully",
  "data": {
    "id": 1,
    "name": "nginx-web",
    "image": "nginx:latest",
    "status": "stopped",
    "ports": ["80:80", "443:443"],
    "networks": ["bridge"],
    "created_at": 1711500000,
    "cpu_usage": null,
    "memory_usage": null
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

## 示例

### 请求示例

```bash
curl -X POST "http://localhost:8080/api/v1/containers/1/stop" \
  -H "Authorization: Bearer <jwt_token>"
```

### 响应示例（200 OK）

```json
{
  "success": true,
  "message": "Container stopped successfully",
  "data": {
    "id": 1,
    "name": "nginx-web",
    "image": "nginx:latest",
    "status": "stopped",
    "ports": ["80:80", "443:443"],
    "networks": ["bridge"],
    "created_at": 1711500000,
    "cpu_usage": null,
    "memory_usage": null
  }
}
```

### 响应示例（409 Conflict）

```json
{
  "success": false,
  "error": "Container 'nginx-web' is already stopped",
  "code": "ALREADY_STOPPED"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证容器 ID 存在性（404 Not Found）
4. 验证容器当前状态（已停止则返回 409 Conflict）
5. 调用 Docker API 停止容器
6. 返回 200 OK + 容器详情

## 版本历史

- **Phase 146** (2026-03-27): 初始版本
