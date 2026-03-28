# 删除共享文件夹 API

**Phase 94** - 共享文件夹管理 API 之删除共享文件夹接口

---

## 接口信息

- **端点:** `DELETE /api/v1/shares/{id}`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
- **响应:** 200 OK

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

### 请求示例

```bash
# 删除共享文件夹
curl -X DELETE "http://localhost:8080/api/v1/shares/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"
```

---

## 响应

### 200 OK - 删除成功

```json
{
  "success": true,
  "message": "Share 1 deleted successfully"
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
  "error": "Only admin users can delete shares",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 共享不存在

```json
{
  "success": false,
  "error": "Share 999 not found",
  "code": "NOT_FOUND"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以删除共享文件夹

---

## 响应字段说明

### DeleteShareResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |

---

## 实现细节

- **文件位置:** `src/handlers/shares_delete.rs`
- **路由注册:** `src/main.rs` - `DELETE /api/v1/shares/{id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - `SqliteRbacRepository` - 角色权限验证

---

## 相关接口

- `GET /api/v1/shares` - 共享文件夹列表（Phase 90）
- `POST /api/v1/shares` - 创建共享文件夹（Phase 89/91）
- `GET /api/v1/shares/{id}` - 共享文件夹详情（Phase 92）
- `PUT /api/v1/shares/{id}` - 更新共享文件夹（Phase 93）

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 94 初始实现 |
