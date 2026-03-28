# 系统定时任务删除 API

## Phase 259

## 接口说明

删除系统定时任务（cron jobs），用于移除不再需要的定时任务，仅限 admin 角色访问。

## 请求

`DELETE /api/v1/system/cron-jobs/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 定时任务 ID |

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

#### 403 Forbidden - 权限不足（非 admin）

```json
{
  "success": false,
  "error": "Only admin users can delete cron jobs",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 任务不存在

```json
{
  "success": false,
  "error": "Cron job 999 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Failed to delete cron job: database error",
  "code": "INTERNAL_ERROR"
}
```

## 示例

### 删除定时任务

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应：**204 No Content**

### 删除不存在的任务

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/cron-jobs/999" \
  -H "Authorization: Bearer <admin_jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Cron job 999 not found",
  "code": "NOT_FOUND"
}
```

### 非 admin 用户删除任务

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "Only admin users can delete cron jobs",
  "code": "FORBIDDEN"
}
```

### 未认证请求

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/cron-jobs/1"
```

响应（401 Unauthorized）：
```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析任务 ID 路径参数
4. 验证任务是否存在（404 Not Found）
5. 从数据库删除定时任务
6. 返回 204 No Content

## 安全说明

- 此接口仅限 admin 用户调用
- 删除操作不可逆，建议添加二次确认机制
- 建议添加操作审计日志
- 删除前应检查任务是否正在运行

## 版本历史

- **Phase 259** (2026-03-28): 系统模块 - 系统定时任务删除 API
