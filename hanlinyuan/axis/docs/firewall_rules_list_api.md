# 防火墙规则列表 API (Phase 130)

## 接口说明

实现获取防火墙规则列表的接口。仅 admin 角色可访问，支持筛选和分页。

## 接口定义

```
GET /api/v1/firewall/rules
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |

## 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| action | string | 否 | - | 按动作筛选（allow/deny） |
| protocol | string | 否 | - | 按协议筛选（tcp/udp/icmp） |
| enabled | boolean | 否 | - | 按启用状态筛选 |
| interface | string | 否 | - | 按接口筛选（eth0/wlan0 等） |
| page | integer | 否 | 1 | 页码 |
| per_page | integer | 否 | 20 | 每页数量（最大 100） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "rule_id": 1,
      "name": "Allow HTTP",
      "priority": 100,
      "action": "allow",
      "protocol": "tcp",
      "source_ip": "0.0.0.0/0",
      "source_port": "*",
      "dest_ip": "192.168.1.100",
      "dest_port": "80",
      "interface": "eth0",
      "enabled": true,
      "created_at": 1710500000,
      "updated_at": 1710500000
    },
    {
      "rule_id": 2,
      "name": "Allow HTTPS",
      "priority": 101,
      "action": "allow",
      "protocol": "tcp",
      "source_ip": "0.0.0.0/0",
      "source_port": "*",
      "dest_ip": "192.168.1.100",
      "dest_port": "443",
      "interface": "eth0",
      "enabled": true,
      "created_at": 1710500000,
      "updated_at": 1710500000
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 5,
    "total_pages": 1
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
  "error": "Only admin users can access firewall rules",
  "code": "FORBIDDEN"
}
```

## 使用示例

### cURL 示例

```bash
# 获取所有防火墙规则
curl -X GET "http://localhost:8080/api/v1/firewall/rules" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 按动作筛选（仅允许规则）
curl -X GET "http://localhost:8080/api/v1/firewall/rules?action=allow" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 按协议筛选（仅 TCP 规则）
curl -X GET "http://localhost:8080/api/v1/firewall/rules?protocol=tcp" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 按启用状态筛选
curl -X GET "http://localhost:8080/api/v1/firewall/rules?enabled=true" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 按接口筛选
curl -X GET "http://localhost:8080/api/v1/firewall/rules?interface=eth0" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 分页查询
curl -X GET "http://localhost:8080/api/v1/firewall/rules?page=1&per_page=10" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 组合筛选
curl -X GET "http://localhost:8080/api/v1/firewall/rules?action=allow&protocol=tcp&interface=eth0" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 非 admin 用户访问（返回 403）
curl -X GET "http://localhost:8080/api/v1/firewall/rules" \
  -H "Authorization: Bearer <user_jwt_token>"
```

### JavaScript 示例

```javascript
// 获取防火墙规则列表
async function getFirewallRules(filters = {}) {
  const params = new URLSearchParams();
  if (filters.action) params.append('action', filters.action);
  if (filters.protocol) params.append('protocol', filters.protocol);
  if (filters.enabled !== undefined) params.append('enabled', filters.enabled);
  if (filters.interface) params.append('interface', filters.interface);
  if (filters.page) params.append('page', filters.page);
  if (filters.per_page) params.append('per_page', filters.per_page);
  
  const response = await fetch(
    `http://localhost:8080/api/v1/firewall/rules?${params}`,
    {
      headers: {
        'Authorization': 'Bearer ' + adminToken
      }
    }
  );
  
  const data = await response.json();
  console.log('Firewall rules:', data.data);
  console.log('Pagination:', data.pagination);
  return data.data;
}

// 使用示例
const rules = await getFirewallRules({ action: 'allow', protocol: 'tcp' });
rules.forEach(rule => {
  console.log(`${rule.name}: ${rule.action} ${rule.protocol} ${rule.dest_ip}:${rule.dest_port}`);
});
```

## 响应字段说明

### FirewallRule

| 字段 | 类型 | 说明 |
|------|------|------|
| rule_id | integer | 规则 ID |
| name | string | 规则名称 |
| priority | integer | 优先级（数字越小优先级越高） |
| action | string | 动作（allow/deny） |
| protocol | string | 协议（tcp/udp/icmp） |
| source_ip | string | 源 IP 地址（支持 CIDR） |
| source_port | string | 源端口（* 表示所有） |
| dest_ip | string | 目标 IP 地址（支持 CIDR） |
| dest_port | string | 目标端口（* 表示所有） |
| interface | string | 网络接口（eth0/wlan0 等） |
| enabled | boolean | 是否启用 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |

### PaginationInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| page | integer | 当前页码 |
| per_page | integer | 每页数量 |
| total | integer | 总记录数 |
| total_pages | integer | 总页数 |

## 筛选参数说明

### action 筛选

- `allow`: 仅返回允许规则
- `deny`: 仅返回拒绝规则

### protocol 筛选

- `tcp`: 仅返回 TCP 协议规则
- `udp`: 仅返回 UDP 协议规则
- `icmp`: 仅返回 ICMP 协议规则

### enabled 筛选

- `true`: 仅返回已启用规则
- `false`: 仅返回已禁用规则

### interface 筛选

- `eth0`: 仅返回 eth0 接口规则
- `wlan0`: 仅返回 wlan0 接口规则
- 等等

## 排序规则

- 按 priority 升序排序（优先级数字越小越靠前）
- 优先级高的规则先匹配

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **Token 验证**: 验证 JWT Token 有效性

## 实现文件

- `src/handlers/firewall_rules.rs` - 防火墙规则列表处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际防火墙系统（如 iptables/nftables）
2. 仅 admin 角色可访问
3. 支持按 action/protocol/enabled/interface 筛选
4. 支持分页（page/per_page）
5. 按 priority 升序排序

## 相关接口

- `GET /api/v1/network/interfaces` - 网络接口列表（Phase 129）
- `GET /api/v1/network/config` - 网络配置（Phase 122）
- `PUT /api/v1/network/config` - 更新网络配置（Phase 123）
