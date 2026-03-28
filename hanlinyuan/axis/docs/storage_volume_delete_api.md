# 删除存储卷 API

**Phase 69** - 存储管理 API 之删除存储卷接口

---

## 接口信息

- **端点:** `DELETE /api/v1/storage/volumes/{id}`
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
| `id` | integer | 是 | 存储卷 ID |

### 请求示例

```bash
# 删除存储卷
curl -X DELETE "http://localhost:8080/api/v1/storage/volumes/3" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"
```

---

## 响应

### 204 No Content - 删除成功

无响应体。

### 400 Bad Request - 卷正在使用

```json
{
  "success": false,
  "error": "Cannot delete volume 'Data Volume': volume is currently in use",
  "code": "VOLUME_IN_USE"
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

### 403 Forbidden - 无权限或系统卷保护

```json
{
  "success": false,
  "error": "Only admin users can delete storage volumes",
  "code": "FORBIDDEN"
}
```

或

```json
{
  "success": false,
  "error": "Cannot delete system volume 'System Volume'",
  "code": "SYSTEM_VOLUME_PROTECTED"
}
```

### 404 Not Found - 存储卷不存在

```json
{
  "success": false,
  "error": "Storage volume 123 not found",
  "code": "NOT_FOUND"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以删除存储卷
3. **系统卷保护**: 系统卷（is_system=true）不可删除
4. **使用中检查**: 正在使用的卷（in_use=true）不可删除

---

## 实现细节

- **文件位置:** `src/handlers/storage_volumes_delete.rs`
- **路由注册:** `src/main.rs` - `DELETE /api/v1/storage/volumes/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/storage/volumes` - 存储卷列表（Phase 60）
- `GET /api/v1/storage/volumes/{id}` - 存储卷详情（Phase 61）
- `POST /api/v1/storage/volumes` - 创建存储卷（Phase 67）
- `PUT /api/v1/storage/volumes/{id}` - 更新存储卷（Phase 68）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 69 初始实现 |
