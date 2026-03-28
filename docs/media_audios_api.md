# 媒体音频列表 API

## Phase 233

## 接口说明

获取媒体库中的音频列表，支持分页和筛选。

## 请求

`GET /api/v1/media/audios`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| page | integer | 否 | 1 | 页码（从 1 开始） |
| per_page | integer | 否 | 20 | 每页数量（最大 100） |
| artist | string | 否 | - | 按艺术家筛选 |
| album | string | 否 | - | 按专辑筛选 |

### 请求示例

**获取音频列表（第一页）：**
```bash
curl -X GET "http://localhost:8080/api/v1/media/audios?page=1&per_page=20" \
  -H "Authorization: Bearer <jwt_token>"
```

**按艺术家筛选：**
```bash
curl -X GET "http://localhost:8080/api/v1/media/audios?artist=Artist%20A" \
  -H "Authorization: Bearer <jwt_token>"
```

**按专辑筛选：**
```bash
curl -X GET "http://localhost:8080/api/v1/media/audios?album=Album%20X" \
  -H "Authorization: Bearer <jwt_token>"
```

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "song_001.mp3",
      "path": "/media/audios/song_001.mp3",
      "size_bytes": 10485760,
      "duration_seconds": 240,
      "artist": "Artist A",
      "album": "Album X",
      "track_number": 1,
      "created_at": 1711497600,
      "thumbnail_path": "/media/thumbnails/song_001.jpg"
    }
  ],
  "total_count": 340,
  "page": 1,
  "per_page": 20
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| success | boolean | 请求是否成功 |
| data | array | 音频列表 |
| data[].id | integer | 音频 ID |
| data[].name | string | 音频文件名 |
| data[].path | string | 音频文件路径 |
| data[].size_bytes | integer | 文件大小（字节） |
| data[].duration_seconds | integer | 时长（秒） |
| data[].artist | string | 艺术家 |
| data[].album | string | 专辑 |
| data[].track_number | integer | 音轨编号 |
| data[].created_at | integer | 创建时间（Unix 时间戳） |
| data[].thumbnail_path | string | 缩略图路径 |
| total_count | integer | 总记录数 |
| page | integer | 当前页码 |
| per_page | integer | 每页数量 |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
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

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 解析分页参数（page/per_page）
3. 解析筛选参数（artist/album）
4. 应用筛选条件
5. 应用分页
6. 返回音频列表和总数

## 版本历史

- **Phase 233** (2026-03-28): 媒体模块 - 音频列表 API
