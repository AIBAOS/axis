# 用户登出 API 文档 (Phase 100)

## 概述

用户登出 API 允许已认证用户安全地登出系统，使当前 Token 失效。

## 接口详情

### POST /api/v1/auth/logout

用户登出。

#### 认证要求

需要有效的 JWT Token，登录用户可访问。

**请求头：**
```
Authorization: Bearer <your_jwt_token>
```

#### 响应格式

**成功响应 (200 OK)**

```json
{
  "success": true,
  "message": "Logout successful"
}
```

**错误响应 (401 Unauthorized) - 未认证**

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 使用示例

### 示例 1：用户登出

```bash
curl -X POST "http://localhost:8080/api/v1/auth/logout" \
  -H "Authorization: Bearer <your_jwt_token>"
```

**响应：**
```json
{
  "success": true,
  "message": "Logout successful"
}
```

### 示例 2：未认证访问（401）

```bash
curl -X POST "http://localhost:8080/api/v1/auth/logout"
```

**响应：**
```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

### 示例 3：Token 已过期

```bash
curl -X POST "http://localhost:8080/api/v1/auth/logout" \
  -H "Authorization: Bearer <expired_token>"
```

**响应：**
```json
{
  "success": false,
  "error": "Invalid or expired token",
  "code": "UNAUTHORIZED"
}
```

## 安全特性

### 1. JWT 认证

- 必须提供有效的 JWT Token
- Token 过期或无效返回 401 Unauthorized

### 2. Token 失效

在实际实现中，登出操作应该：
- 将 Token 加入黑名单
- 或清除服务器端会话
- 或记录登出日志

### 3. 客户端处理

客户端在收到登出成功响应后应该：
- 清除本地存储的 Token
- 清除用户会话信息
- 重定向到登录页面

## 实现文件

- `src/handlers/users_logout.rs` - 用户登出处理器
- `src/handlers/mod.rs` - 模块导出
- `src/main.rs` - 路由注册

## 注意事项

1. **Token 失效**：JWT 是无状态的，服务端无法主动使 Token 失效
2. **Token 黑名单**：如需立即失效 Token，需实现 Token 黑名单机制
3. **客户端清理**：客户端必须清除本地存储的 Token
4. **审计日志**：建议记录所有登出操作到审计日志

## 错误代码列表

| 错误代码 | HTTP 状态码 | 说明 |
|---------|------------|------|
| `UNAUTHORIZED` | 401 | 未认证或 Token 无效 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |

## 相关 API

- **POST /api/v1/auth/login** - 用户登录
- **GET /api/v1/users** - 获取用户列表 (Phase 99)
- **POST /api/v1/auth/refresh** - 刷新 Token

## 响应示例（完整）

### 成功登出

```json
{
  "success": true,
  "message": "Logout successful"
}
```

### 未认证（401）

```json
{
  "success": false,
  "error": "Missing or invalid authorization token",
  "code": "UNAUTHORIZED"
}
```

## 最佳实践

### 1. 客户端清理

登出成功后，客户端应该：

```javascript
// 清除 Token
localStorage.removeItem('access_token');
sessionStorage.removeItem('access_token');

// 清除用户信息
localStorage.removeItem('user_info');

// 重定向到登录页面
window.location.href = '/login';
```

### 2. Token 黑名单

服务端实现 Token 黑名单：

```rust
// 将 Token 加入黑名单
// 设置过期时间等于 Token 剩余有效期
// 存储在 Redis 或其他快速存储中
```

### 3. 审计日志

记录登出操作：

```rust
// 记录以下信息到审计日志：
// - 用户 ID
// - 登出时间
// - IP 地址
// - User-Agent
// - Token ID（如果有）
```

### 4. 多设备登出

如需支持多设备登出：
- 为每个设备生成不同的 Token
- 提供按设备登出的接口
- 或提供全部设备登出的接口

## 前端集成示例

### React

```javascript
const handleLogout = async () => {
  try {
    const response = await fetch('/api/v1/auth/logout', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    
    if (response.ok) {
      // 清除 Token
      localStorage.removeItem('token');
      // 重定向到登录页
      history.push('/login');
    }
  } catch (error) {
    console.error('Logout failed:', error);
  }
};
```

### Vue

```javascript
const handleLogout = async () => {
  try {
    await api.post('/auth/logout');
    // 清除 Token
    localStorage.removeItem('token');
    // 清除 Vuex 状态
    store.commit('CLEAR_AUTH');
    // 重定向
    router.push('/login');
  } catch (error) {
    console.error('Logout failed:', error);
  }
};
```

## 安全性说明

### JWT 登出的挑战

由于 JWT 是无状态的，服务端无法主动使 Token 失效。解决方案：

1. **Token 黑名单**：将登出的 Token 加入黑名单
2. **短有效期**：使用较短的 Token 有效期
3. **Refresh Token**：使用 Refresh Token 机制
4. **客户端配合**：客户端必须清除 Token

### 推荐方案

- 使用短有效期 Access Token（如 15 分钟）
- 使用长有效期 Refresh Token（如 7 天）
- 登出时使 Refresh Token 失效
- Access Token 等待自然过期
