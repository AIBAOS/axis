# 文件搜索 API (Phase 46)

## 接口说明

实现文件搜索功能，支持按名称/类型/时间筛选。

## 接口定义

```
GET /api/v1/files/search
```

## 请求参数

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| q | string | 否 | 搜索关键词（模糊匹配文件名） |
| path | string | 否 | 搜索起始路径（默认用户根目录） |
| recursive | boolean | 否 | 是否递归搜索子目录（默认 false） |
| file_type | string | 否 | 文件类型筛选：file/folder/image/video/document |
| modified_after | integer | 否 | 修改时间之后（Unix 时间戳，秒） |
| modified_before | integer | 否 | 修改时间之前（Unix 时间戳，秒） |
| limit | integer | 否 | 分页限制（默认 50，最大 1000） |
| offset | integer | 否 | 分页偏移（默认 0） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "results": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "report.pdf",
      "path": "documents/report.pdf",
      "full_path": "/data/files/1/documents/report.pdf",
      "size": 102400,
      "is_dir": false,
      "file_type": "document",
      "modified_at": 1711411200,
      "created_at": 1711324800,
      "mime_type": "application/pdf"
    }
  ],
  "total": 1,
  "limit": 50,
  "offset": 0
}
```

### 无结果 (200 OK)

```json
{
  "success": true,
  "results": [],
  "total": 0,
  "limit": 50,
  "offset": 0
}
```

### 参数错误 (400 Bad Request)

```json
{
  "success": false,
  "error": "Invalid file_type: invalid. Valid values: file, folder, image, video, document",
  "code": "INVALID_PARAMS"
}
```

### 路径遍历攻击 (400 Bad Request)

```json
{
  "success": false,
  "error": "Path traversal detected",
  "code": "INVALID_PATH"
}
```

## 使用示例

### 1. 基础搜索（按名称）

```bash
curl -X GET "http://localhost:8080/api/v1/files/search?q=report" \
  -H "Authorization: Bearer <jwt_token>"
```

### 2. 按类型筛选

```bash
# 搜索所有图片
curl -X GET "http://localhost:8080/api/v1/files/search?file_type=image" \
  -H "Authorization: Bearer <jwt_token>"

# 搜索所有文档
curl -X GET "http://localhost:8080/api/v1/files/search?file_type=document" \
  -H "Authorization: Bearer <jwt_token>"
```

### 3. 递归搜索

```bash
# 递归搜索所有子目录
curl -X GET "http://localhost:8080/api/v1/files/search?q=project&recursive=true" \
  -H "Authorization: Bearer <jwt_token>"
```

### 4. 按时间范围筛选

```bash
# 搜索最近 7 天修改的文件
curl -X GET "http://localhost:8080/api/v1/files/search?modified_after=1710806400" \
  -H "Authorization: Bearer <jwt_token>"

# 搜索指定时间范围的文件
curl -X GET "http://localhost:8080/api/v1/files/search?modified_after=1710806400&modified_before=1711411200" \
  -H "Authorization: Bearer <jwt_token>"
```

### 5. 组合搜索

```bash
# 在 documents 目录中递归搜索 PDF 文件
curl -X GET "http://localhost:8080/api/v1/files/search?q=report&path=documents&recursive=true&file_type=document" \
  -H "Authorization: Bearer <jwt_token>"

# 分页搜索
curl -X GET "http://localhost:8080/api/v1/files/search?q=test&limit=20&offset=40" \
  -H "Authorization: Bearer <jwt_token>"
```

## 安全特性

1. **路径遍历防护**: 所有路径请求都会经过规范化处理，确保不会访问用户目录之外的文件
2. **搜索深度限制**: recursive=true 时最多搜索 10 层目录，防止无限递归
3. **用户隔离**: 从 JWT Token 提取 user_id，用户只能搜索自己的文件
4. **参数验证**: 所有查询参数都经过严格验证，非法参数返回 400 错误

## 文件类型说明

| 类型 | 扩展名 |
|------|--------|
| image | jpg, jpeg, png, gif, bmp, webp, svg |
| video | mp4, avi, mkv, mov, wmv, flv, webm |
| document | pdf, doc, docx, xls, xlsx, ppt, pptx, txt, md |
| file | 其他所有文件 |
| folder | 目录 |

## 实现文件

- `src/handlers/files_search.rs` - 搜索处理器实现
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 搜索关键词支持模糊匹配（大小写不敏感）
2. 无搜索结果时返回 200 状态码和空列表
3. limit 参数范围为 1-1000，超过会返回参数错误
4. 时间戳使用 Unix 时间戳（秒级）
