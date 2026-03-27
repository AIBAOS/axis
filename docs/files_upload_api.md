# 文件上传 API 文档 (Phase 119)

## 概述

文件上传 API 允许用户通过 multipart/form-data 格式上传文件到 NAS 系统。

## 接口详情

### POST /api/v1/files/upload

上传文件到指定路径。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
Content-Type: multipart/form-data
```

#### 请求体（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `file` | file | 是 | 要上传的文件 |
| `path` | string | 否 | 目标路径（相对于用户主目录，默认为根目录） |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "message": "File uploaded successfully",
  "data": {
    "filename": "document.pdf",
    "path": "/documents/document.pdf",
    "size_bytes": 2048576,
    "mime_type": "application/pdf",
    "uploaded_at": 1711468800
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `message`: 响应消息
- `data`: 已上传文件信息
  - `filename`: 文件名
  - `path`: 完整路径
  - `size_bytes`: 文件大小（字节）
  - `mime_type`: MIME 类型
  - `uploaded_at`: 上传时间（Unix 时间戳）

**错误响应 (400 Bad Request) - 无效请求**

```json
{
  "success": false,
  "error": "No file uploaded",
  "code": "NO_FILE"
}
```

**错误响应 (400 Bad Request) - 不支持的文件类型**

```json
{
  "success": false,
  "error": "File type not allowed",
  "code": "UNSUPPORTED_TYPE"
}
```

**错误响应 (413 Payload Too Large) - 文件过大**

```json
{
  "success": false,
  "error": "File size exceeds limit of 100 MB",
  "code": "FILE_TOO_LARGE"
}
```

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "error": "No write permission for this path",
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

**错误响应 (500 Internal Server Error) - 服务器错误**

```json
{
  "success": false,
  "error": "Failed to save file",
  "code": "SAVE_ERROR"
}
```

## 使用示例

### 示例 1：上传文件到根目录

```bash
curl -X POST "http://localhost:8080/api/v1/files/upload" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -F "file=@/path/to/document.pdf"
```

**响应：**
```json
{
  "success": true,
  "message": "File uploaded successfully",
  "data": {
    "filename": "document.pdf",
    "path": "/document.pdf",
    "size_bytes": 2048576,
    "mime_type": "application/pdf",
    "uploaded_at": 1711468800
  }
}
```

### 示例 2：上传文件到子目录

```bash
curl -X POST "http://localhost:8080/api/v1/files/upload" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -F "file=@/path/to/document.pdf" \
  -F "path=/documents"
```

**响应：**
```json
{
  "success": true,
  "message": "File uploaded successfully",
  "data": {
    "filename": "document.pdf",
    "path": "/documents/document.pdf",
    "size_bytes": 2048576,
    "mime_type": "application/pdf",
    "uploaded_at": 1711468800
  }
}
```

### 示例 3：未上传文件（400）

```bash
curl -X POST "http://localhost:8080/api/v1/files/upload" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -F "path=/documents"
```

**响应：**
```json
{
  "success": false,
  "error": "No file uploaded",
  "code": "NO_FILE"
}
```

### 示例 4：不支持的文件类型（400）

```bash
curl -X POST "http://localhost:8080/api/v1/files/upload" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -F "file=@/path/to/script.exe"
```

**响应：**
```json
{
  "success": false,
  "error": "File type not allowed",
  "code": "UNSUPPORTED_TYPE"
}
```

### 示例 5：文件过大（413）

```bash
curl -X POST "http://localhost:8080/api/v1/files/upload" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -F "file=@/path/to/large_file.zip"
```

**响应：**
```json
{
  "success": false,
  "error": "File size exceeds limit of 100 MB",
  "code": "FILE_TOO_LARGE"
}
```

### 示例 6：未认证访问（401）

```bash
curl -X POST "http://localhost:8080/api/v1/files/upload" \
  -F "file=@/path/to/document.pdf"
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

### 2. 文件大小限制

- 最大文件大小：100MB
- 超过限制返回 413 Payload Too Large

### 3. 文件类型限制

允许的文件类型：
- 文档：txt, pdf, doc, docx, xls, xlsx
- 图片：jpg, jpeg, png, gif

