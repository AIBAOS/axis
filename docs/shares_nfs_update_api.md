# NFS 共享更新 API 文档

## 概述

本文档描述 Axis NAS 系统中更新 NFS 共享 API 的实现细节。

## API 端点

- **路径**: `PUT /api/v1/shares/nfs/{id}`
- **版本**: v1
- **Phase**: 205

## 认证

- **类型**: JWT Bearer Token
- **权限**: 仅 Admin 用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `id` | number | 是 | NFS 共享 ID |

### Request Body

```json
{
  "name": "string (optional)",
  "path": "string (optional)",
  "comment": "string (optional)",
  "read_only": "boolean (optional)",
  "no_subtree_check": "boolean (optional)",
  "sync": "boolean (optional)",
  "clients": [
    {
      "network": "string (CIDR)",
      "access": "string (ro/rw)"
    }
  ]
}
```

### 字段说明

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `name` | string | 否 | 共享名称（1-64 字符） |
| `path` | string | 否 | 共享路径（必须以 / 开头） |
| `comment` | string | 否 | 共享描述 |
| `read_only` | boolean | 否 | 是否只读 |
| `no_subtree_check` | boolean | 否 | 不检查子树 |
| `sync` | boolean | 否 | 同步写入 |
| `clients` | array | 否 | 客户端配置列表 |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "NFS share updated successfully",
  "data": {
    "id": 1,
    "name": "Home",
    "path": "/srv/nfs/home",
    "comment": "用户主目录共享",
    "read_only": false,
    "no_subtree_check": true,
    "sync": true,
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      }
    ],
    "enabled": true,
    "status": "active",
    "created_at": 1711500000,
    "updated_at": 1711600000
  }
}
```

### 错误响应

#### 404 Not Found - 共享不存在

```json
{
  "success": false,
  "error": "NFS share 999 not found",
  "code": "NOT_FOUND"
}
```

#### 400 Bad Request - 名称格式无效

```json
{
  "success": false,
  "error": "Invalid share name. Must be 1-64 chars, alphanumeric with -_. allowed",
  "code": "INVALID_NAME"
}
```

#### 400 Bad Request - 路径格式无效

```json
{
  "success": false,
  "error": "Invalid share path. Must start with / and be <= 256 chars",
  "code": "INVALID_PATH"
}
```

#### 400 Bad Request - 客户端配置无效

```json
{
  "success": false,
  "error": "Invalid client network '192.168.1'. Must be CIDR format (e.g., 192.168.1.0/24)",
  "code": "INVALID_NETWORK"
}
```

#### 409 Conflict - 名称已存在

```json
{
  "success": false,
  "error": "Share name 'Home' already exists",
  "code": "NAME_CONFLICT"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can update NFS shares",
  "code": "FORBIDDEN"
}
```

#### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

## 数据模型

### UpdateNfsShareRequest

| 字段 | 类型 | 描述 |
|------|------|------|
| `name` | string? | 共享名称 |
| `path` | string? | 共享路径 |
| `comment` | string? | 共享描述 |
| `read_only` | boolean? | 是否只读 |
| `no_subtree_check` | boolean? | 不检查子树 |
| `sync` | boolean? | 同步写入 |
| `clients` | NfsClientConfig[]? | 客户端配置列表 |

### NfsClientConfig

| 字段 | 类型 | 描述 |
|------|------|------|
| `network` | string | 网络 CIDR（如 192.168.1.0/24） |
| `access` | string | 访问权限：`ro` / `rw` |

### NfsShareInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 共享 ID |
| `name` | string | 共享名称 |
| `path` | string | 共享路径 |
| `comment` | string? | 共享描述 |
| `read_only` | boolean | 是否只读 |
| `no_subtree_check` | boolean | 不检查子树 |
| `sync` | boolean | 同步写入 |
| `clients` | NfsClientConfig[] | 客户端配置列表 |
| `enabled` | boolean | 是否启用 |
| `status` | string | 状态：`active` / `inactive` |
| `created_at` | number | 创建时间（Unix 时间戳） |
| `updated_at` | number | 更新时间（Unix 时间戳） |

### UpdateNfsShareResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data` | NfsShareInfo | 更新后的共享信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 403 | 非 admin 用户尝试更新 |
| `NOT_FOUND` | 404 | 共享不存在或非 NFS 协议 |
| `INVALID_NAME` | 400 | 共享名称格式无效 |
| `INVALID_PATH` | 400 | 共享路径格式无效 |
| `INVALID_CLIENTS` | 400 | 客户端配置无效 |
| `INVALID_NETWORK` | 400 | 客户端网络格式无效 |
| `INVALID_ACCESS` | 400 | 客户端访问权限无效 |
| `NAME_CONFLICT` | 409 | 共享名称已存在 |
| `DATABASE_ERROR` | 500 | 数据库操作失败 |

## 示例

### 请求

```bash
curl -X PUT "http://localhost:8080/api/v1/shares/nfs/1" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Home",
    "comment": "更新后的用户主目录共享",
    "read_only": false,
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      }
    ]
  }'
```

### 响应

```json
{
  "success": true,
  "message": "NFS share updated successfully",
  "data": {
    "id": 1,
    "name": "Home",
    "path": "/srv/nfs/home",
    "comment": "更新后的用户主目录共享",
    "read_only": false,
    "no_subtree_check": true,
    "sync": true,
    "clients": [
      {
        "network": "192.168.1.0/24",
        "access": "rw"
      }
    ],
    "enabled": true,
    "status": "active",
    "created_at": 1711500000,
    "updated_at": 1711600000
  }
}
```

## 权限说明

- **Admin 用户**: 可更新任意 NFS 共享
- **普通用户**: 无权访问（返回 403 Forbidden）

## 验证规则

### 名称验证
- 长度：1-64 字符
- 允许字符：字母、数字、`-`、`_`、`.`
- 必须唯一（排除自身）

### 路径验证
- 必须以 `/` 开头
- 最大长度：256 字符

### 客户端配置验证
- 至少一个客户端配置
- `network`: CIDR 格式（如 192.168.1.0/24）
- `access`: 必须为 `ro` 或 `rw`

## 实现细节

- **部分更新**: 仅更新提供的字段
- **协议验证**: 仅允许更新 `protocol = 'nfs'` 的共享
- **名称唯一性**: 排除自身检查
- **数据库**: SQLite
- **框架**: Actix-web
- **仓库**: SqliteShareRepository

## 相关接口

- `GET /api/v1/shares/nfs` - 获取 NFS 共享列表
- `GET /api/v1/shares/nfs/{id}` - 获取 NFS 共享详情
- `POST /api/v1/shares/nfs` - 创建 NFS 共享
- `DELETE /api/v1/shares/nfs/{id}` - 删除 NFS 共享

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test
```

## 版本历史

- **Phase 205** (2026-03-28): 初始实现，从 mock 数据升级为 SQLite 持久化
