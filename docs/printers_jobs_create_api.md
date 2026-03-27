# 创建打印任务 API 文档 (Phase 114)

## 概述

创建打印任务 API 允许用户向指定打印机提交打印任务。

## 接口详情

### POST /api/v1/printers/{printer_id}/jobs

创建新的打印任务。

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

#### 请求体

```json
{
  "document_name": "report.pdf",
  "pages": 5,
  "copies": 2,
  "priority": "normal",
  "submitted_at": "2026-03-26T10:00:00Z"
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `document_name` | string | 是 | - | 文档名称 |
| `pages` | integer | 是 | - | 页数（必须 > 0） |
| `copies` | integer | 是 | - | 份数（必须 > 0） |
| `priority` | string | 否 | `normal` | 优先级（low/normal/high/urgent） |
| `submitted_at` | string | 否 | 当前时间 | 提交时间（ISO 8601） |

#### 响应格式

**成功响应 (201 Created)**

```json
{
  "success": true,
  "message": "Print job created successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "user_id": 101,
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "status": "pending",
    "priority": "normal",
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
  "error": "Document name cannot be empty",
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

**错误响应 (503 Service Unavailable) - CUPS 服务不可用**

```json
{
  "success": false,
  "error": "CUPS service is unavailable",
  "code": "SERVICE_UNAVAILABLE"
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

### 示例 1：创建打印任务

```bash
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "priority": "normal"
  }'
```

**响应：**
```json
{
  "success": true,
  "message": "Print job created successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "user_id": 101,
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "status": "pending",
    "priority": "normal",
    "submitted_at": "2026-03-26T10:00:00Z",
    "started_at": null,
    "completed_at": null,
    "error_message": null
  }
}
```

### 示例 2：创建高优先级任务

```bash
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "urgent_report.pdf",
    "pages": 10,
    "copies": 1,
    "priority": "urgent"
  }'
```

### 示例 3：文档名称为空（400）

```bash
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "",
    "pages": 5,
    "copies": 2
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Document name cannot be empty",
  "code": "INVALID_PARAMS"
}
```

### 示例 4：页数无效（400）

```bash
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "report.pdf",
    "pages": 0,
    "copies": 2
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "Pages must be greater than 0",
  "code": "INVALID_PARAMS"
}
```

### 示例 5：优先级无效（400）

```bash
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
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

### 示例 6：CUPS 服务不可用（503）

```bash
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2
  }'
```

**响应：**
```json
{
  "success": false,
  "error": "CUPS service is unavailable",
  "code": "SERVICE_UNAVAILABLE"
}
```

### 示例 7：未认证访问（401）

```bash
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2
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

### 2. CUPS 服务连接

- 验证 CUPS 服务连接状态
- 服务不可用返回 503 Service Unavailable

### 3. 参数验证

- 文档名称不能为空
- 页数必须大于 0
- 份数必须大于 0
- 优先级必须是有效值

## 优先级说明

| 优先级 | 说明 |
|--------|------|
| `low` | 低优先级 |
| `normal` | 普通优先级（默认） |
| `high` | 高优先级 |
| `urgent` | 紧急 |

## 打印任务状态说明

| 状态 | 说明 |
|------|------|
| `pending` | 等待打印（新创建的任务） |
| `printing` | 正在打印 |
| `completed` | 已完成 |
| `failed` | 打印失败 |
| `canceled` | 已取消 |

## 实现文件

- `src/handlers/printers_jobs_create.rs` - 创建打印任务处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **CUPS 服务**：需要 CUPS 服务运行
3. **参数验证**：必须提供有效的文档名称、页数、份数
4. **优先级**：可选，默认为 normal

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `INVALID_PARAMS` | 400 | 参数无效 |
| `INVALID_PRIORITY` | 400 | 优先级无效 |
| `SERVICE_UNAVAILABLE` | 503 | CUPS 服务不可用 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/printers** - 获取打印机列表 (Phase 111)
- **GET /api/v1/printers/{id}/jobs** - 获取打印机任务列表 (Phase 112)
- **GET /api/v1/printers/{id}/jobs/{job_id}** - 获取打印任务详情 (Phase 113)
- **DELETE /api/v1/printers/{id}/jobs/{job_id}** - 取消打印任务

## 响应示例（完整）

### 成功创建

```json
{
  "success": true,
  "message": "Print job created successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "user_id": 101,
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 2,
    "status": "pending",
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
  "error": "Document name cannot be empty",
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

## 最佳实践

### 1. 前端集成示例

```javascript
const createPrintJob = async (printerId, jobData) => {
  try {
    const response = await fetch(
      `/api/v1/printers/${printerId}/jobs`,
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify(jobData)
      }
    );

    const data = await response.json();
    if (response.ok) {
      console.log('Print job created:', data.data);
      return data.data;
    } else {
      console.error('Failed to create print job:', data.error);
      throw new Error(data.error);
    }
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
};

// 使用示例
createPrintJob(1, {
  document_name: 'report.pdf',
  pages: 5,
  copies: 2,
  priority: 'normal'
});
```

### 2. React 组件示例

```jsx
const CreatePrintJob = ({ printerId, onSuccess }) => {
  const [formData, setFormData] = useState({
    document_name: '',
    pages: 1,
    copies: 1,
    priority: 'normal'
  });
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setSubmitting(true);
    try {
      const response = await fetch(
        `/api/v1/printers/${printerId}/jobs`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token}`
          },
          body: JSON.stringify(formData)
        }
      );
      const data = await response.json();
      if (response.ok) {
        alert('Print job created successfully!');
        onSuccess();
      } else {
        alert(`Failed: ${data.error}`);
      }
    } catch (error) {
      alert('Error creating print job');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <input
        type="text"
        value={formData.document_name}
        onChange={(e) => setFormData({...formData, document_name: e.target.value})}
        placeholder="Document name"
        required
      />
      <input
        type="number"
        value={formData.pages}
        onChange={(e) => setFormData({...formData, pages: parseInt(e.target.value)})}
        min="1"
        placeholder="Pages"
        required
      />
      <input
        type="number"
        value={formData.copies}
        onChange={(e) => setFormData({...formData, copies: parseInt(e.target.value)})}
        min="1"
        placeholder="Copies"
        required
      />
      <select
        value={formData.priority}
        onChange={(e) => setFormData({...formData, priority: e.target.value})}
      >
        <option value="low">Low</option>
        <option value="normal">Normal</option>
        <option value="high">High</option>
        <option value="urgent">Urgent</option>
      </select>
      <button type="submit" disabled={submitting}>
        {submitting ? 'Creating...' : 'Create Print Job'}
      </button>
    </form>
  );
};
```

### 3. 错误处理

```javascript
const handleCreateJobError = (error) => {
  switch (error.code) {
    case 'INVALID_PARAMS':
      return 'Please check your input parameters.';
    case 'INVALID_PRIORITY':
      return 'Invalid priority. Choose: low, normal, high, or urgent.';
    case 'SERVICE_UNAVAILABLE':
      return 'Print service is unavailable. Please try again later.';
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    default:
      return 'Failed to create print job. Please try again.';
  }
};
```

### 4. 审计日志

所有创建打印任务操作都应该记录到审计日志中，包括：
- 创建时间
- 执行创建的用户 ID
- 打印机 ID
- 文档名称
- 页数和份数
- 优先级
- 创建结果（成功/失败）
