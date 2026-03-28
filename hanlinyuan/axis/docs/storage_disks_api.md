# Phase 180 - 存储磁盘列表 API 文档

## 接口说明

获取 NAS 系统上所有物理磁盘的详细信息，包括型号、容量、健康状态、温度等。

## 接口定义

```
GET /api/v1/storage/disks
```

## 请求头

| 头 | 值 | 必填 | 说明 |
|------|------|------|------|
| Authorization | Bearer \<jwt_token\> | 是 | JWT Token（任意登录用户） |

## 查询参数

| 参数 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| page | integer | 否 | 1 | 页码（从 1 开始） |
| limit | integer | 否 | 20 | 每页数量（最大 100） |
| disk_type | string | 否 | - | 磁盘类型筛选：hdd/ssd/nvme |
| smart_status | string | 否 | - | SMART 状态筛选：healthy/warning/failed/unknown |
| status | string | 否 | - | 磁盘状态筛选：online/offline |

## 请求示例

```bash
# 获取所有磁盘列表
curl -X GET http://localhost:8080/api/v1/storage/disks \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 分页查询（第 2 页，每页 10 条）
curl -X GET "http://localhost:8080/api/v1/storage/disks?page=2&limit=10" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 筛选 HDD 磁盘
curl -X GET "http://localhost:8080/api/v1/storage/disks?disk_type=hdd" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 筛选健康状态为 warning 的磁盘
curl -X GET "http://localhost:8080/api/v1/storage/disks?smart_status=warning" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 筛选离线磁盘
curl -X GET "http://localhost:8080/api/v1/storage/disks?status=offline" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 组合筛选
curl -X GET "http://localhost:8080/api/v1/storage/disks?disk_type=hdd&smart_status=healthy&page=1&limit=10" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## 响应格式

### 成功响应 (200 OK)

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "Disk 1",
      "path": "/dev/sda",
      "model": "WD Red Pro 4TB",
      "serial_number": "WD-WCC12345678",
      "type": "hdd",
      "size_bytes": 4398046511104,
      "size_human": "4.00 TB",
      "temperature": 35,
      "smart_status": "healthy",
      "health_status": "good",
      "speed_rpm": 7200,
      "power_on_hours": 8760,
      "status": "online",
      "in_use": true,
      "created_at": 1710489600,
      "updated_at": 1711440000
    },
    {
      "id": 2,
      "name": "Disk 2",
      "path": "/dev/sdb",
      "model": "WD Red Pro 4TB",
      "serial_number": "WD-WCC87654321",
      "type": "hdd",
      "size_bytes": 4398046511104,
      "size_human": "4.00 TB",
      "temperature": 37,
      "smart_status": "healthy",
      "health_status": "good",
      "speed_rpm": 7200,
      "power_on_hours": 8760,
      "status": "online",
      "in_use": true,
      "created_at": 1710489600,
      "updated_at": 1711440000
    },
    {
      "id": 3,
      "name": "Disk 3",
      "path": "/dev/nvme0n1",
      "model": "Samsung 970 EVO 1TB",
      "serial_number": "S464NX0M123456",
      "type": "nvme",
      "size_bytes": 1099511627776,
      "size_human": "1.00 TB",
      "temperature": 42,
      "smart_status": "healthy",
      "health_status": "good",
      "speed_rpm": null,
      "power_on_hours": 4380,
      "status": "online",
      "in_use": false,
      "created_at": 1710489600,
      "updated_at": 1711440000
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 7,
    "total_pages": 1
  }
}
```

### 错误响应

#### 401 Unauthorized - 未认证

```json
{
  "success": false,
  "error": "Missing or invalid Authorization header",
  "code": "UNAUTHORIZED"
}
```

#### 401 Unauthorized - Token 无效

