# Phase 98 - 删除共享文件夹权限 API 文档

**接口:** `DELETE /api/v1/shared-folders/{id}/permissions/{permission_id}`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

删除指定共享文件夹的权限配置，允许管理员移除用户或用户组的访问权限。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 仅 `admin` 角色可调用此接口

**功能特性:**
- 删除权限配置
- 验证共享文件夹 ID 存在
- 验证权限 ID 存在
- 返回删除结果

---

## 🔐 认证方式

```
Authorization: Bearer <access_token>
```

---

## 📤 请求参数

### 请求头

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `Authorization` | string | 是 | JWT Bearer Token |

### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | number | 是 | 共享文件夹 ID |
| `permission_id` | number | 是 | 权限 ID |

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "message": "Permission deleted successfully",
  "data": {
    "id": 2,
    "shared_folder_id": 1,
    "deleted_at": 1774432000
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data.id` | number | 已删除的权限 ID |
| `data.shared_folder_id` | number | 所属共享文件夹 ID |
| `data.deleted_at` | number | 删除时间（Unix 时间戳） |

---

## ❌ 错误响应

### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden

```json
{
  "success": false,
  "error": "Only admin users can delete permissions",
  "code": "FORBIDDEN"
}
```

### 404 Not Found (共享文件夹不存在)

```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "NOT_FOUND"
}
```

### 404 Not Found (权限不存在)

```json
{
  "success": false,
  "error": "Permission 999 not found",
  "code": "NOT_FOUND"
}
```

---

## 🧪 使用示例

```bash
# 删除权限 ID 为 2 的权限配置
curl -X DELETE "http://localhost:8080/api/v1/shared-folders/1/permissions/2" \
  -H "Authorization: Bearer <admin_token>"
```

```bash
# 删除不存在的权限（返回 404）
curl -X DELETE "http://localhost:8080/api/v1/shared-folders/1/permissions/999" \
  -H "Authorization: Bearer <admin_token>"
# 响应：404 Not Found - Permission not found
```

```bash
# 删除不存在的共享文件夹的权限（返回 404）
curl -X DELETE "http://localhost:8080/api/v1/shared-folders/999/permissions/1" \
  -H "Authorization: Bearer <admin_token>"
# 响应：404 Not Found - Shared folder not found
```

```bash
# 非 admin 用户（403）
curl -X DELETE "http://localhost:8080/api/v1/shared-folders/1/permissions/2" \
  -H "Authorization: Bearer <user_token>"
# 响应：403 Forbidden - Only admin users can delete permissions
```

---

## 📝 注意事项

1. **权限要求**: 仅 admin 角色可删除权限，普通用户返回 403
2. **共享文件夹不存在**: 返回 404 Not Found
3. **权限 ID 不存在**: 返回 404 Not Found
4. **删除操作**: 删除后无法恢复，请谨慎操作
5. **影响范围**: 删除权限后，相应用户/用户组将失去对该共享文件夹的访问权限

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/shared-folders/{id}/permissions` | 权限列表 (Phase 95) |
| `POST /api/v1/shared-folders/{id}/permissions` | 添加权限 (Phase 96) |
| `PUT /api/v1/shared-folders/{id}/permissions/{permission_id}` | 更新权限 (Phase 97) |
| `GET /api/v1/shared-folders` | 共享文件夹列表 (Phase 90) |

---

*文档维护：兵部尚书*
