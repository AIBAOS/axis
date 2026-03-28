# SMB 共享列表 API - Phase 202

## 接口规范

### 基本信息

- **端点**: `GET /api/v1/shares/smb`
- **认证**: JWT Bearer Token（必需）
- **权限**: 任意登录用户可访问
- **方法**: GET

---

## 请求

### 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|-----|------|-----|--------|------|
| page | u32 | 否 | 1 | 页码（从 1 开始） |
| limit | u32 | 否 | 20 | 每页数量（最大 100） |
| public | bool | 否 | - | 筛选公共/私有共享 |

### 请求示例

**获取第 1 页，每页 20 条**:
```
GET /api/v1/shares/smb?page=1&limit=20
```

**筛选公共共享**:
```
GET /api/v1/shares/smb?public=true
```

**获取第 3 页，每页 50 条**:
```
GET /api/v1/shares/smb?page=3&limit=50
```

---

## 响应

### 200 OK - 成功

```json
{
  "success": true,
  "message": "获取共享列表成功",
  "data": {
    "items": [
      {
        "id": "share_123456",
        "name": "Public Share",
        "path": "/data/public",
        "description": "公共共享文件夹",
        "public": true,
        "created_at": 1711468800,
        "updated_at": 1711468800
      },
      {
        "id": "share_789012",
        "name": "Private Share",
        "path": "/data/private",
        "description": null,
        "public": false,
        "created_at": 1711468800,
        "updated_at": 1711468800
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 50,
      "total_pages": 3
    }
  }
}
```

### 响应字段说明

**ShareInfo**:
| 字段 | 类型 | 说明 |
|-----|------|------|
| id | string | 共享 ID |
| name | string | 共享名称 |
| path | string | 共享路径 |
| description | string\|null | 描述 |
| public | bool | 是否公共共享 |
| created_at | i64 | 创建时间（Unix 时间戳） |
| updated_at | i64 | 更新时间（Unix 时间戳） |

**Pagination**:
| 字段 | 类型 | 说明 |
|-----|------|------|
| page | u32 | 当前页码 |
| limit | u32 | 每页数量 |
| total | u32 | 总记录数 |
| total_pages | u32 | 总页数 |

### 401 Unauthorized - 未登录

```json
{
  "success": false,
  "message": "未授权访问",
  "error_code": "UNAUTHORIZED"
}
```

### 500 Internal Server Error - 服务器错误

```json
{
  "success": false,
  "message": "数据库查询失败",
  "error_code": "INTERNAL_ERROR"
}
```

---

## 使用示例

### cURL

```bash
curl -X GET "http://localhost:8080/api/v1/shares/smb?page=1&limit=20" \
  -H "Authorization: Bearer <jwt_token>"
```

**筛选公共共享**:
```bash
curl -X GET "http://localhost:8080/api/v1/shares/smb?public=true" \
  -H "Authorization: Bearer <jwt_token>"
```

### JavaScript (fetch)

```javascript
const response = await fetch('http://localhost:8080/api/v1/shares/smb?page=1&limit=20', {
  method: 'GET',
  headers: {
    'Authorization': 'Bearer <jwt_token>',
  },
});

const data = await response.json();
console.log(data);

// 遍历共享列表
data.data.items.forEach(share => {
  console.log(`${share.name}: ${share.path}`);
});

// 分页信息
console.log(`第 ${data.data.pagination.page} / ${data.data.pagination.total_pages} 页`);
console.log(`共 ${data.data.pagination.total} 条记录`);
```

### Python (requests)

```python
import requests

url = 'http://localhost:8080/api/v1/shares/smb'
headers = {
    'Authorization': 'Bearer <jwt_token>',
}
params = {
    'page': 1,
    'limit': 20,
    'public': True,  # 可选：筛选公共共享
}

response = requests.get(url, headers=headers, params=params)
data = response.json()

print(f"第 {data['data']['pagination']['page']} / {data['data']['pagination']['total_pages']} 页")
print(f"共 {data['data']['pagination']['total']} 条记录")

for share in data['data']['items']:
    print(f"{share['name']}: {share['path']} (public={share['public']})")
```

### TypeScript

```typescript
interface ShareInfo {
  id: string;
  name: string;
  path: string;
  description: string | null;
  public: boolean;
  created_at: number;
  updated_at: number;
}

interface Pagination {
  page: number;
  limit: number;
  total: number;
  total_pages: number;
}

interface ListResponse {
  success: boolean;
  message: string;
  data: {
    items: ShareInfo[];
    pagination: Pagination;
  };
}

async function listShares(page = 1, limit = 20, isPublic?: boolean) {
  const params = new URLSearchParams({
    page: page.toString(),
    limit: limit.toString(),
  });
  
  if (isPublic !== undefined) {
    params.append('public', isPublic.toString());
  }
  
  const response = await fetch(`/api/v1/shares/smb?${params}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  
  const data: ListResponse = await response.json();
  return data;
}
```

---

## 分页说明

### 页码计算

```
总页数 = ceil(总记录数 / 每页数量)
偏移量 = (页码 - 1) × 每页数量
```

### 示例

| 总记录 | 每页 | 页码 | 偏移量 | 总页数 |
|-------|-----|-----|-------|-------|
| 100 | 20 | 1 | 0 | 5 |
| 100 | 20 | 3 | 40 | 5 |
| 101 | 20 | 1 | 0 | 6 |
| 50 | 50 | 1 | 0 | 1 |

### 边界情况

- **page < 1**: 自动修正为 1
- **limit > 100**: 自动限制为 100
- **limit = 0**: 使用默认值 20
- **total = 0**: total_pages = 1

---

## 筛选说明

### public 参数

| 值 | 说明 |
|---|------|
| `true` | 仅返回公共共享 |
| `false` | 仅返回私有共享 |
| 不提供 | 返回所有共享 |

---

## 安全说明

1. **认证要求**: 需要有效的 JWT Token
2. **权限控制**: 任意登录用户可访问（无需 admin）
3. **数据隔离**: 未来可扩展为用户仅查看自己的共享

---

## 相关文件

- 实现：`src/handlers/shares_smb_list.rs`
- 路由注册：`src/main.rs`
- 数据库：`src/database/share_store.rs`

---

**版本**: v1.0.0  
**最后更新**: 2026-03-28 02:00 UTC  
**Phase**: 202
