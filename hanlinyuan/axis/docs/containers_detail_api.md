# 容器详情 API

## Phase 167

## 接口说明

获取指定容器的详细信息。

## 请求

`GET /api/v1/containers/{id}`

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
  "data": {
    "id": 1,
    "name": "nginx-web",
    "image": "nginx:latest",
    "status": "running",
    "ports": ["80:80", "443:443"],
    "networks": ["bridge"],
    "created_at": "2026-03-27T06:00:00Z",
    "started_at": "2026-03-27T06:00:05Z",
    "cpu_usage": 2.5,
    "memory_usage": 134217728
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
  "error": "Only admin users can view container details",
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

### 获取容器详情

```bash
curl "http://localhost:8080/api/v1/containers/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的容器

```bash
curl "http://localhost:8080/api/v1/containers/999" \
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

## 响应字段说明

### 容器详情字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 容器 ID |
| name | string | 容器名称 |
| image | string | 镜像名称 |
| status | string | 状态（running/stopped/paused） |
| ports | string[] | 端口映射列表 |
| networks | string[] | 网络列表 |
| created_at | string | 创建时间（ISO 8601 格式） |
| started_at | string\|null | 启动时间（ISO 8601 格式） |
| cpu_usage | f64\|null | CPU 使用率（百分比） |
| memory_usage | u64\|null | 内存使用量（字节） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据容器 ID 查找容器
4. 容器不存在返回 404 Not Found
5. 返回 200 OK + 容器详情

## 版本历史

- **Phase 167** (2026-03-27): 容器管理模块 - 容器详情 API
