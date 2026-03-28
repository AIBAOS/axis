# 媒体照片列表 API 文档

## 概述

本文档描述 Axis NAS 系统中获取照片列表 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/media/photos`
- **版本**: v1
- **Phase**: 234

## 认证

- **类型**: JWT Bearer Token
- **权限**: 任意登录用户可访问

## 请求参数

### Query 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `page` | number | 否 | 1 | 页码（从 1 开始） |
| `per_page` | number | 否 | 20 | 每页数量（最大 100） |
| `album` | string | 否 | - | 相册名称筛选 |
| `date_range` | string | 否 | - | 日期范围筛选（YYYY-MM-DD 或 YYYY-MM-DD:YYYY-MM-DD） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "photo_001.jpg",
      "path": "/media/photos/photo_001.jpg",
      "size_bytes": 3145728,
      "width": 4032,
      "height": 3024,
      "taken_at": 1711500000,
      "created_at": 1711500000,
      "thumbnail_path": "/media/thumbnails/photo_001.jpg",
      "album": "Vacation 2026"
    }
  ],
  "total_count": 1520,
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

### PhotoInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 照片 ID |
| `name` | string | 照片文件名 |
| `path` | string | 照片文件路径 |
| `size_bytes` | number | 文件大小（字节） |
| `width` | number | 照片宽度（像素） |
| `height` | number | 照片高度（像素） |
| `taken_at` | number | 拍摄时间戳（Unix 时间戳） |
| `created_at` | number | 创建时间戳（Unix 时间戳） |
| `thumbnail_path` | string | 缩略图路径 |
| `album` | string | 所属相册名称 |

### PhotoListResponse

| 字段 | 类型 | 描述 |
|------|------|------|
| `success` | boolean | 操作是否成功 |
| `data` | PhotoInfo[] | 照片列表 |
| `total_count` | number | 照片总数 |
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
curl -X GET "http://localhost:8080/api/v1/media/photos?page=1&per_page=20" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "photo_001.jpg",
      "path": "/media/photos/photo_001.jpg",
      "size_bytes": 3145728,
      "width": 4032,
      "height": 3024,
      "taken_at": 1711500000,
      "created_at": 1711500000,
      "thumbnail_path": "/media/thumbnails/photo_001.jpg",
      "album": "Vacation 2026"
    },
    {
      "id": 2,
      "name": "photo_002.jpg",
      "path": "/media/photos/photo_002.jpg",
      "size_bytes": 2621440,
      "width": 3840,
      "height": 2160,
      "taken_at": 1711400000,
      "created_at": 1711400000,
      "thumbnail_path": "/media/thumbnails/photo_002.jpg",
      "album": "Vacation 2026"
    }
  ],
  "total_count": 1520,
  "page": 1,
  "per_page": 20
}
```

### 请求（按相册筛选）

```bash
curl -X GET "http://localhost:8080/api/v1/media/photos?album=Vacation%202026&per_page=50" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

### 请求（按日期范围筛选）

```bash
curl -X GET "http://localhost:8080/api/v1/media/photos?date_range=2026-03-01:2026-03-31" \
  -H "Authorization: Bearer USER_JWT_TOKEN"
```

## 权限说明

- **任意登录用户**: 可访问照片列表
- **未认证用户**: 无权访问（返回 401 Unauthorized）

## 实现细节

### 分页逻辑
- 默认每页 20 条记录
- 最大每页 100 条记录
- 页码从 1 开始

### 筛选功能
- **album**: 按相册名称精确匹配
- **date_range**: 
  - 单日期：`YYYY-MM-DD`
  - 日期范围：`YYYY-MM-DD:YYYY-MM-DD`

### 照片字段说明
- **size_bytes**: 照片文件大小（字节）
- **width/height**: 照片分辨率（像素）
- **taken_at**: 照片拍摄时间（EXIF 信息）
- **thumbnail_path**: 缩略图文件路径
- **album**: 照片所属相册名称

### 数据来源
- 当前为模拟实现，返回固定照片列表
- 实际实现可：
  - 扫描媒体库目录获取照片文件
  - 查询媒体数据库获取照片信息
  - 使用 exiftool 等工具提取照片元数据

## 相关接口

- `GET /api/v1/media/info` - 获取媒体库统计信息
- `GET /api/v1/media/videos` - 获取视频列表
- `GET /api/v1/media/audios` - 获取音频列表
- `GET /api/v1/media/photos/{id}` - 获取照片详情

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test

# 测试获取照片列表
curl -X GET "http://localhost:8080/api/v1/media/photos?page=1&per_page=20" \
  -H "Authorization: Bearer USER_JWT_TOKEN"

# 预期：200 OK + 照片列表

# 测试未认证访问
curl -X GET "http://localhost:8080/api/v1/media/photos"

# 预期：401 Unauthorized

# 测试相册筛选
curl -X GET "http://localhost:8080/api/v1/media/photos?album=Vacation%202026" \
  -H "Authorization: Bearer USER_JWT_TOKEN"

# 预期：200 OK + 筛选后的照片列表
```

## 版本历史

- **Phase 234** (2026-03-28): 初始实现，模拟照片列表
