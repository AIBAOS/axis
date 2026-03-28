# 文件列表 API 文档 (Phase 117)

## 概述

文件列表 API 提供指定目录下的文件和文件夹列表。

## 接口详情

### GET /api/v1/files/list

获取指定目录下的文件/文件夹列表。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `path` | string | 否 | `/` | 目录路径（相对于根目录） |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "data": [
    {
      "name": "documents",
      "type": "dir",
      "size": 0,
      "modified": 1711468800
    },
    {
      "name": "report.pdf",
      "type": "file",
      "size": 2048576,
      "modified": 1711468800
    }
  ],
  "path": "/documents"
}
```

**字段说明**

- `success`: 请求是否成功
- `data`: 文件/文件夹列表
  - `name`: 名称
  - `type`: 类型（`file` 或 `dir`）
  - `size`: 大小（字节，文件夹为 0）
  - `modified`: 修改时间（Unix 时间戳）
- `path`: 当前查询的路径

**错误响应 (400 Bad Request) - 路径无效**

```json
{
  "success": false,
  "message": "Invalid path format",
  "code": "INVALID_PATH"
}
```

**错误响应 (404 Not Found) - 目录不存在**

```json
{
  "success": false,
  "message": "Directory not found: /nonexistent",
  "code": "NOT_FOUND"
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

### 示例 1：获取根目录列表

```bash
curl -X GET "http://localhost:8080/api/v1/files/list" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": true,
  "data": [
    {
      "name": "documents",
      "type": "dir",
      "size": 0,
      "modified": 1711468800
    },
    {
      "name": "report.pdf",
      "type": "file",
      "size": 2048576,
      "modified": 1711468800
    }
  ],
  "path": "/"
}
```

### 示例 2：获取子目录列表

```bash
curl -X GET "http://localhost:8080/api/v1/files/list?path=/documents" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": true,
  "data": [
    {
      "name": "reports",
      "type": "dir",
      "size": 0,
      "modified": 1711468800
    },
    {
      "name": "report.pdf",
      "type": "file",
      "size": 2048576,
      "modified": 1711468800
    }
  ],
  "path": "/documents"
}
```

### 示例 3：目录不存在（404）

```bash
curl -X GET "http://localhost:8080/api/v1/files/list?path=/nonexistent" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "message": "Directory not found: /nonexistent",
  "code": "NOT_FOUND"
}
```

### 示例 4：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/files/list"
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

### 2. 路径验证

- 验证路径格式合法性
- 验证目录存在性
- 防止路径遍历攻击

## 实现文件

- `src/handlers/files_list.rs` - 文件列表处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **路径格式**：相对于根目录的路径
3. **根目录**：使用 `/` 或省略 path 参数

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `INVALID_PATH` | 400 | 路径格式无效 |
| `NOT_FOUND` | 404 | 目录不存在 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/files/{id}/download** - 下载文件
- **POST /api/v1/files/upload** - 上传文件
- **PUT /api/v1/files/{id}** - 更新文件
- **DELETE /api/v1/files/{id}** - 删除文件

## 响应示例（完整）

### 根目录列表

```json
{
  "success": true,
  "data": [
    {
      "name": "documents",
      "type": "dir",
      "size": 0,
      "modified": 1711468800
    },
    {
      "name": "photos",
      "type": "dir",
      "size": 0,
      "modified": 1711468800
    },
    {
      "name": "report.pdf",
      "type": "file",
      "size": 2048576,
      "modified": 1711468800
    }
  ],
  "path": "/"
}
```

### 子目录列表

```json
{
  "success": true,
  "data": [
    {
      "name": "2024",
      "type": "dir",
      "size": 0,
      "modified": 1711468800
    },
    {
      "name": "report.pdf",
      "type": "file",
      "size": 2048576,
      "modified": 1711468800
    }
  ],
  "path": "/documents"
}
```

### 目录不存在（404）

```json
{
  "success": false,
  "message": "Directory not found: /nonexistent",
  "code": "NOT_FOUND"
}
```

## 最佳实践

### 1. 前端集成示例

```javascript
const getFileList = async (path = '/') => {
  try {
    const params = new URLSearchParams({ path });
    const response = await fetch(
      `/api/v1/files/list?${params}`,
      {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      }
    );

    const data = await response.json();
    if (response.ok) {
      console.log('File list:', data.data);
      return data.data;
    } else {
      console.error('Failed to get file list:', data.error);
      throw new Error(data.error);
    }
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
};

// 使用示例 - 获取根目录
getFileList('/');

// 使用示例 - 获取子目录
getFileList('/documents');
```

### 2. React 组件示例

```jsx
const FileList = ({ path = '/' }) => {
  const [files, setFiles] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchFiles = async () => {
      try {
        const params = new URLSearchParams({ path });
        const response = await fetch(
          `/api/v1/files/list?${params}`,
          {
            headers: {
              'Authorization': `Bearer ${token}`
            }
          }
        );
        const data = await response.json();
        if (response.ok) {
          setFiles(data.data);
        }
      } catch (error) {
        console.error('Error:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchFiles();
  }, [path]);

  if (loading) return <div>Loading...</div>;

  return (
    <div>
      <h3>Files in {path}</h3>
      {files.map(file => (
        <div key={file.name}>
          {file.type === 'dir' ? '📁' : '📄'} {file.name}
        </div>
      ))}
    </div>
  );
};
```

### 3. 错误处理

```javascript
const handleFileListError = (error) => {
  switch (error.code) {
    case 'NOT_FOUND':
      return 'Directory not found.';
    case 'INVALID_PATH':
      return 'Invalid path format.';
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    default:
      return 'Failed to get file list. Please try again.';
  }
};
```

### 4. 审计日志

所有文件列表查询操作都应该记录到审计日志中，包括：
- 查询时间
- 执行查询的用户 ID
- 查询的路径
- 返回结果数量
- 查询结果（成功/失败）
