# Phase 66 - 删除存储池 API 文档

**接口:** `DELETE /api/v1/storage/pools/{id}`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

删除指定的存储池。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 仅 `admin` 角色可调用此接口

**功能特性:**
- 验证存储池 ID 存在
- 检查是否有卷在使用该池
- 删除成功返回删除信息

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
| `id` | number | 是 | 存储池 ID |

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "message": "Storage pool deleted successfully",
  "data": {
    "id": 2,
    "name": "backup",
    "deleted_at": 1774345600
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `message` | string | 响应消息 |
| `data.id` | number | 已删除的存储池 ID |
| `data.name` | string | 已删除的存储池名称 |
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
  "error": "Only admin users can delete storage pools",
  "code": "FORBIDDEN"
}
```

### 404 Not Found

```json
{
  "success": false,
  "error": "Storage pool 999 not found",
  "code": "NOT_FOUND"
}
```

### 400 Bad Request

```json
{
  "success": false,
  "error": "Cannot delete storage pool 'media': volumes are using this pool",
  "code": "POOL_IN_USE"
}
```

---

## 🧪 使用示例

```bash
# 删除存储池 ID 为 2 的存储池
curl -X DELETE "http://localhost:8080/api/v1/storage/pools/2" \
  -H "Authorization: Bearer <admin_token>"
```

```bash
# 删除不存在的存储池（返回 404）
curl -X DELETE "http://localhost:8080/api/v1/storage/pools/999" \
  -H "Authorization: Bearer <admin_token>"
```

```bash
# 删除有卷在使用的存储池（返回 400）
curl -X DELETE "http://localhost:8080/api/v1/storage/pools/3" \
  -H "Authorization: Bearer <admin_token>"
```

---

## 🔒 安全说明

### 权限控制

- 仅 `admin` 角色可删除存储池
- 非 admin 用户调用返回 403 Forbidden

### 使用检查

- 删除前检查是否有卷在使用该存储池
- 有卷使用时返回 400 Bad Request，错误码 `POOL_IN_USE`

### 数据完整性

- 存储池不存在返回 404 Not Found
- 删除操作不可恢复，请谨慎操作

---

## 📝 注意事项

1. **权限要求**: 仅 admin 角色可删除，普通用户返回 403
2. **使用检查**: 删除前必须确保无卷使用该存储池
3. **不可恢复**: 删除操作是永久性的
4. **建议流程**:
   - 先删除或迁移所有关联的卷
   - 确认无卷使用后执行删除

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/storage/pools` | 存储池列表 (Phase 62) |
| `GET /api/v1/storage/pools/{id}` | 存储池详情 (Phase 63) |
| `POST /api/v1/storage/pools` | 创建存储池 (Phase 64) |
| `PUT /api/v1/storage/pools/{id}` | 更新存储池 (Phase 65) |
| `GET /api/v1/storage/volumes` | 存储卷列表 |

---

*文档维护：兵部尚书*
