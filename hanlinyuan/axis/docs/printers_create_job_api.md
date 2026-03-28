# 创建打印任务 API

**Phase 114** - 打印机管理 API 之创建打印任务接口

---

## 接口信息

- **端点:** `POST /api/v1/printers/{printer_id}/jobs`
- **认证:** 需要 JWT Bearer Token（登录用户）
- **权限:** 所有已认证用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |
| `Content-Type` | 是 | `application/json` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `printer_id` | integer | 是 | 打印机 ID |

### 请求体

```json
{
  "document_name": "string",
  "pages": "integer",
  "copies": "integer",
  "priority": "integer (可选，默认 5)",
  "submitted_at": "integer (可选)"
}
```

### 参数说明

| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `document_name` | string | 是 | - | 文档名称 |
| `pages` | integer | 是 | - | 页数（必须 > 0） |
| `copies` | integer | 是 | - | 份数（必须 > 0） |
| `priority` | integer | 否 | 5 | 优先级（0-10，0 最高） |
| `submitted_at` | integer | 否 | 当前时间 | 提交时间（Unix 时间戳） |

### 请求示例

```bash
# 创建打印任务
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "report.pdf",
    "pages": 5,
    "copies": 1,
    "priority": 5
  }'

# 创建高优先级打印任务
curl -X POST "http://localhost:8080/api/v1/printers/1/jobs" \
  -H "Authorization: Bearer <JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "document_name": "urgent.docx",
    "pages": 10,
    "copies": 3,
    "priority": 0
  }'
```

---

## 响应

### 201 Created - 创建成功

```json
{
  "success": true,
  "message": "Print job created successfully",
  "data": {
    "id": 11,
    "printer_id": 1,
    "document_name": "report.pdf",
    "user": "admin",
    "pages": 5,
    "copies": 1,
    "state": "pending",
    "priority": 5,
    "submitted_at": 1711526400,
    "completed_at": null,
    "error_message": null
  }
}
```

### 400 Bad Request - 参数错误

```json
{
  "success": false,
  "error": "document_name is required",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "pages must be greater than 0",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "priority must be between 0 and 10",
  "code": "INVALID_PARAMS"
}
```

或

```json
{
  "success": false,
  "error": "Printer 'Brother MFC' is in error state",
  "code": "PRINTER_ERROR"
}
```

### 401 Unauthorized - 未认证

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

### 404 Not Found - 打印机不存在

```json
{
  "success": false,
  "error": "Printer 999 not found",
  "code": "NOT_FOUND"
}
```

### 503 Service Unavailable - CUPS 服务不可用

```json
{
  "success": false,
  "error": "CUPS service is not available",
  "code": "SERVICE_UNAVAILABLE"
}
```

---

## 安全说明

1. **JWT 认证**: 必须提供有效的 JWT Bearer Token
2. **CUPS 服务**: 依赖 CUPS 打印服务，服务不可用时返回 503
3. **打印机状态**: 错误状态的打印机不能接收新任务

---

## 响应字段说明

### CreatePrintJobResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `message` | string | 响应消息 |
| `data` | object | 打印任务信息 |

### PrintJobInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | integer | 打印任务 ID |
| `printer_id` | integer | 打印机 ID |
| `document_name` | string | 文档名称 |
| `user` | string | 提交用户 |
| `pages` | integer | 页数 |
| `copies` | integer | 份数 |
| `state` | string | 状态：pending/printing/completed/failed/canceled |
| `priority` | integer | 优先级（0-10，0 最高） |
| `submitted_at` | integer | 提交时间（Unix 时间戳） |
| `completed_at` | integer\|null | 完成时间（Unix 时间戳） |
| `error_message` | string\|null | 错误信息（失败时） |

---

## 打印任务状态说明

| 状态 | 说明 |
|------|------|
| `pending` | 等待打印 |
| `printing` | 正在打印 |
| `completed` | 打印完成 |
| `failed` | 打印失败 |
| `canceled` | 已取消 |

---

## 实现细节

- **文件位置:** `src/handlers/printers_create_job.rs`
- **路由注册:** `src/main.rs` - `POST /api/v1/printers/{printer_id}/jobs`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - CUPS - 打印服务（模拟）

---

## 相关接口

- `GET /api/v1/printers` - 打印机列表（Phase 111）
- `GET /api/v1/printers/{id}/jobs` - 打印机任务列表（Phase 112）
- `GET /api/v1/printers/{id}/jobs/{job_id}` - 打印机任务详情（Phase 113）
- `DELETE /api/v1/printers/{id}/jobs/{job_id}` - 取消打印任务

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 114 初始实现 |