```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

## 响应字段说明

### DiskInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| id | integer | 磁盘 ID |
| name | string | 磁盘名称（如 Disk 1） |
| path | string | 设备路径（如 /dev/sda） |
| model | string | 磁盘型号 |
| serial_number | string | 序列号 |
| type | string | 磁盘类型：hdd/ssd/nvme |
| size_bytes | integer | 容量（字节） |
| size_human | string | 容量（人类可读格式，如 4.00 TB） |
| temperature | integer/null | 温度（摄氏度） |
| smart_status | string | SMART 状态：healthy/warning/failed/unknown |
| health_status | string | 健康状态：good/warning/bad/unknown |
| speed_rpm | integer/null | 转速（RPM，SSD/NVMe 为 null） |
| power_on_hours | integer/null | 通电时间（小时） |
| status | string | 磁盘状态：online/offline |
| in_use | boolean | 是否正在使用 |
| created_at | integer | 创建时间（Unix 时间戳） |
| updated_at | integer | 更新时间（Unix 时间戳） |

### PaginationInfo

| 字段 | 类型 | 说明 |
|------|------|------|
| page | integer | 当前页码 |
| limit | integer | 每页数量 |
| total | integer | 总记录数 |
| total_pages | integer | 总页数 |

## 磁盘类型说明

| 类型 | 说明 | 特点 |
|------|------|------|
| hdd | 机械硬盘 | 容量大，速度较慢，有转速 |
| ssd | 固态硬盘（SATA） | 速度快，无噪音，无转速 |
| nvme | NVMe 固态硬盘 | 速度最快，M.2 接口 |

## SMART 状态说明

| 状态 | 说明 | 建议操作 |
|------|------|----------|
| healthy | 健康 | 正常使用 |
| warning | 警告 | 建议备份数据，准备更换 |
| failed | 故障 | 立即更换 |
| unknown | 未知 | 无法获取 SMART 信息 |

## 使用示例

### JavaScript 示例

```javascript
async function listStorageDisks(filters = {}) {
  const params = new URLSearchParams({
    page: filters.page || 1,
    limit: filters.limit || 20,
    ...(filters.disk_type && { disk_type: filters.disk_type }),
    ...(filters.smart_status && { smart_status: filters.smart_status }),
    ...(filters.status && { status: filters.status }),
  });

  const response = await fetch(
    `http://localhost:8080/api/v1/storage/disks?${params}`,
    {
      headers: {
        'Authorization': 'Bearer ' + jwtToken
      }
    }
  );

  const data = await response.json();
  
  if (data.success) {
    console.log('Disks:', data.data);
    console.log('Pagination:', data.pagination);
    return data.data;
  } else {
    console.error('Error:', data.error);
    throw new Error(data.error);
  }
}

// 获取所有磁盘
listStorageDisks();

// 筛选 HDD 磁盘
listStorageDisks({ disk_type: 'hdd' });

// 获取第 2 页，每页 10 条
listStorageDisks({ page: 2, limit: 10 });
```

### Python 示例

```python
import requests

def list_storage_disks(jwt_token, **filters):
    url = "http://localhost:8080/api/v1/storage/disks"
    headers = {
        "Authorization": f"Bearer {jwt_token}"
    }
    params = {
        "page": filters.get("page", 1),
        "limit": filters.get("limit", 20),
    }
    
    if "disk_type" in filters:
        params["disk_type"] = filters["disk_type"]
    if "smart_status" in filters:
        params["smart_status"] = filters["smart_status"]
    if "status" in filters:
        params["status"] = filters["status"]
    
    response = requests.get(url, headers=headers, params=params)
    data = response.json()
    
    if data["success"]:
        print(f"Found {data['pagination']['total']} disks")
        for disk in data["data"]:
            print(f"  - {disk['name']}: {disk['model']} ({disk['size_human']})")
        return data["data"]
    else:
        print(f"Error: {data['error']}")
        raise Exception(data["error"])

# 使用示例
list_storage_disks(admin_token)
list_storage_disks(admin_token, disk_type="hdd")
list_storage_disks(admin_token, page=2, limit=10)
```

## 安全特性

1. **JWT 认证**: 任意登录用户可访问（无需 admin 权限）
2. **数据脱敏**: 序列号部分隐藏（实际实现中）
3. **分页限制**: 单次请求最多返回 100 条记录

## 性能优化

1. **分页查询**: 避免一次性加载大量数据
2. **筛选参数**: 支持按类型、状态等筛选，减少传输数据量
3. **缓存建议**: 客户端可缓存磁盘列表，定期刷新

## 注意事项

1. **温度数据**: 部分磁盘可能不支持温度读取（返回 null）
2. **转速数据**: SSD/NVMe 磁盘无转速（返回 null）
3. **离线磁盘**: 离线磁盘的部分信息可能为 unknown
4. **实时性**: 磁盘状态为快照数据，非实时更新

## 相关接口

- `GET /api/v1/storage/disks/{id}` - 磁盘详情 (Phase 181)
- `GET /api/v1/storage/disks/{id}/health` - 磁盘健康状态
- `GET /api/v1/storage/usage` - 存储使用量汇总
- `GET /api/v1/storage/pools` - 存储池列表

## 实现文件

- `src/handlers/storage_disks.rs` - 磁盘列表处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 变更日志

| 版本 | 日期 | 变更内容 |
|------|------|----------|
| Phase 180 | 2026-03-27 | 初始实现：存储磁盘列表 API |

---

**兵部尚书 签发**
2026-03-27 14:00 UTC
