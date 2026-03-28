# 取消打印任务 API (Phase 116)

## 接口说明

实现取消/删除打印任务的接口。登录用户可访问，用户只能取消自己的任务（admin 除外）。

## 接口定义

```
DELETE /api/v1/printers/{printer_id}/jobs/{job_id}
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

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Print job canceled successfully",
  "data": {
    "id": 1,
    "printer_id": 1,
    "document_name": "report.pdf"
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

### 无权限取消 (403 Forbidden)

```json
{
  "success": false,
  "error": "You can only cancel your own print jobs",
  "code": "FORBIDDEN"
}
```

### CUPS 服务不可用 (503 Service Unavailable)

```json
{
  "success": false,
  "error": "CUPS service is unavailable",
  "code": "SERVICE_UNAVAILABLE"
}
```

## 使用示例

### cURL 示例

```bash
# 取消自己的打印任务
curl -X DELETE "http://localhost:8080/api/v1/printers/1/jobs/1" \
  -H "Authorization: Bearer <jwt_token>"

# Admin 取消任意用户的任务
curl -X DELETE "http://localhost:8080/api/v1/printers/1/jobs/2" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 取消不存在的打印机任务（返回 404）
curl -X DELETE "http://localhost:8080/api/v1/printers/999/jobs/1" \
  -H "Authorization: Bearer <jwt_token>"

# 取消不存在的任务（返回 404）
curl -X DELETE "http://localhost:8080/api/v1/printers/1/jobs/999" \
  -H "Authorization: Bearer <jwt_token>"

# 普通用户取消他人任务（返回 403）
curl -X DELETE "http://localhost:8080/api/v1/printers/1/jobs/2" \
  -H "Authorization: Bearer <user_jwt_token>"
```

### JavaScript 示例

```javascript
// 取消打印任务
async function cancelJob(printerId, jobId) {
  const response = await fetch(
    `http://localhost:8080/api/v1/printers/${printerId}/jobs/${jobId}`,
    {
      method: 'DELETE',
      headers: {
        'Authorization': 'Bearer ' + token
      }
    }
  );
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error);
  }
  
  const data = await response.json();
  console.log('Job canceled:', data.data);
  return data.data;
}

// 使用示例
try {
  const result = await cancelJob(1, 1);
  console.log(`Job ${result.id} for ${result.document_name} canceled`);
} catch (e) {
  console.error('Cancel failed:', e.message);
}
```

## 响应字段说明

### DeletedJobInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 任务 ID |
| printer_id | integer | 打印机 ID |
| document_name | string | 文档名称 |

## 权限说明

| 用户类型 | 可取消任务 |
|----------|-----------|
| 普通用户 | 仅自己的任务 |
| admin | 任意用户任务 |

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **登录用户可访问**: 无需 admin 权限（普通用户可取消自己的任务）
3. **权限隔离**: 普通用户只能取消自己的任务
4. **Admin 特权**: admin 可取消任意用户任务
5. **打印机存在性验证**: 防止操作不存在打印机
6. **任务存在性验证**: 防止操作不存在任务
7. **CUPS 服务验证**: 检查 CUPS 打印服务连接状态

## 实现文件

- `src/handlers/printers_delete_job.rs` - 删除任务处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际 CUPS 服务
2. 普通用户只能取消自己的任务
3. Admin 用户可以取消任意用户任务
4. 删除操作不可逆，请谨慎操作

## 相关接口

- `GET /api/v1/printers` - 打印机列表（Phase 111）
- `GET /api/v1/printers/{id}` - 打印机详情（Phase 114）
- `GET /api/v1/printers/{id}/jobs` - 打印机任务列表（Phase 112）
- `GET /api/v1/printers/{id}/jobs/{job_id}` - 打印任务详情（Phase 113）
- `POST /api/v1/printers/{id}/jobs` - 创建打印任务（Phase 114）
- `PUT /api/v1/printers/{id}/jobs/{job_id}` - 更新打印任务（Phase 115）

## 打印机管理模块完成状态

| Phase | 接口 | 状态 |
|-------|------|------|
| 111 | GET /api/v1/printers | ✅ |
| 112 | GET /api/v1/printers/{id}/jobs | ✅ |
| 113 | GET /api/v1/printers/{id}/jobs/{job_id} | ✅ |
| 114 | POST /api/v1/printers/{id}/jobs | ✅ |
| 115 | PUT /api/v1/printers/{id}/jobs/{job_id} | ✅ |
| 116 | DELETE /api/v1/printers/{id}/jobs/{job_id} | ✅ |
