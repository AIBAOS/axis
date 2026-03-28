# 通知详情 API

## Phase 209

## 接口说明

获取单个通知的详细信息。系统通知和用户个人通知都可通过此接口查看。

## 请求

`GET /api/v1/system/notifications/{id}`

### 请求头

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| Authorization | string | 是 | JWT Token，格式：`Bearer <token>` |

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 通知 ID |

### 请求体

无

## 响应

### 成功响应（200 OK）

```json
{
  "success": true,
  "data": {
    "id": 123,
    "type": "info",
    "title": "系统维护通知",
    "message": "系统将于今晚 23:00 进行例行维护",
    "source": "system",
    "status": "unread",
    "created_at": 1711584000,
    "read_at": null,
    "metadata": {
      "priority": "high",
      "target_user_id": null,
      "action_url": "/admin/maintenance"
    }
  }
}
```

### 返回字段说明

#### data 对象字段

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
| metadata | object | 通知元数据 |

#### metadata 对象字段

| 字段 | 类型 | 说明 |
| ---- | ---- |
| priority | string | 优先级（low/normal/high/critical） |
| target_user_id | integer | 目标用户 ID（系统通知为 null） |
| action_url | string | 操作链接（可选） |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 非归属用户

```json
{
  "success": false,
  "error": "只能查看自己的通知",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 通知不存在

```json
{
  "success": false,
  "error": "Notification 123 not found",
  "code": "NOT_FOUND"
}
```

#### 500 Internal Server Error - 数据库错误

```json
{
  "success": false,
  "error": "查询通知失败：database is locked",
  "code": "DATABASE_ERROR"
}
```

## 示例

### 获取通知详情

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications/123" \
  -H "Authorization: Bearer <jwt_token>"
```

### 响应（成功）

```json
{
  "success": true,
  "data": {
    "id": 123,
    "type": "info",
    "title": "系统维护通知",
    "message": "系统将于今晚 23:00 进行例行维护",
    "source": "system",
    "status": "unread",
    "created_at": 1711584000,
    "read_at": null,
    "metadata": {
      "priority": "high",
      "target_user_id": null,
      "action_url": "/admin/maintenance"
    }
  }
}
```

## 权限要求

- 需要 JWT 认证
- **登录用户可访问**
- **归属验证**：
  - admin 用户可查看任意通知
  - 普通用户只能查看自己的通知或系统通知（target_user_id IS NULL）

## 业务逻辑

1. 验证 JWT Token 有效性
2. 验证用户角色（admin 可查看任意）
3. 验证通知归属（普通用户仅能查看自己的或系统通知）
4. 查询通知详情
5. 返回完整信息

## 版本历史

- **Phase 209** (2026-03-28): 通知详情 API