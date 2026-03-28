# 文件浏览 API 文档 (Phase 104)

## 概述

文件浏览 API 提供文件系统的浏览功能，支持查看指定目录下的文件夹和文件列表。

## 接口详情

### GET /api/v1/files/browse

浏览指定路径的文件和文件夹。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 路径参数

无

#### 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `path` | string | 否 | `/` | 浏览路径（绝对路径） |
| `page` | integer | 否 | 1 | 页码（从 1 开始） |
| `limit` | integer | 否 | 50 | 每页数量（最大 200） |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "current_path": "/documents",
  "parent_path": "/",
  "folders": [
    {
      "name": "photos",
      "path": "/documents/photos",
      "size_bytes": 0,
      "modified_at": 1711468800
    },
    {
      "name": "reports",
      "path": "/documents/reports",
      "size_bytes": 0,
      "modified_at": 1711468800
    }
  ],
  "files": [
    {
      "name": "readme.txt",
      "path": "/documents/readme.txt",
      "size_bytes": 1024,
      "mime_type": "text/plain",
      "modified_at": 1711468800
    },
    {
      "name": "report.pdf",
      "path": "/documents/report.pdf",
      "size_bytes": 2048576,
      "mime_type": "application/pdf",
      "modified_at": 1711468800
    }
  ],
  "total_items": 4,
  "pagination": {
    "page": 1,
    "limit": 50,
    "total_items": 4,
    "total_pages": 1
  }
}
```

**字段说明**

- `current_path`: 当前浏览路径
- `parent_path`: 父目录路径（根目录时为 null）
- `folders`: 子文件夹列表
  - `name`: 文件夹名称
  - `path`: 文件夹路径
  - `size_bytes`: 大小（字节，文件夹为 0）
  - `modified_at`: 修改时间（Unix 时间戳）
- `files`: 文件列表
  - `name`: 文件名称
  - `path`: 文件路径
  - `size_bytes`: 文件大小（字节）
  - `mime_type`: MIME 类型
  - `modified_at`: 修改时间（Unix 时间戳）
- `total_items`: 总项目数（文件夹 + 文件）
- `pagination`: 分页信息
  - `page`: 当前页码
  - `limit`: 每页数量
  - `total_items`: 总项目数
  - `total_pages`: 总页数

**错误响应 (404 Not Found) - 路径不存在**

```json
{
  "success": false,
  "error": "Path '/nonexistent' not found",
  "code": "PATH_NOT_FOUND"
}
```

**错误响应 (400 Bad Request) - 不是目录**

```json
{
  "success": false,
  "error": "Path is not a directory",
  "code": "NOT_DIRECTORY"
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

### 示例 1：浏览根目录

```bash
curl -X GET "http://localhost:8080/api/v1/files/browse" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "current_path": "/",
  "parent_path": null,
  "folders": [
    {
      "name": "documents",
      "path": "/documents",
      "size_bytes": 0,
      "modified_at": 1711468800
    },
    {
      "name": "photos",
      "path": "/photos",
      "size_bytes": 0,
      "modified_at": 1711468800
    }
  ],
  "files": [],
  "total_items": 2,
  "pagination": {
    "page": 1,
    "limit": 50,
    "total_items": 2,
    "total_pages": 1
  }
}
```

### 示例 2：浏览指定目录

```bash
curl -X GET "http://localhost:8080/api/v1/files/browse?path=/documents" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 3：分页浏览

```bash
curl -X GET "http://localhost:8080/api/v1/files/browse?path=/documents&page=2&limit=20" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 4：浏览不存在的目录（404）

```bash
curl -X GET "http://localhost:8080/api/v1/files/browse?path=/nonexistent" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Path '/nonexistent' not found",
  "code": "PATH_NOT_FOUND"
}
```

### 示例 5：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/files/browse"
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

### 2. 路径安全

- 路径验证：确保路径存在且为目录
- 防止路径遍历攻击
- 限制访问范围在基础目录内

### 3. 分页限制

- 每页最大 200 项
- 防止大量数据一次性返回

## MIME 类型说明

| 扩展名 | MIME 类型 |
|--------|----------|
| jpg, jpeg | image/jpeg |
| png | image/png |
| gif | image/gif |
| bmp | image/bmp |
| webp | image/webp |
| mp4 | video/mp4 |
| avi | video/x-msvideo |
| mov | video/quicktime |
| pdf | application/pdf |
| doc | application/msword |
| docx | application/vnd.openxmlformats-officedocument.wordprocessingml.document |
| xls | application/vnd.ms-excel |
| xlsx | application/vnd.openxmlformats-officedocument.spreadsheetml.sheet |
| ppt | application/vnd.ms-powerpoint |
| pptx | application/vnd.openxmlformats-officedocument.presentationml.presentation |
| txt | text/plain |
| md | text/markdown |
| html | text/html |
| json | application/json |
| zip | application/zip |
| 其他 | application/octet-stream |

## 实现文件

- `src/handlers/files_browse.rs` - 文件浏览处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **路径验证**：路径必须存在且为目录
3. **分页限制**：每页最大 200 项
4. **MIME 类型**：根据文件扩展名自动识别

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `PATH_NOT_FOUND` | 404 | 路径不存在 |
| `NOT_DIRECTORY` | 400 | 路径不是目录 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/files/{path}** - 获取文件详情
- **POST /api/v1/files/{path}** - 上传文件
- **DELETE /api/v1/files/{path}** - 删除文件/文件夹
- **GET /api/v1/files/{path}/download** - 下载文件

## 响应示例（完整）

### 根目录浏览

```json
{
  "current_path": "/",
  "parent_path": null,
  "folders": [
    {
      "name": "documents",
      "path": "/documents",
      "size_bytes": 0,
      "modified_at": 1711468800
    }
  ],
  "files": [
    {
      "name": "readme.txt",
      "path": "/readme.txt",
      "size_bytes": 1024,
      "mime_type": "text/plain",
      "modified_at": 1711468800
    }
  ],
  "total_items": 2,
  "pagination": {
    "page": 1,
    "limit": 50,
    "total_items": 2,
    "total_pages": 1
  }
}
```

### 路径不存在（404）

```json
{
  "success": false,
  "error": "Path '/nonexistent' not found",
  "code": "PATH_NOT_FOUND"
}
```

### 不是目录（400）

```json
{
  "success": false,
  "error": "Path is not a directory",
  "code": "NOT_DIRECTORY"
}
```

## 最佳实践

### 1. 路径处理

使用绝对路径浏览：

```javascript
// 浏览根目录
const browseRoot = async () => {
  const response = await fetch('/api/v1/files/browse', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return await response.json();
};

// 浏览子目录
const browseDirectory = async (path) => {
  const response = await fetch(`/api/v1/files/browse?path=${encodeURIComponent(path)}`, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return await response.json();
};
```

### 2. 分页处理

处理大量文件时分页加载：

```javascript
const browseWithPagination = async (path, page = 1, limit = 50) => {
  const response = await fetch(
    `/api/v1/files/browse?path=${encodeURIComponent(path)}&page=${page}&limit=${limit}`,
    {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    }
  );
  return await response.json();
};
```

### 3. 错误处理

处理各种错误情况：

```javascript
try {
  const data = await browseDirectory('/documents');
  if (data.folders) {
    console.log('Folders:', data.folders);
  }
  if (data.files) {
    console.log('Files:', data.files);
  }
} catch (error) {
  if (error.code === 'PATH_NOT_FOUND') {
    console.error('Directory not found');
  } else if (error.code === 'UNAUTHORIZED') {
    console.error('Unauthorized access');
  }
}
```

### 4. 前端集成示例

React 组件示例：

```jsx
const FileBrowser = () => {
  const [currentPath, setCurrentPath] = useState('/');
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const response = await fetch(
          `/api/v1/files/browse?path=${encodeURIComponent(currentPath)}`,
          {
            headers: {
              'Authorization': `Bearer ${token}`
            }
          }
        );
        const data = await response.json();
        setData(data);
      } catch (error) {
        console.error('Failed to fetch:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [currentPath]);

  if (loading) return <div>Loading...</div>;
  if (!data) return <div>Error loading data</div>;

  return (
    <div>
      <div>Current: {data.current_path}</div>
      {data.parent_path && (
        <button onClick={() => setCurrentPath(data.parent_path)}>
          ⬆ Parent
        </button>
      )}
      <div>
        {data.folders.map(folder => (
          <div key={folder.path} onClick={() => setCurrentPath(folder.path)}>
            📁 {folder.name}
          </div>
        ))}
        {data.files.map(file => (
          <div key={file.path}>
            📄 {file.name}
          </div>
        ))}
      </div>
    </div>
  );
};
```
