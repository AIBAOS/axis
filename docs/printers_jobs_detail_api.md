# 打印机任务详情 API

**Phase 113** - 打印机管理 API 之获取打印任务详情接口

---

## 接口信息

- **端点:** `GET /api/v1/printers/{printer_id}/jobs/{job_id}`
- **认证:** 需要 JWT Bearer Token（登录用户）
- **权限:** 所有已认证用户可访问
- **内容类型:** `application/json`

---

## 请求

### 请求头

|  Header  | 必需 | 说明 |
|----------|------|------|
| `Authorization` | 是 | `Bearer <JWT_TOKEN>` |

### 路径参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `printer_id` | integer | 是 | 打印机 ID |
| `job_id` | integer | 是 | 打印任务 ID |

### 请求示例

```bash
# 获取打印任务详情
curl -X GET "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <JWT_TOKEN>"
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "data": {
    "id": 1,
    "printer_id": 1,
    "document_name": "report.pdf",
    "user": "admin",
    "pages": 5,
    "copies": 1,
    "state": "completed",
    "priority": 0,
    "submitted_at": 1711440000,
    "completed_at": 1711440300,
    "error_message": null
  }
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

### 404 Not Found - 资源不存在

```json
{
  "success": false,
  "error": "Printer 999 not found",
  "code": "NOT_FOUND"
}
```

或

```json
{
  "success": false,
  "error": "Print job 999 not found for printer 1",
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

---

## 响应字段说明

### PrintJobDetailResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `success` | boolean | 请求是否成功 |
| `data` | object | 打印任务详情 |

### PrintJobDetail

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

- **文件位置:** `src/handlers/printers_jobs_detail.rs`
- **路由注册:** `src/main.rs` - `GET /api/v1/printers/{printer_id}/jobs/{job_id}`
- **依赖:**
  - `jsonwebtoken` - JWT 验证
  - CUPS - 打印服务（模拟）

---

## 相关接口

- `GET /api/v1/printers` - 打印机列表（Phase 111）
- `GET /api/v1/printers/{id}/jobs` - 打印机任务列表（Phase 112）
- `POST /api/v1/printers/{id}/jobs` - 创建打印任务
- `DELETE /api/v1/printers/{id}/jobs/{job_id}` - 取消打印任务

---

## 变更日志

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-03-26 | 1.0 | Phase 113 初始实现 |
