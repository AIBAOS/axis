# 系统通知详情 API

## Phase 209

## 接口说明

获取指定系统通知的详细信息。

## 请求

`GET /api/v1/system/notifications/{id}`

### 路径参数

| 字段 | 类型 | 必填 | 说明 |
| ---- | ---- | ---- | ---- |
| id | integer | 是 | 通知 ID |

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
  "data": {
    "id": 15,
    "type": "system",
    "title": "系统维护通知",
    "message": "系统将于今晚 23:00 进行例行维护",
    "source": "system",
    "status": "unread",
    "created_at": 1711584000,
    "read_at": null,
    "metadata": null
  }
}
```

### 返回字段说明

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | integer | 通知 ID |
| type | string | 通知类型（system/alert/info/warning/error） |
| title | string | 通知标题 |
| message | string | 通知内容 |
| source | string | 通知来源 |
| status | string | 状态（read/unread） |
| created_at | integer | 创建时间（Unix 时间戳） |
| read_at | integer | 已读时间（Unix 时间戳，null 表示未读） |
| metadata | object | 扩展元数据（可选） |

### 错误响应

#### 401 Unauthorized - 未认证或 Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

#### 403 Forbidden - 无权查看此通知

```json
{
  "success": false,
  "error": "无权查看此通知详情",
  "code": "FORBIDDEN"
}
```

#### 404 Not Found - 通知不存在

```json
{
  "success": false,
  "error": "Notification 999 not found",
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

### 获取系统通知详情

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications/15" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（200 OK）：
```json
{
  "success": true,
  "data": {
    "id": 15,
    "type": "system",
    "title": "系统维护通知",
    "message": "系统将于今晚 23:00 进行例行维护",
    "source": "system",
    "status": "unread",
    "created_at": 1711584000,
    "read_at": null,
    "metadata": null
  }
}
```

### 获取他人的通知（非 admin）

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications/10" \
  -H "Authorization: Bearer <user_jwt_token>"
```

响应（403 Forbidden）：
```json
{
  "success": false,
  "error": "无权查看此通知详情",
  "code": "FORBIDDEN"
}
```

### 获取不存在的通知

```bash
curl -X GET "http://localhost:8080/api/v1/system/notifications/999" \
  -H "Authorization: Bearer <jwt_token>"
```

响应（404 Not Found）：
```json
{
  "success": false,
  "error": "Notification 999 not found",
  "code": "NOT_FOUND"
}
```

## 权限要求

- 需要 JWT 认证
- 登录用户可访问
- 权限规则：
  - admin 用户可查看任意通知
  - 普通用户只能查看自己的通知（target_user_id = 当前用户 ID）
  - 系统通知（target_user_id IS NULL）对所有用户可见

## 业务逻辑

1. 验证 JWT Token 有效性
2. 查询通知是否存在（404 Not Found）
3. 验证通知归属权限（403 Forbidden）
4. 返回通知详情

## 通知类型

| 类型 | 说明 |
| ---- | ---- |
| system | 系统通知 |
| alert | 告警通知 |
| info | 信息通知 |
| warning | 警告通知 |
| error | 错误通知 |

## 版本历史

- **Phase 209** (2026-03-28): 通知管理模块 - 系统通知详情 API
