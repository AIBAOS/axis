# 防火墙规则更新 API

**Phase 137** - 防火墙管理 API 之更新防火墙规则接口

---

## 接口信息

- **端点:** `PUT /api/v1/firewall/rules/{rule_id}`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <ADMIN_JWT_TOKEN>` |
| `Content-Type` | 是 | `application/json` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `rule_id` | integer | 是 | 防火墙规则 ID |

### 请求体

```json
{
  "name": "string (可选)",
  "priority": "integer (可选)",
  "action": "string (可选)",
  "protocol": "string (可选)",
  "source_ip": "string (可选)",
  "source_port": "string (可选)",
  "dest_ip": "string (可选)",
  "dest_port": "string (可选)",
  "interface": "string (可选)",
  "enabled": "boolean (可选)"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 规则名称（必须唯一） |
| `priority` | integer | 否 | 优先级（0-9999，数字越小优先级越高） |
| `action` | string | 否 | 动作：allow/deny/drop |
| `protocol` | string | 否 | 协议：tcp/udp/icmp/any |
| `source_ip` | string | 否 | 源 IP 地址（支持 CIDR） |
| `source_port` | string | 否 | 源端口（支持范围：80-443） |
| `dest_ip` | string | 否 | 目标 IP 地址（支持 CIDR） |
| `dest_port` | string | 否 | 目标端口（支持范围） |
| `interface` | string | 否 | 网络接口名称 |
| `enabled` | boolean | 否 | 是否启用 |

### 请求示例

```bash
# 更新规则名称
curl -X PUT "http://localhost:8080/api/v1/firewall/rules/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Allow HTTP Traffic"
  }'

# 更新多个字段
curl -X PUT "http://localhost:8080/api/v1/firewall/rules/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "priority": 50,
    "action": "deny",
    "enabled": false
  }'

# 更新 IP 地址和端口
curl -X PUT "http://localhost:8080/api/v1/firewall/rules/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "source_ip": "10.0.0.0/8",
    "dest_port": "443"
  }'
```

---

## 响应

### 200 OK - 更新成功

```json
{
  "success": true,
  "message": "Firewall rule updated successfully",
  "data": {
    "rule_id": 1,
    "name": "Allow HTTP Traffic",
    "priority": 50,
    "action": "deny",
    "protocol": "tcp",
    "source_ip": "10.0.0.0/8",
    "source_port": "80",
    "dest_ip": "192.168.1.0/24",
    "dest_port": "443",
    "interface": "eth0",
    "enabled": false,
    "created_at": 1711440000,
    "updated_at": 1711526400
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "priority must be between 0 and 9999",
  "code": "INVALID_PRIORITY"
}
```

或

```json
{
  "success": false,
  "error": "Invalid action. Valid actions: allow, deny, drop",
  "code": "INVALID_ACTION"
}
```

或

```json
{
  "success": false,
  "error": "Invalid source_ip format: 999.999.999.999",
  "code": "INVALID_IP"
}
```

或

```json
{
  "success": false,
  "error": "Invalid source_port format: 99999",
  "code": "INVALID_PORT"
}
```

### 401 Unauthorized - 未认证

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden - 无权限

```json
{
  "success": false,
  "error": "Only admin users can update firewall rules",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 规则不存在

```json
{
  "success": false,
  "error": "Firewall rule 999 not found",
  "code": "NOT_FOUND"
}
```

### 409 Conflict - 规则名称已存在

```json
{
  "success": false,
  "error": "Firewall rule name 'Allow HTTP' already exists",
  "code": "NAME_CONFLICT"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以更新防火墙规则
3. **名称唯一性**: 规则名称必须全局唯一（排除自身）
4. **参数验证**: priority/action/protocol/IP/port 都必须符合格式要求

---

## 响应字段说明

### UpdateFirewallRuleResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 更新后的规则详情 |

### FirewallRuleDetail

| 字段 | 类型 | 说明 |
|------|------|------|
| `rule_id` | integer | 规则 ID |
| `name` | string | 规则名称 |
| `priority` | integer | 优先级（0-9999） |
| `action` | string | 动作：allow/deny/drop |
| `protocol` | string | 协议：tcp/udp/icmp/any |
| `source_ip` | string\|null | 源 IP 地址（支持 CIDR） |
| `source_port` | string\|null | 源端口（支持范围） |
| `dest_ip` | string\|null | 目标 IP 地址（支持 CIDR） |
| `dest_port` | string\|null | 目标端口（支持范围） |
| `interface` | string\|null | 网络接口名称 |
| `enabled` | boolean | 是否启用 |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

---

## 实现细节

- **文件位置:** `src/handlers/firewall_rule_update.rs`
- **路由注册:** `src/main.rs` - `PUT /api/v1/firewall/rules/{rule_id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `GET /api/v1/firewall/rules` - 防火墙规则列表（Phase 130）
- `POST /api/v1/firewall/rules` - 创建防火墙规则（Phase 135）
- `GET /api/v1/firewall/rules/{id}` - 防火墙规则详情（Phase 136）
- `DELETE /api/v1/firewall/rules/{id}` - 删除防火墙规则

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-27 | 1.0 | Phase 137 初始实现 |
