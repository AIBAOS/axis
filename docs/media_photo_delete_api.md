# 媒体照片删除 API

## Phase 240

## 接口说明

删除指定照片，仅照片所有者可删除。

## 请求

`DELETE /api/v1/media/photos/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 照片 ID |

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

#### 403 Forbidden - 无权限（非照片所有者）

```json
{
  "success": false,
  "error": "Only photo owner can delete this photo",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 照片不存在

```json
{
  "success": false,
  "error": "Photo 999 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to delete photo: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 删除自己的照片

```bash
curl -X DELETE "http://localhost:8080/api/v1/media/photos/1" \
  -H "Authorization: Bearer <owner_jwt_token>"
```

响应：**204 No Content**

### 删除他人的照片（无权限）

```bash
curl -X DELETE "http://localhost:8080/api/v1/media/photos/3" \
  -H "Authorization: Bearer <other_user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only photo owner can delete this photo",
  "code": "FORBIDDEN"
}
```

### 删除不存在的照片

```bash
curl -X DELETE "http://localhost:8080/api/v1/media/photos/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Photo 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问
- **仅照片所有者可删除**

## 业务逻辑

1. 验证 JWT Token 有效性
2. 解析照片 ID 路径参数
3. 查询照片详情及所有者
4. 照片不存在返回 404 Not Found
5. 验证当前用户是否为照片所有者（403 Forbidden）
6. 删除照片文件及数据库记录
7. 返回 204 No Content

## 安全说明

- 此接口需要 JWT 认证
- 权限控制：仅照片所有者可删除
- 删除操作不可逆，建议添加二次确认机制
- 建议添加操作审计日志

## 版本历史

- **Phase 240** (2026-03-28): 媒体模块 - 照片删除 API
