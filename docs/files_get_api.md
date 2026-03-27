# 文件详情 API (Phase 121)

## 接口说明

实现获取单个文件详情的接口。登录用户可访问，返回文件元数据。

## 接口定义

```
GET /api/v1/files/{id}
```

## 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | integer | 是 | 文件 ID |

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（需要登录状态） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "report.pdf",
    "path": "/Documents/report.pdf",
    "size": 524288,
    "mime_type": "application/pdf",
    "created_at": 1711500000,
    "updated_at": 1711500000,
    "owner_id": 1
  }
}
```

### 未授权 (401 Unauthorized)

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 文件不存在 (404 Not Found)

```json
{
  "success": false,
  "error": "File 999 not found",
  "code": "NOT_FOUND"
}
```

### 权限不足 (403 Forbidden)

```json
{
  "success": false,
  "error": "No permission to access this file",
  "code": "FORBIDDEN"
}
```

## 使用示例

### cURL 示例

```bash
# 获取文件详情
curl -X GET "http://localhost:8080/api/v1/files/1" \
  -H "Authorization: Bearer <jwt_token>"

# 获取不存在的文件（返回 404）
curl -X GET "http://localhost:8080/api/v1/files/999" \
  -H "Authorization: Bearer <jwt_token>"
```

### JavaScript 示例

```javascript
// 获取文件详情
async function getFileDetail(fileId) {
  const response = await fetch(
    `http://localhost:8080/api/v1/files/${fileId}`,
    {
      headers: {
        'Authorization': 'Bearer ' + token
      }
    }
  );
  
  const data = await response.json();
  console.log('File detail:', data.data);
  return data.data;
}

// 使用示例
const file = await getFileDetail(1);
console.log(`${file.name} (${file.size} bytes)`);
console.log(`Location: ${file.path}`);
console.log(`Owner: ${file.owner_id}`);
```

## 响应字段说明

### FileDetail

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 文件 ID |
| name | string | 文件名称 |
| path | string | 文件完整路径 |
| size | integer | 文件大小（字节） |
| mime_type | string | MIME 类型 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |
| owner_id | integer | 所有者用户 ID |

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **登录用户可访问**: 无需 admin 权限
3. **文件存在性验证**: 防止访问不存在文件
4. **权限验证**: 防止访问无权限文件

## 实现文件

- `src/handlers/files_get.rs` - 文件详情处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际文件系统
2. 时间戳使用 Unix 时间戳（秒级）
3. 权限验证简化实现，后续应检查文件所有权或共享权限

## 相关接口

- `GET /api/v1/files/list` - 文件列表（Phase 117）
- `POST /api/v1/files/upload` - 文件上传（Phase 119）
- `GET /api/v1/files/{id}/download` - 文件下载（Phase 120）
- `DELETE /api/v1/files/{id}` - 文件删除
- `PUT /api/v1/files/{id}` - 文件更新
