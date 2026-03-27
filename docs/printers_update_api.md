# 更新打印机 API (Phase 128)

## 接口说明

实现更新打印机信息的接口。仅 admin 角色可访问。

## 接口定义

```
PUT /api/v1/printers/{printer_id}
```

## 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| printer_id | integer | 是 | 打印机 ID |

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |
| Content-Type | application/json | 是 | 请求体格式 |

## 请求体

```json
{
  "name": "Updated Printer Name",
  "model": "New Model",
  "status": "idle",
  "ip_address": "192.168.1.200",
  "location": "New Location",
  "is_default": false
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 否 | 打印机名称 |
| model | string | 否 | 打印机型号 |
| status | string | 否 | 状态（idle/printing/error/offline） |
| ip_address | string | 否 | IP 地址 |
| location | string | 否 | 物理位置 |
| is_default | boolean | 否 | 是否为默认打印机 |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "message": "Printer updated successfully",
  "data": {
    "printer_id": 1,
    "name": "Updated Printer Name",
    "model": "New Model",
    "status": "idle",
    "ip_address": "192.168.1.200",
    "location": "New Location",
    "is_default": false
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

### 禁止访问 (403 Forbidden)

```json
{
  "success": false,
  "error": "Only admin users can update printers",
  "code": "FORBIDDEN"
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

### IP 地址格式错误 (400 Bad Request)

```json
{
  "success": false,
  "error": "Invalid IP address format",
  "code": "INVALID_IP"
}
```

### 状态值错误 (400 Bad Request)

```json
{
  "success": false,
  "error": "Invalid status. Valid values: idle, printing, error, offline",
  "code": "INVALID_STATUS"
}
```

## 使用示例

### cURL 示例

```bash
# 更新打印机名称
curl -X PUT "http://localhost:8080/api/v1/printers/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Printer Name"
  }'

# 更新 IP 地址
curl -X PUT "http://localhost:8080/api/v1/printers/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "ip_address": "192.168.1.200"
  }'

# 更新多个字段
curl -X PUT "http://localhost:8080/api/v1/printers/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Name",
    "location": "New Location",
    "status": "offline",
    "is_default": false
  }'

# 无效 IP 地址（返回 400）
curl -X PUT "http://localhost:8080/api/v1/printers/1" \
  -H "Authorization: Bearer <admin_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "ip_address": "999.999.999.999"
  }'

# 非 admin 用户访问（返回 403）
curl -X PUT "http://localhost:8080/api/v1/printers/1" \
  -H "Authorization: Bearer <user_jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Name"
  }'
```

### JavaScript 示例

```javascript
// 更新打印机
async function updatePrinter(printerId, updates) {
  const response = await fetch(
    `http://localhost:8080/api/v1/printers/${printerId}`,
    {
      method: 'PUT',
      headers: {
        'Authorization': 'Bearer ' + adminToken,
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
  console.log('Printer updated:', data.data);
  return data.data;
}

// 使用示例
try {
  await updatePrinter(1, {
    name: 'New Printer Name',
    ip_address: '192.168.1.200'
  });
  console.log('Update successful');
} catch (e) {
  console.error('Update failed:', e.message);
}
```

## 响应字段说明

### PrinterInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| printer_id | integer | 打印机 ID |
| name | string | 打印机名称 |
| model | string | 打印机型号 |
| status | string | 状态（idle/printing/error/offline） |
| ip_address | string | IP 地址 |
| location | string | 物理位置 |
| is_default | boolean | 是否为默认打印机 |

## 验证规则

### IP 地址格式
- 必须是有效的 IPv4 格式（x.x.x.x）
- 每段 0-255

### 状态值
- `idle`: 空闲
- `printing`: 正在打印
- `error`: 错误
- `offline`: 离线

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **IP 格式验证**: 验证 IP 地址格式合法性
4. **状态值验证**: 验证状态值是否在有效范围内

## 实现文件

- `src/handlers/printers_update.rs` - 打印机更新处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际系统
2. 仅 admin 角色可访问
3. 所有字段均为可选，只更新提供的字段
4. IP 地址格式必须合法

## 相关接口

- `GET /api/v1/printers` - 打印机列表（Phase 126）
- `GET /api/v1/printers/{id}` - 打印机详情（Phase 127）
- `DELETE /api/v1/printers/{id}` - 删除打印机
- `GET /api/v1/network/interfaces` - 网络接口列表（Phase 124）
