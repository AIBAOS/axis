# 媒体音频详情 API

## Phase 237

## 接口说明

获取单个音频的详细信息。

## 请求

`GET /api/v1/media/audios/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 音频 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "song_001.mp3",
    "path": "/media/audios/song_001.mp3",
    "size_bytes": 10485760,
    "duration_seconds": 240,
    "artist": "Artist A",
    "album": "Album X",
    "track_number": 1,
    "genre": "Rock",
    "bitrate": 320000,
    "sample_rate": 44100,
    "created_at": 1711497600,
    "updated_at": 1711497600,
    "thumbnail_path": "/media/thumbnails/song_001.jpg"
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | object | 音频详情 |
| data.id | integer | 音频 ID |
| data.name | string | 音频文件名 |
| data.path | string | 音频文件路径 |
| data.size_bytes | integer | 文件大小（字节） |
| data.duration_seconds | integer | 时长（秒） |
| data.artist | string | 艺术家 |
| data.album | string | 专辑 |
| data.track_number | integer | 音轨编号 |
| data.genre | string | 音乐类型 |
| data.bitrate | integer | 比特率（bps） |
| data.sample_rate | integer | 采样率（Hz） |
| data.created_at | integer | 创建时间（Unix 时间戳） |
| data.updated_at | integer | 修改时间（Unix 时间戳） |
| data.thumbnail_path | string | 缩略图路径 |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 404 Not Found - 音频不存在

```json
{
  "success": false,
  "error": "Audio 999 not found",
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

### 获取音频详情

```bash
curl -X GET "http://localhost:8080/api/v1/media/audios/1" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "song_001.mp3",
    "path": "/media/audios/song_001.mp3",
    "size_bytes": 10485760,
    "duration_seconds": 240,
    "artist": "Artist A",
    "album": "Album X",
    "track_number": 1,
    "genre": "Rock",
    "bitrate": 320000,
    "sample_rate": 44100,
    "created_at": 1711497600,
    "updated_at": 1711497600,
    "thumbnail_path": "/media/thumbnails/song_001.jpg"
  }
}
```

### 获取不存在的音频

```bash
curl -X GET "http://localhost:8080/api/v1/media/audios/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Audio 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 解析音频 ID 路径参数
3. 查询音频详情
4. 音频不存在返回 404 Not Found
5. 返回音频完整详情

## 版本历史

- **Phase 237** (2026-03-28): 媒体模块 - 音频详情 API
