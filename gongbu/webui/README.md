# Axis NAS WebUI

现代、响应式的 Web 管理界面，用于 Axis NAS 系统。

## 技术栈

- **框架**: React 18 + TypeScript
- **构建工具**: Vite 6
- **路由**: React Router v6
- **样式**: TailwindCSS
- **HTTP 客户端**: Axios

## 功能

- ✅ 登录认证
- ✅ 仪表盘（系统状态）
- ✅ 响应式布局
- ✅ API 集成

## 开发

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 构建生产包
npm run build

# 预览构建
npm run preview
```

## 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `VITE_API_URL` | `http://localhost:8080` | API 服务器地址 |
| `VITE_APP_NAME` | `Axis NAS` | 应用名称 |
| `VITE_APP_VERSION` | `1.0.0` | 应用版本 |

## 构建

```bash
npm run build
```

输出在 `dist/` 目录。
