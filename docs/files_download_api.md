# 文件下载 API 文档 (Phase 120)

## 概述

文件下载 API 允许用户下载指定文件，支持 HTTP Range 请求（断点续传）。

## 接口详情

### GET /api/v1/files/{id}/download

下载指定文件。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 文件 ID |

#### 响应头

| 头 | 说明 |
|------|------|
| `Content-Type` | 文件的 MIME 类型 |
| `Content-Disposition` | `attachment; filename="<filename>"` |
| `Content-Range` | Range 请求时返回（格式：`bytes start-end/total`） |
| `Accept-Ranges` | `bytes`（支持 Range 请求） |
| `Content-Length` | 响应内容长度 |

#### 响应格式

**成功响应 (200 OK)**

完整文件下载：
```
HTTP/1.1 200 OK
Content-Type: application/pdf
Content-Disposition: attachment; filename="document.pdf"
Accept-Ranges: bytes
Content-Length: 2048576

<file content>
```

**部分响应 (206 Partial Content)**

断点续传：
```
HTTP/1.1 206 Partial Content
Content-Type: application/pdf
Content-Disposition: attachment; filename="document.pdf"
Content-Range: bytes 1000-1999/2048576
Content-Length: 1000

<partial file content>
```

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

**错误响应 (416 Range Not Satisfiable) - Range 无效**

```json
{
  "success": false,
  "error": "Invalid range: bytes=1000-2000/1000",
  "code": "INVALID_RANGE"
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

### 示例 1：下载完整文件

```bash
curl -X GET "http://localhost:8080/api/v1/files/1/download" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -o downloaded_file.pdf
```

### 示例 2：断点续传（下载部分文件）

```bash
# 下载前 1MB
curl -X GET "http://localhost:8080/api/v1/files/1/download" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Range: bytes=0-1048575" \
  -o part1.pdf

# 继续下载剩余部分
curl -X GET "http://localhost:8080/api/v1/files/1/download" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Range: bytes=1048576-" \
  -o part2.pdf
```

### 示例 4：文件不存在（404）

```bash
curl -X GET "http://localhost:8080/api/v1/files/999/download" \
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

### 示例 5：Range 无效（416）

```bash
curl -X GET "http://localhost:8080/api/v1/files/1/download" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Range: bytes=10000000-20000000"
```

**响应：**
```json
{
  "success": false,
  "error": "Invalid range: bytes=10000000-20000000/2048576",
  "code": "INVALID_RANGE"
}
```

### 示例 6：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/files/1/download"
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

### 3. Range 请求支持

- 支持断点续传
- 支持部分下载
- 无效 Range 返回 416 Range Not Satisfiable

## 实现文件

- `src/handlers/files_download.rs` - 文件下载处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **Range 支持**：支持断点续传和部分下载
3. **Content-Disposition**：自动设置下载文件名

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `PERMISSION_DENIED` | 403 | 无权限访问该文件 |
| `NOT_FOUND` | 404 | 文件不存在 |
| `INVALID_RANGE` | 416 | Range 无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/files/list** - 获取文件列表 (Phase 117)
- **POST /api/v1/files/upload** - 上传文件 (Phase 119)
- **PUT /api/v1/files/{id}** - 更新文件
- **DELETE /api/v1/files/{id}** - 删除文件

## 响应示例（完整）

### 成功下载（200 OK）

```
HTTP/1.1 200 OK
Content-Type: application/pdf
Content-Disposition: attachment; filename="document.pdf"
Accept-Ranges: bytes
Content-Length: 2048576

<file content>
```

### 部分下载（206 Partial Content）

```
HTTP/1.1 206 Partial Content
Content-Type: application/pdf
Content-Disposition: attachment; filename="document.pdf"
Content-Range: bytes 1000-1999/2048576
Content-Length: 1000

<partial file content>
```

### 文件不存在（404）

```json
{
  "success": false,
  "error": "File not found",
  "code": "NOT_FOUND"
}
```

### Range 无效（416）

```json
{
  "success": false,
  "error": "Invalid range: bytes=1000-2000/1000",
  "code": "INVALID_RANGE"
}
```

## 最佳实践

### 1. 前端下载示例

```javascript
const downloadFile = async (fileId, filename) => {
  try {
    const response = await fetch(
      `/api/v1/files/${fileId}/download`,
      {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      }
    );

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error);
    }

    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    window.URL.revokeObjectURL(url);
    document.body.removeChild(a);
  } catch (error) {
    console.error('Download error:', error);
    alert(`Download failed: ${error.message}`);
  }
};

// 使用示例
downloadFile(1, 'document.pdf');
```

### 2. 断点续传示例

```javascript
const downloadWithResume = async (fileId, filename) => {
  let downloadedSize = 0;
  
  // 检查已下载的文件大小
  const existingFile = await getFileFromStorage(filename);
  if (existingFile) {
    downloadedSize = existingFile.size;
  }

  try {
    const response = await fetch(
      `/api/v1/files/${fileId}/download`,
      {
        headers: {
          'Authorization': `Bearer ${token}`,
          'Range': `bytes=${downloadedSize}-`
        }
      }
    );

    if (response.status === 206) {
      // 部分内容（断点续传）
      const blob = await response.blob();
      // 追加到现有文件
    } else if (response.status === 200) {
      // 完整内容（从头开始）
      const blob = await response.blob();
      // 保存完整文件
    }
  } catch (error) {
    console.error('Download error:', error);
  }
};
```

### 3. 错误处理

```javascript
const handleDownloadError = (error) => {
  switch (error.code) {
    case 'NOT_FOUND':
      return 'File not found.';
    case 'PERMISSION_DENIED':
      return 'No permission to access this file.';
    case 'INVALID_RANGE':
      return 'Invalid range request.';
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    default:
      return 'Download failed. Please try again.';
  }
};
```

### 4. 审计日志

所有文件下载操作都应该记录到审计日志中，包括：
- 下载时间
- 执行下载的用户 ID
- 文件 ID 和名称
- 下载范围（如果是 Range 请求）
- 下载结果（成功/失败）
