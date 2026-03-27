# 更新打印任务 API 文档 (Phase 115)

## 概述

更新打印任务 API 允许用户更新指定打印任务的优先级或状态（如取消任务）。

## 接口详情

### PUT /api/v1/printers/{printer_id}/jobs/{job_id}

更新指定打印任务的信息。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
Content-Type: application/json
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `printer_id` | integer | 是 | 打印机 ID |
| `job_id` | integer | 是 | 打印任务 ID |

#### 请求体

```json
{
  "priority": "high",
  "state": "canceled"
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `priority` | string | 至少一个 | 优先级（low/normal/high/urgent） |
| `state` | string | 至少一个 | 状态（pending/printing/completed/failed/canceled） |

**注意：** 至少提供一个字段（priority 或 state）

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "message": "Print job updated successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "user_id": 101,
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "status": "canceled",
    "priority": "high",
    "submitted_at": "2026-03-26T10:00:00Z",
    "started_at": null,
    "completed_at": null,
    "error_message": null
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `message`: 响应消息
- `data`: 打印任务详细信息
  - `id`: 任务 ID
  - `printer_id`: 打印机 ID
  - `user_id`: 提交用户 ID
  - `document_name`: 文档名称
  - `pages`: 页数
  - `copies`: 份数
  - `status`: 状态（pending/printing/completed/failed/canceled）
  - `priority`: 优先级（low/normal/high/urgent）
  - `submitted_at`: 提交时间
  - `started_at`: 开始打印时间（可选）
  - `completed_at`: 完成时间（可选）
  - `error_message`: 错误信息（可选）

**错误响应 (400 Bad Request) - 参数无效**

```json
{
  "success": false,
  "error": "At least one field (priority or state) must be provided",
  "code": "INVALID_PARAMS"
}
```

**错误响应 (400 Bad Request) - 优先级无效**

```json
{
  "success": false,
  "error": "Invalid priority. Valid values: low, normal, high, urgent",
  "code": "INVALID_PRIORITY"
}
```

**错误响应 (400 Bad Request) - 状态无效**

```json
{
  "success": false,
  "error": "Invalid state. Valid values: pending, printing, completed, failed, canceled",
  "code": "INVALID_STATE"
}
```

**错误响应 (404 Not Found) - 打印机或任务不存在**

```json
{
  "success": false,
  "error": "Printer or job not found",
  "code": "NOT_FOUND"
}
```

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：更新优先级

```bash
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "priority": "high"
  }'
```

**响应：**
```json
{
  "success": true,
  "message": "Print job updated successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "user_id": 101,
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "status": "pending",
    "priority": "high",
    "submitted_at": "2026-03-26T10:00:00Z",
    "started_at": null,
    "completed_at": null,
    "error_message": null
  }
}
```

### 示例 2：取消打印任务

```bash
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "state": "canceled"
  }'
```

**响应：**
```json
{
  "success": true,
  "message": "Print job updated successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "user_id": 101,
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "status": "canceled",
    "priority": "normal",
    "submitted_at": "2026-03-26T10:00:00Z",
    "started_at": null,
    "completed_at": null,
    "error_message": null
  }
}
```

### 示例 3：同时更新优先级和状态

```bash
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "priority": "urgent",
    "state": "printing"
  }'
```

### 示例 4：未提供字段（400）

```bash
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{}'
```

**响应：**
```json
{
  "success": false,
  "error": "At least one field (priority or state) must be provided",
  "code": "INVALID_PARAMS"
}
```

### 示例 5：优先级无效（400）

```bash
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "priority": "invalid"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Invalid priority. Valid values: low, normal, high, urgent",
  "code": "INVALID_PRIORITY"
}
```

### 示例 6：状态无效（400）

```bash
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "state": "invalid"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Invalid state. Valid values: pending, printing, completed, failed, canceled",
  "code": "INVALID_STATE"
}
```

### 示例 7：打印机或任务不存在（404）

```bash
curl -X PUT "http://localhost:8080/api/v1/printers/999/jobs/999" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "priority": "high"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Printer or job not found",
  "code": "NOT_FOUND"
}
```

### 示例 8：未认证访问（401）

```bash
curl -X PUT "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Content-Type: application/json" \
  -d '{
    "priority": "high"
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. 参数验证

- 至少提供一个字段（priority 或 state）
- 优先级必须是有效值
- 状态必须是有效值

## 优先级说明

| 优先级 | 说明 |
|--------|------|
| `low` | 低优先级 |
| `normal` | 普通优先级 |
| `high` | 高优先级 |
| `urgent` | 紧急 |

## 打印任务状态说明

| 状态 | 说明 |
|------|------|
| `pending` | 等待打印 |
| `printing` | 正在打印 |
| `completed` | 已完成 |
| `failed` | 打印失败 |
| `canceled` | 已取消 |

## 实现文件

