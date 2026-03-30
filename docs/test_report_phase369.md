# 第十八轮测试报告 - WebUI 与 API 联调测试

**测试时间:** 2026-03-30 17:32 UTC  
**测试方式:** 代码审计 + 前后端联调验证  
**测试人:** 兵部

---

## 测试范围

1. WebUI 前端与后端 API 数据交互验证
2. 前端表单提交与后端验证联动
3. 前端状态同步（操作后列表刷新）
4. 错误提示联动（API 错误 → 前端 Toast）
5. 认证状态同步（登录/登出/过期 → 前端跳转）

---

## 测试结果

| 模块 | 测试用例数 | 通过 | 失败 | Bug 数 |
|------|:----------:|:----:|:----:|:------:|
| 数据交互验证 | 5 | 5 | 0 | 0 |
| 表单验证联动 | 4 | 4 | 0 | 0 |
| 状态同步 | 3 | 3 | 0 | 0 |
| 错误提示联动 | 3 | 3 | 0 | 0 |
| **总计** | **15** | **15** | **0** | **0** |

---

## 详细测试用例

### 数据交互验证

| # | 测试场景 | 前端调用 | 后端 API | 状态 |
|:-:|----------|----------|----------|:----:|
| 1 | 用户列表 | `api.users.list()` | GET /api/v1/users | ✅ |
| 2 | 文件浏览 | `api.files.browse()` | GET /api/v1/files/browse | ✅ |
| 3 | 存储池列表 | `api.storage.getPools()` | GET /api/v1/storage/pools | ✅ |
| 4 | 共享列表 | `loadShares()` | GET /api/v1/shares | ✅ |
| 5 | 系统信息 | `api.system.info()` | GET /api/v1/system/info | ✅ |

### 表单验证联动

| # | 测试场景 | 前端验证 | 后端验证 | 一致性 |
|:-:|----------|----------|----------|:------:|
| 1 | 用户名 | 3-50字符, a-zA-Z0-9_- | 3-50字符, 字母数字下划线连字符 | ✅ |
| 2 | Pool名称 | 1-100字符, 禁止路径遍历 | 1-100字符, 字符验证 | ✅ |
| 3 | Volume名称 | 1-64字符, a-zA-Z0-9_- | 1-64字符, 字符验证 | ✅ |
| 4 | 密码强度 | 8-128字符, 大小写+数字 | 8-128字符, 大小写+数字 | ✅ |

### 状态同步

| # | 测试场景 | 操作后行为 | 状态 |
|:-:|----------|------------|:----:|
| 1 | 创建用户后 | 调用 `loadUsers()` 刷新列表 | ✅ |
| 2 | 上传文件后 | 调用 `loadFiles()` 刷新列表 | ✅ |
| 3 | 删除共享后 | 调用 `loadShares()` 刷新列表 | ✅ |

### 错误提示联动

| # | 测试场景 | 前端处理 | 状态 |
|:-:|----------|----------|:----:|
| 1 | API 错误 | `catch (error) { showToast('error', ...) }` | ✅ |
| 2 | 401 认证失败 | 清除 token, 跳转 /login | ✅ |
| 3 | 表单验证失败 | 显示红色边框 + 错误提示 | ✅ |

---

## 认证状态同步验证

### 登录流程

```
用户输入 → authStore.login()
  → api.auth.login(username, password)
  → 后端返回 { token, user }
  → localStorage.setItem('jwt_token', token)
  → 跳转到首页
```

### 登出流程

```
用户点击登出 → authStore.logout()
  → localStorage.removeItem('jwt_token')
  → 跳转到登录页
```

### Token 过期处理

```typescript
// client.ts 响应拦截器
if (error.response?.status === 401) {
  localStorage.removeItem('jwt_token')
  window.location.href = '/login'
}
```

**结论:** ✅ 认证状态同步正确

---

## Toast 使用统计

| View | showToast 调用数 |
|------|:----------------:|
| StorageView | 30 |
| FilesView | 24 |
| SettingsView | 40 |
| DownloadsView | 17 |
| JobsView | 15 |
| UsersView | 18 |
| PrintersView | 11 |
| NetworkView | 12 |
| LogsView | 7 |
| SharesView | 7 |
| BackupsView | 5 |
| AppsView | 13 |

**总计:** 199 处 showToast 调用

---

## 发现的 Bug

**无**

---

## 测试结论

✅ **全部通过**

- 前后端数据交互正确
- 表单验证与后端一致
- 操作后状态同步正确
- 错误提示联动完善
- 认证状态同步正确
- 发现 Bug 数: 0