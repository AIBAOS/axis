# 媒体视频详情 API 文档

## 概述

本文档描述 Axis NAS 系统中获取视频详情 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/media/videos/{id}`
- **版本**: v1
- **Phase**: 236

## 认证

- **类型**: JWT Bearer Token
- **权限**: 任意登录用户可访问

## 请求参数

### Path 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `id` | number | 是 | 视频 ID |

## 响应格式

### 成功响应 (200 OK)

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
    "created_at": 1711500000,
    "updated_at": 1711500000,
    "thumbnail_path": "/media/thumbnails/movie_001.jpg",
    "folder": "/media/videos"
  }
}
```

### 错误响应

#### 404 Not Found - 视频不存在

```json
{
  "success": false,
  "error": "Video 999 not found",
  "code": "NOT_FOUND"
}
```

#### 401 Unauthorized - 认证失败

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 500 Internal Server Error - 系统错误

```json
{
  "success": false,
  "error": "Failed to get current time",
  "code": "INTERNAL_ERROR"
}
```

## 数据模型

### VideoDetail

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 视频 ID |
| `name` | string | 视频文件名 |
| `path` | string | 视频文件路径 |
| `size_bytes` | number | 文件大小（字节） |
| `duration_seconds` | number | 视频时长（秒） |
| `resolution` | string | 分辨率（如 "1920x1080"） |
| `codec` | string | 视频编码（如 "H.264", "H.265"） |
| `bitrate` | number | 比特率（bps） |
| `framerate` | number | 帧率（fps） |
| `created_at` | number | 创建时间戳（Unix 时间戳） |
| `updated_at` | number | 更新时间戳（Unix 时间戳） |
| `thumbnail_path` | string | 缩略图路径 |
| `folder` | string | 所属文件夹路径 |

### VideoDetailResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | VideoDetail | 视频详情信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `NOT_FOUND` | 404 | 视频不存在 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求

```bash
curl -X GET "http://localhost:8080/api/v1/media/videos/1" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 响应

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
    "created_at": 1711500000,
    "updated_at": 1711500000,
    "thumbnail_path": "/media/thumbnails/movie_001.jpg",
    "folder": "/media/videos"
  }
}
```

### 请求不存在的视频

```bash
curl -X GET "http://localhost:8080/api/v1/media/videos/999" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 响应

```json
{
  "success": false,
  "error": "Video 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限说明

- **任意登录用户**: 可访问视频详情
- **未认证用户**: 无权访问（返回 401 Unauthorized）

## 实现细节

### 视频字段说明
- **size_bytes**: 视频文件大小（字节）
- **duration_seconds**: 视频时长（秒）
- **resolution**: 视频分辨率（宽 x 高）
- **codec**: 视频编码格式（H.264/H.265/VP9 等）
- **bitrate**: 视频比特率（bps）
- **framerate**: 视频帧率（fps）
- **thumbnail_path**: 缩略图文件路径
- **folder**: 视频所属文件夹路径

### 数据来源
- 当前为模拟实现，返回固定视频详情
- 实际实现可：
  - 查询媒体数据库获取视频信息
  - 使用 ffprobe 等工具提取视频元数据
  - 从文件系统读取视频文件属性

## 相关接口

- `GET /api/v1/media/videos` - 获取视频列表
- `GET /api/v1/media/info` - 获取媒体库统计信息
- `GET /api/v1/media/audios/{id}` - 获取音频详情
- `GET /api/v1/media/photos/{id}` - 获取照片详情

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取视频详情
curl -X GET "http://localhost:8080/api/v1/media/videos/1" \
  -H "Authorization: Bearer USER_JWT_TOKEN"

# 预期：200 OK + 视频详情

# 测试不存在的视频
curl -X GET "http://localhost:8080/api/v1/media/videos/999" \
  -H "Authorization: Bearer USER_JWT_TOKEN"

# 预期：404 Not Found

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/media/videos/1"

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 236** (2026-03-28): 初始实现，模拟视频详情
