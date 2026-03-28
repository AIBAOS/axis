# 媒体照片上传 API

## Phase 239

## 接口说明

上传照片到媒体库，支持文件类型和大小验证。

## 请求

`POST /api/v1/media/photos`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `multipart/form-data` |

### 请求体（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| file | file | 是 | 照片文件（支持 jpg/jpeg/png/webp） |

### 请求示例

```bash
curl -X POST "http://localhost:8080/api/v1/media/photos" \
  -H "Authorization: Bearer <jwt_token>" \
  -F "file=@/path/to/photo.jpg"
```

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "Photo uploaded successfully",
  "data": {
    "id": 123456,
    "name": "photo.jpg",
    "path": "/media/photos/photo.jpg",
    "size_bytes": 5242880,
    "width": 4000,
    "height": 3000,
    "thumbnail_path": "/media/thumbnails/photo.jpg",
    "created_at": 1711584000
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| message | string | 响应消息 |
| data | object | 上传的照片信息 |
| data.id | integer | 照片 ID |
| data.name | string | 照片文件名 |
| data.path | string | 照片文件路径 |
| data.size_bytes | integer | 文件大小（字节） |
| data.width | integer | 宽度（像素） |
| data.height | integer | 高度（像素） |
| data.thumbnail_path | string | 缩略图路径 |
| data.created_at | integer | 创建时间（Unix 时间戳） |

### 错误响应

#### 400 Bad Request - 文件类型无效

```json
{
  "success": false,
  "error": "Invalid file type 'gif'. Allowed: jpg, jpeg, png, webp",
  "code": "INVALID_FILE_TYPE"
}
```

#### 400 Bad Request - 未上传文件

```json
{
  "success": false,
  "error": "No file uploaded",
  "code": "NO_FILE"
}
```

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 413 Payload Too Large - 文件超大

```json
{
  "success": false,
  "error": "File size exceeds 50MB limit",
  "code": "FILE_TOO_LARGE"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to read file: io error",
  "code": "INTERNAL_ERROR"
}
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 解析 multipart/form-data 请求
3. 验证文件类型（jpg/jpeg/png/webp）
4. 验证文件大小（max 50MB）
5. 保存文件到媒体库
6. 自动生成缩略图
7. 返回上传后的照片信息

## 支持的文件格式

| 格式 | 扩展名 | MIME Type |
|------|--------|-----------|
| JPEG | .jpg, .jpeg | image/jpeg |
| PNG | .png | image/png |
| WebP | .webp | image/webp |

## 版本历史

- **Phase 239** (2026-03-28): 媒体模块 - 照片上传 API
