# Phase 94 - 删除共享文件夹 API 文档

**接口:** `DELETE /api/v1/shared-folders/{id}`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

删除指定的共享文件夹。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 仅 `admin` 角色可调用此接口

**功能特性:**
- 删除共享文件夹
- 验证共享文件夹 ID 存在
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

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "message": "Shared folder deleted successfully",
  "data": {
    "id": 1,
    "name": "public",
    "deleted_at": 1774432000
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data.id` | number | 已删除的共享文件夹 ID |
| `data.name` | string | 已删除的共享文件夹名称 |
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
  "error": "Only admin users can delete shared folders",
  "code": "FORBIDDEN"
}
```

### 404 Not Found

```json
{
  "success": false,
  "error": "Shared folder 999 not found",
  "code": "NOT_FOUND"
}
```

---

## 🧪 使用示例

```bash
# 删除共享文件夹 ID 为 1 的共享文件夹
curl -X DELETE "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <admin_token>"
```

```bash
# 删除不存在的共享文件夹（返回 404）
curl -X DELETE "http://localhost:8080/api/v1/shared-folders/999" \
  -H "Authorization: Bearer <admin_token>"
# 响应：404 Not Found - Shared folder not found
```

```bash
# 非 admin 用户（403）
curl -X DELETE "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <user_token>"
# 响应：403 Forbidden - Only admin users can delete shared folders
```

---

## 📝 注意事项

1. **权限要求**: 仅 admin 角色可删除共享文件夹，普通用户返回 403
2. **共享文件夹不存在**: 返回 404 Not Found
3. **删除操作**: 删除后无法恢复，请谨慎操作
4. **关联数据**: 删除前应确保无活跃的 SMB/NFS 连接

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/shared-folders` | 共享文件夹列表 (Phase 90) |
| `GET /api/v1/shared-folders/{id}` | 共享文件夹详情 (Phase 92) |
| `POST /api/v1/shared-folders` | 创建共享文件夹 (Phase 89) |
| `PUT /api/v1/shared-folders/{id}` | 更新共享文件夹 (Phase 93) |

---

*文档维护：兵部尚书*
