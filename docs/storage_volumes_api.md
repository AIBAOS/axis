# 存储卷列表 API (Phase 78)

## 接口说明

实现获取存储卷列表的接口，支持分页查询。任意登录用户可访问。

## 接口定义

```
GET /api/v1/storage/volumes
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（任意登录用户） |

## 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| page | integer | 否 | 1 | 页码（从 1 开始） |
| per_page | integer | 否 | 20 | 每页数量（最大 100） |

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "System Volume",
      "description": "System storage volume",
      "pool_id": 1,
      "total_bytes": 536870912000,
      "used_bytes": 268435456000,
      "available_bytes": 268435456000,
      "usage_percent": 50.0,
      "status": "active",
      "filesystem": "ext4",
      "mount_point": "/mnt/system",
      "created_at": 1710500000,
      "updated_at": 1711400000
    },
    {
      "id": 2,
      "name": "Data Volume 1",
      "description": "Primary data volume",
      "pool_id": 2,
      "total_bytes": 1099511627776,
      "used_bytes": 439804651110,
      "available_bytes": 659706976666,
      "usage_percent": 40.0,
      "status": "active",
      "filesystem": "ext4",
      "mount_point": "/mnt/data1",
      "created_at": 1710600000,
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

### 无数据 (200 OK)

```json
{
  "success": true,
  "data": [],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 0,
    "total_pages": 0
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

### cURL 示例

```bash
# 获取存储卷列表（默认分页）
curl -X GET "http://localhost:8080/api/v1/storage/volumes" \
  -H "Authorization: Bearer <jwt_token>"

# 分页查询
curl -X GET "http://localhost:8080/api/v1/storage/volumes?page=1&per_page=10" \
  -H "Authorization: Bearer <jwt_token>"
```

### JavaScript 示例

```javascript
// 获取存储卷列表
async function getVolumes(page = 1, perPage = 20) {
  const params = new URLSearchParams({
    page: page.toString(),
    per_page: perPage.toString()
  });
  
  const response = await fetch(
    `http://localhost:8080/api/v1/storage/volumes?${params}`,
    {
      headers: {
        'Authorization': 'Bearer ' + token
      }
    }
  );
  
  const data = await response.json();
  console.log('Volumes:', data.data);
  console.log('Pagination:', data.pagination);
  return data;
}

// 使用示例
const volumes = await getVolumes();
volumes.data.forEach(vol => {
  console.log(`${vol.name}: ${vol.usage_percent}% used, ${vol.filesystem}`);
});
```

## 响应字段说明

### VolumeInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 存储卷 ID |
| name | string | 存储卷名称 |
| description | string | 存储卷描述 |
| pool_id | integer | 所属存储池 ID |
| total_bytes | integer | 总容量（字节） |
| used_bytes | integer | 已使用容量（字节） |
| available_bytes | integer | 可用容量（字节） |
| usage_percent | float | 使用率（百分比） |
| status | string | 状态（active/inactive） |
| filesystem | string | 文件系统类型（ext4/xfs/btrfs） |
| mount_point | string | 挂载点 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |

### PaginationInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| page | integer | 当前页码 |
| per_page | integer | 每页数量 |
| total | integer | 总记录数 |
| total_pages | integer | 总页数 |

## 安全特性

1. **JWT 认证**: 必须提供有效的 JWT Token
2. **任意登录用户**: 无需 admin 权限，任意登录用户可访问
3. **只读接口**: 不提供修改存储卷的功能

## 实现文件

- `src/handlers/storage_volumes.rs` - 存储卷列表处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. 当前为模拟数据，后续将连接系统 API 获取真实数据
2. per_page 最大值为 100，超过会自动限制
3. 无数据时返回空数组，不返回错误
4. 容量以字节为单位
5. 时间戳使用 Unix 时间戳（秒级）

## 相关接口

- `GET /api/v1/storage/volumes/{id}` - 存储卷详情（Phase 61）
- `POST /api/v1/storage/volumes` - 创建存储卷（Phase 67）
- `PUT /api/v1/storage/volumes/{id}` - 更新存储卷（Phase 68）
- `DELETE /api/v1/storage/volumes/{id}` - 删除存储卷（Phase 69）
- `GET /api/v1/storage/pools` - 存储池列表（Phase 75）
