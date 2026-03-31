# 第三十三轮主动测试报告 - 打印队列管理功能边界测试

> 测试时间：2026-03-31 02:00 UTC
> 测试方式：代码审计 + API 联调测试
> 测试人员：兵部尚书

## 📊 测试概要

| 项目 | 数据 |
|------|------|
| 测试范围 | 打印队列管理功能（前端 + 后端） |
| 测试场景数 | 6 个大类 |
| 发现 Bug 数 | 2 个 |
| 已修复 Bug 数 | 1 个 |

---

## 🔍 边界测试场景

### 场景 1: 空队列状态测试

**测试内容：** 无任务时的界面显示

**测试结果：✅ 通过**

- 正确显示"打印队列为空"提示
- 刷新按钮正常可用
- 自动刷新功能正常工作

---

### 场景 2: 单任务队列测试

**测试内容：** 上移/下移按钮禁用状态

**测试结果：✅ 通过**

```vue
<button @click="moveJobUp(job, index)" :disabled="index === 0" ...>
<button @click="moveJobDown(job, index)" :disabled="index === activeJobs.length - 1" ...>
```

- 首项上移按钮正确禁用
- 末项下移按钮正确禁用

---

### 场景 3: API 接口联调测试

**测试内容：** 检查前后端 API 是否匹配

**发现问题：**

| Bug ID | 描述 | 严重度 | 状态 |
|--------|------|:------:|:----:|
| **#69** | onUnmounted 未导入 | 🔴 高 | ✅ 已修复 |
| **#70** | 前端 API 路由与后端不匹配 | 🔴 高 | ⏳ 待修复 |

**Bug #70 详情：**

前端定义的 API：
```
PUT /api/v1/printers/queue/{id}/priority - 不存在！
DELETE /api/v1/printers/queue/{id} - 不存在！
GET /api/v1/printers/queue/{id} - 不存在！
```

后端实际路由：
```
GET /api/v1/printers/queue - 存在
GET /api/v1/printers/{id}/jobs/{job_id} - 存在
DELETE /api/v1/printers/{id}/jobs/{job_id} - 存在
```

---

### 场景 4: 生命周期清理测试

**测试内容：** 组件卸载时定时器清理

**测试结果：✅ 通过（修复后）**

- Bug #69 已修复：添加 `onUnmounted` 导入
- 定时器在组件卸载时正确清理

---

### 场景 5: 快速连续点击测试

**测试内容：** 频繁点击上移/下移按钮

**测试结果：⚠️ 存在风险**

**问题：** 没有防抖/节流机制，可能导致：
- 多次 API 调用
- 竞态条件

**建议：** 添加 loading 状态防止重复点击

---

### 场景 6: 网络异常测试

**测试内容：** 刷新时断网、API 失败重试

**测试结果：✅ 通过**

- 错误处理已实现
- Toast 提示正常显示

---

## 🔧 修复详情

### Bug #69: onUnmounted 未导入

**文件：** `webui/src/views/PrintersView.vue`

**修复：**
```typescript
import { ref, computed, onMounted, onUnmounted } from 'vue'
```

---

### Bug #70: 前端 API 路由与后端不匹配

**问题：** 前端调用的队列管理 API 在后端不存在

**建议修复方案：**

1. **方案 A：修改前端使用现有后端 API**
```typescript
// 使用现有的 jobs API
moveUp: (printerId: number, jobId: number) => 
  apiClient.put(`/api/v1/printers/${printerId}/jobs/${jobId}`, { priority: 'up' }),
```

2. **方案 B：添加后端 API 支持**
```rust
// 添加新路由
.route("/api/v1/printers/queue/{id}/priority", web::put().to(update_queue_priority))
.route("/api/v1/printers/queue/{id}", web::delete().to(cancel_queue_job))
```

---

## 📈 测试结论

**第三十三轮边界测试完成**

| 指标 | 结果 |
|------|:----:|
| 测试通过 | ⚠️ 有 Bug |
| 发现 Bug | 2 个 |
| 已修复 | 1 个 |
| 遗留问题 | 1 个（API 不匹配） |

---

## 🏹 兵部尚书签发

2026-03-31 02:05 UTC