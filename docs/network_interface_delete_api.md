# 网络接口删除 API

## Phase 185

## 接口说明

删除指定的网络接口。

## 请求

`DELETE /api/v1/network/interfaces/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 网络接口 ID |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 请求体

无

## 响应

### 成功响应（204 No Content）

删除成功，无响应体。

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can delete network interfaces",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 接口不存在

```json
{
  "success": false,
  "error": "Network interface 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 删除网络接口

```bash
curl -X DELETE "http://localhost:8080/api/v1/network/interfaces/1" \
  -H "Authorization: Bearer <jwt_token>"
```

响应：`204 No Content`

### 删除不存在的接口

```bash
curl -X DELETE "http://localhost:8080/api/v1/network/interfaces/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Network interface 999 not found",
  "code": "NOT_FOUND"
}
```

### 无权限访问

```bash
curl -X DELETE "http://localhost:8080/api/v1/network/interfaces/1" \
  -H "Authorization: Bearer <user_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can delete network interfaces",
  "code": "FORBIDDEN"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 根据接口 ID 查找接口
4. 接口不存在返回 404 Not Found
5. 删除接口
6. 返回 204 No Content

## 版本历史

- **Phase 185** (2026-03-27): 网络管理模块 - 网络接口删除 API
