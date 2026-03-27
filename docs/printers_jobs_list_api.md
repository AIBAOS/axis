# 打印机任务列表 API 文档 (Phase 112)

## 概述

打印机任务列表 API 提供指定打印机的打印任务队列信息。

## 接口详情

### GET /api/v1/printers/{id}/jobs

获取指定打印机的打印任务列表。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | integer | 是 | 打印机 ID |

#### 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `page` | integer | 否 | 1 | 页码（从 1 开始） |
| `page_size` | integer | 否 | 20 | 每页数量 |
| `status` | string | 否 | - | 状态过滤（pending/printing/completed/failed/canceled） |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "printer_id": 1,
      "user_id": 101,
      "document_name": "report.pdf",
      "pages": 5,
      "copies": 2,
      "status": "printing",
      "priority": "normal",
      "submitted_at": "2026-03-26T10:00:00Z",
      "started_at": "2026-03-26T10:05:00Z",
      "completed_at": null
    },
    {
      "id": 2,
      "printer_id": 1,
      "user_id": 102,
      "document_name": "invoice.pdf",
      "pages": 2,
      "copies": 1,
      "status": "pending",
      "priority": "normal",
      "submitted_at": "2026-03-26T10:10:00Z",
      "started_at": null,
      "completed_at": null
    }
  ],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total": 2,
    "total_pages": 1
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `data`: 打印任务列表
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
- `pagination`: 分页信息
  - `page`: 当前页码
  - `page_size`: 每页数量
  - `total`: 总记录数
  - `total_pages`: 总页数

**错误响应 (404 Not Found) - 打印机不存在**

```json
{
  "success": false,
  "error": "Printer not found",
  "code": "NOT_FOUND"
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

### 示例 1：获取打印机任务列表

```bash
curl -X GET "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 2：按状态筛选

```bash
# 只查询正在打印的任务
curl -X GET "http://localhost:8080/api/v1/printers/1/jobs?status=printing" \
  -H "Authorization: Bearer <your_jwt_token>"

# 只查询等待中的任务
curl -X GET "http://localhost:8080/api/v1/printers/1/jobs?status=pending" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 3：分页查询

```bash
curl -X GET "http://localhost:8080/api/v1/printers/1/jobs?page=2&page_size=10" \
  -H "Authorization: Bearer <your_jwt_token>"
```

### 示例 4：获取不存在的打印机任务（404）

```bash
curl -X GET "http://localhost:8080/api/v1/printers/999/jobs" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Printer not found",
  "code": "NOT_FOUND"
}
```

### 示例 5：CUPS 服务不可用（503）

```bash
curl -X GET "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "CUPS service is unavailable",
  "code": "SERVICE_UNAVAILABLE"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. CUPS 服务连接

- 验证 CUPS 服务连接状态
- 服务不可用返回 503 Service Unavailable

## 打印任务状态说明

| 状态 | 说明 |
|------|------|
| `pending` | 等待打印 |
| `printing` | 正在打印 |
| `completed` | 已完成 |
| `failed` | 打印失败 |
| `canceled` | 已取消 |

## 优先级说明

| 优先级 | 说明 |
|--------|------|
| `low` | 低优先级 |
| `normal` | 普通优先级 |
| `high` | 高优先级 |
| `urgent` | 紧急 |

## 实现文件

- `src/handlers/printers_jobs_list.rs` - 打印机任务列表处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **CUPS 服务**：需要 CUPS 服务运行
3. **分页限制**：page_size 有最大值限制

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `NOT_FOUND` | 404 | 打印机不存在 |
| `SERVICE_UNAVAILABLE` | 503 | CUPS 服务不可用 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/printers** - 获取打印机列表 (Phase 111)
- **GET /api/v1/printers/{id}** - 获取打印机详情
- **POST /api/v1/printers/{id}/jobs** - 创建打印任务
- **DELETE /api/v1/printers/{id}/jobs/{job_id}** - 取消打印任务

## 响应示例（完整）

### 成功响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "printer_id": 1,
      "user_id": 101,
      "document_name": "report.pdf",
      "pages": 5,
      "copies": 2,
      "status": "printing",
      "priority": "normal",
      "submitted_at": "2026-03-26T10:00:00Z",
      "started_at": "2026-03-26T10:05:00Z",
      "completed_at": null
    }
  ],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total": 1,
    "total_pages": 1
  }
}
```

### 打印机不存在（404）

```json
{
  "success": false,
  "error": "Printer not found",
  "code": "NOT_FOUND"
}
```

### CUPS 服务不可用（503）

```json
{
  "success": false,
  "error": "CUPS service is unavailable",
  "code": "SERVICE_UNAVAILABLE"
}
```

## 最佳实践

### 1. 前端集成示例

```javascript
const getPrinterJobs = async (printerId, filters = {}) => {
  try {
    const params = new URLSearchParams(filters);
    const response = await fetch(
      `/api/v1/printers/${printerId}/jobs?${params}`,
      {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      }
    );

    const data = await response.json();
    if (response.ok) {
      console.log('Printer jobs:', data.data);
      return data.data;
    } else {
      console.error('Failed to get printer jobs:', data.error);
      throw new Error(data.error);
    }
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
};

// 使用示例
getPrinterJobs(1, { status: 'pending' });
```

### 2. React 组件示例

```jsx
const PrinterJobList = ({ printerId }) => {
  const [jobs, setJobs] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchJobs = async () => {
      try {
        const response = await fetch(
          `/api/v1/printers/${printerId}/jobs`,
          {
            headers: {
              'Authorization': `Bearer ${token}`
            }
          }
        );
        const data = await response.json();
        if (response.ok) {
          setJobs(data.data);
        }
      } catch (error) {
        console.error('Error:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchJobs();
  }, [printerId]);

  if (loading) return <div>Loading...</div>;

  return (
    <div>
      <h3>Print Queue</h3>
      {jobs.map(job => (
        <div key={job.id}>
          <p>{job.document_name} - {job.status}</p>
          <p>Pages: {job.pages}, Copies: {job.copies}</p>
        </div>
      ))}
    </div>
  );
};
```

### 3. 错误处理

```javascript
const handlePrinterJobError = (error) => {
  switch (error.code) {
    case 'NOT_FOUND':
      return 'Printer not found.';
    case 'SERVICE_UNAVAILABLE':
      return 'CUPS service is unavailable. Please try again later.';
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    default:
      return 'Failed to get printer jobs. Please try again.';
  }
};
```

### 4. 状态过滤

```javascript
// 只获取等待中的任务
getPrinterJobs(1, { status: 'pending' });

// 只获取正在打印的任务
getPrinterJobs(1, { status: 'printing' });
```

### 5. 审计日志

所有打印机任务列表查询操作都应该记录到审计日志中，包括：
- 查询时间
- 执行查询的用户 ID
- 打印机 ID
- 查询参数（过滤条件）
- 返回结果数量
- 查询结果（成功/失败）
