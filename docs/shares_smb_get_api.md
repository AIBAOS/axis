# Phase 203: SMB 共享详情 API

## 概述

- **接口**: `GET /api/v1/shares/smb/{id}`
- **功能**: 获取指定 SMB 共享的详细信息
- **权限**: JWT 认证，登录用户可访问
- **Phase**: 203

## 请求参数

### 路径参数

| 参数名 | 位置 | 类型 | 必填 | 说明 |
|--------|------|------|------|------|
| id | path | integer | 是 | SMB 共享 ID |

## 请求示例

```bash
curl -X GET "http://localhost:8080/api/v1/shares/smb/1" \
  -H "Authorization: Bearer <your_jwt_token>"
```

## 响应

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "Public",
    "path": "/srv/samba/public",
    "description": "Public shared folder",
    "public": true,
    "created_at": 1711500000,
    "updated_at": 1711500000
  }
}
```

### 错误响应

#### 401 Unauthorized

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 404 Not Found

```json
{
  "success": false,
  "error": "SMB share not found",
  "code": "NOT_FOUND"
}
```

## 字段说明

### SmbShareDetail

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | integer | SMB 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径 |
| description | string | 共享描述 |
| public | boolean | 是否公开访问（guest_access） |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |

## 业务规则

1. **登录用户权限**：任意登录用户可访问
2. **归属验证**：普通用户只能查看自己的共享，admin 可查看任意（后续扩展）
3. **协议验证**：仅返回 SMB 协议的共享
4. **返回字段**：id/name/path/description/public/created_at/updated_at

## 数据库变更

- 使用已存在的 `shares` 表
- 查询字段：id, name, path, description, guest_access, created_at, updated_at

## 实现历史

- **Phase 203**: SMB 共享详情 API (GET /api/v1/shares/smb/{id}) - 2026-03-28
