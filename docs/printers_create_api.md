# Phase 55 - 创建打印机 API 文档

## 端点信息

- **方法**: `POST`
- **路径**: `/api/v1/printers`
- **认证**: Bearer Token (需 admin 角色)
- **功能**: 创建新打印机（支持 network/usb/virtual 三种类型）（支持 network/usb/virtual 三种类型）

---

## 请求参数

### 请求体 (JSON Body)

```json
{
  "name": "HP LaserJet Pro",
  "type": "network",
  "model": "HP LaserJet Pro MFP M125nw",
  "manufacturer": "HP",
  "ip_address": "192.168.1.101",
  "port": 9100,
  "location": "一楼办公区",
  "is_default": true,
  "capabilities": {
    "color": false,
    "duplex": true,
    "staple": false,
    "scanning": true,
    "fax": false
  }
}
```

### 请求字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 是 | 打印机名称 |
| type | string | 是 | 类型：network/usb/virtual |
| model | string | 否 | 型号 |
| manufacturer | string | 否 | 制造商 |
| ip_address | string | 条件 | network 类型必填 |
| port | number | 否 | 端口 (默认 9100) |
| usb_device | string | 条件 | USB 类型必填 |
| location | string | 否 | 物理位置 |
| is_default | boolean | 否 | 是否为默认打印机 |
| capabilities | object | 否 | 打印能力 |

### Printer Type 选项
- `network` - 网络打印机（需 `ip_address`）
- `usb` - USB 打印机（需 `usb_device`）
- `virtual` - 虚拟打印机

---

## 响应格式

### 成功响应 (201 Created)
```json
{
  "success": true,
  "message": "Printer 'HP LaserJet Pro' created successfully",
  "data": {
    "id": 5,
    "name": "HP LaserJet Pro",
    "model": "HP LaserJet Pro MFP M125nw",
    "manufacturer": "HP",
    "type": "network",
    "status": "idle",
    "ip_address": "192.168.1.101",
    "port": 9100,
    "location": "一楼办公区",
    "is_default": true,
    "capabilities": {
      "color": false,
      "duplex": true,
      "staple": false,
      "scanning": true,
      "fax": false
    },
    "created_at": "2026-03-26T05:37:00Z",
    "updated_at": "2026-03-26T05:37:00Z"
  }
}
```

### 错误响应

#### 400 Bad Request - 参数错误
```json
{
  "success": false,
  "message": "ip_address is required for network printer"
}
```

#### 401 Unauthorized
```json
{
  "success": false,
  "message": "Only admin users can create printers"
}
```

#### 403 Forbidden
```json
{
  "success": false,
  "message": "Only admin users can create printers"
}
```

#### 409 Conflict
```json
{
  "success": false,
  "message": "Printer 'name' already exists"
}
```

---

## 使用示例

### cURL 示例
```bash
# 创建网络打印机
curl -X POST http://localhost:8080/api/v1/printers \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "HP LaserJet Pro",
    "type": "network",
    "model": "HP LaserJet Pro MFP M125nw",
    "manufacturer": "HP",
    "ip_address": "192.168.1.101",
    "port": 9100,
    "location": "一楼办公区",
    "is_default": true
  }'

# 创建 USB 打印机
curl -X POST http://localhost:8080/api/v1/printers \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Epson USB",
    "type": "usb",
    "usb_device": "/dev/usb/lp0",
    "location": "打印室"
  }'
```

### JavaScript 示例
```javascript
// 创建网络打印机
fetch('http://localhost:8080/api/v1/printers', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer ' + jwtToken,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    name: 'HP LaserJet Pro',
    type: 'network',
    model: 'HP LaserJet Pro MFP M125nw',
    manufacturer: 'HP',
    ip_address: '192.168.1.101',
    port: 9100,
    location: '一楼办公区',
    is_default: true
  })
})
.then(res => res.json())
.then(data => {
  if (data.success) {
    console.log('打印机创建成功:', data.data);
  } else {
    console.error('创建失败:', data.message);
  }
});
```

---

## 安全特性

1. **JWT 认证**: 仅 admin 角色可创建打印机
2. **类型验证**: 根据打印机类型验证必要字段
3. **参数验证**: 验证输入参数的有效性

---

## 注意事项

1. **Network 类型** - 必须提供 `ip_address`
2. **USB 类型** - 必须提供 `usb_device`（如 `/dev/usb/lp0`）
3. **Virtual 类型** - 无需额外字段
4. `port` 默认为 9100（仅 network 类型）
5. 创建成功后返回 201 Created 状态码
