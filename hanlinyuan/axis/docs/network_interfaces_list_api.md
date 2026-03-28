# 网络接口列表 API

## Phase 182

## 接口说明

获取系统所有网络接口的信息列表。

## 请求

`GET /api/v1/network/interfaces`

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
      "name": "Primary Network",
      "interface": "eth0",
      "ip_address": "192.168.1.100",
      "netmask": "255.255.255.0",
      "gateway": "192.168.1.1",
      "mac_address": "00:1A:2B:3C:4D:5E",
      "status": "up",
      "interface_type": "ethernet",
      "speed_mbps": 1000,
      "mtu": 1500,
      "created_at": "2026-03-27T06:00:00Z",
      "updated_at": "2026-03-27T06:00:00Z"
    },
    {
      "id": 2,
      "name": "Secondary Network",
      "interface": "eth1",
      "ip_address": "192.168.2.100",
      "netmask": "255.255.255.0",
      "gateway": "192.168.2.1",
      "mac_address": "00:1A:2B:3C:4D:5F",
      "status": "up",
      "interface_type": "ethernet",
      "speed_mbps": 1000,
      "mtu": 1500,
      "created_at": "2026-03-27T06:00:00Z",
      "updated_at": "2026-03-27T06:00:00Z"
    },
    {
      "id": 3,
      "name": "Wireless Network",
      "interface": "wlan0",
      "ip_address": "192.168.1.150",
      "netmask": "255.255.255.0",
      "gateway": "192.168.1.1",
      "mac_address": "00:1A:2B:3C:4D:60",
      "status": "down",
      "interface_type": "wireless",
      "speed_mbps": 300,
      "mtu": 1500,
      "created_at": "2026-03-27T06:00:00Z",
      "updated_at": "2026-03-27T06:00:00Z"
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

### 获取网络接口列表

```bash
curl "http://localhost:8080/api/v1/network/interfaces" \
  -H "Authorization: Bearer <jwt_token>"
```

### 无权限访问

```bash
curl "http://localhost:8080/api/v1/network/interfaces" \
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
curl "http://localhost:8080/api/v1/network/interfaces"
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
- 任意登录用户可访问

## 响应字段说明

### 网络接口字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 接口 ID |
| name | string | 接口名称 |
| interface | string | 接口设备名（eth0/eth1/wlan0 等） |
| ip_address | string | IP 地址 |
| netmask | string | 子网掩码 |
| gateway | string | 默认网关 |
| mac_address | string | MAC 地址 |
| status | string | 状态（up/down） |
| interface_type | string | 接口类型（ethernet/wireless） |
| speed_mbps | number\|null | 速度（Mbps） |
| mtu | number | MTU 值 |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

## 接口类型说明

| 类型 | 说明 |
| ---- | ---- |
| ethernet | 有线以太网接口 |
| wireless | 无线 WiFi 接口 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 获取所有网络接口信息
3. 返回 200 OK + 接口列表

## 版本历史

- **Phase 182** (2026-03-27): 网络管理模块 - 网络接口列表 API
