# 网络接口更新 API

## Phase 185

## 接口说明

更新指定网络接口的配置。

## 请求

`PUT /api/v1/network/interfaces/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 网络接口 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |
| Content-Type | string | 是 | `application/json` |

### 请求体

所有字段均为可选，支持部分更新：

```json
{
  "name": "Updated Network Interface",
  "ip_address": "192.168.1.200",
  "netmask": "255.255.255.0",
  "gateway": "192.168.1.1",
  "mac_address": "00:1A:2B:3C:4D:5E",
  "interface_type": "ethernet",
  "speed_mbps": 1000,
  "mtu": 1500,
  "status": "up"
}
```

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| name | string | 否 | 接口名称 |
| ip_address | string | 否 | IP 地址 |
| netmask | string | 否 | 子网掩码 |
| gateway | string | 否 | 默认网关 |
| mac_address | string | 否 | MAC 地址 |
| interface_type | string | 否 | 接口类型（ethernet/wireless） |
| speed_mbps | number | 否 | 速度（Mbps） |
| mtu | number | 否 | MTU 值 |
| status | string | 否 | 状态（up/down） |

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "message": "Network interface updated successfully",
  "data": {
    "id": 1,
    "name": "Updated Network Interface",
    "interface": "eth0",
    "ip_address": "192.168.1.200",
    "netmask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "mac_address": "00:1A:2B:3C:4D:5E",
    "status": "up",
    "interface_type": "ethernet",
    "speed_mbps": 1000,
    "mtu": 1500,
    "created_at": "2026-03-27T06:00:00Z",
    "updated_at": "2026-03-27T15:30:00Z"
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
  "error": "Only admin users can update network interfaces",
  "code": "FORBIDDEN"
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

### 更新网络接口 IP 地址

```bash
curl -X PUT "http://localhost:8080/api/v1/network/interfaces/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "ip_address": "192.168.1.200"
  }'
```

### 更新多个字段

```bash
curl -X PUT "http://localhost:8080/api/v1/network/interfaces/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Network Interface",
    "ip_address": "192.168.1.200",
    "status": "down"
  }'
```

### 更新不存在的接口

```bash
curl -X PUT "http://localhost:8080/api/v1/network/interfaces/999" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "ip_address": "192.168.1.200"
  }'
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Network interface 999 not found",
  "code": "NOT_FOUND"
}
```

### 无效的 IP 地址

```bash
curl -X PUT "http://localhost:8080/api/v1/network/interfaces/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "ip_address": "192.168.1"
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
3. 根据接口 ID 查找接口
4. 接口不存在返回 404 Not Found
5. 验证 IP 地址/子网掩码/网关格式（如果提供）
6. 部分更新接口配置
7. 更新时间戳
8. 返回 200 OK + 更新后的接口详情

## 版本历史

- **Phase 185** (2026-03-27): 网络管理模块 - 网络接口更新 API
