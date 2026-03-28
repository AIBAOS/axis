# 网络配置列表 API

## Phase 179

## 接口说明

获取系统所有网络接口的配置列表。

## 请求

`GET /api/v1/network/config`

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
      "interface": "eth0",
      "ip_address": "192.168.1.100",
      "netmask": "255.255.255.0",
      "gateway": "192.168.1.1",
      "dns_primary": "8.8.8.8",
      "dns_secondary": "8.8.4.4",
      "dhcp_enabled": false,
      "enabled": true,
      "created_at": "2026-03-27T06:00:00Z",
      "updated_at": "2026-03-27T06:00:00Z"
    },
    {
      "id": 2,
      "interface": "eth1",
      "ip_address": "192.168.2.100",
      "netmask": "255.255.255.0",
      "gateway": "192.168.2.1",
      "dns_primary": "1.1.1.1",
      "dns_secondary": "1.0.0.1",
      "dhcp_enabled": true,
      "enabled": true,
      "created_at": "2026-03-27T06:00:00Z",
      "updated_at": "2026-03-27T06:00:00Z"
    },
    {
      "id": 3,
      "interface": "wlan0",
      "ip_address": "192.168.1.150",
      "netmask": "255.255.255.0",
      "gateway": "192.168.1.1",
      "dns_primary": "8.8.8.8",
      "dns_secondary": "8.8.4.4",
      "dhcp_enabled": true,
      "enabled": false,
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

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can view network configuration",
  "code": "FORBIDDEN"
}
```

## 示例

### 获取网络配置列表

```bash
curl "http://localhost:8080/api/v1/network/config" \
  -H "Authorization: Bearer <jwt_token>"
```

### 无权限访问

```bash
curl "http://localhost:8080/api/v1/network/config" \
  -H "Authorization: Bearer <user_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can view network configuration",
  "code": "FORBIDDEN"
}
```

### 未认证访问

```bash
curl "http://localhost:8080/api/v1/network/config"
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

### 网络配置字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 配置 ID |
| interface | string | 网络接口名称（eth0/eth1/wlan0 等） |
| ip_address | string | IP 地址 |
| netmask | string | 子网掩码 |
| gateway | string | 默认网关 |
| dns_primary | string | 主 DNS 服务器 |
| dns_secondary | string | 备用 DNS 服务器 |
| dhcp_enabled | boolean | 是否启用 DHCP |
| enabled | boolean | 是否启用该配置 |
| created_at | string | 创建时间（ISO 8601 格式） |
| updated_at | string | 更新时间（ISO 8601 格式） |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 获取所有网络接口配置
4. 返回 200 OK + 配置列表

## 版本历史

- **Phase 179** (2026-03-27): 网络配置模块 - 网络配置列表 API
