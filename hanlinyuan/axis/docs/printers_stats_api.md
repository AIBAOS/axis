# 打印机统计信息 API 文档 (Phase 118)

## 概述

打印机统计信息 API 提供指定打印机的详细统计信息，包括任务数量、完成情况等。

## 接口详情

### GET /api/v1/printers/{printer_id}/stats

获取指定打印机的统计信息。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `printer_id` | integer | 是 | 打印机 ID |

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "data": {
    "printer_id": 1,
    "printer_name": "HP LaserJet Pro",
    "total_jobs": 150,
    "completed_jobs": 120,
    "failed_jobs": 10,
    "canceled_jobs": 15,
    "pending_jobs": 3,
    "printing_jobs": 2,
    "total_pages": 1500,
    "avg_completion_time_seconds": 300.5,
    "last_job_at": 1711468800
  }
}
```

**字段说明**

- `success`: 请求是否成功
- `data`: 打印机统计信息
  - `printer_id`: 打印机 ID
  - `printer_name`: 打印机名称
  - `total_jobs`: 总任务数
  - `completed_jobs`: 已完成任务数
  - `failed_jobs`: 失败任务数
  - `canceled_jobs`: 已取消任务数
  - `pending_jobs`: 等待中任务数
  - `printing_jobs`: 打印中任务数
  - `total_pages`: 总页数
  - `avg_completion_time_seconds`: 平均完成时间（秒）
  - `last_job_at`: 最后任务时间（Unix 时间戳）

**错误响应 (404 Not Found) - 打印机不存在**

```json
{
  "success": false,
  "error": "Printer 999 not found",
  "code": "NOT_FOUND"
}
```

**错误响应 (503 Service Unavailable) - CUPS 服务不可用**

```json
{
  "success": false,
  "error": "CUPS service is not available",
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

### 示例 1：获取打印机统计信息

```bash
curl -X GET "http://localhost:8080/api/v1/printers/1/stats" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": true,
  "data": {
    "printer_id": 1,
    "printer_name": "HP LaserJet Pro",
    "total_jobs": 150,
    "completed_jobs": 120,
    "failed_jobs": 10,
    "canceled_jobs": 15,
    "pending_jobs": 3,
    "printing_jobs": 2,
    "total_pages": 1500,
    "avg_completion_time_seconds": 300.5,
    "last_job_at": 1711468800
  }
}
```

### 示例 2：获取不存在的打印机统计（404）

```bash
curl -X GET "http://localhost:8080/api/v1/printers/999/stats" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Printer 999 not found",
  "code": "NOT_FOUND"
}
```

### 示例 3：CUPS 服务不可用（503）

```bash
curl -X GET "http://localhost:8080/api/v1/printers/1/stats" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "CUPS service is not available",
  "code": "SERVICE_UNAVAILABLE"
}
```

### 示例 4：未认证访问（401）

```bash
curl -X GET "http://localhost:8080/api/v1/printers/1/stats"
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

## 统计字段说明

| 字段 | 说明 |
|------|------|
| `total_jobs` | 总任务数（所有状态的任务总数） |
| `completed_jobs` | 已完成任务数（status = completed） |
| `failed_jobs` | 失败任务数（status = failed） |
| `canceled_jobs` | 已取消任务数（status = canceled） |
| `pending_jobs` | 等待中任务数（status = pending） |
| `printing_jobs` | 打印中任务数（status = printing） |
| `total_pages` | 总页数（所有任务的页数总和） |
| `avg_completion_time_seconds` | 平均完成时间（仅计算已完成任务） |
| `last_job_at` | 最后任务时间（最近一次任务提交时间） |

## 实现文件

- `src/handlers/printers_stats.rs` - 打印机统计信息处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **认证要求**：登录用户可访问
2. **CUPS 服务**：需要 CUPS 服务运行
3. **统计数据**：基于历史打印任务计算

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `NOT_FOUND` | 404 | 打印机不存在 |
| `SERVICE_UNAVAILABLE` | 503 | CUPS 服务不可用 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **GET /api/v1/printers** - 获取打印机列表 (Phase 111)
- **GET /api/v1/printers/{id}/jobs** - 获取打印机任务列表 (Phase 112)
- **GET /api/v1/printers/{id}/jobs/{job_id}** - 获取打印任务详情 (Phase 113)
- **POST /api/v1/printers/{id}/jobs** - 创建打印任务 (Phase 114)
- **PUT /api/v1/printers/{id}/jobs/{job_id}** - 更新打印任务 (Phase 115)
- **DELETE /api/v1/printers/{id}/jobs/{job_id}** - 取消打印任务 (Phase 116)

## 响应示例（完整）

### 成功响应

```json
{
  "success": true,
  "data": {
    "printer_id": 1,
    "printer_name": "HP LaserJet Pro",
    "total_jobs": 150,
    "completed_jobs": 120,
    "failed_jobs": 10,
    "canceled_jobs": 15,
    "pending_jobs": 3,
    "printing_jobs": 2,
    "total_pages": 1500,
    "avg_completion_time_seconds": 300.5,
    "last_job_at": 1711468800
  }
}
```

### 打印机不存在（404）

```json
{
  "success": false,
  "error": "Printer 999 not found",
  "code": "NOT_FOUND"
}
```

### CUPS 服务不可用（503）

```json
{
  "success": false,
  "error": "CUPS service is not available",
  "code": "SERVICE_UNAVAILABLE"
}
```

## 最佳实践

### 1. 前端集成示例

```javascript
const getPrinterStats = async (printerId) => {
  try {
    const response = await fetch(
      `/api/v1/printers/${printerId}/stats`,
      {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      }
    );

    const data = await response.json();
    if (response.ok) {
      console.log('Printer stats:', data.data);
      return data.data;
    } else {
      console.error('Failed to get printer stats:', data.error);
      throw new Error(data.error);
    }
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
};

// 使用示例
getPrinterStats(1);
```

### 2. React 组件示例

```jsx
const PrinterStats = ({ printerId }) => {
  const [stats, setStats] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchStats = async () => {
      try {
        const response = await fetch(
          `/api/v1/printers/${printerId}/stats`,
          {
            headers: {
              'Authorization': `Bearer ${token}`
            }
          }
        );
        const data = await response.json();
        if (response.ok) {
          setStats(data.data);
        }
      } catch (error) {
        console.error('Error:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchStats();
  }, [printerId]);

  if (loading) return <div>Loading...</div>;
  if (!stats) return <div>No stats available</div>;

  return (
    <div>
      <h3>Printer Statistics</h3>
      <p>Total Jobs: {stats.total_jobs}</p>
      <p>Completed: {stats.completed_jobs}</p>
      <p>Failed: {stats.failed_jobs}</p>
      <p>Canceled: {stats.canceled_jobs}</p>
      <p>Pending: {stats.pending_jobs}</p>
      <p>Printing: {stats.printing_jobs}</p>
      <p>Total Pages: {stats.total_pages}</p>
      <p>Avg Completion Time: {stats.avg_completion_time_seconds?.toFixed(1)}s</p>
    </div>
  );
};
```

### 3. 错误处理

```javascript
const handlePrinterStatsError = (error) => {
  switch (error.code) {
    case 'NOT_FOUND':
      return 'Printer not found.';
    case 'SERVICE_UNAVAILABLE':
      return 'Print service is unavailable. Please try again later.';
    case 'UNAUTHORIZED':
      return 'Authentication failed. Please login again.';
    default:
      return 'Failed to get printer stats. Please try again.';
  }
};
```

### 4. 审计日志

所有打印机统计信息查询操作都应该记录到审计日志中，包括：
- 查询时间
- 执行查询的用户 ID
- 打印机 ID
- 查询结果（成功/失败）
