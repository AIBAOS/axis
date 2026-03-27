# 网络接口详情 API

## Phase 183

## 接口说明

获取指定网络接口的详细信息。

## 请求

`GET /api/v1/network/interfaces/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 网络接口 ID |

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
    "rx_bytes": 1073741824,
    "tx_bytes": 536870912,
    "rx_packets": 1000000,
    "tx_packets": 500000,
    "rx_errors": 0,
    "tx_errors": 0,
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

#### 404 Not Found - 接口不存在

```json
{
  "success": false,
  "error": "Network interface 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 获取网络接口详情

```bash
curl "http://localhost:8080/api/v1/network/interfaces/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取不存在的接口

```bash
curl "http://localhost:8080/api/v1/network/interfaces/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Network interface 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 响应字段说明

### 网络接口详情字段

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
| rx_bytes | u64 | 接收字节数 |
| tx_bytes | u64 | 发送字节数 |
| rx_packets | u64 | 接收数据包数 |
| tx_packets | u64 | 发送数据包数 |
| rx_errors | u64 | 接收错误数 |
| tx_errors | u64 | 发送错误数 |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

## 接口类型说明

| 类型 | 说明 |
| ---- | ---- |
| ethernet | 有线以太网接口 |
| wireless | 无线 WiFi 接口 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 根据接口 ID 查找接口
3. 接口不存在返回 404 Not Found
4. 返回 200 OK + 接口详情（包含流量统计）

## 版本历史

- **Phase 183** (2026-03-27): 网络管理模块 - 网络接口详情 API
