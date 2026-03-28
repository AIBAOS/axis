# 打印机详情 API (Phase 127)

## 接口说明

实现获取单个打印机详情的接口。仅 admin 角色可访问。

## 接口定义

```
GET /api/v1/printers/{printer_id}
```

## 路径参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| printer_id | integer | 是 | 打印机 ID |

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "printer_id": 1,
    "name": "Office Printer 1",
    "model": "HP LaserJet Pro M404n",
    "status": "idle",
    "ip_address": "192.168.1.101",
    "location": "Building A, Floor 2",
    "is_default": true,
    "capabilities": ["duplex", "color", "wifi"]
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
  "error": "Only admin users can access printer details",
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

## 使用示例

### cURL 示例

```bash
# 获取打印机详情
curl -X GET "http://localhost:8080/api/v1/printers/1" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 获取不存在的打印机（返回 404）
curl -X GET "http://localhost:8080/api/v1/printers/999" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 非 admin 用户访问（返回 403）
curl -X GET "http://localhost:8080/api/v1/printers/1" \
  -H "Authorization: Bearer <user_jwt_token>"
```

### JavaScript 示例

```javascript
// 获取打印机详情
async function getPrinterDetail(printerId) {
  const response = await fetch(
    `http://localhost:8080/api/v1/printers/${printerId}`,
    {
      headers: {
        'Authorization': 'Bearer ' + adminToken
      }
    }
  );
  
  const data = await response.json();
  console.log('Printer detail:', data.data);
  return data.data;
}

// 使用示例
const printer = await getPrinterDetail(1);
console.log(`${printer.name}: ${printer.model}`);
console.log(`Status: ${printer.status}, IP: ${printer.ip_address}`);
console.log(`Capabilities: ${printer.capabilities?.join(', ')}`);
```

## 响应字段说明

### PrinterDetail

| 字段 | 类型 | 说明 |
|------|------|------|
| printer_id | integer | 打印机 ID |
| name | string | 打印机名称 |
| model | string | 打印机型号 |
| status | string | 状态（idle/printing/error/offline） |
| ip_address | string | IP 地址 |
| location | string | 物理位置 |
| is_default | boolean | 是否为默认打印机 |
| capabilities | array/null | 功能列表（duplex/color/wifi/scan 等） |

## 打印机状态说明

| 状态 | 说明 |
|------|------|
| idle | 空闲，等待任务 |
| printing | 正在打印 |
| error | 错误状态（缺纸/卡纸等） |
| offline | 离线 |

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **Admin 权限**: 仅 admin 角色用户可访问
3. **存在性验证**: 验证打印机是否存在

## 实现文件

- `src/handlers/printers_get.rs` - 打印机详情处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际系统
2. 仅 admin 角色可访问
3. 打印机不存在返回 404 Not Found

## 相关接口

- `GET /api/v1/printers` - 打印机列表（Phase 126）
- `POST /api/v1/printers/{id}/jobs` - 创建打印任务
- `GET /api/v1/network/interfaces` - 网络接口列表（Phase 124）
