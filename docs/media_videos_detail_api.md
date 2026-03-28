# 媒体视频详情 API

## Phase 236

## 接口说明

获取单个视频的详细信息。

## 请求

`GET /api/v1/media/videos/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 视频 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "movie_001.mp4",
    "path": "/media/videos/movie_001.mp4",
    "size_bytes": 2147483648,
    "duration_seconds": 7200,
    "resolution": "1920x1080",
    "codec": "H.264",
    "bitrate": 5000000,
    "framerate": 30.0,
    "created_at": 1711497600,
    "modified_at": 1711497600,
    "thumbnail_path": "/media/thumbnails/movie_001.jpg",
    "metadata": {
      "folder": "/media/videos",
      "tags": ["movie", "hd"]
    }
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | object | 视频详情 |
| data.id | integer | 视频 ID |
| data.name | string | 视频文件名 |
| data.path | string | 视频文件路径 |
| data.size_bytes | integer | 文件大小（字节） |
| data.duration_seconds | integer | 时长（秒） |
| data.resolution | string | 分辨率（如 1920x1080） |
| data.codec | string | 编码格式（如 H.264, H.265） |
| data.bitrate | integer | 比特率（bps） |
| data.framerate | float | 帧率（fps） |
| data.created_at | integer | 创建时间（Unix 时间戳） |
| data.modified_at | integer | 修改时间（Unix 时间戳） |
| data.thumbnail_path | string | 缩略图路径 |
| data.metadata | object | 元数据（可选） |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 404 Not Found - 视频不存在

```json
{
  "success": false,
  "error": "Video 999 not found",
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

### 获取视频详情

```bash
curl -X GET "http://localhost:8080/api/v1/media/videos/1" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "movie_001.mp4",
    "path": "/media/videos/movie_001.mp4",
    "size_bytes": 2147483648,
    "duration_seconds": 7200,
    "resolution": "1920x1080",
    "codec": "H.264",
    "bitrate": 5000000,
    "framerate": 30.0,
    "created_at": 1711497600,
    "modified_at": 1711497600,
    "thumbnail_path": "/media/thumbnails/movie_001.jpg",
    "metadata": {
      "folder": "/media/videos"
    }
  }
}
```

### 获取不存在的视频

```bash
curl -X GET "http://localhost:8080/api/v1/media/videos/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Video 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 解析视频 ID 路径参数
3. 查询视频详情
4. 视频不存在返回 404 Not Found
5. 返回视频完整详情

## 版本历史

- **Phase 236** (2026-03-28): 媒体模块 - 视频详情 API
