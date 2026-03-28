# 系统通知列表 API

## Phase 197

## 接口说明

获取系统级别的通知列表，支持分页和优先级筛选。系统通知是指 `target_user_id` 为 NULL 的全局通知。

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
| page_size | integer | 否 | 20 | 每页数量（最大 100） |
| priority | string | 否 | - | 优先级筛选（low/normal/high/critical） |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "notifications": [
      {
        "id": 15,
        "title": "系统维护通知",
        "message": "系统将于今晚 23:00 进行例行维护",
        "type": "system",
        "priority": "high",
        "is_read": false,
        "created_at": 1711584000,
        "action_url": "/admin/maintenance"
      },
      {
        "id": 14,
        "title": "存储空间警告",
        "message": "存储池使用率已超过 80%",
        "type": "alert",
        "priority": "normal",
        "is_read": true,
        "created_at": 1711497600,
        "action_url": "/storage/pools"
      }
    ],
    "total": 15,
    "page": 1,
    "page_size": 20,
    "has_more": false
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | integer | 通知 ID |
| title | string | 通知标题 |
| message | string | 通知内容 |
| type | string | 通知类型（system/alert/info/warning/error） |
| priority | string | 优先级（low/normal/high/critical） |
| is_read | boolean | 是否已读 |
| created_at | integer | 创建时间（Unix 时间戳） |
| action_url | string | 操作链接（可选） |
| total | integer | 总记录数 |
| page | integer | 当前页码 |
| page_size | integer | 每页数量 |
| has_more | boolean | 是否有更多数据 |

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
  "error": "查询系统通知失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 获取系统通知列表（第一页）

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications?page=1&page_size=20" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取高优先级系统通知

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications?priority=high" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取第二页（每页 10 条）

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications?page=2&page_size=10" \
  -H "Authorization: Bearer <jwt_token>"
```

## 权限要求

- 需要 JWT 认证
- 任意登录用户可访问

## 业务逻辑

1. 验证 JWT Token 有效性
2. 查询系统通知（target_user_id IS NULL）
3. 可选按优先级筛选
4. 按 created_at 降序排列（最新的在前）
5. 应用分页
6. 返回通知列表和分页信息

## 通知类型

| 类型 | 说明 |
| ---- | ---- |
| system | 系统通知 |
| alert | 告警通知 |
| info | 信息通知 |
| warning | 警告通知 |
| error | 错误通知 |

## 优先级

| 优先级 | 说明 |
| ---- | ---- |
| low | 低优先级 |
| normal | 普通优先级 |
| high | 高优先级 |
| critical | 紧急优先级 |

## 版本历史

- **Phase 197** (2026-03-28): 通知管理模块 - 系统通知列表 API
