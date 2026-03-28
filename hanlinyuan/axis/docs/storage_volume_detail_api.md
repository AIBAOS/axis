# 存储卷详情 API

## Phase 188

## 接口说明

获取指定存储卷的详细信息。

## 请求

`GET /api/v1/storage/volumes/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 存储卷 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "System Volume",
    "total_bytes": 500107862016,
    "used_bytes": 250053931008,
    "available_bytes": 250053931008,
    "usage_percent": 50.0,
    "status": "active",
    "filesystem_type": "ext4",
    "mount_point": "/",
    "created_at": "2026-03-27T06:00:00Z",
    "updated_at": "2026-03-27T06:00:00Z"
  }
}
```

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 404 Not Found - 卷不存在

```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 获取存储卷详情

```bash
curl "http://localhost:8080/api/v1/storage/volumes/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的卷

```bash
curl "http://localhost:8080/api/v1/storage/volumes/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Storage volume 999 not found",
  "code": "NOT_FOUND"
}
```

### 无权限访问

```bash
curl "http://localhost:8080/api/v1/storage/volumes/1" \
  -H "Authorization: Bearer <user_token>"
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

## 权限要求

- 需要 JWT 认证
- 登录用户可访问

## 响应字段说明

### 存储卷详情字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 卷 ID |
| name | string | 卷名称 |
| total_bytes | u64 | 总容量（字节） |
| used_bytes | u64 | 已用容量（字节） |
| available_bytes | u64 | 可用容量（字节） |
| usage_percent | number | 使用率百分比 |
| status | string | 状态（active/inactive） |
| filesystem_type | string | 文件系统类型（ext4/xfs/btrfs 等） |
| mount_point | string | 挂载点 |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 根据卷 ID 查找卷
3. 卷不存在返回 404 Not Found
4. 返回 200 OK + 卷详情

## 版本历史

- **Phase 188** (2026-03-27): 存储管理模块 - 存储卷详情 API
