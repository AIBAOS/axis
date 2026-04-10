# WebUI 全局加载状态优化

## 变更文件
- `webui/src/components/SkeletonCard.vue` - 卡片骨架屏组件（新建）
- `webui/src/components/SkeletonTable.vue` - 表格骨架屏组件（新建）
- `webui/src/views/BackupsView.vue` - 使用骨架屏替代loading图标
- `webui/src/views/ContainersView.vue` - 使用骨架屏替代loading图标

## 实现内容

### 1. 骨架屏组件
**SkeletonCard.vue**：
- 卡片头部骨架：图标 + 标题 + 状态徽章
- 卡片内容骨架：IP地址/MAC地址/速度/DHCP状态
- 操作按钮骨架：配置/测试按钮

**SkeletonTable.vue**：
- 表头骨架：列标题
- 表格行骨架：可配置行数和列数
- 动态宽度：根据列索引自动调整宽度

### 2. 页面优化
**BackupsView.vue**：
- loading状态：6个SkeletonCard网格布局
- 替代：animate-spin loading图标

**ContainersView.vue**：
- loading状态：6个SkeletonCard网格布局
- 替代：animate-spin loading图标

### 3. 动画效果
- `animate-pulse`：骨架屏脉冲动画
- `animate-spin`：按钮loading旋转图标
- 状态过渡：平滑切换

### 4. 响应式布局
- BackupsView：grid-cols-1 md:grid-cols-2 lg:grid-cols-3
- ContainersView：grid-cols-1 md:grid-cols-2 lg:grid-cols-3

## 编译状态
- pnpm build: ✅ 0 errors 0 warnings
- 构建大小：
  - BackupsView-BaANFjW0.js: 12.69 KB (gzip: 4.13 KB)
  - ContainersView-pDIMgrdf.js: 15.41 KB (gzip: 4.41 KB)

---

**更新时间**：2026-04-10 20:40 UTC
**工程师**：兵部于谦 🏹