# 存储磁盘详情 API

## Phase 181

## 接口说明

获取指定存储磁盘的详细信息。

## 请求

`GET /api/v1/storage/disks/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 磁盘 ID |

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
    "name": "System Disk",
    "path": "/dev/sda",
    "model": "Samsung SSD 860 EVO 500GB",
    "serial_number": "S3Z1NB0K123456",
    "disk_type": "ssd",
    "size_bytes": 500107862016,
    "size_human": "500.00 GB",
    "temperature": 35,
    "smart_status": "passed",
    "health_status": "good",
    "speed_rpm": null,
    "power_on_hours": 8760,
    "status": "online",
    "in_use": true,
    "storage_pool_id": 1
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

#### 404 Not Found - 磁盘不存在

```json
{
  "success": false,
  "error": "Disk 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 获取磁盘详情

```bash
curl "http://localhost:8080/api/v1/storage/disks/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的磁盘

```bash
curl "http://localhost:8080/api/v1/storage/disks/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Disk 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 响应字段说明

### 磁盘详情字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 磁盘 ID |
| name | string | 磁盘名称 |
| path | string | 设备路径（/dev/sda 等） |
| model | string | 磁盘型号 |
| serial_number | string | 序列号 |
| disk_type | string | 磁盘类型（hdd/ssd/nvme） |
| size_bytes | u64 | 总容量（字节） |
| size_human | string | 总容量（人类可读格式，如 500.00 GB） |
| temperature | number | 温度（摄氏度） |
| smart_status | string | SMART 状态（passed/failed） |
| health_status | string | 健康状态（good/warning/critical） |
| speed_rpm | number\|null | 转速（RPM，SSD 为 null） |
| power_on_hours | number | 通电时间（小时） |
| status | string | 状态（online/offline） |
| in_use | boolean | 是否使用中 |
| storage_pool_id | number\|null | 所属存储池 ID |

## 磁盘类型说明

| 类型 | 说明 |
| ---- | ---- |
| hdd | 机械硬盘 |
| ssd | 固态硬盘 |
| nvme | NVMe 固态硬盘 |

## 健康状态说明

| 状态 | 说明 |
| ---- | ---- |
| good | 磁盘健康，无问题 |
| warning | 磁盘有警告，需关注 |
| critical | 磁盘严重问题，需立即处理 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 根据磁盘 ID 查找磁盘
3. 磁盘不存在返回 404 Not Found
4. 返回 200 OK + 磁盘详情（容量自动格式化为人类可读格式）

## 版本历史

- **Phase 181** (2026-03-27): 存储管理模块 - 存储磁盘详情 API
