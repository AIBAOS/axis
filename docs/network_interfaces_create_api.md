# 创建网络接口 API

## Phase 184

## 接口说明

创建新的网络接口。

## 请求

`POST /api/v1/network/interfaces`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

```json
{
  "name": "New Network Interface",
  "interface": "eth2",
  "ip_address": "192.168.3.100",
  "netmask": "255.255.255.0",
  "gateway": "192.168.3.1",
  "interface_type": "ethernet",
  "speed_mbps": 1000,
  "mtu": 1500
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 是 | 接口名称 |
| interface | string | 是 | 接口设备名（eth0/eth1/wlan0 等） |
| ip_address | string | 是 | IP 地址 |
| netmask | string | 是 | 子网掩码 |
| gateway | string | 是 | 默认网关 |
| interface_type | string | 是 | 接口类型（ethernet/wireless） |
| speed_mbps | number | 否 | 速度（Mbps） |
| mtu | number | 否 | MTU 值（默认 1500） |

## 响应

### 成功响应（201 Created）

```json
{
  "success": true,
  "message": "Network interface created successfully",
  "data": {
    "id": 4,
    "name": "New Network Interface",
    "interface": "eth2",
    "ip_address": "192.168.3.100",
    "netmask": "255.255.255.0",
    "gateway": "192.168.3.1",
    "mac_address": "00:1A:2B:3C:4D:5E",
    "status": "up",
    "interface_type": "ethernet",
    "speed_mbps": 1000,
    "mtu": 1500,
    "created_at": "2026-03-27T15:00:00Z",
    "updated_at": "2026-03-27T15:00:00Z"
  }
}
```

### 错误响应

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid IP address format",
  "code": "INVALID_IP_ADDRESS"
}
```

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
  "error": "Only admin users can create network interfaces",
  "code": "FORBIDDEN"
}
```

#### 409 Conflict - 接口已存在

```json
{
  "success": false,
  "error": "Network interface 'eth0' already exists",
  "code": "INTERFACE_EXISTS"
}
```

## 示例

### 创建网络接口

```bash
curl -X POST "http://localhost:8080/api/v1/network/interfaces" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Network Interface",
    "interface": "eth2",
    "ip_address": "192.168.3.100",
    "netmask": "255.255.255.0",
    "gateway": "192.168.3.1",
    "interface_type": "ethernet",
    "speed_mbps": 1000,
    "mtu": 1500
  }'
```

### 创建已存在的接口

```bash
curl -X POST "http://localhost:8080/api/v1/network/interfaces" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Duplicate Interface",
    "interface": "eth0",
    "ip_address": "192.168.1.200",
    "netmask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "interface_type": "ethernet"
  }'
```

响应（409 Conflict）：
```json
{
  "success": false,
  "error": "Network interface 'eth0' already exists",
  "code": "INTERFACE_EXISTS"
}
```

### 无效的 IP 地址

```bash
curl -X POST "http://localhost:8080/api/v1/network/interfaces" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Invalid Interface",
    "interface": "eth3",
    "ip_address": "192.168.1",
    "netmask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "interface_type": "ethernet"
  }'
```

响应（400 Bad Request）：
```json
{
  "success": false,
  "error": "Invalid IP address format",
  "code": "INVALID_IP_ADDRESS"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证 IP 地址、子网掩码、网关格式
4. 验证接口名称唯一性
5. 生成 MAC 地址
6. 创建新接口
7. 返回 201 Created + 接口详情

## 版本历史

- **Phase 184** (2026-03-27): 网络管理模块 - 创建网络接口 API
