# SMB 共享列表 API (Phase 202)

## 接口说明

获取 SMB 共享文件夹列表，支持分页和按公开状态筛选。

## 接口定义

```
GET /api/v1/shares/smb
```

## 请求参数

### 查询参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
|--------|------|------|--------|------|
| page | integer | 否 | 1 | 页码（从 1 开始） |
| limit | integer | 否 | 20 | 每页数量（最大 100） |
| public | boolean | 否 | - | 公开状态筛选（true/false） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "Public",
      "path": "/srv/samba/public",
      "description": "Public shared folder",
      "public": true,
      "created_at": 1710500000,
      "updated_at": 1710500000
    },
    {
      "id": 2,
      "name": "Home",
      "path": "/srv/samba/home",
      "description": "Home directories",
      "public": false,
      "created_at": 1710600000,
      "updated_at": 1710600000
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
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

## 使用示例

### 1. 获取所有 SMB 共享列表（默认分页）

```bash
curl -X GET "http://localhost:8080/api/v1/shares/smb" \
  -H "Authorization: Bearer <jwt_token>"
```

### 2. 分页查询

```bash
# 第一页，每页 10 个
curl -X GET "http://localhost:8080/api/v1/shares/smb?page=1&limit=10" \
  -H "Authorization: Bearer <jwt_token>"

# 第二页
curl -X GET "http://localhost:8080/api/v1/shares/smb?page=2&limit=10" \
  -H "Authorization: Bearer <jwt_token>"
```

### 3. 按公开状态筛选

```bash
# 只看公开共享
curl -X GET "http://localhost:8080/api/v1/shares/smb?public=true" \
  -H "Authorization: Bearer <jwt_token>"

# 只看私有共享
curl -X GET "http://localhost:8080/api/v1/shares/smb?public=false" \
  -H "Authorization: Bearer <jwt_token>"
```

### 4. 组合查询

```bash
# 查看公开共享，每页 5 个
curl -X GET "http://localhost:8080/api/v1/shares/smb?public=true&limit=5" \
  -H "Authorization: Bearer <jwt_token>"
```

## 字段说明

### SMB 共享信息

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径（服务器上的实际路径） |
| description | string | 共享描述 |
| public | boolean | 是否公开（true=公开，false=私有） |
| created_at | integer | 创建时间戳 |
| updated_at | integer | 更新时间戳 |

### 分页信息

| 字段 | 类型 | 说明 |
|------|------|------|
| page | integer | 当前页码 |
| limit | integer | 每页数量 |
| total | integer | 总记录数 |
| total_pages | integer | 总页数 |

## 功能特性

- ✅ JWT 认证，任意登录用户可访问
- ✅ 支持分页查询（page/limit）
- ✅ 支持按公开状态筛选（public=true/false）
- ✅ 返回 7 个字段：id/name/path/description/public/created_at/updated_at
- ✅ limit 最大 100 条/页
- ✅ 无数据返回空数组

## 公开状态说明

| public 值 | 说明 | 访问权限 |
|-----------|------|----------|
| true | 公开共享 | 所有用户可访问（包括访客） |
| false | 私有共享 | 仅授权用户可访问 |

## 注意事项

1. limit 参数最大值为 100，超过会自动限制
2. page 从 1 开始计数
3. 无匹配数据时返回空数组和 total=0
4. public 筛选为精确匹配

## 实现文件

- `src/handlers/shares_smb_list_v2.rs` - SMB 共享列表处理器（Phase 202 增强版）
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册
