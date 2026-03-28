# 打印机列表 API (Phase 126)

## 接口说明

实现获取打印机列表的接口。仅 admin 角色可访问，支持分页查询。

## 接口定义

```
GET /api/v1/printers
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（仅 admin 角色） |

## 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| page | integer | 否 | 1 | 页码 |
| per_page | integer | 否 | 20 | 每页数量 |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "printer_id": 1,
      "name": "Office Printer 1",
      "model": "HP LaserJet Pro M404n",
      "status": "idle",
      "ip_address": "192.168.1.101",
      "location": "Building A, Floor 2",
      "is_default": true
    },
    {
      "printer_id": 2,
      "name": "Office Printer 2",
      "model": "Canon imageRUNNER 2530i",
      "status": "printing",
      "ip_address": "192.168.1.102",
      "location": "Building A, Floor 3",
      "is_default": false
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 5,
    "total_pages": 1
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
  "error": "Only admin users can access printer list",
  "code": "FORBIDDEN"
}
```

## 使用示例

### cURL 示例

```bash
# 获取打印机列表（默认分页）
curl -X GET "http://localhost:8080/api/v1/printers" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 分页查询
curl -X GET "http://localhost:8080/api/v1/printers?page=1&per_page=10" \
  -H "Authorization: Bearer <admin_jwt_token>"

# 非 admin 用户访问（返回 403）
curl -X GET "http://localhost:8080/api/v1/printers" \
  -H "Authorization: Bearer <user_jwt_token>"
```

### JavaScript 示例

```javascript
// 获取打印机列表
async function getPrinters(page = 1, perPage = 20) {
  const params = new URLSearchParams({
    page: page.toString(),
    per_page: perPage.toString()
  });
  
  const response = await fetch(
    `http://localhost:8080/api/v1/printers?${params}`,
    {
      headers: {
        'Authorization': 'Bearer ' + adminToken
      }
    }
  );
  
  const data = await response.json();
  console.log('Printers:', data.data);
  console.log('Pagination:', data.pagination);
  return data.data;
}

// 使用示例
const printers = await getPrinters();
printers.data.forEach(printer => {
  console.log(`${printer.name}: ${printer.status} (${printer.model})`);
});
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

### PaginationInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| page | integer | 当前页码 |
| per_page | integer | 每页数量 |
| total | integer | 总记录数 |
| total_pages | integer | 总页数 |

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
3. **分页支持**: 防止大量数据一次性返回

## 实现文件

- `src/handlers/printers_list.rs` - 打印机列表处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟实现，后续将连接实际系统
2. 仅 admin 角色可访问
3. 分页参数可选，有默认值

## 相关接口

- `GET /api/v1/printers/{id}` - 打印机详情
- `POST /api/v1/printers/{id}/jobs` - 创建打印任务
- `GET /api/v1/network/interfaces` - 网络接口列表（Phase 124）
