# Axis NAS WebUI

<<<<<<< HEAD
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
=======
Axis NAS 管理面板前端 - Vue 3 + Vite + TailwindCSS

## 技术栈

- **框架**: Vue 3.4 + TypeScript
- **构建工具**: Vite 5
- **样式**: TailwindCSS 3
- **状态管理**: Pinia
- **路由**: Vue Router 4
- **HTTP 客户端**: Axios

## 快速开始

### 安装依赖

```bash
cd webui
npm install
```

### 配置环境变量
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)

```bash
cp .env.example .env
```

<<<<<<< HEAD
编辑 `.env` 文件：

```env
VITE_API_BASE_URL=http://localhost:8080
```

### 3. 启动开发服务器
=======
编辑 `.env` 文件，设置 API 地址：

```env
VITE_API_URL=http://localhost:8080
VITE_APP_VERSION=0.1.0
```

### 开发模式
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)

```bash
npm run dev
```

访问 http://localhost:3000

<<<<<<< HEAD
### 4. 构建生产版本
=======
### 构建生产版本
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)

```bash
npm run build
```

<<<<<<< HEAD
构建产物在 `dist/` 目录

### 5. 预览生产构建
=======
构建产物输出到 `dist/` 目录

### 预览生产构建
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)

```bash
npm run preview
```

## 项目结构

```
webui/
├── src/
<<<<<<< HEAD
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
=======
│   ├── assets/          # 静态资源
│   ├── components/      # Vue 组件
│   ├── layouts/         # 布局组件
│   ├── router/          # 路由配置
│   ├── stores/          # Pinia 状态管理
│   ├── styles/          # 全局样式
│   ├── utils/           # 工具函数
│   ├── views/           # 页面视图
│   ├── App.vue          # 根组件
│   └── main.ts          # 入口文件
├── index.html
├── package.json
├── vite.config.ts
├── tailwind.config.js
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)
└── tsconfig.json
```

## API 集成

<<<<<<< HEAD
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
=======
前端通过 `src/utils/api.ts` 与后端 API 通信。

默认 API 地址：`http://localhost:8080`

可通过环境变量 `VITE_API_URL` 配置。

## 开发规范

- 使用 TypeScript 严格模式
- 组件使用 `<script setup>` 语法
- 样式使用 TailwindCSS 工具类
- 状态管理使用 Pinia
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)

## License

MIT
