# 防火墙规则详情 API

**Phase 136** - 防火墙管理 API 之获取防火墙规则详情接口

---

## 接口信息

- **端点:** `GET /api/v1/firewall/rules/{rule_id}`
- **认证:** 需要 JWT Bearer Token（仅 admin 角色）
- **权限:** 仅 admin 用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <ADMIN_JWT_TOKEN>` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `rule_id` | integer | 是 | 防火墙规则 ID |

### 请求示例

```bash
# 获取防火墙规则详情
curl -X GET "http://localhost:8080/api/v1/firewall/rules/1" \
  -H "Authorization: Bearer <ADMIN_JWT_TOKEN>"
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "data": {
    "rule_id": 1,
    "name": "Allow HTTP",
    "priority": 100,
    "action": "allow",
    "protocol": "tcp",
    "source_ip": "0.0.0.0/0",
    "source_port": "80",
    "dest_ip": "192.168.1.0/24",
    "dest_port": "80",
    "interface": "eth0",
    "enabled": true,
    "created_at": 1711440000,
    "updated_at": 1711440000
  }
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
  "error": "Only admin users can access firewall rules",
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

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **权限控制**: 仅 admin 角色用户可以访问防火墙规则详情

---

## 响应字段说明

### FirewallRuleDetailResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `data` | object | 防火墙规则详情 |

### FirewallRuleDetail

| 字段 | 类型 | 说明 |
|------|------|------|
| `rule_id` | integer | 规则 ID |
| `name` | string | 规则名称 |
| `priority` | integer | 优先级（数字越小优先级越高） |
| `action` | string | 动作：allow/deny/drop |
| `protocol` | string | 协议：tcp/udp/icmp/any |
| `source_ip` | string\|null | 源 IP 地址（支持 CIDR） |
| `source_port` | string\|null | 源端口 |
| `dest_ip` | string\|null | 目标 IP 地址（支持 CIDR） |
| `dest_port` | string\|null | 目标端口 |
| `interface` | string\|null | 网络接口名称 |
| `enabled` | boolean | 是否启用 |
| `created_at` | integer | 创建时间（Unix 时间戳） |
| `updated_at` | integer | 更新时间（Unix 时间戳） |

---

## 实现细节

- **文件位置:** `src/handlers/firewall_rule_detail.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/firewall/rules/{rule_id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证

---

## 相关接口

- `GET /api/v1/firewall/rules` - 防火墙规则列表（Phase 130）
- `POST /api/v1/firewall/rules` - 创建防火墙规则（Phase 135）
- `PUT /api/v1/firewall/rules/{id}` - 更新防火墙规则
- `DELETE /api/v1/firewall/rules/{id}` - 删除防火墙规则

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-27 | 1.0 | Phase 136 初始实现 |
