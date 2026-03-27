# 共享文件夹权限列表 API

**Phase 95** - 共享文件夹权限管理 API 之获取权限列表接口

---

## 接口信息

- **端点:** `GET /api/v1/shared-folders/{id}/permissions`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <ADMIN_JWT_TOKEN>` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 共享文件夹 ID |

### 查询参数

| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `page` | integer | 否 | 1 | 页码（从 1 开始） |
| `per_page` | integer | 否 | 20 | 每页数量（最大 100） |

### 请求示例

```bash
# 获取共享文件夹权限列表
curl -X GET "http://localhost:8080/api/v1/shared-folders/1/permissions" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"

# 带分页参数
curl -X GET "http://localhost:8080/api/v1/shared-folders/1/permissions?page=1&per_page=10" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "shared_folder_id": 1,
      "user_id": 1,
      "group_id": null,
      "permissions": ["read", "write"],
      "created_at": 1710489600,
      "updated_at": 1711440000
    },
    {
      "id": 2,
      "shared_folder_id": 1,
      "user_id": 2,
      "group_id": null,
      "permissions": ["read"],
      "created_at": 1710489600,
      "updated_at": 1711440000
    },
    {
      "id": 3,
      "shared_folder_id": 1,
      "user_id": null,
      "group_id": 1,
      "permissions": ["read", "write", "admin"],
      "created_at": 1710489600,
      "updated_at": 1711440000
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 3,
    "total_pages": 1
  }
}
```

### 401 Unauthorized - 未认证

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden - 无权限

```json
{
  "success": false,
  "error": "Only admin users can access permissions",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 共享文件夹不存在

```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "NOT_FOUND"
}
```

### 200 OK - 空列表（无权限设置）

```json
{
  "success": true,
  "data": [],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 0,
    "total_pages": 0
  }
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以访问权限列表

---

## 响应字段说明

### PermissionListResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `data` | array | 权限列表 |
| `pagination` | object | 分页信息 |

### PermissionInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 权限 ID |
| `shared_folder_id` | integer | 共享文件夹 ID |
| `user_id` | integer\|null | 用户 ID（如果是用户权限） |
| `group_id` | integer\|null | 用户组 ID（如果是组权限） |
| `permissions` | array | 权限列表：["read", "write", "admin"] |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

### PaginationMeta

| 字段 | 类型 | 说明 |
|------|------|------|
| `page` | integer | 当前页码 |
| `per_page` | integer | 每页数量 |
| `total` | integer | 总记录数 |
| `total_pages` | integer | 总页数 |

---

## 权限说明

| 权限 | 说明 |
|------|------|
| `read` | 读取权限（浏览、下载文件） |
| `write` | 写入权限（上传、修改、删除文件） |
| `admin` | 管理权限（修改共享设置、管理权限） |

---

## 实现细节

- **文件位置:** `src/handlers/shared_folder_permissions_list.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/shared-folders/{id}/permissions`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `POST /api/v1/shared-folders/{id}/permissions` - 添加权限
- `PUT /api/v1/shared-folders/{id}/permissions/{permission_id}` - 更新权限
- `DELETE /api/v1/shared-folders/{id}/permissions/{permission_id}` - 删除权限
- `GET /api/v1/shares` - 共享文件夹列表

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 95 初始实现 |
