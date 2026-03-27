# 共享文件夹详情 API 文档 (Phase 92)

## 概述

共享文件夹详情 API 提供单个共享文件夹的详细信息。

## 接口详情

### GET /api/v1/shares/{id}

获取指定共享文件夹的详细信息。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 共享文件夹 ID |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "public",
    "volume_id": 2,
    "volume_name": "data",
    "path": "/mnt/volumes/data/public",
    "description": "Public shared folder",
    "protocols": ["smb", "nfs"],
    "read_only": false,
    "guest_access": true,
    "status": "active",
    "created_at": 1710000000,
    "updated_at": 1710000000,
    "created_by": "admin"
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `data`: 共享文件夹详细信息
  - `id`: 共享文件夹 ID
  - `name`: 共享文件夹名称
  - `volume_id`: 所属存储卷 ID
  - `volume_name`: 所属存储卷名称
  - `path`: 文件夹路径
  - `description`: 共享文件夹描述
  - `protocols`: 支持的协议列表
  - `read_only`: 是否只读访问
  - `guest_access`: 是否允许访客访问
  - `status`: 状态（active/inactive）
  - `created_at`: 创建时间（Unix 时间戳）
  - `updated_at`: 更新时间（Unix 时间戳）
  - `created_by`: 创建者用户名

**错误响应 (404 Not Found) - 共享文件夹不存在**

```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "SHARE_NOT_FOUND"
}
```

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：获取共享文件夹详情

```bash
curl -X GET "http://localhost:8080/api/v1/shares/1" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 2：获取不存在的共享文件夹（404）

```bash
curl -X GET "http://localhost:8080/api/v1/shares/999" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "SHARE_NOT_FOUND"
}
```

### 示例 3：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/shares/1"
```

**响应：**
```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized
- 任意登录用户可访问，不需要 admin 权限

## 实现文件

- `src/handlers/shares_detail.rs` - 共享文件夹详情处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问，不需要 admin 权限
2. **404 处理**：共享文件夹不存在返回 404

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `SHARE_NOT_FOUND` | 404 | 共享文件夹不存在 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/shares** - 获取共享文件夹列表 (Phase 90)
- **POST /api/v1/shares** - 创建共享文件夹 (Phase 91)
- **PUT /api/v1/shares/{id}** - 更新共享文件夹
- **DELETE /api/v1/shares/{id}** - 删除共享文件夹

## 响应示例（完整）

### 成功响应

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "public",
    "volume_id": 2,
    "volume_name": "data",
    "path": "/mnt/volumes/data/public",
    "description": "Public shared folder",
    "protocols": ["smb", "nfs"],
    "read_only": false,
    "guest_access": true,
    "status": "active",
    "created_at": 1710000000,
    "updated_at": 1710000000,
    "created_by": "admin"
  }
}
```

### 共享文件夹不存在（404）

```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "SHARE_NOT_FOUND"
}
```

### 未认证（401）

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 字段说明

| 字段 | 说明 |
|------|------|
| `id` | 共享文件夹唯一标识符 |
| `name` | 共享文件夹名称 |
| `volume_id` | 所属存储卷 ID |
| `volume_name` | 所属存储卷名称 |
| `path` | 文件夹路径（绝对路径） |
| `description` | 共享文件夹描述 |
| `protocols` | 支持的协议列表 |
| `read_only` | 是否只读访问 |
| `guest_access` | 是否允许访客（匿名）访问 |
| `status` | 共享状态（active/inactive） |
| `created_at` | 创建时间戳 |
| `updated_at` | 更新时间戳 |
| `created_by` | 创建者用户名 |

## 协议说明

| 协议 | 说明 | 适用场景 |
|------|------|---------|
| `smb` | Server Message Block | Windows 文件共享 |
| `nfs` | Network File System | Linux/Unix 文件共享 |
| `afp` | Apple Filing Protocol | macOS 文件共享 |
| `ftp` | File Transfer Protocol | 通用文件传输 |

## 权限说明

| 字段 | 值 | 说明 |
|------|-----|------|
| `read_only` | `true` | 只读访问，用户不能修改文件 |
| `read_only` | `false` | 读写访问，用户可以修改文件 |
| `guest_access` | `true` | 允许访客（匿名）访问 |
| `guest_access` | `false` | 需要认证才能访问 |

## 状态说明

| 状态 | 说明 |
|------|------|
| `active` | 共享文件夹处于活动状态，可正常访问 |
| `inactive` | 共享文件夹已禁用，无法访问 |
