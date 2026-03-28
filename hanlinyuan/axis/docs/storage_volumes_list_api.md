# 存储卷列表 API

## Phase 187

## 接口说明

获取所有存储卷的基本信息列表。

## 请求

`GET /api/v1/storage/volumes`

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
  "data": [
    {
      "id": 1,
      "name": "System Volume",
      "total_bytes": 500107862016,
      "used_bytes": 250053931008,
      "available_bytes": 250053931008,
      "usage_percent": 50.0,
      "status": "active",
      "filesystem_type": "ext4",
      "mount_point": "/"
    },
    {
      "id": 2,
      "name": "Data Volume",
      "total_bytes": 1000204886016,
      "used_bytes": 600122931610,
      "available_bytes": 400081954406,
      "usage_percent": 60.0,
      "status": "active",
      "filesystem_type": "ext4",
      "mount_point": "/data"
    },
    {
      "id": 3,
      "name": "Backup Volume",
      "total_bytes": 2000398934016,
      "used_bytes": 400079786803,
      "available_bytes": 1600319147213,
      "usage_percent": 20.0,
      "status": "active",
      "filesystem_type": "ext4",
      "mount_point": "/backup"
    }
  ]
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

## 示例

### 获取存储卷列表

```bash
curl "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <jwt_token>"
```

### 无权限访问

```bash
curl "http://localhost:8080/api/v1/storage/volumes" \
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

### 未认证访问

```bash
curl "http://localhost:8080/api/v1/storage/volumes"
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

## 权限要求

- 需要 JWT 认证
- 登录用户可访问

## 响应字段说明

### 存储卷字段

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

## 业务逻辑

1. 验证 JWT Token 有效性
2. 获取所有存储卷信息
3. 返回 200 OK + 卷列表

## 版本历史

- **Phase 187** (2026-03-27): 存储管理模块 - 存储卷列表 API
