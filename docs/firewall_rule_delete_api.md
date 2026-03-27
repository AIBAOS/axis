# 防火墙规则删除 API

## 接口说明

删除指定的防火墙规则。

## 请求

```
DELETE /api/v1/firewall/rules/{rule_id}
```

### 路径参数

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| rule_id | u64 | 是 | 防火墙规则 ID |

### 请求头

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 请求体

无

## 响应

### 成功响应（204 No Content）

删除成功，无响应体。

### 错误响应

#### 401 Unauthorized

未认证或 Token 无效。

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden

非 admin 角色访问。

```json
{
  "success": false,
  "error": "Only admin users can delete firewall rules",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found

规则 ID 不存在。

```json
{
  "success": false,
  "error": "Firewall rule {rule_id} not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 请求示例

```bash
curl -X DELETE "http://localhost:8080/api/v1/firewall/rules/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 响应示例（204 No Content）

```
HTTP/1.1 204 No Content
```

### 响应示例（404 Not Found）

```json
{
  "success": false,
  "error": "Firewall rule 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证规则 ID 存在性
4. 删除指定规则
5. 返回 204 No Content

## 版本历史

- **Phase 138** (2026-03-27): 初始版本
