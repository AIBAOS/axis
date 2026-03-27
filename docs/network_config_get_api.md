# 网络配置 API 文档 (Phase 122)

## 概述

网络配置 API 提供系统网络配置的详细信息。

## 接口详情

### GET /api/v1/network/config

获取系统网络配置信息。

#### 认证要求

需要有效的 JWT Token，仅 admin 角色可访问。

**请求头：**
```
Authorization: Bearer <admin_jwt_token>
```

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "data": {
    "hostname": "nas-server",
    "ip_address": "192.168.1.100",
    "subnet_mask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "dns_primary": "8.8.8.8",
    "dns_secondary": "8.8.4.4",
    "dhcp_enabled": false,
    "mac_address": "00:1A:2B:3C:4D:5E",
    "connection_status": "connected"
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `data`: 网络配置信息
  - `hostname`: 主机名
  - `ip_address`: IP 地址
  - `subnet_mask`: 子网掩码
  - `gateway`: 网关地址
  - `dns_primary`: 主 DNS 服务器
  - `dns_secondary`: 备用 DNS 服务器
  - `dhcp_enabled`: 是否启用 DHCP
  - `mac_address`: MAC 地址
  - `connection_status`: 连接状态（connected/disconnected）

**错误响应 (403 Forbidden) - 权限不足**

```json
{
  "success": false,
  "error": "Only admin users can access network configuration",
  "code": "FORBIDDEN"
}
```

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：获取网络配置

```bash
curl -X GET "http://localhost:8080/api/v1/network/config" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

**响应：**
```json
{
  "success": true,
  "data": {
    "hostname": "nas-server",
    "ip_address": "192.168.1.100",
    "subnet_mask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "dns_primary": "8.8.8.8",
    "dns_secondary": "8.8.4.4",
    "dhcp_enabled": false,
    "mac_address": "00:1A:2B:3C:4D:5E",
    "connection_status": "connected"
  }
}
```

### 示例 2：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/network/config"
```

**响应：**
```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

### 示例 3：非 admin 用户访问（403）

```bash
curl -X GET "http://localhost:8080/api/v1/network/config" \
  -H "Authorization: Bearer <user_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Only admin users can access network configuration",
  "code": "FORBIDDEN"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 角色授权

- 仅 `admin` 角色可访问网络配置
- 非 admin 角色返回 403 Forbidden

## 实现文件

- `src/handlers/network_config_get.rs` - 网络配置处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：仅 admin 角色可访问
2. **敏感信息**：网络配置属于敏感信息，仅限管理员访问

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `FORBIDDEN` | 403 | 权限不足（非 admin 角色） |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **PUT /api/v1/network/config** - 更新网络配置
- **GET /api/v1/network/interfaces** - 获取网络接口列表
- **GET /api/v1/system/info** - 获取系统信息

## 响应示例（完整）

### 成功响应

```json
{
  "success": true,
  "data": {
    "hostname": "nas-server",
    "ip_address": "192.168.1.100",
    "subnet_mask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "dns_primary": "8.8.8.8",
    "dns_secondary": "8.8.4.4",
    "dhcp_enabled": false,
    "mac_address": "00:1A:2B:3C:4D:5E",
    "connection_status": "connected"
  }
}
```

### 未认证（401）

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

### 权限不足（403）

```json
{
  "success": false,
  "error": "Only admin users can access network configuration",
  "code": "FORBIDDEN"
}
```

## 最佳实践

### 1. 前端集成示例

```javascript
const getNetworkConfig = async () => {
  try {
    const response = await fetch('/api/v1/network/config', {
      headers: {
        'Authorization': `Bearer ${adminToken}`
      }
    });

    const data = await response.json();
    if (response.ok) {
      console.log('Network config:', data.data);
      return data.data;
    } else {
      console.error('Failed to get network config:', data.error);
      throw new Error(data.error);
    }
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
};

// 使用示例
getNetworkConfig();
```

### 2. React 组件示例

```jsx
const NetworkConfig = () => {
  const [config, setConfig] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchConfig = async () => {
      try {
        const response = await fetch('/api/v1/network/config', {
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });
        const data = await response.json();
        if (response.ok) {
          setConfig(data.data);
        }
      } catch (error) {
        console.error('Error:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchConfig();
  }, []);

  if (loading) return <div>Loading...</div>;
  if (!config) return <div>No config available</div>;

  return (
    <div>
      <h3>Network Configuration</h3>
      <p>Hostname: {config.hostname}</p>
      <p>IP Address: {config.ip_address}</p>
      <p>Subnet Mask: {config.subnet_mask}</p>
      <p>Gateway: {config.gateway}</p>
      <p>DNS Primary: {config.dns_primary}</p>
      <p>DNS Secondary: {config.dns_secondary}</p>
      <p>DHCP: {config.dhcp_enabled ? 'Enabled' : 'Disabled'}</p>
      <p>MAC Address: {config.mac_address}</p>
      <p>Status: {config.connection_status}</p>
    </div>
  );
};
```

### 3. 错误处理

```javascript
const handleNetworkConfigError = (error) => {
  switch (error.code) {
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    case 'FORBIDDEN':
      return 'Only admin users can access network configuration.';
    default:
      return 'Failed to get network configuration. Please try again.';
  }
};
```

### 4. 审计日志

所有网络配置查询操作都应该记录到审计日志中，包括：
- 查询时间
- 执行查询的用户 ID
- 查询结果（成功/失败）
