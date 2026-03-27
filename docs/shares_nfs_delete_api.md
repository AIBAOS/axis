# NFS 共享删除 API

## Phase 160

## 接口说明

删除指定的 NFS 共享。

## 请求

`DELETE /api/v1/shares/nfs/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | u64 | 是 | 共享 ID |

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
  "error": "Only admin users can delete NFS shares",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 共享不存在

```json
{
  "success": false,
  "error": "NFS share 999 not found",
  "code": "NOT_FOUND"
}
```

## 示例

### 删除 NFS 共享

```bash
curl -X DELETE "http://localhost:8080/api/v1/shares/nfs/1" \
  -H "Authorization: Bearer <jwt_token>"
```

响应：`204 No Content`

### 删除不存在的共享

```bash
curl -X DELETE "http://localhost:8080/api/v1/shares/nfs/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "NFS share 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证共享 ID 存在性（404 Not Found）
4. 删除共享配置
5. 返回 204 No Content

## 版本历史

- **Phase 160** (2026-03-27): 初始版本
