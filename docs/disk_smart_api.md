# 磁盘 S.M.A.R.T. 信息 API

## Phase 188

## 接口说明

获取指定磁盘的 S.M.A.R.T. 健康信息。

## 请求

`GET /api/v1/storage/disks/{id}/smart`

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
    "disk_id": 1,
    "model": "Samsung SSD 860 EVO 500GB",
    "serial_number": "S3Z1NB0K123456",
    "firmware_version": "RVT04B6Q",
    "temperature": 35,
    "power_on_hours": 8760,
    "spin_up_time": 15,
    "reallocated_sectors": 0,
    "pending_sectors": 0,
    "uncorrectable_sectors": 0,
    "wear_leveling": 95,
    "health_status": "good",
    "last_check": "2026-03-27T16:00:00Z"
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

### 获取磁盘 S.M.A.R.T. 信息

```bash
curl "http://localhost:8080/api/v1/storage/disks/1/smart" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的磁盘

```bash
curl "http://localhost:8080/api/v1/storage/disks/999/smart" \
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
- 登录用户可访问

## 响应字段说明

### S.M.A.R.T. 信息字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| disk_id | u64 | 磁盘 ID |
| model | string | 磁盘型号 |
| serial_number | string | 序列号 |
| firmware_version | string | 固件版本 |
| temperature | number | 温度（摄氏度） |
| power_on_hours | number | 通电时间（小时） |
| spin_up_time | number | 起转时间（秒） |
| reallocated_sectors | number | 重映射扇区数 |
| pending_sectors | number | 待映射扇区数 |
| uncorrectable_sectors | number | 不可纠正扇区数 |
| wear_leveling | number\|null | 磨损均衡（仅 SSD） |
| health_status | string | 健康状态（good/warning/critical） |
| last_check | string | 最后检查时间（ISO 8601 格式） |

## 健康状态说明

| 状态 | 说明 |
| ---- | ---- |
| good | 磁盘健康，无问题 |
| warning | 磁盘有警告，需关注 |
| critical | 磁盘严重问题，需立即更换 |

## S.M.A.R.T. 指标说明

| 指标 | 说明 |
| ---- | ---- |
| temperature | 磁盘工作温度，正常范围 30-50°C |
| power_on_hours | 磁盘累计通电时间，反映使用寿命 |
| spin_up_time | HDD 起转时间，SSD 无此指标 |
| reallocated_sectors | 重映射扇区数，>0 表示有坏道 |
| pending_sectors | 待映射扇区数，>0 表示可能有坏道 |
| uncorrectable_sectors | 不可纠正扇区数，>0 表示严重问题 |
| wear_leveling | SSD 磨损均衡百分比，越低表示磨损越严重 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 根据磁盘 ID 查找磁盘
3. 磁盘不存在返回 404 Not Found
4. 读取磁盘 S.M.A.R.T. 信息
5. 返回 200 OK + S.M.A.R.T. 信息

## 版本历史

- **Phase 188** (2026-03-27): 存储管理模块 - 磁盘 S.M.A.R.T. 信息 API
