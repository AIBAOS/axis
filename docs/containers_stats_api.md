# 容器统计信息 API

## Phase 150

## 接口说明

获取指定容器的实时统计信息，包括 CPU、内存、网络、磁盘等资源使用情况。

## 请求

`GET /api/v1/containers/{id}/stats`

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
    "container_id": 1,
    "cpu_percent": 2.5,
    "memory_usage_bytes": 134217728,
    "memory_limit_bytes": 536870912,
    "memory_percent": 25.0,
    "network_rx_bytes": 1048576,
    "network_tx_bytes": 524288,
    "block_read_bytes": 2097152,
    "block_write_bytes": 1048576,
    "pids": 10,
    "status": "running"
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
  "error": "Only admin users can view container stats",
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

### 获取容器统计信息

```bash
curl "http://localhost:8080/api/v1/containers/1/stats" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取已停止容器的统计信息

```bash
curl "http://localhost:8080/api/v1/containers/3/stats" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（已停止容器返回零统计）：
```json
{
  "success": true,
  "data": {
    "container_id": 3,
    "cpu_percent": 0.0,
    "memory_usage_bytes": 0,
    "memory_limit_bytes": 0,
    "memory_percent": 0.0,
    "network_rx_bytes": 0,
    "network_tx_bytes": 0,
    "block_read_bytes": 0,
    "block_write_bytes": 0,
    "pids": 0,
    "status": "stopped"
  }
}
```

### 获取不存在的容器统计信息

```bash
curl "http://localhost:8080/api/v1/containers/999/stats" \
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

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| container_id | u64 | 容器 ID |
| cpu_percent | f64 | CPU 使用率百分比 |
| memory_usage_bytes | u64 | 内存使用量（字节） |
| memory_limit_bytes | u64 | 内存限制（字节） |
| memory_percent | f64 | 内存使用率百分比 |
| network_rx_bytes | u64 | 网络接收字节数 |
| network_tx_bytes | u64 | 网络发送字节数 |
| block_read_bytes | u64 | 磁盘读取字节数 |
| block_write_bytes | u64 | 磁盘写入字节数 |
| pids | u32 | 进程数 |
| status | string | 容器状态（running/stopped） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证容器 ID 存在性（404 Not Found）
4. 调用 Docker API 获取容器统计信息
5. 返回 200 OK + 统计信息数据

## 版本历史

- **Phase 150** (2026-03-27): 初始版本
