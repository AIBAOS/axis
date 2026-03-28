# SMB 共享列表 API 文档

## 概述

本文档描述 Axis NAS 系统中 SMB 共享列表 API 的实现细节。

## API 端点

- **路径**: `GET /api/v1/shares/smb`
- **版本**: v1
- **Phase**: 198

## 认证

- **类型**: JWT Bearer Token
- **权限**: 登录用户可访问

## 请求参数

### Query 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `page` | number | 否 | 1 | 页码（从 1 开始） |
| `per_page` | number | 否 | 20 | 每页数量（最大 100） |
| `status` | string | 否 | - | 状态筛选：`active` / `inactive` |
| `path` | string | 否 | - | 路径模糊搜索 |

## 响应格式

### 成功响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "Public",
      "path": "/srv/samba/public",
      "description": "公共共享文件夹",
      "status": "active",
      "created_at": 1711500000,
      "updated_at": 1711500000
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

### 错误响应

```json
{
  "success": false,
  "error": "错误描述",
  "code": "ERROR_CODE"
}
```

## 数据模型

### SmbShareInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | number | 共享 ID |
| `name` | string | 共享名称 |
| `path` | string | 共享路径 |
| `description` | string? | 共享描述 |
| `status` | string | 状态：`active` / `inactive` |
| `created_at` | number | 创建时间（Unix 时间戳） |
| `updated_at` | number | 更新时间（Unix 时间戳） |

### PaginationInfo

| 字段 | 类型 | 描述 |
|------|------|------|
| `page` | number | 当前页码 |
| `per_page` | number | 每页数量 |
| `total` | number | 总记录数 |
| `total_pages` | number | 总页数 |

## 错误代码

| 代码 | 描述 |
|------|------|
| `UNAUTHORIZED` | 未提供或无效的认证令牌 |
| `FORBIDDEN` | 权限不足 |
| `DATABASE_ERROR` | 数据库查询失败 |

## 示例

### 请求

```bash
curl -X GET "http://localhost:8080/api/v1/shares/smb?page=1&per_page=20&status=active" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### 响应

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "Public",
      "path": "/srv/samba/public",
      "description": "公共共享文件夹",
      "status": "active",
      "created_at": 1711500000,
      "updated_at": 1711500000
    },
    {
      "id": 2,
      "name": "Users",
      "path": "/srv/samba/users",
      "description": null,
      "status": "active",
      "created_at": 1711500000,
      "updated_at": 1711500000
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 2,
    "total_pages": 1
  }
}
```

## 数据库表结构

```sql
CREATE TABLE IF NOT EXISTS shares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    protocol TEXT NOT NULL CHECK(protocol IN ('smb', 'nfs')),
    status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'inactive')),
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
```

## 相关接口

- `POST /api/v1/shares/smb` - 创建 SMB 共享
- `GET /api/v1/shares/smb/{id}` - 获取 SMB 共享详情
- `PUT /api/v1/shares/smb/{id}` - 更新 SMB 共享
- `DELETE /api/v1/shares/smb/{id}` - 删除 SMB 共享

## 实现细节

- **排序**: 按 `created_at` 降序（最新的在前）
- **分页**: 使用 `LIMIT` 和 `OFFSET`
- **模糊搜索**: 使用 SQL `LIKE` 操作符
- **数据库**: SQLite
- **框架**: Actix-web

## 测试验证

```bash
# 编译检查
cargo check

# 运行测试（如果有）
cargo test
```

## 版本历史

- **Phase 198** (2026-03-28): 初始实现，从 mock 数据升级为 SQLite 持久化