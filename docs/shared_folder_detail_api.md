# Phase 92 - 共享文件夹详情 API 文档

**接口:** `GET /api/v1/shared-folders/{id}`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

获取单个共享文件夹的详细信息。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 登录用户可访问

**功能特性:**
- 返回共享文件夹完整信息
- 验证共享文件夹 ID 存在
- 共享文件夹不存在返回 404 Not Found

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
  "data": {
    "id": 1,
    "name": "public",
    "path": "/public",
    "volume_id": 1,
    "volume_name": "data",
    "description": "Public shared folder",
    "protocols": ["smb", "nfs"],
    "is_public": true,
    "read_only": false,
    "guest_access": true,
    "status": "active",
    "created_at": 1774259200,
    "updated_at": 1774259200,
    "created_by": "admin"
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | object | 共享文件夹详情 |
| `data.id` | number | 共享文件夹 ID |
| `data.name` | string | 共享文件夹名称 |
| `data.path` | string | 共享路径 |
| `data.volume_id` | number | 所属存储卷 ID |
| `data.volume_name` | string | 所属存储卷名称 |
| `data.description` | string/null | 共享文件夹描述 |
| `data.protocols` | array | 支持的协议列表 |
| `data.is_public` | boolean | 是否公开访问 |
| `data.read_only` | boolean | 是否只读 |
| `data.guest_access` | boolean | 是否允许访客访问 |
| `data.status` | string | 状态：`active` / `inactive` |
| `data.created_at` | number | 创建时间（Unix 时间戳） |
| `data.updated_at` | number | 更新时间（Unix 时间戳） |
| `data.created_by` | string | 创建者用户名 |

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
# 获取共享文件夹 ID 为 1 的详情
curl -X GET "http://localhost:8080/api/v1/shared-folders/1" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 获取不存在的共享文件夹（返回 404）
curl -X GET "http://localhost:8080/api/v1/shared-folders/999" \
  -H "Authorization: Bearer <access_token>"
# 响应：404 Not Found - Shared folder not found
```

```bash
# 未认证（401）
curl -X GET "http://localhost:8080/api/v1/shared-folders/1"
# 响应：401 Unauthorized - Missing or invalid Authorization header
```

---

## 📝 注意事项

1. **权限要求**: 登录用户可访问，无需 admin 权限
2. **共享文件夹不存在**: 返回 404 Not Found
3. **协议列表**: 包含该共享文件夹支持的所有协议（smb/nfs/afp/ftp）

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/shared-folders` | 共享文件夹列表 (Phase 90) |
| `POST /api/v1/shared-folders` | 创建共享文件夹 (Phase 89) |
| `PUT /api/v1/shared-folders/{id}` | 更新共享文件夹（待实现） |
| `DELETE /api/v1/shared-folders/{id}` | 删除共享文件夹（待实现） |

---

*文档维护：兵部尚书*
