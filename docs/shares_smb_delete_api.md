# SMB 共享删除 API

## Phase 212

## 接口说明

删除 SMB 共享配置。

## 请求

`DELETE /api/v1/shares/smb/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | SMB 共享 ID |

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

#### 403 Forbidden - 非 admin 用户

```json
{
  "success": false,
  "error": "Only admin users can delete SMB shares",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - SMB 共享不存在

```json
{
  "success": false,
  "error": "SMB share 1 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "删除共享失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 删除 SMB 共享

```bash
curl -X DELETE "http://localhost:8080/api/v1/shares/smb/1" \
  -H "Authorization: Bearer <jwt_token>"
```

### 响应（成功）

```
204 No Content
```

## 权限要求

- 需要 JWT 认证
- **仅 admin 用户可删除 SMB 共享**

## 业务逻辑

1. 验证 JWT Token 有效性
2. 验证用户角色是否为 admin
3. 查询 SMB 共享是否存在（404 Not Found）
4. 验证协议类型（仅允许 SMB）
5. 删除共享记录
6. 返回 204 No Content

## 版本历史

- **Phase 212** (2026-03-28): SMB 共享删除 API - SqliteShareRepository 真实删除
- **Phase 159** (2026-03-27): SMB 共享删除 API 初始实现（模拟）
