# 共享文件夹列表 API 文档 (Phase 90)

## 概述

共享文件夹列表 API 提供系统中所有共享文件夹的信息列表，支持分页和筛选。

## 接口详情

### GET /api/v1/shares

获取共享文件夹列表。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `page` | integer | 否 | 1 | 页码（从 1 开始） |
| `per_page` | integer | 否 | 20 | 每页数量（最大 100） |
| `protocol` | string | 否 | - | 协议筛选（smb/nfs/afp/ftp） |
| `volume_id` | integer | 否 | - | 存储卷 ID 筛选 |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "public",
      "path": "/mnt/volumes/data/public",
      "volume_id": 2,
      "volume_name": "data",
      "description": "Public shared folder",
      "protocols": ["smb", "nfs"],
      "read_only": false,
      "guest_access": true,
      "created_at": 1710000000,
      "updated_at": 1710000000
    },
    {
      "id": 2,
      "name": "media",
      "path": "/mnt/volumes/data/media",
      "volume_id": 2,
      "volume_name": "data",
      "description": "Media files shared folder",
      "protocols": ["smb", "nfs", "afp"],
      "read_only": false,
      "guest_access": false,
      "created_at": 1710000000,
      "updated_at": 1710000000
    }
  ],
  "pagination": {
    "total": 2,
    "page": 1,
    "per_page": 20,
    "total_pages": 1
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `data`: 共享文件夹列表
  - `id`: 共享文件夹 ID
  - `name`: 共享文件夹名称
  - `path`: 文件夹路径
  - `volume_id`: 所属存储卷 ID
  - `volume_name`: 所属存储卷名称
  - `description`: 共享文件夹描述
  - `protocols`: 支持的协议列表
  - `read_only`: 是否只读
  - `guest_access`: 是否允许访客访问
  - `created_at`: 创建时间（Unix 时间戳）
  - `updated_at`: 更新时间（Unix 时间戳）
- `pagination`: 分页信息
  - `total`: 总记录数
  - `page`: 当前页码
  - `per_page`: 每页数量
  - `total_pages`: 总页数

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：获取共享文件夹列表（默认分页）

```bash
curl -X GET "http://localhost:8080/api/v1/shares" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 2：分页查询

```bash
# 第 2 页，每页 10 条
curl -X GET "http://localhost:8080/api/v1/shares?page=2&per_page=10" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 3：按协议筛选

```bash
# 只查询支持 SMB 的共享
curl -X GET "http://localhost:8080/api/v1/shares?protocol=smb" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 4：按存储卷 ID 筛选

```bash
# 只查询存储卷 2 的共享
curl -X GET "http://localhost:8080/api/v1/shares?volume_id=2" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 5：组合筛选

```bash
# 查询存储卷 2 支持 NFS 的共享
curl -X GET "http://localhost:8080/api/v1/shares?volume_id=2&protocol=nfs" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 6：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/shares"
```

**响应：**
```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

### 示例 7：无数据返回

```bash
curl -X GET "http://localhost:8080/api/v1/shares?protocol=ftp" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": true,
  "data": [],
  "pagination": {
    "total": 0,
    "page": 1,
    "per_page": 20,
    "total_pages": 0
  }
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized
- 任意登录用户可访问，不需要 admin 权限

## 实现文件

- `src/handlers/shares_list.rs` - 共享文件夹列表处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问，不需要 admin 权限
2. **分页限制**：per_page 最大值为 100
3. **空结果**：无数据时返回空数组
4. **协议筛选**：支持 smb/nfs/afp/ftp

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **POST /api/v1/shares** - 创建共享文件夹 (Phase 89)
- **GET /api/v1/shares/{id}** - 获取共享文件夹详情
- **PUT /api/v1/shares/{id}** - 更新共享文件夹
- **DELETE /api/v1/shares/{id}** - 删除共享文件夹

## 响应示例（完整）

### 成功响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "public",
      "path": "/mnt/volumes/data/public",
      "volume_id": 2,
      "volume_name": "data",
      "description": "Public shared folder",
      "protocols": ["smb", "nfs"],
      "read_only": false,
      "guest_access": true,
      "created_at": 1710000000,
      "updated_at": 1710000000
    }
  ],
  "pagination": {
    "total": 1,
    "page": 1,
    "per_page": 20,
    "total_pages": 1
  }
}
```

### 空结果

```json
{
  "success": true,
  "data": [],
  "pagination": {
    "total": 0,
    "page": 1,
    "per_page": 20,
    "total_pages": 0
  }
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

## 分页说明

- **页码从 1 开始**（page=1 表示第一页）
- **每页最大 100 条**（per_page 超过 100 会自动限制为 100）
- **总页数计算**：`total_pages = ceil(total / per_page)`
- **空结果**：当 total=0 时，total_pages=0

## 筛选说明

### 协议筛选（protocol）

| 协议 | 说明 |
|------|------|
| `smb` | Server Message Block（Windows） |
| `nfs` | Network File System（Linux） |
| `afp` | Apple Filing Protocol（macOS） |
| `ftp` | File Transfer Protocol（通用） |

### 存储卷筛选（volume_id）

- 根据存储卷 ID 筛选共享文件夹
- 返回指定存储卷上的所有共享

## 字段说明

| 字段 | 说明 |
|------|------|
| `id` | 共享文件夹唯一标识符 |
| `name` | 共享文件夹名称 |
| `path` | 文件夹路径（绝对路径） |
| `volume_id` | 所属存储卷 ID |
| `volume_name` | 所属存储卷名称 |
| `description` | 共享文件夹描述 |
| `protocols` | 支持的协议列表 |
| `read_only` | 是否只读访问 |
| `guest_access` | 是否允许访客（匿名）访问 |
| `created_at` | 创建时间戳 |
| `updated_at` | 更新时间戳 |
