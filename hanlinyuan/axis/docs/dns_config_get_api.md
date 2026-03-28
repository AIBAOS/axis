# DNS 配置 API (Phase 125)

## 接口说明

实现获取和更新 DNS 配置信息的接口。仅 admin 角色可访问。

## 接口定义

### 获取 DNS 配置
```
GET /api/v1/network/dns
```

### 更新 DNS 配置
```
PUT /api/v1/network/dns
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "dns_primary": "8.8.8.8",
    "dns_secondary": "8.8.4.4",
    "dns_search_domains": ["local", "lan"],
    "dns_mode": "manual"
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
  "error": "Only admin users can access DNS configuration",
  "code": "FORBIDDEN"
}
```

## 更新 DNS 配置请求体

```json
{
  "dns_primary": "8.8.8.8",
  "dns_secondary": "8.8.4.4",
  "dns_mode": "manual"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| dns_primary | string | 否 | 主 DNS 服务器地址 |
| dns_secondary | string | 否 | 备用 DNS 服务器地址 |
| dns_mode | string | 否 | DNS 模式（auto/manual） |

**注意：** manual 模式时 dns_primary 必填

## 使用示例

### cURL 示例

```bash
# 获取 DNS 配置
curl -X GET "http://localhost:8080/api/v1/network/dns" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 更新 DNS 配置（手动模式）
curl -X PUT "http://localhost:8080/api/v1/network/dns" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "dns_mode": "manual",
    "dns_primary": "8.8.8.8",
    "dns_secondary": "8.8.4.4"
  }'

# 更新 DNS 配置（自动模式）
curl -X PUT "http://localhost:8080/api/v1/network/dns" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "dns_mode": "auto"
  }'

# 非 admin 用户访问（返回 403）
curl -X GET "http://localhost:8080/api/v1/network/dns" \
  -H "Authorization: Bearer <user_jwt_token>"
```

### JavaScript 示例

```javascript
// 获取 DNS 配置
async function getDnsConfig() {
  const response = await fetch(
    'http://localhost:8080/api/v1/network/dns',
    {
      headers: {
        'Authorization': 'Bearer ' + adminToken
      }
    }
  );
  
  const data = await response.json();
  console.log('DNS config:', data.data);
  return data.data;
}

// 使用示例
const config = await getDnsConfig();
console.log(`Primary DNS: ${config.dns_primary}`);
console.log(`Secondary DNS: ${config.dns_secondary}`);
console.log(`Search domains: ${config.dns_search_domains.join(', ')}`);
console.log(`DNS mode: ${config.dns_mode}`);
```

## 响应字段说明

### DnsConfig

| 字段 | 类型 | 说明 |
|------|------|------|
| dns_primary | string | 主 DNS 服务器地址 |
| dns_secondary | string | 备用 DNS 服务器地址 |
| dns_search_domains | array | DNS 搜索域列表 |
| dns_mode | string | DNS 模式（auto/manual） |

## DNS 模式说明

| 模式 | 说明 |
|------|------|
| auto | 自动获取（通过 DHCP） |
| manual | 手动配置 |

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **敏感信息保护**: DNS 配置属于敏感信息，限制访问权限

## 实现文件

- `src/handlers/dns_config_get.rs` - DNS 配置处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际系统 DNS 配置
2. 仅 admin 角色可访问
3. DNS 配置属于敏感信息，应妥善保管

## 相关接口

- `GET /api/v1/network/config` - 获取网络配置（Phase 122）
- `PUT /api/v1/network/config` - 更新网络配置（Phase 123）
- `GET /api/v1/network/interfaces` - 网络接口列表（Phase 124）
