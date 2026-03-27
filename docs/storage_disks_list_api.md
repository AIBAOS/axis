# 存储磁盘列表 API

## Phase 180

## 接口说明

获取系统所有存储磁盘的信息列表。

## 请求

`GET /api/v1/storage/disks`

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
      "device": "/dev/sda",
      "model": "Samsung SSD 860 EVO 500GB",
      "serial": "S3Z1NB0K123456",
      "size": 500107862016,
      "used": 250053931008,
      "available": 250053931008,
      "usage_percent": 50.0,
      "status": "online",
      "health": "good",
      "temperature": 35,
      "created_at": "2026-03-27T06:00:00Z"
    },
    {
      "id": 2,
      "device": "/dev/sdb",
      "model": "Western Digital WD Blue 1TB",
      "serial": "WD-WCC4E1234567",
      "size": 1000204886016,
      "used": 600122931610,
      "available": 400081954406,
      "usage_percent": 60.0,
      "status": "online",
      "health": "good",
      "temperature": 38,
      "created_at": "2026-03-27T06:00:00Z"
    },
    {
      "id": 3,
      "device": "/dev/sdc",
      "model": "Seagate Barracuda 2TB",
      "serial": "ZDH1234567",
      "size": 2000398934016,
      "used": 400079786803,
      "available": 1600319147213,
      "usage_percent": 20.0,
      "status": "online",
      "health": "good",
      "temperature": 32,
      "created_at": "2026-03-27T06:00:00Z"
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

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can view storage disks",
  "code": "FORBIDDEN"
}
```

## 示例

### 获取存储磁盘列表

```bash
curl "http://localhost:8080/api/v1/storage/disks" \
  -H "Authorization: Bearer <jwt_token>"
```

### 无权限访问

```bash
curl "http://localhost:8080/api/v1/storage/disks" \
  -H "Authorization: Bearer <user_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view storage disks",
  "code": "FORBIDDEN"
}
```

### 未认证访问

```bash
curl "http://localhost:8080/api/v1/storage/disks"
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
- 仅限 admin 角色访问

## 响应字段说明

### 磁盘信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 磁盘 ID |
| device | string | 设备路径（/dev/sda 等） |
| model | string | 磁盘型号 |
| serial | string | 序列号 |
| size | u64 | 总容量（字节） |
| used | u64 | 已用容量（字节） |
| available | u64 | 可用容量（字节） |
| usage_percent | number | 使用率百分比 |
| status | string | 状态（online/offline） |
| health | string | 健康状态（good/warning/critical） |
| temperature | number | 温度（摄氏度） |
| created_at | string | 创建时间（ISO 8601 格式） |

## 磁盘状态说明

| 状态 | 说明 |
| ---- | ---- |
| online | 磁盘在线，可正常访问 |
| offline | 磁盘离线，无法访问 |

## 健康状态说明

| 状态 | 说明 |
| ---- | ---- |
| good | 磁盘健康，无问题 |
| warning | 磁盘有警告，需关注 |
| critical | 磁盘严重问题，需立即处理 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 获取所有存储磁盘信息
4. 返回 200 OK + 磁盘列表

## 版本历史

- **Phase 180** (2026-03-27): 存储管理模块 - 存储磁盘列表 API
