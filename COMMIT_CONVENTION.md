# Git Commit Message 规范

## 格式

```
<type>(<scope>): <subject>

<body>
```

## 类型 (type)

| 类型 | 说明 |
|------|------|
| feat | 新功能 |
| fix | 修复 bug |
| docs | 文档更新 |
| style | 代码格式（不影响逻辑） |
| refactor | 重构（不新增功能或修复 bug） |
| test | 测试相关 |
| chore | 构建/工具/依赖更新 |

## 范围 (scope)

可选，表示影响范围：

- `auth` - 认证模块
- `file` - 文件管理
- `session` - 会话管理
- `rbac` - 权限管理
- `storage` - 存储管理
- `api` - API 相关
- `config` - 配置文件

## 示例

```
feat(auth): 添加 JWT 刷新令牌功能

- 支持 refresh_token 自动续期
- 配置文件新增 token 过期时间设置
```

```
fix(file): 修复文件上传路径遍历漏洞

对文件名进行安全校验，禁止 .. 和绝对路径
```

```
docs: 添加 PROGRESS.md 项目进度追踪文件
```

## 要求

1. **描述代码的影响及解决的问题**
2. 禁止使用内部任务编号（如 Phase *）
3. 使用中文
4. 首行不超过 50 字符
5. 复杂改动在 body 详细说明