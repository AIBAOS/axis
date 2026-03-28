# 系统定时任务删除 API (Phase 259)

## 接口信息

- **路径**: `DELETE /api/v1/system/cron-jobs/{id}`
- **认证**: JWT Bearer Token (Required)
- **权限**: admin 角色
- **状态**: ✅ 已完成

## 请求参数

### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | u32 | 是 | 定时任务 ID |

### 请求头

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| Authorization | string | 是 | Bearer {JWT_TOKEN} |

## 响应

### 204 No Content - 删除成功

删除成功，无响应体。

### 401 Unauthorized - 未认证

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can delete cron jobs",
  "code": "FORBIDDEN"
}
```

### 404 Not Found - 任务不存在

```json
{
  "success": false,
  "error": "Cron job {id} not found",
  "code": "NOT_FOUND"
}
```

### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "error": "Internal server error",
  "code": "INTERNAL_ERROR"
}
```

## 请求示例

```bash
curl -X DELETE "http://localhost:8080/api/v1/system/cron-jobs/1" \
  -H "Authorization: Bearer {JWT_TOKEN}"
```

## 响应示例

### 删除成功 (204)

```
HTTP/1.1 204 No Content
```

### 任务不存在 (404)

```json
{
  "success": false,
  "error": "Cron job 999 not found",
  "code": "NOT_FOUND"
}
```

## 错误码

| 错误码 | HTTP 状态码 | 说明 |
|--------|------------|------|
| UNAUTHORIZED | 401 | 未提供或无效的 JWT Token |
| FORBIDDEN | 403 | 非 admin 用户无权删除 |
| NOT_FOUND | 404 | 指定的定时任务不存在 |
| INTERNAL_ERROR | 500 | 服务器内部错误 |

## 实现细节

- JWT Token 验证失败返回 401
- 非 admin 用户返回 403
- 任务 ID 不存在返回 404
- 删除成功返回 204 No Content（无响应体）

## 单元测试

测试用例覆盖：

1. ✅ 删除成功（返回 204）
2. ✅ 任务不存在（返回 404）
3. ✅ 未认证请求（返回 401）

## 相关文件

- 实现：`src/handlers/system_cron_jobs_delete.rs`
- 路由：`src/main.rs`
- 模块：`src/handlers/mod.rs`

## 更新记录

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-28 | 1.0 | Phase 259 初始实现 |
