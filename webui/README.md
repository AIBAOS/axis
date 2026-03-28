# Axis NAS WebUI

Axis NAS 管理面板前端项目

## 技术栈

- Vue 3 + TypeScript
- Vite 5
- TailwindCSS 3
- Vue Router 4
- Pinia (状态管理)
- Axios (HTTP 客户端)

## 开发环境要求

- Node.js >= 18.0.0
- npm >= 9.0.0

## 快速开始

### 1. 安装依赖

```bash
npm install
```

### 2. 配置环境变量

复制 `.env.example` 到 `.env` 并修改配置：

```bash
cp .env.example .env
```

编辑 `.env` 文件：

```env
VITE_API_BASE_URL=http://localhost:8080
```

### 3. 启动开发服务器

```bash
npm run dev
```

访问 http://localhost:3000

### 4. 构建生产版本

```bash
npm run build
```

构建产物在 `dist/` 目录

### 5. 预览生产构建

```bash
npm run preview
```

## 项目结构

```
webui/
├── src/
│   ├── api/          # API 客户端
│   ├── components/   # 可复用组件
│   ├── router/       # 路由配置
│   ├── stores/       # Pinia 状态管理
│   ├── styles/       # 全局样式
│   ├── views/        # 页面组件
│   ├── App.vue       # 根组件
│   └── main.ts       # 入口文件
├── index.html
├── package.json
├── tailwind.config.js
├── vite.config.ts
└── tsconfig.json
```

## API 集成

API 客户端已配置在 `src/api/index.ts`，支持：

- JWT Token 自动注入
- 401 自动跳转登录
- 请求/响应拦截器

示例：

```typescript
import apiClient from '@/api'

// GET 请求
const response = await apiClient.get('/api/v1/backups')

// POST 请求
const response = await apiClient.post('/api/v1/backups', {
  name: 'Daily Backup',
  source_path: '/data',
  destination: '/backup/daily',
  backup_type: 'full'
})
```

## 开发规范

- 使用 TypeScript
- 使用 Composition API (`<script setup>`)
- 遵循 ESLint 规则
- 组件命名使用 PascalCase
- 文件命名使用 camelCase

## License

MIT
