# 系统进程列表 API

## Phase 251

## 接口说明

获取 NAS 系统进程列表，用于 Web UI 系统监控展示，仅限 admin 角色访问。

## 请求

`GET /api/v1/system/processes`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| limit | integer | 否 | 50 | 返回数量（最大 200） |
| offset | integer | 否 | 0 | 偏移量 |
| sort | string | 否 | pid | 排序字段（cpu/memory/pid） |
| order | string | 否 | desc | 排序顺序（asc/desc） |
| user | string | 否 | - | 按用户筛选 |
| status | string | 否 | - | 按状态筛选（running/sleeping/zombie） |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": [
    {
      "pid": 5678,
      "name": "postgres",
      "user": "postgres",
      "cpu_percent": 5.0,
      "memory_percent": 8.5,
      "status": "running",
      "start_time": 1711544400,
      "command": "postgres: main process"
    },
    {
      "pid": 1234,
      "name": "nginx",
      "user": "www-data",
      "cpu_percent": 2.5,
      "memory_percent": 1.2,
      "status": "running",
      "start_time": 1711544400,
      "command": "nginx: master process"
    }
  ],
  "total": 150,
  "limit": 50,
  "offset": 0
}
```

### 返回字段说明

#### 进程信息

| 字段 | 类型 | 说明 |
|------|------|------|
| pid | integer | 进程 ID |
| name | string | 进程名称 |
| user | string | 运行用户 |
| cpu_percent | float | CPU 使用率百分比（0-100） |
| memory_percent | float | 内存使用率百分比（0-100） |
| status | string | 进程状态（running/sleeping/zombie/stopped） |
| start_time | integer | 启动时间（Unix 时间戳） |
| command | string | 完整命令行 |

#### 分页信息

| 字段 | 类型 | 说明 |
|------|------|------|
| total | integer | 总进程数 |
| limit | integer | 每页数量 |
| offset | integer | 偏移量 |

### 错误响应

#### 400 Bad Request - 参数格式错误

```json
{
  "success": false,
  "error": "Invalid sort field. Must be cpu, memory, or pid",
  "code": "INVALID_SORT"
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

#### 403 Forbidden - 权限不足（非 admin）

```json
{
  "success": false,
  "error": "Only admin users can view system processes",
  "code": "FORBIDDEN"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to get system processes: io error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取进程列表（默认参数）

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": [
    {
      "pid": 5678,
      "name": "postgres",
      "user": "postgres",
      "cpu_percent": 5.0,
      "memory_percent": 8.5,
      "status": "running",
      "start_time": 1711544400,
      "command": "postgres: main process"
    }
  ],
  "total": 150,
  "limit": 50,
  "offset": 0
}
```

### 按 CPU 使用率降序排序

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes?sort=cpu&order=desc&limit=10" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 筛选特定用户的进程

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes?user=postgres" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 筛选运行中的进程

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes?status=running" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 分页查询（第 2 页，每页 20 条）

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes?limit=20&offset=20" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 非 admin 用户获取进程列表

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view system processes",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes"
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 无效的排序字段

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes?sort=invalid" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid sort field. Must be cpu, memory, or pid",
  "code": "INVALID_SORT"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析并验证查询参数
4. 从系统读取进程信息
5. 应用筛选条件（user/status）
6. 应用排序（cpu/memory/pid）
7. 应用分页
8. 返回进程列表

## 进程状态说明

| 状态 | 说明 |
|------|------|
| running | 运行中 |
| sleeping | 睡眠中（等待事件） |
| zombie | 僵尸进程（已终止但未回收） |
| stopped | 已停止 |

## 查询参数说明

| 参数 | 说明 | 可选值 | 默认值 |
|------|------|--------|--------|
| limit | 返回数量 | 1-200 | 50 |
| offset | 偏移量 | ≥0 | 0 |
| sort | 排序字段 | cpu/memory/pid | pid |
| order | 排序顺序 | asc/desc | desc |
| user | 用户筛选 | 用户名 | - |
| status | 状态筛选 | running/sleeping/zombie | - |

## 安全说明

- 此接口仅限 admin 用户调用
- 进程信息包含系统敏感信息，建议添加访问审计
- 建议限制调用频率防止资源消耗（建议间隔≥5 秒）

## 版本历史

- **Phase 251** (2026-03-28): 系统模块 - 系统进程列表 API
