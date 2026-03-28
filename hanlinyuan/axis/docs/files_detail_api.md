# 文件详情 API 文档 (Phase 121)

## 概述

文件详情 API 提供指定文件的详细元数据信息。

## 接口详情

### GET /api/v1/files/{id}

获取指定文件的详细信息。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | string | 是 | 文件 ID |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "data": {
    "id": "file_001",
    "name": "document.pdf",
    "path": "/Documents/document.pdf",
    "size_bytes": 524288,
    "mime_type": "application/pdf",
    "volume_id": 1,
    "owner_id": 1,
    "created_at": 1774259200,
    "modified_at": 1774345600
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `data`: 文件详细信息
  - `id`: 文件 ID
  - `name`: 文件名
  - `path`: 完整路径
  - `size_bytes`: 文件大小（字节）
  - `mime_type`: MIME 类型
  - `volume_id`: 所属存储卷 ID
  - `owner_id`: 所有者用户 ID
  - `created_at`: 创建时间（Unix 时间戳）
  - `modified_at`: 修改时间（Unix 时间戳）

**错误响应 (404 Not Found) - 文件不存在**

```json
{
  "success": false,
  "error": "File not found",
  "code": "NOT_FOUND"
}
```

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "error": "No permission to access this file",
  "code": "PERMISSION_DENIED"
}
```

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：获取文件详情

```bash
curl -X GET "http://localhost:8080/api/v1/files/file_001" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": true,
  "data": {
    "id": "file_001",
    "name": "document.pdf",
    "path": "/Documents/document.pdf",
    "size_bytes": 524288,
    "mime_type": "application/pdf",
    "volume_id": 1,
    "owner_id": 1,
    "created_at": 1774259200,
    "modified_at": 1774345600
  }
}
```

### 示例 2：获取不存在的文件（404）

```bash
curl -X GET "http://localhost:8080/api/v1/files/nonexistent" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "File not found",
  "code": "NOT_FOUND"
}
```

### 示例 3：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/files/file_001"
```

**响应：**
```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 权限验证

- 验证用户对文件的访问权限
- 无权限返回 403 Forbidden

## 实现文件

- `src/handlers/files_detail.rs` - 文件详情处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **权限验证**：验证用户对文件的访问权限

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `PERMISSION_DENIED` | 403 | 无权限访问该文件 |
| `NOT_FOUND` | 404 | 文件不存在 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/files/list** - 获取文件列表 (Phase 117)
- **POST /api/v1/files/upload** - 上传文件 (Phase 119)
- **GET /api/v1/files/{id}/download** - 下载文件 (Phase 120)
- **PUT /api/v1/files/{id}** - 更新文件
- **DELETE /api/v1/files/{id}** - 删除文件

## 响应示例（完整）

### 成功响应

```json
{
  "success": true,
  "data": {
    "id": "file_001",
    "name": "document.pdf",
    "path": "/Documents/document.pdf",
    "size_bytes": 524288,
    "mime_type": "application/pdf",
    "volume_id": 1,
    "owner_id": 1,
    "created_at": 1774259200,
    "modified_at": 1774345600
  }
}
```

### 文件不存在（404）

```json
{
  "success": false,
  "error": "File not found",
  "code": "NOT_FOUND"
}
```

### 权限不足（403）

```json
{
  "success": false,
  "error": "No permission to access this file",
  "code": "PERMISSION_DENIED"
}
```

## 最佳实践

### 1. 前端集成示例

```javascript
const getFileDetail = async (fileId) => {
  try {
    const response = await fetch(
      `/api/v1/files/${fileId}`,
      {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      }
    );

    const data = await response.json();
    if (response.ok) {
      console.log('File detail:', data.data);
      return data.data;
    } else {
      console.error('Failed to get file detail:', data.error);
      throw new Error(data.error);
    }
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
};

// 使用示例
getFileDetail('file_001');
```

### 2. React 组件示例

```jsx
const FileDetail = ({ fileId }) => {
  const [file, setFile] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchDetail = async () => {
      try {
        const response = await fetch(
          `/api/v1/files/${fileId}`,
          {
            headers: {
              'Authorization': `Bearer ${token}`
            }
          }
        );
        const data = await response.json();
        if (response.ok) {
          setFile(data.data);
        }
      } catch (error) {
        console.error('Error:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchDetail();
  }, [fileId]);

  if (loading) return <div>Loading...</div>;
  if (!file) return <div>File not found</div>;

  return (
    <div>
      <h3>File Details</h3>
      <p>Name: {file.name}</p>
      <p>Path: {file.path}</p>
      <p>Size: {formatSize(file.size_bytes)}</p>
      <p>MIME Type: {file.mime_type}</p>
      <p>Created: {formatDate(file.created_at)}</p>
      <p>Modified: {formatDate(file.modified_at)}</p>
    </div>
  );
};
```

### 3. 错误处理

```javascript
const handleFileDetailError = (error) => {
  switch (error.code) {
    case 'NOT_FOUND':
      return 'File not found.';
    case 'PERMISSION_DENIED':
      return 'No permission to access this file.';
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    default:
      return 'Failed to get file detail. Please try again.';
  }
};
```

### 4. 审计日志

所有文件详情查询操作都应该记录到审计日志中，包括：
- 查询时间
- 执行查询的用户 ID
- 文件 ID
- 查询结果（成功/失败）
