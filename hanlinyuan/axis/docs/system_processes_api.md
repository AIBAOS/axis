# 系统进程列表 API 文档

## 概述

本文档描述 Axis NAS 系统中获取系统进程列表 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/system/processes`
- **版本**: v1
- **Phase**: 251

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Query 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `limit` | number | 否 | 50 | 每页数量（最大 200） |
| `offset` | number | 否 | 0 | 偏移量 |
| `sort` | string | 否 | pid | 排序字段：cpu/memory/pid |
| `order` | string | 否 | desc | 排序顺序：asc/desc |
| `user` | string | 否 | - | 按用户筛选 |
| `status` | string | 否 | - | 按状态筛选：running/sleeping/zombie |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "pid": 1234,
      "name": "nginx",
      "user": "www-data",
      "cpu_percent": 2.5,
      "memory_percent": 1.2,
      "status": "running",
      "start_time": 1711500000,
      "command": "nginx: master process"
    }
  ],
  "pagination": {
    "limit": 50,
    "offset": 0,
    "total": 150
  }
}
```

### 错误响应

#### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can view system processes",
  "code": "FORBIDDEN"
}
```

#### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "Invalid sort field. Valid fields: cpu, memory, pid",
  "code": "INVALID_SORT"
}
```

#### 500 Internal Server Error - 系统错误

```json
{
  "success": false,
  "error": "Failed to get current time",
  "code": "INTERNAL_ERROR"
}
```

## 数据模型

### ProcessStatus

进程状态枚举值：
- `running` - 运行中
- `sleeping` - 睡眠中
- `zombie` - 僵尸进程
- `stopped` - 已停止

### ProcessInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `pid` | number | 进程 ID |
| `name` | string | 进程名称 |
| `user` | string | 所属用户 |
| `cpu_percent` | number | CPU 使用率百分比（0-100） |
| `memory_percent` | number | 内存使用率百分比（0-100） |
| `status` | ProcessStatus | 进程状态 |
| `start_time` | number | 启动时间戳（Unix 时间戳） |
| `command` | string | 完整命令行 |

### PaginationInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `limit` | number | 每页数量 |
| `offset` | number | 偏移量 |
| `total` | number | 总进程数 |

### ProcessesResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | ProcessInfo[] | 进程列表 |
| `pagination` | PaginationInfo | 分页信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试访问 |
| `INVALID_SORT` | 400 | 无效的排序字段 |
| `INVALID_ORDER` | 400 | 无效的排序顺序 |
| `INVALID_STATUS` | 400 | 无效的进程状态 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求（默认参数）

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": [
    {
      "pid": 1,
      "name": "systemd",
      "user": "root",
      "cpu_percent": 0.1,
      "memory_percent": 0.5,
      "status": "sleeping",
      "start_time": 1711500000,
      "command": "/sbin/init"
    },
    {
      "pid": 2345,
      "name": "postgres",
      "user": "postgres",
      "cpu_percent": 5.0,
      "memory_percent": 8.5,
      "status": "running",
      "start_time": 1711500000,
      "command": "postgres: main process"
    }
  ],
  "pagination": {
    "limit": 50,
    "offset": 0,
    "total": 150
  }
}
```

### 请求（按 CPU 使用率降序排序，限制 10 条）

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes?sort=cpu&order=desc&limit=10" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

### 请求（筛选特定用户的运行中进程）

```bash
curl -X GET "http://localhost:8080/api/v1/system/processes?user=www-data&status=running" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

## 权限说明

- **Admin 用户**: 可访问系统进程列表
- **普通用户**: 无权访问（返回 403 Forbidden）

## 实现细节

### 排序说明
- **pid**: 按进程 ID 排序（默认）
- **cpu**: 按 CPU 使用率排序
- **memory**: 按内存使用率排序
- **order**: asc（升序）/ desc（降序，默认）

### 筛选说明
- **user**: 按用户名精确匹配
- **status**: 按进程状态筛选（running/sleeping/zombie）

### 数据来源
- 当前为模拟实现，返回固定进程数据
- 实际实现可：
  - 读取 /proc/[pid]/stat 获取进程信息
  - 读取 /proc/[pid]/statm 获取内存信息
  - 使用 ps 命令获取进程列表

## 相关接口

- `GET /api/v1/system/info` - 获取系统信息
- `GET /api/v1/system/resources` - 获取系统资源使用情况
- `GET /api/v1/system/logs` - 获取系统日志

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取进程列表
curl -X GET "http://localhost:8080/api/v1/system/processes" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 进程列表

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/system/processes"

# 预期：401 Unauthorized

# 测试排序和筛选
curl -X GET "http://localhost:8080/api/v1/system/processes?sort=cpu&order=desc&limit=10&status=running" \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"

# 预期：200 OK + 按 CPU 降序排序的运行中进程
```

## 版本历史

- **Phase 251** (2026-03-28): 初始实现，模拟进程数据