### 4. 路径权限验证

- 验证用户对目标路径的写权限
- 无权限返回 403 Forbidden

## 实现文件

- `src/handlers/files_upload.rs` - 文件上传处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **文件大小**：最大 100MB
3. **文件类型**：仅限支持的类型
4. **路径**：相对于用户主目录

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `NO_FILE` | 400 | 未上传文件 |
| `UNSUPPORTED_TYPE` | 400 | 不支持的文件类型 |
| `FILE_TOO_LARGE` | 413 | 文件过大 |
| `PERMISSION_DENIED` | 403 | 权限不足 |
| `SAVE_ERROR` | 500 | 保存文件失败 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/files/list** - 获取文件列表 (Phase 117)
- **GET /api/v1/files/{id}/download** - 下载文件
- **PUT /api/v1/files/{id}** - 更新文件
- **DELETE /api/v1/files/{id}** - 删除文件

## 响应示例（完整）

### 成功上传

```json
{
  "success": true,
  "message": "File uploaded successfully",
  "data": {
    "filename": "document.pdf",
    "path": "/documents/document.pdf",
    "size_bytes": 2048576,
    "mime_type": "application/pdf",
    "uploaded_at": 1711468800
  }
}
```

### 未上传文件（400）

```json
{
  "success": false,
  "error": "No file uploaded",
  "code": "NO_FILE"
}
```

### 不支持的文件类型（400）

```json
{
  "success": false,
  "error": "File type not allowed",
  "code": "UNSUPPORTED_TYPE"
}
```

### 文件过大（413）

```json
{
  "success": false,
  "error": "File size exceeds limit of 100 MB",
  "code": "FILE_TOO_LARGE"
}
```

## 最佳实践

### 1. 前端集成示例

```javascript
const uploadFile = async (file, path = '/') => {
  const formData = new FormData();
  formData.append('file', file);
  formData.append('path', path);

  try {
    const response = await fetch('/api/v1/files/upload', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`
      },
      body: formData
    });

    const data = await response.json();
    if (response.ok) {
      console.log('File uploaded:', data.data);
      return data.data;
    } else {
      console.error('Upload failed:', data.error);
      throw new Error(data.error);
    }
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
};

// 使用示例
const fileInput = document.querySelector('input[type="file"]');
const file = fileInput.files[0];
uploadFile(file, '/documents');
```

### 2. React 组件示例

```jsx
const FileUpload = ({ path = '/' }) => {
  const [uploading, setUploading] = useState(false);
  const [progress, setProgress] = useState(0);

  const handleUpload = async (event) => {
    const file = event.target.files[0];
    if (!file) return;

    const formData = new FormData();
    formData.append('file', file);
    formData.append('path', path);

    setUploading(true);
    try {
      const response = await fetch('/api/v1/files/upload', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`
        },
        body: formData
      });
      const data = await response.json();
      if (response.ok) {
        alert('File uploaded successfully!');
      } else {
        alert(`Upload failed: ${data.error}`);
      }
    } catch (error) {
      alert('Upload error');
    } finally {
      setUploading(false);
    }
  };

  return (
    <div>
      <input
        type="file"
        onChange={handleUpload}
        disabled={uploading}
      />
      {uploading && <progress value={progress} max="100" />}
    </div>
  );
};
```

### 3. 错误处理

```javascript
const handleUploadError = (error) => {
  switch (error.code) {
    case 'NO_FILE':
      return 'Please select a file to upload.';
    case 'UNSUPPORTED_TYPE':
      return 'File type not allowed. Allowed types: txt, pdf, doc, docx, xls, xlsx, jpg, jpeg, png, gif.';
    case 'FILE_TOO_LARGE':
      return 'File size exceeds limit of 100 MB.';
    case 'PERMISSION_DENIED':
      return 'No write permission for this path.';
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    default:
      return 'Upload failed. Please try again.';
  }
};
```

### 4. 审计日志

所有文件上传操作都应该记录到审计日志中，包括：
- 上传时间
- 执行上传的用户 ID
- 文件名和大小
- 目标路径
- 上传结果（成功/失败）
