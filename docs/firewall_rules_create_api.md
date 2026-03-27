# 防火墙规则创建 API (Phase 135)

## 接口说明

实现创建防火墙规则的接口。仅 admin 角色可访问。

## 接口定义

```
POST /api/v1/firewall/rules
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |
| Content-Type | application/json | 是 | 请求体格式 |

## 请求体

```json
{
  "name": "Allow HTTP",
  "priority": 100,
  "action": "allow",
  "protocol": "tcp",
  "source_ip": "0.0.0.0/0",
  "source_port": "*",
  "dest_ip": "192.168.1.100",
  "dest_port": "80",
  "interface": "eth0",
  "enabled": true
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 是 | 规则名称（必填，唯一） |
| priority | integer | 是 | 优先级（数字，越小优先级越高） |
| action | string | 是 | 动作（allow/deny/drop） |
| protocol | string | 是 | 协议（tcp/udp/icmp/any） |
| source_ip | string | 否 | 源 IP 地址（支持 CIDR） |
| source_port | string | 否 | 源端口（支持范围如 80-443） |
| dest_ip | string | 否 | 目标 IP 地址（支持 CIDR） |
| dest_port | string | 否 | 目标端口（支持范围） |
| interface | string | 否 | 网络接口名称 |
| enabled | boolean | 否 | 是否启用（默认 true） |

## 响应格式

### 成功响应 (201 Created)

```json
{
  "success": true,
  "message": "Firewall rule created successfully",
  "data": {
    "rule_id": 6,
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
    "created_at": 1711500000,
    "updated_at": 1711500000
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
  "error": "Only admin users can create firewall rules",
  "code": "FORBIDDEN"
}
```

### 规则名称已存在 (409 Conflict)

```json
{
  "success": false,
  "error": "Firewall rule 'Allow HTTP' already exists",
  "code": "CONFLICT"
}
```

### 参数错误 (400 Bad Request)

```json
{
  "success": false,
  "error": "priority must be greater than 0",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Invalid action. Valid values: allow, deny, drop",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Invalid source_ip format",
  "code": "INVALID_IP"
}
```

或

```json
{
  "success": false,
  "error": "Invalid source_port format",
  "code": "INVALID_PORT"
}
```

## 使用示例

### cURL 示例

```bash
# 创建允许 HTTP 的规则
curl -X POST "http://localhost:8080/api/v1/firewall/rules" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Allow HTTP",
    "priority": 100,
    "action": "allow",
    "protocol": "tcp",
    "source_ip": "0.0.0.0/0",
    "source_port": "*",
    "dest_ip": "192.168.1.100",
    "dest_port": "80",
    "interface": "eth0",
    "enabled": true
  }'

# 创建拒绝所有规则
curl -X POST "http://localhost:8080/api/v1/firewall/rules" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Deny All",
    "priority": 1000,
    "action": "deny",
    "protocol": "any",
    "source_ip": "0.0.0.0/0",
    "source_port": "*",
    "dest_ip": "0.0.0.0/0",
    "dest_port": "*",
    "interface": "any",
    "enabled": true
  }'

# 创建允许端口范围的规则
curl -X POST "http://localhost:8080/api/v1/firewall/rules" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Allow HTTP-HTTPS",
    "priority": 100,
    "action": "allow",
    "protocol": "tcp",
    "source_ip": "0.0.0.0/0",
    "source_port": "*",
    "dest_ip": "192.168.1.100",
    "dest_port": "80-443",
    "interface": "eth0",
    "enabled": true
  }'

# 规则名称已存在（返回 409）
curl -X POST "http://localhost:8080/api/v1/firewall/rules" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Allow HTTP",
    "priority": 101,
    "action": "allow",
    "protocol": "tcp"
  }'

# 无效的 action（返回 400）
curl -X POST "http://localhost:8080/api/v1/firewall/rules" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Invalid Rule",
    "priority": 100,
    "action": "invalid",
    "protocol": "tcp"
  }'

# 非 admin 用户访问（返回 403）
curl -X POST "http://localhost:8080/api/v1/firewall/rules" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "User Rule",
    "priority": 100,
    "action": "allow",
    "protocol": "tcp"
  }'
```

### JavaScript 示例

```javascript
// 创建防火墙规则
async function createFirewallRule(ruleData) {
  const response = await fetch(
    'http://localhost:8080/api/v1/firewall/rules',
    {
      method: 'POST',
      headers: {
        'Authorization': 'Bearer ' + adminToken,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(ruleData)
    }
  );
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error);
  }
  
  const data = await response.json();
  console.log('Firewall rule created:', data.data);
  return data.data;
}

// 使用示例
try {
  await createFirewallRule({
    name: 'Allow HTTP',
    priority: 100,
    action: 'allow',
    protocol: 'tcp',
    source_ip: '0.0.0.0/0',
    dest_ip: '192.168.1.100',
    dest_port: '80',
    interface: 'eth0',
    enabled: true
  });
  console.log('Rule created successfully');
} catch (e) {
  console.error('Creation failed:', e.message);
}
```

## 响应字段说明

### FirewallRule

| 字段 | 类型 | 说明 |
|------|------|------|
| rule_id | integer | 规则 ID |
| name | string | 规则名称 |
| priority | integer | 优先级（数字越小优先级越高） |
| action | string | 动作（allow/deny/drop） |
| protocol | string | 协议（tcp/udp/icmp/any） |
| source_ip | string | 源 IP 地址（支持 CIDR） |
| source_port | string | 源端口（* 表示所有，支持范围如 80-443） |
| dest_ip | string | 目标 IP 地址（支持 CIDR） |
| dest_port | string | 目标端口（* 表示所有，支持范围） |
| interface | string | 网络接口名称（any 表示所有接口） |
| enabled | boolean | 是否启用 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |

## 验证规则

### 规则名称
- 必填
- 必须唯一
- 不能为空

### Priority（优先级）
- 必填
- 必须大于 0
- 数字越小优先级越高

### Action（动作）
- 必填
- 有效值：allow, deny, drop
- allow: 允许通过
- deny: 拒绝并返回拒绝响应
- drop: 直接丢弃数据包

### Protocol（协议）
- 必填
- 有效值：tcp, udp, icmp, any

### IP 地址格式
- 支持普通 IP 地址（如 192.168.1.100）
- 支持 CIDR 格式（如 192.168.1.0/24）
- 支持 0.0.0.0/0 表示所有 IP

### 端口格式
- 支持单个端口（如 80）
- 支持端口范围（如 80-443）
- 支持 * 表示所有端口

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **名称唯一性**: 规则名称必须全局唯一
5. **IP 格式验证**: 验证 IP 地址和 CIDR 格式合法性
6. **端口格式验证**: 验证端口和端口范围合法性

## 实现文件

- `src/handlers/firewall_rules_create.rs` - 防火墙规则创建处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际防火墙系统（如 iptables/nftables）
2. 仅 admin 角色可访问
3. 规则名称必须唯一
4. 优先级数字越小优先级越高

## 相关接口

- `GET /api/v1/firewall/rules` - 防火墙规则列表（Phase 130）
- `PUT /api/v1/firewall/rules/{id}` - 更新防火墙规则
- `DELETE /api/v1/firewall/rules/{id}` - 删除防火墙规则
- `GET /api/v1/network/interfaces` - 网络接口列表（Phase 129）
