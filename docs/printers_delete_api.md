# Phase 58 - 删除打印机 API 文档

## 端点信息

- **方法**: `DELETE`
- **路径**: `/api/v1/printers/{id}`
- **认证**: Bearer Token (需 admin 角色)
- **功能**: 删除指定打印机

---

## 请求参数

### 路径参数 (Path Parameters)

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| id | integer | 是 | 打印机 ID |

### 请求示例
```
DELETE /api/v1/printers/1
DELETE /api/v1/printers/5
```

---

## 响应格式

### 成功响应 (204 No Content)
删除成功后返回空响应体（HTTP 204）

### 错误响应

#### 400 Bad Request - 参数错误
```json
{
  "success": false,
  "message": "Invalid printer ID"
}
```

#### 401 Unauthorized - 未认证
```json
{
  "success": false,
  "message": "Only admin users can delete printers"
}
```

#### 403 Forbidden - 权限不足
```json
{
  "success": false,
  "message": "Only admin users can delete printers"
}
```

#### 404 Not Found - 打印机不存在
```json
{
  "success": false,
  "message": "Printer 999 not found"
}
```

---

## 使用示例

### cURL 示例
```bash
# 删除打印机 ID 为 1 的打印机
curl -X DELETE http://localhost:8080/api/v1/printers/1 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 删除打印机 ID 为 5 的打印机
curl -X DELETE http://localhost:8080/api/v1/printers/5 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### JavaScript 示例
```javascript
// 删除打印机
fetch('http://localhost:8080/api/v1/printers/1', {
  method: 'DELETE',
  headers: {
    'Authorization': 'Bearer ' + jwtToken
  }
})
.then(res => {
  if (res.status === 204) {
    console.log('Printer deleted successfully');
  } else {
    return res.json().then(data => {
      console.error('Error:', data.message);
    });
  }
});
```

---

## 安全特性

1. **JWT 认证**: 仅 admin 角色可删除打印机
2. **ID 验证**: 确保打印机 ID 存在
3. **权限校验**: 非 admin 用户收到 403 Forbidden

---

## 注意事项

1. 删除操作为**硬删除**（数据不可恢复）
2. 删除前请确认打印机不再使用
3. 建议在删除前备份相关配置
