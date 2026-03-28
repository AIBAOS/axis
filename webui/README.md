# Axis NAS WebUI

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

```bash
cp .env.example .env
```

编辑 `.env` 文件，设置 API 地址：

```env
VITE_API_URL=http://localhost:8080
VITE_APP_VERSION=0.1.0
```

### 开发模式

```bash
npm run dev
```

访问 http://localhost:3000

### 构建生产版本

```bash
npm run build
```

构建产物输出到 `dist/` 目录

### 预览生产构建

```bash
npm run preview
```

## 项目结构

```
webui/
├── src/
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
└── tsconfig.json
```

## API 集成

前端通过 `src/utils/api.ts` 与后端 API 通信。

默认 API 地址：`http://localhost:8080`

可通过环境变量 `VITE_API_URL` 配置。

## 开发规范

- 使用 TypeScript 严格模式
- 组件使用 `<script setup>` 语法
- 样式使用 TailwindCSS 工具类
- 状态管理使用 Pinia

## License

MIT
