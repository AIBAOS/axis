# 容器更新 API

## Phase 170

## 接口说明

更新指定容器的配置，支持部分字段更新。

## 请求

`PUT /api/v1/containers/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 容器 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

所有字段均为可选，支持部分更新：

```json
{
  "name": "updated-app",
  "image": "node:20",
  "ports": ["3000:3000", "3001:3001"],
  "networks": ["bridge"],
  "env": ["NODE_ENV=production"],
  "cpu_limit": 2.0,
  "memory_limit": 536870912
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 否 | 容器名称（1-128 字符） |
| image | string | 否 | 镜像名称（1-256 字符） |
| ports | string[] | 否 | 端口映射列表 |
| networks | string[] | 否 | 网络列表 |
| env | string[] | 否 | 环境变量列表 |
| cpu_limit | number | 否 | CPU 限制（核心数） |
| memory_limit | number | 否 | 内存限制（字节） |

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "Container updated successfully",
  "data": {
    "id": 1,
    "name": "updated-app",
    "image": "node:20",
    "status": "running",
    "ports": ["3000:3000", "3001:3001"],
    "networks": ["bridge"],
    "env": ["NODE_ENV=production"],
    "cpu_limit": 2.0,
    "memory_limit": 536870912,
    "created_at": "2026-03-27T06:00:00Z",
    "updated_at": "2026-03-27T11:30:00Z"
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid container name. Must be 1-128 chars",
  "code": "INVALID_NAME"
}
```

```json
{
  "success": false,
  "error": "Invalid image name. Must be 1-256 chars",
  "code": "INVALID_IMAGE"
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
  "error": "Only admin users can update containers",
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

#### 409 Conflict - 名称冲突

```json
{
  "success": false,
  "error": "Container name 'nginx-web' already exists",
  "code": "NAME_CONFLICT"
}
```

## 示例

### 更新容器名称

```bash
curl -X PUT "http://localhost:8080/api/v1/containers/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "updated-nginx"
  }'
```

### 更新多个字段

```bash
curl -X PUT "http://localhost:8080/api/v1/containers/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "updated-app",
    "image": "node:20",
    "cpu_limit": 2.0,
    "memory_limit": 536870912
  }'
```

### 更新环境变量

```bash
curl -X PUT "http://localhost:8080/api/v1/containers/2" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "env": ["POSTGRES_PASSWORD=newsecret", "POSTGRES_USER=admin"]
  }'
```

### 尝试更新不存在的容器

```bash
curl -X PUT "http://localhost:8080/api/v1/containers/999" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "new-name"
  }'
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Container 999 not found",
  "code": "NOT_FOUND"
}
```

### 尝试使用已存在的名称

```bash
curl -X PUT "http://localhost:8080/api/v1/containers/2" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "nginx-web"
  }'
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "Container name 'nginx-web' already exists",
  "code": "NAME_CONFLICT"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 更新结果字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| success | boolean | 是否成功 |
| message | string | 响应消息 |
| data | object | 更新后的容器信息 |

### 容器信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 容器 ID |
| name | string | 容器名称 |
| image | string | 镜像名称 |
| status | string | 状态（created/running/stopped/paused） |
| ports | string[] | 端口映射列表 |
| networks | string[] | 网络列表 |
| env | string[] | 环境变量列表 |
| cpu_limit | number\|null | CPU 限制（核心数） |
| memory_limit | number\|null | 内存限制（字节） |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证容器 ID 存在性（404 Not Found）
4. 验证名称格式（如果提供）
5. 验证镜像格式（如果提供）
6. 验证名称唯一性（排除自身）
7. 部分更新容器配置
8. 更新时间戳
9. 返回 200 OK + 更新后的容器详情

## 版本历史

- **Phase 170** (2026-03-27): 容器管理模块 - 容器更新 API
