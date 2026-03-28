# 系统设置获取 API

## Phase 246

## 接口说明

获取系统设置信息，仅限 admin 角色访问。

## 请求

### 获取所有设置

`GET /api/v1/settings`

### 获取单个设置

`GET /api/v1/settings/{key}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| key | string | 否 | 设置键名（如 network.host, system.timezone） |

### 请求体

无

## 响应

### 成功响应 - 获取所有设置（200 OK）

```json
{
  "network.host": "0.0.0.0",
  "network.port": 8080,
  "storage.path": "/data",
  "system.timezone": "Asia/Shanghai",
  "user.prefer_theme": "dark"
}
```

### 成功响应 - 获取单个设置（200 OK）

```json
{
  "key": "system.timezone",
  "value": "Asia/Shanghai",
  "updated_at": "2026-03-28T12:00:00Z"
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| key | string | 设置键名 |
| value | any | 设置值（支持多种类型） |
| updated_at | string | 最后更新时间（ISO 8601 格式） |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "message": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 404 Not Found - 设置不存在

```json
{
  "success": false,
  "message": "Setting 'invalid.key' not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "message": "Failed to get settings: io error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 获取所有系统设置

```bash
curl -X GET "http://localhost:8080/api/v1/settings" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

### 获取单个设置

```bash
curl -X GET "http://localhost:8080/api/v1/settings/system.timezone" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（200 OK）：
```json
{
  "key": "system.timezone",
  "value": "Asia/Shanghai",
  "updated_at": "2026-03-28T12:00:00Z"
}
```

### 获取不存在的设置

```bash
curl -X GET "http://localhost:8080/api/v1/settings/invalid.key" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "message": "Setting 'invalid.key' not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 建议仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 解析请求路径
3. 查询设置存储
4. 设置不存在返回 404 Not Found
5. 返回设置信息

## 设置键名规范

| 键名前缀 | 说明 | 示例 |
|----------|------|------|
| network.* | 网络设置 | network.host, network.port |
| storage.* | 存储设置 | storage.path |
| system.* | 系统设置 | system.timezone |
| user.* | 用户设置 | user.prefer_theme |

## 安全说明

- 此接口需要 JWT 认证
- 设置可能包含敏感信息，建议添加访问审计
- 建议限制调用频率

## 版本历史

- **Phase 72-74** (2026-03-18): 系统设置模块 - 初始实现
- **Phase 246** (2026-03-28): 系统设置模块 - 文档补充
