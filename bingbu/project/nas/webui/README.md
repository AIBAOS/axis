# Axis NAS WebUI

Axis NAS 系统的 Web 管理面板。

## 技术栈

- **框架**: Vue 3 (Composition API)
- **构建工具**: Vite
- **样式**: TailwindCSS
- **路由**: Vue Router
- **HTTP 客户端**: Axios

## 快速开始

### 安装依赖

```bash
npm install
```

### 配置环境变量

复制 `.env.example` 为 `.env` 并根据需要修改：

```bash
cp .env.example .env
```

### 开发模式

```bash
npm run dev
```

访问 http://localhost:3000

### 生产构建

```bash
npm run build
```

### 预览构建结果

```bash
npm run preview
```

## 项目结构

```
webui/
├── src/
│   ├── api/              # API 客户端
│   │   └── client.js
│   ├── components/       # 可复用组件
│   ├── layouts/          # 布局组件
│   │   └── DefaultLayout.vue
│   ├── router/           # 路由配置
│   │   └── index.js
│   ├── views/            # 页面组件
│   │   ├── Home.vue
│   │   ├── Login.vue
│   │   └── Dashboard.vue
│   ├── App.vue
│   ├── main.js
│   └── style.css
├── .env                  # 环境变量
├── .env.example          # 环境变量示例
├── index.html
├── package.json
├── postcss.config.js
├── tailwind.config.js
└── vite.config.js
```

## API 配置

默认 API 地址：`http://localhost:8080/api/v1`

在 `.env` 文件中修改 `VITE_API_BASE_URL` 来更改 API 地址。

## 功能规划

- [x] 项目基础框架
- [x] 路由配置
- [x] API 客户端（含 JWT 认证）
- [x] 默认布局
- [x] 首页
- [ ] 用户登录
- [ ] 控制面板
- [ ] 文件管理
- [ ] 存储管理
- [ ] 用户管理
- [ ] 备份管理
- [ ] 系统设置

## 开发规范

- 使用 Composition API (`<script setup>`)
- 组件命名使用 PascalCase
- 文件命名与组件名一致
- 使用 TailwindCSS 进行样式开发

## License

MIT
