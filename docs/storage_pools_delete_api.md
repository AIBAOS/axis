# 存储池删除 API

**Phase 66** - 存储管理 API 之删除存储池接口

---

## 接口信息

- **端点:** `DELETE /api/v1/storage/pools/{id}`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
- **响应:** 204 No Content

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 存储池 ID |

### 请求示例

```bash
# 删除存储池
curl -X DELETE "http://localhost:8080/api/v1/storage/pools/3" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"
```

---

## 响应

### 204 No Content - 删除成功

无响应体。

### 400 Bad Request - 存储池正在使用

```json
{
  "success": false,
  "error": "Cannot delete storage pool 'Data Pool': volumes are using this pool",
  "code": "POOL_IN_USE"
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

或

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden - 无权限

```json
{
  "success": false,
  "error": "Only admin users can delete storage pools",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 存储池不存在

```json
{
  "success": false,
  "error": "Storage pool 123 not found",
  "code": "NOT_FOUND"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以删除存储池
3. **使用检查**: 有卷正在使用的存储池不允许删除

---

## 实现细节

- **文件位置:** `src/handlers/storage_pools_delete.rs`
- **路由注册:** `src/main.rs` - `DELETE /api/v1/storage/pools/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/storage/pools` - 存储池列表（Phase 62）
- `GET /api/v1/storage/pools/{id}` - 存储池详情（Phase 63）
- `POST /api/v1/storage/pools` - 创建存储池（Phase 64）
- `PUT /api/v1/storage/pools/{id}` - 更新存储池（Phase 65）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 66 初始实现 |
