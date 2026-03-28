# 媒体音频列表 API 文档

## 概述

本文档描述 Axis NAS 系统中获取音频列表 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/media/audios`
- **版本**: v1
- **Phase**: 233

## 认证

- **类型**: JWT Bearer Token
- **权限**: 任意登录用户可访问

## 请求参数

### Query 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `page` | number | 否 | 1 | 页码（从 1 开始） |
| `per_page` | number | 否 | 20 | 每页数量（最大 100） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "song_001.mp3",
      "path": "/media/audios/song_001.mp3",
      "size_bytes": 5242880,
      "duration_seconds": 240,
      "bitrate": 320,
      "format": "MP3",
      "created_at": 1711500000
    }
  ],
  "total_count": 340,
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

### AudioInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 音频 ID |
| `name` | string | 音频文件名 |
| `path` | string | 音频文件路径 |
| `size_bytes` | number | 文件大小（字节） |
| `duration_seconds` | number | 音频时长（秒） |
| `bitrate` | number | 比特率（kbps） |
| `format` | string | 音频格式（MP3/FLAC/AAC 等） |
| `created_at` | number | 创建时间戳（Unix 时间戳） |

### AudioListResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | AudioInfo[] | 音频列表 |
| `total_count` | number | 音频总数 |
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
curl -X GET "http://localhost:8080/api/v1/media/audios?page=1&per_page=20" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "song_001.mp3",
      "path": "/media/audios/song_001.mp3",
      "size_bytes": 5242880,
      "duration_seconds": 240,
      "bitrate": 320,
      "format": "MP3",
      "created_at": 1711500000
    },
    {
      "id": 2,
      "name": "song_002.flac",
      "path": "/media/audios/song_002.flac",
      "size_bytes": 31457280,
      "duration_seconds": 300,
      "bitrate": 1411,
      "format": "FLAC",
      "created_at": 1711400000
    }
  ],
  "total_count": 340,
  "page": 1,
  "per_page": 20
}
```

### 请求（第二页，每页 50 个）

```bash
curl -X GET "http://localhost:8080/api/v1/media/audios?page=2&per_page=50" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

## 权限说明

- **任意登录用户**: 可访问音频列表
- **未认证用户**: 无权访问（返回 401 Unauthorized）

## 实现细节

### 分页逻辑
- 默认每页 20 条记录
- 最大每页 100 条记录
- 页码从 1 开始

### 音频字段说明
- **size_bytes**: 音频文件大小（字节）
- **duration_seconds**: 音频时长（秒）
- **bitrate**: 音频比特率（kbps）
  - MP3: 通常 128-320 kbps
  - FLAC: 通常 700-1411 kbps (lossless)
- **format**: 音频格式（MP3/FLAC/AAC/WAV 等）

### 数据来源
- 当前为模拟实现，返回固定音频列表
- 实际实现可：
  - 扫描媒体库目录获取音频文件
  - 查询媒体数据库获取音频信息
  - 使用 ffprobe 等工具提取音频元数据

## 相关接口

- `GET /api/v1/media/info` - 获取媒体库统计信息
- `GET /api/v1/media/videos` - 获取视频列表
- `GET /api/v1/media/photos` - 获取照片列表
- `GET /api/v1/media/audios/{id}` - 获取音频详情

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取音频列表
curl -X GET "http://localhost:8080/api/v1/media/audios?page=1&per_page=20" \
  -H "Authorization: Bearer USER_JWT_TOKEN"

# 预期：200 OK + 音频列表

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/media/audios"

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 233** (2026-03-28): 初始实现，模拟音频列表
