# 添加共享文件夹权限 API 文档 (Phase 96)

## 概述

添加共享文件夹权限 API 允许管理员为指定共享文件夹添加用户或用户组的访问权限。

## 接口详情

### POST /api/v1/shared-folders/{id}/permissions

为指定共享文件夹添加权限配置。

#### 认证要求

需要有效的 JWT Token，且用户必须具有 `admin` 角色。

**请求头：**
```
Authorization: Bearer <admin_jwt_token>
Content-Type: application/json
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 共享文件夹 ID |

#### 请求体

```json
{
  "target_type": "user",
  "target_id": 101,
  "permissions": ["read", "write"]
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `target_type` | string | 是 | 目标类型（user/group） |
| `target_id` | integer | 是 | 目标 ID（用户 ID 或用户组 ID） |
| `permissions` | array | 是 | 权限列表（read/write/admin） |

#### 响应格式

**成功响应 (201 Created)**

```json
{
  "success": true,
  "message": "Permission added successfully",
  "data": {
    "id": 1,
    "shared_folder_id": 1,
    "user_id": 101,
    "group_id": null,
    "permissions": ["read", "write"],
    "created_at": 1711468800,
    "updated_at": 1711468800
  }
}
```

**错误响应 (400 Bad Request) - 参数无效**

```json
{
  "success": false,
  "error": "permissions array cannot be empty",
  "code": "INVALID_PARAMS"
}
```

**错误响应 (400 Bad Request) - 目标类型无效**

```json
{
  "success": false,
  "error": "Invalid target_type. Must be 'user' or 'group'",
  "code": "INVALID_TARGET_TYPE"
}
```

**错误响应 (400 Bad Request) - 权限值无效**

```json
{
  "success": false,
  "error": "Invalid permission 'invalid'. Valid permissions: read, write, admin",
  "code": "INVALID_PERMISSION"
}
```

**错误响应 (404 Not Found) - 共享文件夹不存在**

```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "FOLDER_NOT_FOUND"
}
```

**错误响应 (404 Not Found) - 用户/用户组不存在**

```json
{
  "success": false,
  "error": "User 999 not found",
  "code": "TARGET_NOT_FOUND"
}
```

**错误响应 (409 Conflict) - 权限已存在**

```json
{
  "success": false,
  "error": "Permission already exists for this target",
  "code": "PERMISSION_EXISTS"
}
```

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "error": "Only admin users can manage permissions",
  "code": "FORBIDDEN"
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

### 示例 1：为用户添加只读权限

```bash
curl -X POST "http://localhost:8080/api/v1/shared-folders/1/permissions" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "user",
    "target_id": 101,
    "permissions": ["read"]
  }'
```

### 示例 2：为用户组添加读写权限

```bash
curl -X POST "http://localhost:8080/api/v1/shared-folders/1/permissions" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "group",
    "target_id": 201,
    "permissions": ["read", "write"]
  }'
```

### 示例 3：添加管理员权限

```bash
curl -X POST "http://localhost:8080/api/v1/shared-folders/1/permissions" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "group",
    "target_id": 201,
    "permissions": ["read", "write", "admin"]
  }'
```

### 示例 4：目标类型无效（400）

```bash
curl -X POST "http://localhost:8080/api/v1/shared-folders/1/permissions" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "invalid",
    "target_id": 101,
    "permissions": ["read"]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Invalid target_type. Must be 'user' or 'group'",
  "code": "INVALID_TARGET_TYPE"
}
```

### 示例 5：权限值无效（400）

```bash
curl -X POST "http://localhost:8080/api/v1/shared-folders/1/permissions" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "user",
    "target_id": 101,
    "permissions": ["invalid"]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Invalid permission 'invalid'. Valid permissions: read, write, admin",
  "code": "INVALID_PERMISSION"
}
```

### 示例 6：权限数组为空（400）

```bash
curl -X POST "http://localhost:8080/api/v1/shared-folders/1/permissions" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "user",
    "target_id": 101,
    "permissions": []
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "permissions array cannot be empty",
  "code": "INVALID_PARAMS"
}
```

### 示例 7：共享文件夹不存在（404）

```bash
curl -X POST "http://localhost:8080/api/v1/shared-folders/999/permissions" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "user",
    "target_id": 101,
    "permissions": ["read"]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "FOLDER_NOT_FOUND"
}
```

### 示例 8：非 admin 用户访问（403）

```bash
curl -X POST "http://localhost:8080/api/v1/shared-folders/1/permissions" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "user",
    "target_id": 101,
    "permissions": ["read"]
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Only admin users can manage permissions",
  "code": "FORBIDDEN"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可管理共享文件夹权限
- 非 admin 角色返回 403 Forbidden

### 3. 输入验证

- **目标类型验证**：必须是 user 或 group
- **权限值验证**：必须是 read/write/admin
- **权限数组验证**：不能为空
- **共享文件夹验证**：共享文件夹必须存在
- **目标存在性验证**：用户/用户组必须存在

## 权限级别说明

| 权限 | 说明 |
|------|------|
| `read` | 只读权限，可以查看和下载文件 |
| `write` | 读写权限，可以上传、修改、删除文件 |
| `admin` | 管理权限，可以管理权限配置 |

## 目标类型说明

| 类型 | 说明 |
|------|------|
| `user` | 用户级别权限 |
| `group` | 用户组级别权限 |

## 实现文件

- `src/handlers/shared_folder_permissions_add.rs` - 共享文件夹权限添加处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **权限控制**：仅 admin 角色可管理共享文件夹权限
2. **权限级别**：必须是 read/write/admin
3. **目标类型**：必须是 user/group
4. **权限数组**：不能为空，可包含多个权限
5. **重复权限**：如果已存在相同目标的权限，返回 409 Conflict

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `FOLDER_NOT_FOUND` | 404 | 共享文件夹不存在 |
| `TARGET_NOT_FOUND` | 404 | 用户/用户组不存在 |
| `PERMISSION_EXISTS` | 409 | 权限已存在 |
| `INVALID_PARAMS` | 400 | 参数无效 |
| `INVALID_TARGET_TYPE` | 400 | 目标类型无效 |
| `INVALID_PERMISSION` | 400 | 权限值无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/shared-folders/{id}/permissions** - 获取共享文件夹权限列表 (Phase 95)
- **PUT /api/v1/shared-folders/{id}/permissions/{permission_id}** - 更新权限
- **DELETE /api/v1/shared-folders/{id}/permissions/{permission_id}** - 删除权限

## 响应示例（完整）

### 成功创建

```json
{
  "success": true,
  "message": "Permission added successfully",
  "data": {
    "id": 1,
    "shared_folder_id": 1,
    "user_id": 101,
    "group_id": null,
    "permissions": ["read", "write"],
    "created_at": 1711468800,
    "updated_at": 1711468800
  }
}
```

### 目标类型无效（400）

```json
{
  "success": false,
  "error": "Invalid target_type. Must be 'user' or 'group'",
  "code": "INVALID_TARGET_TYPE"
}
```

### 权限不足（403）

```json
{
  "success": false,
  "error": "Only admin users can manage permissions",
  "code": "FORBIDDEN"
}
```

## 最佳实践

### 1. 最小权限原则

为用户分配最小必要的权限：
- 普通用户：`["read"]` 权限
- 内容创作者：`["read", "write"]` 权限
- 管理员：`["read", "write", "admin"]` 权限

### 2. 优先使用用户组

建议通过用户组管理权限，而不是直接分配给用户：
- 便于批量管理
- 用户离职时只需从组中移除
- 权限审计更清晰

### 3. 权限命名规范

使用有意义的权限级别：
- `read`: 只读
- `write`: 读写
- `admin`: 管理

### 4. 审计日志

所有权限添加操作都应该记录到审计日志中，包括：
- 添加时间
- 执行添加的管理员 ID
- 共享文件夹 ID
- 目标类型和目标 ID
- 权限级别列表
