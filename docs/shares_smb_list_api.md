# SMB 共享列表 API

## Phase 151

## 接口说明

获取 SMB 共享列表，支持分页和状态过滤。

## 请求

`GET /api/v1/shares/smb`

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ---- | ---- |
| page | u32 | 否 | 1 | 页码（从 1 开始） |
| per_page | u32 | 否 | 20 | 每页数量（最大 100） |
| status | string | 否 | - | 状态过滤（active/inactive） |

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "Public",
      "path": "/srv/samba/public",
      "status": "active",
      "read_only": false,
      "guest_access": true,
      "enabled": true,
      "created_at": 1711500000,
      "updated_at": 1711500000
    },
    {
      "id": 2,
      "name": "Users",
      "path": "/srv/samba/users",
      "status": "active",
      "read_only": false,
      "guest_access": false,
      "enabled": true,
      "created_at": 1711500000,
      "updated_at": 1711500000
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 4,
    "total_pages": 1
  }
}
```

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 权限不足

```json
{
  "success": false,
  "error": "Only admin users can list SMB shares",
  "code": "FORBIDDEN"
}
```

## 示例

### 获取 SMB 共享列表（默认分页）

```bash
curl "http://localhost:8080/api/v1/shares/smb" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取第 2 页，每页 10 条

```bash
curl "http://localhost:8080/api/v1/shares/smb?page=2&per_page=10" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取活跃的 SMB 共享

```bash
curl "http://localhost:8080/api/v1/shares/smb?status=active" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取非活跃的 SMB 共享

```bash
curl "http://localhost:8080/api/v1/shares/smb?status=inactive" \
  -H "Authorization: Bearer <jwt_token>"
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 共享列表字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径 |
| status | string | 状态（active/inactive） |
| read_only | boolean | 是否只读 |
| guest_access | boolean | 是否允许访客访问 |
| enabled | boolean | 是否启用 |
| created_at | u64 | 创建时间戳 |
| updated_at | u64 | 更新时间戳 |

### 分页字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| page | u32 | 当前页码 |
| per_page | u32 | 每页数量 |
| total | u64 | 总记录数 |
| total_pages | u32 | 总页数 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 解析查询参数（page/per_page/status）
4. 获取 SMB 共享列表
5. 应用状态过滤
6. 应用分页
7. 返回 200 OK + 共享列表 + 分页信息

## 版本历史

- **Phase 151** (2026-03-27): 初始版本