- `src/handlers/printers_jobs_update.rs` - 更新打印任务处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **参数要求**：至少提供一个字段（priority 或 state）
3. **权限控制**：用户只能更新自己的任务（实际实现应验证）

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `INVALID_PARAMS` | 400 | 参数无效 |
| `INVALID_PRIORITY` | 400 | 优先级无效 |
| `INVALID_STATE` | 400 | 状态无效 |
| `NOT_FOUND` | 404 | 打印机或任务不存在 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/printers** - 获取打印机列表 (Phase 111)
- **GET /api/v1/printers/{id}/jobs** - 获取打印机任务列表 (Phase 112)
- **GET /api/v1/printers/{id}/jobs/{job_id}** - 获取打印任务详情 (Phase 113)
- **POST /api/v1/printers/{id}/jobs** - 创建打印任务 (Phase 114)
- **DELETE /api/v1/printers/{id}/jobs/{job_id}** - 取消打印任务

## 响应示例（完整）

### 成功更新优先级

```json
{
  "success": true,
  "message": "Print job updated successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "user_id": 101,
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "status": "pending",
    "priority": "high",
    "submitted_at": "2026-03-26T10:00:00Z",
    "started_at": null,
    "completed_at": null,
    "error_message": null
  }
}
```

### 成功取消任务

```json
{
  "success": true,
  "message": "Print job updated successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "user_id": 101,
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "status": "canceled",
    "priority": "normal",
    "submitted_at": "2026-03-26T10:00:00Z",
    "started_at": null,
    "completed_at": null,
    "error_message": null
  }
}
```

### 参数无效（400）

```json
{
  "success": false,
  "error": "At least one field (priority or state) must be provided",
  "code": "INVALID_PARAMS"
}
```

### 优先级无效（400）

```json
{
  "success": false,
  "error": "Invalid priority. Valid values: low, normal, high, urgent",
  "code": "INVALID_PRIORITY"
}
```

### 状态无效（400）

```json
{
  "success": false,
  "error": "Invalid state. Valid values: pending, printing, completed, failed, canceled",
  "code": "INVALID_STATE"
}
```

## 最佳实践

### 1. 前端集成示例

```javascript
const updatePrintJob = async (printerId, jobId, updates) => {
  try {
    const response = await fetch(
      `/api/v1/printers/${printerId}/jobs/${jobId}`,
      {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify(updates)
      }
    );

    const data = await response.json();
    if (response.ok) {
      console.log('Print job updated:', data.data);
      return data.data;
    } else {
      console.error('Failed to update print job:', data.error);
      throw new Error(data.error);
    }
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
};

// 使用示例 - 更新优先级
updatePrintJob(1, 1, { priority: 'high' });

// 使用示例 - 取消任务
updatePrintJob(1, 1, { state: 'canceled' });
```

### 2. React 组件示例

```jsx
const UpdatePrintJob = ({ printerId, jobId, onSuccess }) => {
  const [formData, setFormData] = useState({
    priority: '',
    state: ''
  });
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!formData.priority && !formData.state) {
      alert('Please provide at least one field (priority or state)');
      return;
    }
    
    setSubmitting(true);
    try {
      const updates = {};
      if (formData.priority) updates.priority = formData.priority;
      if (formData.state) updates.state = formData.state;
      
      const response = await fetch(
        `/api/v1/printers/${printerId}/jobs/${jobId}`,
        {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token}`
          },
          body: JSON.stringify(updates)
        }
      );
      const data = await response.json();
      if (response.ok) {
        alert('Print job updated successfully!');
        onSuccess();
      } else {
        alert(`Failed: ${data.error}`);
      }
    } catch (error) {
      alert('Error updating print job');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <select
        value={formData.priority}
        onChange={(e) => setFormData({...formData, priority: e.target.value})}
      >
        <option value="">Priority (optional)</option>
        <option value="low">Low</option>
        <option value="normal">Normal</option>
        <option value="high">High</option>
        <option value="urgent">Urgent</option>
      </select>
      <select
        value={formData.state}
        onChange={(e) => setFormData({...formData, state: e.target.value})}
      >
        <option value="">State (optional)</option>
        <option value="pending">Pending</option>
        <option value="printing">Printing</option>
        <option value="completed">Completed</option>
        <option value="failed">Failed</option>
        <option value="canceled">Canceled</option>
      </select>
      <button type="submit" disabled={submitting}>
        {submitting ? 'Updating...' : 'Update Print Job'}
      </button>
    </form>
  );
};
```

### 3. 错误处理

```javascript
const handleUpdateJobError = (error) => {
  switch (error.code) {
    case 'INVALID_PARAMS':
      return 'Please provide at least one field (priority or state).';
    case 'INVALID_PRIORITY':
      return 'Invalid priority. Choose: low, normal, high, or urgent.';
    case 'INVALID_STATE':
      return 'Invalid state. Choose: pending, printing, completed, failed, or canceled.';
    case 'NOT_FOUND':
      return 'Printer or job not found.';
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    default:
      return 'Failed to update print job. Please try again.';
  }
};
```

### 4. 审计日志

所有更新打印任务操作都应该记录到审计日志中，包括：
- 更新时间
- 执行更新的用户 ID
- 打印机 ID
- 打印任务 ID
- 更新的字段（优先级/状态）
- 更新结果（成功/失败）
