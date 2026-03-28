# 系统告警列表 API

## Phase 174

## 接口说明

获取系统告警列表，支持分页和筛选。

## 请求

`GET /api/v1/system/alerts`

### 查询参数

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| ---- | ---- | ---- | ---- | ---- |
| page | u32 | 否 | 1 | 页码（从 1 开始） |
| per_page | u32 | 否 | 20 | 每页数量（最大 100） |
| status | string | 否 | - | 告警状态（active/resolved/acknowledged） |
| severity | string | 否 | - | 告警级别（critical/warning/info） |
| source | string | 否 | - | 告警来源（system/storage/network/等） |

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
      "title": "High CPU Usage",
      "message": "CPU usage exceeded 90% for more than 5 minutes",
      "severity": "critical",
      "status": "active",
      "source": "system",
      "created_at": "2026-03-27T12:30:00Z",
      "acknowledged_at": null,
      "resolved_at": null
    },
    {
      "id": 2,
      "title": "Disk Space Low",
      "message": "Disk usage exceeded 80% on /dev/sda1",
      "severity": "warning",
      "status": "acknowledged",
      "source": "storage",
      "created_at": "2026-03-27T12:00:00Z",
      "acknowledged_at": "2026-03-27T12:05:00Z",
      "resolved_at": null
    },
    {
      "id": 3,
      "title": "Network Interface Down",
      "message": "Network interface eth1 is down",
      "severity": "critical",
      "status": "resolved",
      "source": "network",
      "created_at": "2026-03-27T11:30:00Z",
      "acknowledged_at": "2026-03-27T11:35:00Z",
      "resolved_at": "2026-03-27T11:50:00Z"
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

#### 400 Bad Request - 参数无效

```json
{
  "success": false,
  "error": "Invalid status. Valid statuses: active, resolved, acknowledged",
  "code": "INVALID_STATUS"
}
```

```json
{
  "success": false,
  "error": "Invalid severity. Valid severities: critical, warning, info",
  "code": "INVALID_SEVERITY"
}
```

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
  "error": "Only admin users can view system alerts",
  "code": "FORBIDDEN"
}
```

## 示例

### 获取系统告警列表（默认分页）

```bash
curl "http://localhost:8080/api/v1/system/alerts" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取活跃告警

```bash
curl "http://localhost:8080/api/v1/system/alerts?status=active" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取严重级别告警

```bash
curl "http://localhost:8080/api/v1/system/alerts?severity=critical" \
  -H "Authorization: Bearer <jwt_token>"
```

### 获取特定来源的告警

```bash
curl "http://localhost:8080/api/v1/system/alerts?source=network" \
  -H "Authorization: Bearer <jwt_token>"
```

### 组合筛选

```bash
curl "http://localhost:8080/api/v1/system/alerts?status=active&severity=warning&per_page=10" \
  -H "Authorization: Bearer <jwt_token>"
```

## 权限要求

- 需要 JWT 认证
- 仅限 admin 角色访问

## 响应字段说明

### 告警列表字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| id | u64 | 告警 ID |
| title | string | 告警标题 |
| message | string | 告警消息 |
| severity | string | 告警级别（critical/warning/info） |
| status | string | 告警状态（active/resolved/acknowledged） |
| source | string | 告警来源（system/storage/network/backup/auth/等） |
| created_at | string | 创建时间（ISO 8601 格式） |
| acknowledged_at | string\|null | 确认时间（ISO 8601 格式） |
| resolved_at | string\|null | 解决时间（ISO 8601 格式） |

### 分页字段

| 字段 | 类型 | 说明 |
| ---- | ---- | ---- |
| page | u32 | 当前页码 |
| per_page | u32 | 每页数量 |
| total | u64 | 总记录数 |
| total_pages | u32 | 总页数 |

## 告警级别说明

| 级别 | 说明 | 示例 |
| ---- | ---- | ---- |
| critical | 严重告警，需立即处理 | CPU 使用率>90%、磁盘故障 |
| warning | 警告，需关注 | 磁盘使用率>80%、内存使用率高 |
| info | 信息性通知 | 备份完成、配置更新 |

## 告警状态说明

| 状态 | 说明 |
| ---- | ---- |
| active | 活跃状态，未处理 |
| acknowledged | 已确认，处理中 |
| resolved | 已解决 |

## 告警来源说明

| 来源 | 说明 |
| ---- | ---- |
| system | 系统级告警 |
| storage | 存储相关告警 |
| network | 网络相关告警 |
| backup | 备份任务告警 |
| auth | 认证和安全告警 |
| container | 容器相关告警 |
| docker | Docker 相关告警 |

## 业务逻辑

1. 验证 JWT Token 有效性
2. 检查用户角色是否为 admin
3. 验证告警状态参数（如果提供）
4. 验证告警级别参数（如果提供）
5. 获取系统告警列表
6. 应用状态、级别、来源过滤
7. 按 created_at 降序排序
8. 应用分页
9. 返回 200 OK + 告警列表 + 分页信息

## 版本历史

- **Phase 174** (2026-03-27): 系统告警模块 - 系统告警列表 API
