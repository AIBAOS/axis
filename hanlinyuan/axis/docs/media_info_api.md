# 媒体信息 API 文档

## 概述

本文档描述 Axis NAS 系统中获取媒体库统计信息 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/media/info`
- **版本**: v1
- **Phase**: 231

## 认证

- **类型**: JWT Bearer Token
- **权限**: 任意登录用户可访问

## 请求参数

无

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "video_count": 125,
    "audio_count": 340,
    "photo_count": 1520,
    "total_size_bytes": 107374182400,
    "last_updated": 1711600000
  }
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

### MediaInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `video_count` | number | 视频文件数量 |
| `audio_count` | number | 音频文件数量 |
| `photo_count` | number | 照片文件数量 |
| `total_size_bytes` | number | 媒体库总大小（字节） |
| `last_updated` | number | 最后更新时间戳（Unix 时间戳） |

### MediaInfoResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | MediaInfo | 媒体库统计信息 |

## 错误代码

| 代码 | HTTP 状态码 | 描述 |
|------|-----------|------|
| `UNAUTHORIZED` | 401 | 未提供或无效的认证令牌 |
| `INTERNAL_ERROR` | 500 | 系统错误 |

## 示例

### 请求

```bash
curl -X GET "http://localhost:8080/api/v1/media/info" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": {
    "video_count": 125,
    "audio_count": 340,
    "photo_count": 1520,
    "total_size_bytes": 107374182400,
    "last_updated": 1711600000
  }
}
```

## 权限说明

- **任意登录用户**: 可访问媒体库统计信息
- **未认证用户**: 无权访问（返回 401 Unauthorized）

## 实现细节

### 统计信息说明
- **video_count**: 视频文件数量（.mp4, .mkv, .avi 等）
- **audio_count**: 音频文件数量（.mp3, .flac, .aac 等）
- **photo_count**: 照片文件数量（.jpg, .png, .raw 等）
- **total_size_bytes**: 所有媒体文件占用的总存储空间
- **last_updated**: 媒体库最后扫描/更新的时间戳

### 数据来源
- 当前为模拟实现，返回固定统计值
- 实际实现可：
  - 扫描媒体库目录统计文件
  - 查询媒体数据库获取统计信息
  - 使用文件系统 API 计算总大小

## 相关接口

- `GET /api/v1/media/videos` - 获取视频列表
- `GET /api/v1/media/audios` - 获取音频列表
- `GET /api/v1/media/photos` - 获取照片列表
- `GET /api/v1/media/scan` - 触发媒体库扫描

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取媒体信息
curl -X GET "http://localhost:8080/api/v1/media/info" \
  -H "Authorization: Bearer USER_JWT_TOKEN"

# 预期：200 OK + 媒体库统计信息

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/media/info"

# 预期：401 Unauthorized
```

## 版本历史

- **Phase 231** (2026-03-28): 初始实现，模拟媒体库统计
