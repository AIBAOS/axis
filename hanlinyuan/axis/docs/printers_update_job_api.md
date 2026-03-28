# 更新打印任务 API (Phase 115)

## 接口说明

实现更新打印任务的接口。登录用户可访问，支持部分更新（优先级/状态）。

## 接口定义

```
PUT /api/v1/printers/{printer_id}/jobs/{job_id}
```

## 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| printer_id | integer | 是 | 打印机 ID |
| job_id | integer | 是 | 打印任务 ID |

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（需要登录状态） |
| Content-Type | application/json | 是 | 请求体格式 |

## 请求体

```json
{
  "priority": 1,
  "state": "canceled"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| priority | integer | 否 | 优先级（1-5，1 最高） |
| state | string | 否 | 状态（pending/printing/completed/failed/canceled） |

**注意：** 至少提供一个字段（priority 或 state）

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Print job updated successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "document_name": "report.pdf",
    "user": "admin",
    "pages": 5,
    "copies": 2,
    "state": "canceled",
    "priority": 1,
    "submitted_at": 1711500000,
    "completed_at": 1711500300,
    "error_message": null
  }
}
```

### 未授权 (401 Unauthorized)

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 打印机不存在 (404 Not Found)

```json
{
  "success": false,
  "error": "Printer 999 not found",
  "code": "NOT_FOUND"
}
```

### 任务不存在 (404 Not Found)

```json
{
  "success": false,
  "error": "Job 999 not found for printer 1",
  "code": "NOT_FOUND"
}
```

### 参数错误 (400 Bad Request)

```json
{
  "success": false,
  "error": "At least one field (priority or state) must be provided",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "priority must be between 1 and 5",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Invalid state 'invalid'. Valid states: pending, printing, completed, failed, canceled",
  "code": "INVALID_PARAMS"
}
```

## 使用示例

### cURL 示例

```bash
# 更新任务优先级
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "priority": 1
  }'

# 取消任务
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "state": "canceled"
  }'

# 同时更新优先级和状态
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "priority": 1,
    "state": "canceled"
  }'

# 未提供字段（返回 400）
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{}'

# 更新不存在的打印机任务（返回 404）
curl -X PUT "http://localhost:8080/api/v1/printers/999/jobs/1" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"priority": 1}'
```

### JavaScript 示例

```javascript
// 更新打印任务
async function updateJob(printerId, jobId, updates) {
  const response = await fetch(
    `http://localhost:8080/api/v1/printers/${printerId}/jobs/${jobId}`,
    {
      method: 'PUT',
      headers: {
        'Authorization': 'Bearer ' + token,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(updates)
    }
  );
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error);
  }
  
  const data = await response.json();
  console.log('Job updated:', data.data);
  return data.data;
}

// 使用示例
try {
  // 提高优先级
  const job = await updateJob(1, 1, { priority: 1 });
  console.log(`Job ${job.id} priority updated to ${job.priority}`);
  
  // 取消任务
  const canceled = await updateJob(1, 1, { state: 'canceled' });
  console.log(`Job ${canceled.id} canceled at ${canceled.completed_at}`);
} catch (e) {
  console.error('Update failed:', e.message);
}
```

## 响应字段说明

### PrintJobDetail

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 任务 ID |
| printer_id | integer | 打印机 ID |
| document_name | string | 文档名称 |
| user | string | 提交用户 |
| pages | integer | 页数 |
| copies | integer | 份数 |
| state | string | 状态（pending/printing/completed/failed/canceled） |
| priority | integer | 优先级（1-5，1 最高） |
| submitted_at | integer | 提交时间（Unix 时间戳） |
| completed_at | integer/null | 完成时间（Unix 时间戳） |
| error_message | string/null | 错误消息（失败时） |

## 打印任务状态说明

| 状态 | 说明 |
|------|------|
| pending | 等待打印 |
| printing | 正在打印 |
| completed | 打印完成 |
| failed | 打印失败 |
| canceled | 已取消 |

## 优先级说明

| 优先级 | 说明 |
|--------|------|
| 1 | 最高优先级 |
| 2 | 高优先级 |
| 3 | 普通优先级（默认） |
| 4 | 低优先级 |
| 5 | 最低优先级 |

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **登录用户可访问**: 无需 admin 权限
3. **打印机存在性验证**: 防止更新不存在打印机
4. **任务存在性验证**: 防止更新不存在任务
5. **参数验证**: 严格验证优先级和状态值
6. **部分更新**: 至少提供一个更新字段

## 实现文件

- `src/handlers/printers_update_job.rs` - 更新任务处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际 CUPS 服务
2. 至少提供 priority 或 state 之一
3. priority 范围为 1-5
4. state 为 completed/failed/canceled 时会自动设置 completed_at
5. 时间戳使用 Unix 时间戳（秒级）

## 相关接口

- `GET /api/v1/printers` - 打印机列表（Phase 111）
- `GET /api/v1/printers/{id}` - 打印机详情（Phase 114）
- `GET /api/v1/printers/{id}/jobs` - 打印机任务列表（Phase 112）
- `GET /api/v1/printers/{id}/jobs/{job_id}` - 打印任务详情（Phase 113）
- `POST /api/v1/printers/{id}/jobs` - 创建打印任务（Phase 114）
- `DELETE /api/v1/printers/{id}/jobs/{job_id}` - 取消打印任务
