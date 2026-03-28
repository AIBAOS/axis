# 媒体照片详情 API

## Phase 238

## 接口说明

获取单个照片的详细信息。

## 请求

`GET /api/v1/media/photos/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 照片 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "photo_001.jpg",
    "path": "/media/photos/photo_001.jpg",
    "size_bytes": 5242880,
    "width": 4000,
    "height": 3000,
    "taken_at": 1711497600,
    "created_at": 1711497600,
    "updated_at": 1711497600,
    "thumbnail_path": "/media/thumbnails/photo_001.jpg",
    "album": "Album A",
    "exif": {
      "camera": "Canon EOS R5",
      "lens": "RF 24-70mm F2.8",
      "focal_length": "50mm",
      "aperture": "f/2.8",
      "shutter_speed": "1/250",
      "iso": 400,
      "location": "Beijing, China"
    }
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | object | 照片详情 |
| data.id | integer | 照片 ID |
| data.name | string | 照片文件名 |
| data.path | string | 照片文件路径 |
| data.size_bytes | integer | 文件大小（字节） |
| data.width | integer | 宽度（像素） |
| data.height | integer | 高度（像素） |
| data.taken_at | integer | 拍摄时间（Unix 时间戳，可选） |
| data.created_at | integer | 创建时间（Unix 时间戳） |
| data.updated_at | integer | 修改时间（Unix 时间戳） |
| data.thumbnail_path | string | 缩略图路径 |
| data.album | string | 所属相册（可选） |
| data.exif | object | EXIF 元数据（可选） |
| data.exif.camera | string | 相机型号 |
| data.exif.lens | string | 镜头型号 |
| data.exif.focal_length | string | 焦距 |
| data.exif.aperture | string | 光圈 |
| data.exif.shutter_speed | string | 快门速度 |
| data.exif.iso | integer | ISO 感光度 |
| data.exif.location | string | 拍摄地点 |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 404 Not Found - 照片不存在

```json
{
  "success": false,
  "error": "Photo 999 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to get current time",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取照片详情

```bash
curl -X GET "http://localhost:8080/api/v1/media/photos/1" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "photo_001.jpg",
    "path": "/media/photos/photo_001.jpg",
    "size_bytes": 5242880,
    "width": 4000,
    "height": 3000,
    "taken_at": 1711497600,
    "created_at": 1711497600,
    "updated_at": 1711497600,
    "thumbnail_path": "/media/thumbnails/photo_001.jpg",
    "album": "Album A",
    "exif": {
      "camera": "Canon EOS R5",
      "location": "Beijing, China"
    }
  }
}
```

### 获取不存在的照片

```bash
curl -X GET "http://localhost:8080/api/v1/media/photos/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Photo 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 解析照片 ID 路径参数
3. 查询照片详情
4. 照片不存在返回 404 Not Found
5. 返回照片完整详情（含 EXIF 元数据）

## 版本历史

- **Phase 238** (2026-03-28): 媒体模块 - 照片详情 API
