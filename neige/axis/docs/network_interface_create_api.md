# 网络接口创建 API - Phase 132

## 接口规范

### 基本信息

- **端点**: `POST /api/v1/network/interfaces`
- **认证**: JWT Bearer Token（必需）
- **权限**: 仅 `admin` 角色可访问
- **Content-Type**: `application/json`

---

## 请求

### 请求体

```json
{
  "name": "eth0",
  "type": "ethernet",
  "mac_address": "00:1A:2B:3C:4D:5E",
  "ip_address": "192.168.1.100",
  "netmask": "255.255.255.0",
  "gateway": "192.168.1.1",
  "dhcp_enabled": false
}
```

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|-----|------|-----|--------|------|
| name | string | 是 | - | 接口名称（必须唯一） |
| type | string | 是 | - | 接口类型：ethernet/wifi/bridge/vlan |
| mac_address | string | 否 | - | MAC 地址（XX:XX:XX:XX:XX:XX 格式） |
| ip_address | string | 否 | - | IPv4 地址 |
| netmask | string | 否 | - | 子网掩码（IPv4 格式） |
| gateway | string | 否 | - | 网关（IPv4 格式） |
| dhcp_enabled | boolean | 否 | false | 是否启用 DHCP |

---

## 响应

### 201 Created - 创建成功

```json
{
  "success": true,
  "message": "网络接口创建成功",
  "data": {
    "id": "net_123456",
    "name": "eth0",
    "interface_type": "ethernet",
    "mac_address": "00:1A:2B:3C:4D:5E",
    "ip_address": "192.168.1.100",
    "netmask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "dhcp_enabled": false,
    "status": "active",
    "created_at": 1711468800,
    "updated_at": 1711468800
  }
}
```

### 400 Bad Request - 参数格式错误

**无效的接口类型**:
```json
{
  "success": false,
  "message": "无效的接口类型，合法值：ethernet, wifi, bridge, vlan",
  "error_code": "BAD_REQUEST"
}
```

**MAC 地址格式无效**:
```json
{
  "success": false,
  "message": "MAC 地址格式无效，应为 XX:XX:XX:XX:XX:XX 格式",
  "error_code": "BAD_REQUEST"
}
```

**IP 地址格式无效**:
```json
{
  "success": false,
  "message": "IP 地址格式无效，应为 IPv4 格式（如 192.168.1.1）",
  "error_code": "BAD_REQUEST"
}
```

### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "message": "仅 admin 角色可创建网络接口",
  "error_code": "FORBIDDEN"
}
```

### 409 Conflict - 接口名称已存在

```json
{
  "success": false,
  "message": "接口名称已存在：eth0",
  "error_code": "CONFLICT"
}
```

### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "message": "数据库创建失败",
  "error_code": "INTERNAL_ERROR"
}
```

---

## 使用示例

### cURL

```bash
curl -X POST http://localhost:8080/api/v1/network/interfaces \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "eth0",
    "type": "ethernet",
    "mac_address": "00:1A:2B:3C:4D:5E",
    "ip_address": "192.168.1.100",
    "netmask": "255.255.255.0",
    "gateway": "192.168.1.1",
    "dhcp_enabled": false
  }'
```

### JavaScript (fetch)

```javascript
const response = await fetch('http://localhost:8080/api/v1/network/interfaces', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer <admin_jwt_token>',
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    name: 'eth0',
    type: 'ethernet',
    mac_address: '00:1A:2B:3C:4D:5E',
    ip_address: '192.168.1.100',
    netmask: '255.255.255.0',
    gateway: '192.168.1.1',
    dhcp_enabled: false,
  }),
});

const data = await response.json();
console.log(data);
```

### Python (requests)

```python
import requests

url = 'http://localhost:8080/api/v1/network/interfaces'
headers = {
    'Authorization': 'Bearer <admin_jwt_token>',
    'Content-Type': 'application/json',
}
data = {
    'name': 'eth0',
    'type': 'ethernet',
    'mac_address': '00:1A:2B:3C:4D:5E',
    'ip_address': '192.168.1.100',
    'netmask': '255.255.255.0',
    'gateway': '192.168.1.1',
    'dhcp_enabled': False,
}

response = requests.post(url, headers=headers, json=data)
print(response.json())
```

---

## 验证规则

### 接口类型（type）
合法值：
- `ethernet` - 以太网接口
- `wifi` - 无线接口
- `bridge` - 桥接接口
- `vlan` - VLAN 接口

### MAC 地址格式
- 支持 `:` 分隔：`00:1A:2B:3C:4D:5E`
- 支持 `-` 分隔：`00-1A-2B-3C-4D-5E`
- 不区分大小写

### IPv4 地址格式
- 标准点分十进制：`192.168.1.1`
- 每段范围：0-255

---

## 安全说明

1. **权限控制**: 仅 `admin` 角色可调用此接口
2. **名称唯一性**: 接口名称必须全局唯一
3. **格式校验**: 所有网络参数均进行格式验证
4. **审计日志**: 所有创建操作记录日志（操作者、接口名称、类型）

---

## 相关文件

- 实现：`src/handlers/network_interfaces.rs`
- 路由注册：`src/main.rs`
- 数据库：`src/database/network_store.rs`

---

**版本**: v1.0.0  
**最后更新**: 2026-03-27 00:15 UTC  
**Phase**: 132
