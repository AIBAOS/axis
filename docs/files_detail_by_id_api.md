# 文件详情 API (Phase 107)

## 接口说明

实现获取文件详细信息的接口。登录用户可访问。

## 接口定义

```
GET /api/v1/files/{id}
```

## 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | string | 是 | 文件 ID |

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
    "file_id": "file_1711500000",
    "name": "document.pdf",
    "path": "/Documents/document.pdf",
    "size_bytes": 524288,
    "mime_type": "application/pdf",
    "volume_id": 1,
    "volume_name": "System Volume",
    "created_at": 1711500000,
    "modified_at": 1711500000,
    "owner_id": 1,
    "is_shared": false
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
  "error": "File 'file_999' not found",
  "code": "NOT_FOUND"
}
```

## 使用示例

### cURL 示例

```bash
# 获取文件详情
curl -X GET "http://localhost:8080/api/v1/files/file_1711500000" \
  -H "Authorization: Bearer <jwt_token>"

# 获取不存在的文件（返回 404）
curl -X GET "http://localhost:8080/api/v1/files/file_nonexistent" \
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
  return data;
}

// 使用示例
const file = await getFileDetail('file_1711500000');
console.log(`${file.data.name} (${file.data.size_bytes} bytes)`);
console.log(`Location: ${file.data.volume_name}${file.data.path}`);
```

## 响应字段说明

### FileDetail

| 字段 | 类型 | 说明 |
|------|------|------|
| file_id | string | 文件唯一标识 |
| name | string | 文件名称 |
| path | string | 文件完整路径 |
| size_bytes | integer | 文件大小（字节） |
| mime_type | string | MIME 类型 |
| volume_id | integer | 所属存储卷 ID |
| volume_name | string | 所属存储卷名称 |
| created_at | integer | 创建时间（Unix 时间戳） |
| modified_at | integer | 修改时间（Unix 时间戳） |
| owner_id | integer | 所有者用户 ID |
| is_shared | boolean | 是否已共享 |

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **登录用户可访问**: 无需 admin 权限
3. **文件存在性验证**: 防止访问不存在文件

## 实现文件

- `src/handlers/files_detail_by_id.rs` - 文件详情处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际文件系统
2. 时间戳使用 Unix 时间戳（秒级）
3. 文件不存在返回 404 Not Found

## 相关接口

- `GET /api/v1/files/browse` - 文件浏览（Phase 104）
- `POST /api/v1/files/upload` - 文件上传（Phase 105）
- `DELETE /api/v1/files/{id}` - 文件删除（Phase 106）
- `GET /api/v1/files/download` - 文件下载
- `PUT /api/v1/files/rename` - 文件重命名
