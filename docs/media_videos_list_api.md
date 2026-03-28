# 媒体视频列表 API 文档

## 概述

本文档描述 Axis NAS 系统中获取视频列表 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/media/videos`
- **版本**: v1
- **Phase**: 235

## 认证

- **类型**: JWT Bearer Token
- **权限**: 任意登录用户可访问

## 请求参数

### Query 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `page` | number | 否 | 1 | 页码（从 1 开始） |
| `per_page` | number | 否 | 20 | 每页数量（最大 100） |
| `folder` | string | 否 | - | 目录筛选（可选，按路径前缀过滤） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "movie_001.mp4",
      "path": "/media/videos/movie_001.mp4",
      "size_bytes": 2147483648,
      "duration_seconds": 7200,
      "resolution": "1920x1080",
      "created_at": 1711500000,
      "thumbnail_path": "/media/thumbnails/movie_001.jpg"
    }
  ],
  "total_count": 125,
  "page": 1,
  "per_page": 20
}
```

### 错误响应

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

### VideoInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 视频 ID |
| `name` | string | 视频文件名 |
| `path` | string | 视频文件路径 |
| `size_bytes` | number | 文件大小（字节） |
| `duration_seconds` | number | 视频时长（秒） |
| `resolution` | string | 分辨率（如 "1920x1080"） |
| `created_at` | number | 创建时间戳（Unix 时间戳） |
| `thumbnail_path` | string | 缩略图路径 |

### VideoListResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | VideoInfo[] | 视频列表 |
| `total_count` | number | 视频总数 |
| `page` | number | 当前页码 |
| `per_page` | number | 每页数量 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求（第一页，每页 20 个）

```bash
curl -X GET "http://localhost:8080/api/v1/media/videos?page=1&per_page=20" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "movie_001.mp4",
      "path": "/media/videos/movie_001.mp4",
      "size_bytes": 2147483648,
      "duration_seconds": 7200,
      "resolution": "1920x1080",
      "created_at": 1711500000,
      "thumbnail_path": "/media/thumbnails/movie_001.jpg"
    },
    {
      "id": 2,
      "name": "movie_002.mp4",
      "path": "/media/videos/movie_002.mp4",
      "size_bytes": 3221225472,
      "duration_seconds": 9000,
      "resolution": "3840x2160",
      "created_at": 1711400000,
      "thumbnail_path": "/media/thumbnails/movie_002.jpg"
    }
  ],
  "total_count": 125,
  "page": 1,
  "per_page": 20
}
```

### 请求（按目录筛选）

```bash
curl -X GET "http://localhost:8080/api/v1/media/videos?folder=/media/videos/movies&per_page=50" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 请求（第二页，每页 50 个）

```bash
curl -X GET "http://localhost:8080/api/v1/media/videos?page=2&per_page=50" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

## 权限说明

- **任意登录用户**: 可访问视频列表
- **未认证用户**: 无权访问（返回 401 Unauthorized）

## 实现细节

### 分页逻辑
- 默认每页 20 条记录
- 最大每页 100 条记录
- 页码从 1 开始

### 筛选功能
- **folder**: 按路径前缀精确匹配
  - 例如：`folder=/media/videos/movies` 将返回所有路径以 `/media/videos/movies` 开头的视频

### 视频字段说明
- **size_bytes**: 视频文件大小（字节）
- **duration_seconds**: 视频时长（秒）
- **resolution**: 视频分辨率（宽 x 高）
- **thumbnail_path**: 缩略图文件路径

### 数据来源
- 当前为模拟实现，返回固定视频列表
- 实际实现可：
  - 扫描媒体库目录获取视频文件
  - 查询媒体数据库获取视频信息
  - 使用 ffprobe 等工具提取视频元数据

## 相关接口

- `GET /api/v1/media/info` - 获取媒体库统计信息
- `GET /api/v1/media/audios` - 获取音频列表
- `GET /api/v1/media/photos` - 获取照片列表
- `GET /api/v1/media/videos/{id}` - 获取视频详情

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取视频列表
curl -X GET "http://localhost:8080/api/v1/media/videos?page=1&per_page=20" \
  -H "Authorization: Bearer USER_JWT_TOKEN"

# 预期：200 OK + 视频列表

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/media/videos"

# 预期：401 Unauthorized

# 测试目录筛选
curl -X GET "http://localhost:8080/api/v1/media/videos?folder=/media/videos" \
  -H "Authorization: Bearer USER_JWT_TOKEN"

# 预期：200 OK + 筛选后的视频列表
```

## 版本历史

- **Phase 235** (2026-03-28): 增强版实现，添加 folder 筛选功能
