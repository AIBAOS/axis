# Phase 81 - 存储卷快照列表 API 文档

**接口:** `GET /api/v1/storage/volumes/{id}/snapshots`

**版本:** v0.1.0

**最后更新:** 2026-03-26

---

## 📋 接口说明

获取指定存储卷的快照列表。

**权限要求:**
- 需要 JWT Bearer Token 认证
- 任意登录用户可访问

**功能特性:**
- 返回指定存储卷下的所有快照
- 支持分页查询
- 验证存储卷 ID 存在
- 存储卷不存在返回 404 Not Found

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
| `id` | number | 是 | 存储卷 ID |

### 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `page` | number | 否 | `1` | 页码 |
| `per_page` | number | 否 | `20` | 每页数量（最大 100） |

---

## 📥 响应结果

### 200 OK

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "snapshot-2026-03-20",
      "description": "Daily backup snapshot",
      "volume_id": 1,
      "size_bytes": 549755813888,
      "created_at": 1774259200,
      "updated_at": 1774259200,
      "status": "completed"
    },
    {
      "id": 2,
      "name": "snapshot-2026-03-25",
      "description": "Pre-update snapshot",
      "volume_id": 1,
      "size_bytes": 549755813888,
      "created_at": 1774345600,
      "updated_at": 1774345600,
      "status": "completed"
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

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | array | 快照列表 |
| `data[].id` | number | 快照 ID |
| `data[].name` | string | 快照名称 |
| `data[].description` | string/null | 描述信息 |
| `data[].volume_id` | number | 所属存储卷 ID |
| `data[].size_bytes` | number | 快照大小（字节） |
| `data[].created_at` | number | 创建时间（Unix 时间戳） |
| `data[].updated_at` | number | 更新时间（Unix 时间戳） |
| `data[].status` | string | 状态：`creating` / `completed` / `failed` / `deleting` |
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

### 404 Not Found

```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "VOLUME_NOT_FOUND"
}
```

---

## 🧪 使用示例

```bash
# 获取存储卷 ID 为 1 的快照列表
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 分页查询：第 2 页，每页 10 条
curl -X GET "http://localhost:8080/api/v1/storage/volumes/1/snapshots?page=2&per_page=10" \
  -H "Authorization: Bearer <access_token>"
```

```bash
# 获取不存在的存储卷的快照（返回 404）
curl -X GET "http://localhost:8080/api/v1/storage/volumes/999/snapshots" \
  -H "Authorization: Bearer <access_token>"
```

---

## 📝 注意事项

1. **权限要求**: 任意登录用户可访问，无需 admin 权限
2. **存储卷不存在**: 返回 404 Not Found
3. **分页**: 支持 page/per_page 参数，per_page 最大值为 100
4. **空数据**: 存储卷下无快照时返回空数组，不返回错误
5. **容量单位**: 所有容量字段单位为字节（bytes）
6. **快照状态**:
   - `creating`: 创建中
   - `completed`: 已完成
   - `failed`: 失败
   - `deleting`: 删除中

---

## 🔗 相关接口

| 接口 | 说明 |
|------|------|
| `GET /api/v1/storage/volumes` | 存储卷列表 (Phase 78) |
| `GET /api/v1/storage/volumes/{id}` | 存储卷详情 (Phase 79) |
| `POST /api/v1/storage/volumes` | 创建存储卷 (Phase 80) |
| `PUT /api/v1/storage/volumes/{id}` | 更新存储卷 |
| `DELETE /api/v1/storage/volumes/{id}` | 删除存储卷 |
| `POST /api/v1/storage/volumes/{id}/snapshots` | 创建快照（待实现） |

---

*文档维护：兵部尚书*
