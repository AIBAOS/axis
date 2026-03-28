# 网络配置更新 API (Phase 123)

## 接口说明

实现更新网络配置的接口。仅 admin 角色可访问，支持 DHCP/静态 IP 配置。

## 接口定义

```
PUT /api/v1/network/config
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |
| Content-Type | application/json | 是 | 请求体格式 |

## 请求体

```json
{
  "hostname": "nas-server",
  "dhcp_enabled": true,
  "ip_address": "192.168.1.100",
  "subnet_mask": "255.255.255.0",
  "gateway": "192.168.1.1",
  "dns_primary": "8.8.8.8",
  "dns_secondary": "8.8.4.4"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| hostname | string | 否 | 主机名 |
| dhcp_enabled | boolean | 否 | 是否启用 DHCP |
| ip_address | string | 否 | IP 地址（DHCP 启用时可省略） |
| subnet_mask | string | 否 | 子网掩码（DHCP 启用时可省略） |
| gateway | string | 否 | 网关地址（DHCP 启用时可省略） |
| dns_primary | string | 否 | 主 DNS（DHCP 启用时可省略） |
| dns_secondary | string | 否 | 备用 DNS（DHCP 启用时可省略） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Network configuration updated successfully",
  "data": {
    "hostname": "nas-server",
    "ip_address": "192.168.1.100",
    "subnet_mask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "dns_primary": "8.8.8.8",
    "dns_secondary": "8.8.4.4",
    "dhcp_enabled": true,
    "mac_address": "00:1A:2B:3C:4D:5E",
    "connection_status": "connected"
  }
}
```

### 未授权 (401 Unauthorized)

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 禁止访问 (403 Forbidden)

```json
{
  "success": false,
  "error": "Only admin users can update network configuration",
  "code": "FORBIDDEN"
}
```

### IP 地址格式错误 (400 Bad Request)

```json
{
  "success": false,
  "error": "Invalid IP address format",
  "code": "INVALID_IP"
}
```

## 使用示例

### cURL 示例

```bash
# 启用 DHCP（自动获取 IP）
curl -X PUT "http://localhost:8080/api/v1/network/config" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "dhcp_enabled": true
  }'

# 配置静态 IP
curl -X PUT "http://localhost:8080/api/v1/network/config" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "dhcp_enabled": false,
    "ip_address": "192.168.1.100",
    "subnet_mask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "dns_primary": "8.8.8.8",
    "dns_secondary": "8.8.4.4"
  }'

# 仅修改主机名
curl -X PUT "http://localhost:8080/api/v1/network/config" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "hostname": "my-nas-server"
  }'

# 无效 IP 地址（返回 400）
curl -X PUT "http://localhost:8080/api/v1/network/config" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "ip_address": "999.999.999.999"
  }'
```

### JavaScript 示例

```javascript
// 更新网络配置
async function updateNetworkConfig(config) {
  const response = await fetch(
    'http://localhost:8080/api/v1/network/config',
    {
      method: 'PUT',
      headers: {
        'Authorization': 'Bearer ' + adminToken,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(config)
    }
  );
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error);
  }
  
  const data = await response.json();
  console.log('Network config updated:', data.data);
  return data.data;
}

// 使用示例
try {
  // 启用 DHCP
  await updateNetworkConfig({ dhcp_enabled: true });
  
  // 配置静态 IP
  await updateNetworkConfig({
    dhcp_enabled: false,
    ip_address: '192.168.1.100',
    subnet_mask: '255.255.255.0',
    gateway: '192.168.1.1',
    dns_primary: '8.8.8.8',
    dns_secondary: '8.8.4.4'
  });
} catch (e) {
  console.error('Update failed:', e.message);
}
```

## 响应字段说明

### NetworkConfig

| 字段 | 类型 | 说明 |
|------|------|------|
| hostname | string | 主机名 |
| ip_address | string | IP 地址 |
| subnet_mask | string | 子网掩码 |
| gateway | string | 网关地址 |
| dns_primary | string | 主 DNS 服务器 |
| dns_secondary | string | 备用 DNS 服务器 |
| dhcp_enabled | boolean | 是否启用 DHCP |
| mac_address | string | MAC 地址 |
| connection_status | string | 连接状态（connected/disconnected） |

## DHCP 与静态 IP

### DHCP 模式（dhcp_enabled: true）

- 自动获取 IP 地址、子网掩码、网关、DNS
- 提交的静态 IP 配置将被忽略
- 适合大多数家庭/办公网络环境

### 静态 IP 模式（dhcp_enabled: false）

- 需要手动配置所有网络参数
- 必须提供：ip_address, subnet_mask, gateway
- 建议提供：dns_primary, dns_secondary
- 适合需要固定 IP 的服务器环境

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **IP 格式验证**: 验证所有 IP 地址格式合法性
4. **DHCP 优先**: DHCP 启用时自动忽略静态 IP 配置

## 实现文件

- `src/handlers/network_config_update.rs` - 网络配置更新处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际系统网络配置
2. 仅 admin 角色可访问
3. IP 地址格式必须合法（x.x.x.x，每段 0-255）
4. DHCP 启用时静态 IP 配置将被忽略
5. 网络配置更改可能导致网络连接中断

## 相关接口

- `GET /api/v1/network/config` - 获取网络配置（Phase 122）
- `GET /api/v1/network/interfaces` - 网络接口列表
- `GET /api/v1/network/wifi/status` - WiFi 状态
