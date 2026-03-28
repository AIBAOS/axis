# 容器创建 API

## Phase 169

## 接口说明

创建新的容器。

## 请求

`POST /api/v1/containers`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

```json
{
  "name": "my-app",
  "image": "node:18",
  "ports": ["3000:3000"],
  "networks": ["bridge"],
  "env": ["NODE_ENV=production"]
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 是 | 容器名称（1-128 字符） |
| image | string | 是 | 镜像名称（1-256 字符） |
| ports | string[] | 否 | 端口映射列表 |
| networks | string[] | 否 | 网络列表（默认 bridge） |
| env | string[] | 否 | 环境变量列表 |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "Container created successfully",
  "data": {
    "id": 6,
    "name": "my-app",
    "image": "node:18",
    "status": "created",
    "ports": ["3000:3000"],
    "networks": ["bridge"],
    "created_at": "2026-03-27T11:00:00Z"
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
  "error": "Only admin users can create containers",
  "code": "FORBIDDEN"
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

### 创建容器

```bash
curl -X POST "http://localhost:8080/api/v1/containers" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-app",
    "image": "node:18",
    "ports": ["3000:3000"],
    "networks": ["bridge"],
    "env": ["NODE_ENV=production"]
  }'
```

### 创建最小化容器

```bash
curl -X POST "http://localhost:8080/api/v1/containers" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "minimal-app",
    "image": "alpine:latest"
  }'
```

### 尝试创建已存在的容器

```bash
curl -X POST "http://localhost:8080/api/v1/containers" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "nginx-web",
    "image": "nginx:latest"
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

### 创建结果字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| success | boolean | 是否成功 |
| message | string | 响应消息 |
| data | object | 创建的容器信息 |

### 容器信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 容器 ID |
| name | string | 容器名称 |
| image | string | 镜像名称 |
| status | string | 状态（created/running/stopped/paused） |
| ports | string[] | 端口映射列表 |
| networks | string[] | 网络列表 |
| created_at | string | 创建时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证容器名称格式（1-128 字符）
4. 验证镜像名称格式（1-256 字符）
5. 验证名称唯一性
6. 创建容器
7. 返回 201 Created + 容器详情

## 版本历史

- **Phase 169** (2026-03-27): 容器管理模块 - 容器创建 API
