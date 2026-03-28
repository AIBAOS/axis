# Phase 90 - 共享文件夹列表 API 文档

**接口:** `GET /api/v1/shared-folders`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

获取共享文件夹列表，支持分页和筛选。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 登录用户可访问

**功能特性:**
- 返回共享文件夹列表
- 支持分页查询
- 支持按协议/存储卷 ID/状态筛选
- 无数据返回空数组

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

### 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `page` | number | 否 | `1` | 页码 |
| `per_page` | number | 否 | `20` | 每页数量（最大 100） |
| `protocol` | string | 否 | - | 协议筛选：`smb` / `nfs` / `afp` / `ftp` |
| `volume_id` | number | 否 | - | 存储卷 ID 筛选 |
| `status` | string | 否 | - | 状态筛选：`active` / `inactive` |

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "public",
      "path": "/public",
      "volume_id": 1,
      "volume_name": "data",
      "description": "Public shared folder",
      "protocols": ["smb", "nfs"],
      "is_public": true,
      "status": "active",
      "created_at": 1774259200,
      "created_by": "admin"
    },
    {
      "id": 2,
      "name": "homes",
      "path": "/homes",
      "volume_id": 1,
      "volume_name": "data",
      "description": "User home directories",
      "protocols": ["smb"],
      "is_public": false,
      "status": "active",
      "created_at": 1774345600,
      "created_by": "admin"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 4,
    "total_pages": 1
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | array | 共享文件夹列表 |
| `data[].id` | number | 共享文件夹 ID |
| `data[].name` | string | 共享文件夹名称 |
| `data[].path` | string | 共享路径 |
| `data[].volume_id` | number | 所属存储卷 ID |
| `data[].volume_name` | string | 所属存储卷名称 |
| `data[].description` | string/null | 共享文件夹描述 |
| `data[].protocols` | array | 支持的协议列表 |
| `data[].is_public` | boolean | 是否公开访问 |
| `data[].status` | string | 状态：`active` / `inactive` |
| `data[].created_at` | number | 创建时间（Unix 时间戳） |
| `data[].created_by` | string | 创建者用户名 |
| `pagination` | object | 分页信息 |
| `pagination.page` | number | 当前页码 |
| `pagination.per_page` | number | 每页数量 |
| `pagination.total` | number | 总记录数 |
| `pagination.total_pages` | number | 总页数 |

### 200 OK (空数组)

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

## ❌ 错误响应

### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

---

## 🧪 使用示例

```bash
# 获取全部共享文件夹列表
curl -X GET "http://localhost:8080/api/v1/shared-folders" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 分页查询：第 2 页，每页 10 条
curl -X GET "http://localhost:8080/api/v1/shared-folders?page=2&per_page=10" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 按协议筛选（仅 SMB 共享）
curl -X GET "http://localhost:8080/api/v1/shared-folders?protocol=smb" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 按存储卷 ID 筛选
curl -X GET "http://localhost:8080/api/v1/shared-folders?volume_id=1" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 按状态筛选（仅活跃）
curl -X GET "http://localhost:8080/api/v1/shared-folders?status=active" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 组合筛选：存储卷 1 的 SMB 活跃共享
curl -X GET "http://localhost:8080/api/v1/shared-folders?volume_id=1&protocol=smb&status=active" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 未认证（401）
curl -X GET "http://localhost:8080/api/v1/shared-folders"
# 响应：401 Unauthorized - Missing or invalid Authorization header
```

---

## 📝 注意事项

1. **权限要求**: 登录用户可访问，无需 admin 权限
2. **分页**: 支持 page/per_page 参数，per_page 最大值为 100
3. **空数据**: 无共享文件夹时返回空数组，不返回错误
4. **协议筛选**: 匹配共享文件夹支持的任一协议
5. **状态筛选**: 支持 `active` / `inactive` 状态筛选

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `POST /api/v1/shared-folders` | 创建共享文件夹 (Phase 89) |
| `GET /api/v1/shared-folders/{id}` | 共享文件夹详情（待实现） |
| `PUT /api/v1/shared-folders/{id}` | 更新共享文件夹（待实现） |
| `DELETE /api/v1/shared-folders/{id}` | 删除共享文件夹（待实现） |
| `GET /api/v1/storage/volumes` | 存储卷列表 (Phase 78) |

---

*文档维护：兵部尚书*
