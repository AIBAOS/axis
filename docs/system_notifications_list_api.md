# 系统通知列表 API

## Phase 207

## 接口说明

获取系统通知列表，支持分页和多种筛选条件。系统通知是指 `target_user_id` 为 NULL 的全局通知。

## 请求

`GET /api/v1/system/notifications`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ------- | ---- |
| page | integer | 否 | 1 | 页码（从 1 开始） |
| per_page | integer | 否 | 20 | 每页数量（最大 100） |
| type | string | 否 | - | 通知类型筛选（info/warning/error/critical） |
| status | string | 否 | - | 状态筛选（unread/read） |
| source | string | 否 | - | 来源筛选（如：nfs、share、system等） |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "data": [
    {
      "id": 15,
      "type": "info",
      "title": "系统维护通知",
      "message": "系统将于今晚 23:00 进行例行维护",
      "source": "system",
      "status": "unread",
      "created_at": 1711584000,
      "read_at": null
    },
    {
      "id": 14,
      "type": "warning",
      "title": "存储空间警告",
      "message": "存储池使用率已超过 80%",
      "source": "nfs",
      "status": "read",
      "created_at": 1711497600,
      "read_at": 1711500000
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 15,
    "total_pages": 1
  }
}
```

### 返回字段说明

#### data 数组字段

| 字段 | 类型 | 说明 |
| ---- | ---- |
| id | integer | 通知 ID |
| type | string | 通知类型（info/warning/error/critical） |
| title | string | 通知标题 |
| message | string | 通知内容 |
| source | string | 通知来源（可选） |
| status | string | 状态（unread/read） |
| created_at | integer | 创建时间（Unix 时间戳） |
| read_at | integer | 已读时间（未读时为 null） |

#### pagination 对象字段

| 字段 | 类型 | 说明 |
| ---- | ---- |
| page | integer | 当前页码 |
| per_page | integer | 每页数量 |
| total | integer | 总记录数 |
| total_pages | integer | 总页数 |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "查询通知列表失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 获取系统通知列表（第一页）

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications?page=1&per_page=20" \
  -H "Authorization: Bearer <jwt_token>"
```

### 按类型筛选

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications?type=warning" \
  -H "Authorization: Bearer <jwt_token>"
```

### 按状态筛选

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications?status=unread" \
  -H "Authorization: Bearer <jwt_token>"
```

### 组合筛选

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications?type=info&source=nfs&page=1&per_page=10" \
  -H "Authorization: Bearer <jwt_token>"
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 查询系统通知（target_user_id IS NULL）
3. 支持按 type/status/source 筛选
4. 按 created_at 降序排列（最新的在前）
5. 应用分页
6. 返回通知列表和分页信息

## 通知类型

| 类型 | 说明 |
| ---- | ---- |
| info | 信息通知 |
| warning | 警告通知 |
| error | 错误通知 |
| critical | 严重通知 |

## 状态

| 状态 | 说明 |
| ---- | ---- |
| unread | 未读 |
| read | 已读 |

## 版本历史

- **Phase 207** (2026-03-28): 系统通知列表 API - 重写实现，支持 type/status/source 筛选
- **Phase 197** (2026-03-28 00:50): 通知管理模块 - 系统通知列表 API 初始实现
