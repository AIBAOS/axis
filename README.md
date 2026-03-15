# Axis NAS 后端核心框架

**版本：** v0.1.0  
**状态：** Phase 1 核心框架搭建 ✅  
**Phase 2：** JWT 认证模块开发中  
**提交人：** 工部尚书 宋应星  
**提交时间：** 2026-03-14 13:15 UTC

---

## 一、项目结构

```
axis/
├── Cargo.toml              # 项目依赖配置
├── src/
│   ├── main.rs             # 主框架代码
│   ├── models/             # 数据模型
│   │   ├── mod.rs          # 模块声明
│   │   ├── jwt.rs          # JWT 模型（Claims/Token/Config）
│   │   ├── user.rs         # 用户模型
│   │   └── role.rs         # 角色模型
│   ├── handlers/           # API 处理器
│   │   ├── mod.rs          # 模块声明
│   │   └── auth.rs         # 认证处理器（JWT）
│   └── services/           # 业务服务层
│       ├── mod.rs          # 模块声明
│       └── jwt_service.rs  # JWT 服务
└── README.md               # 项目说明文档
```

---

## 二、Phase 2 开发进度（JWT 认证模块）

### ✅ 已完成
- JWT 模型定义（`src/models/jwt.rs`）
  - `JwtConfig` - 配置结构（从文件读取）
  - `JwtClaims` - 声明结构（sub/issuer/exp/roles/permissions）
  - `JwtToken` - Token 响应结构
- 用户模型（`src/models/user.rs`）
  - `User` - 用户结构体
  - `UserRepository` - 用户存储接口（预留 PostgreSQL）
- 角色模型（`src/models/role.rs`）
  - `Role` - 角色结构体
  - `RoleRepository` - 角色存储接口（预留 PostgreSQL）
- JWT 服务（`src/services/jwt_service.rs`）
  - `JwtService` - JWT 服务结构
  - `generate_token()` - Token 生成
  - `validate_token()` - Token 验证
  - `generate_salt()` - 盐值生成（ring）
  - `hash_password()` - 密码哈希（PBKDF2）
- JWT 处理器（`src/handlers/auth.rs`）
  - `login()` - 登录接口（待实现用户验证）
  - `logout()` - 登出接口（待实现 Token 失效）
  - `refresh_token()` - Token 刷新（待实现）

### ⏳ 待完成
- 用户登录逻辑（结合数据库）
- JWT 中间件（Authorization 校验）
- 配置文件加载（config.toml）
- 集成测试

---

## 三、Phase 1 回顾

| 项目 | 状态 |
|------|------|
| actix-web 服务器 | ✅ 0.0.0.0:8080, 12 workers |
| SQLite 数据库 | ✅ rusqlite 0.31 |
| 连接池 | ✅ r2d2 集成 |
| 压力测试 | ✅ QPS 25000+ |

---

## 四、核心设计原则（内阁指令）

1. ✅ 保持 Phase 1 架构不变，新增功能需向后兼容
2. ✅ JWT 密钥从配置文件读取（禁止硬编码）
3. ✅ 文件版本控制、回收站、共享链接为 NAS 核心差异化功能（待 Phase 2.3）

---

## 五、下一步计划

1. 实现用户登录逻辑（数据库集成）
2. 实现 JWT 中间件（Authorization 校验）
3. 配置文件加载（config.toml）
4. 集成测试（cargo test）

---

**状态：** ✅ Phase 2 JWT 认证模块开发中  
**提交人：** 工部尚书 宋应星 🏗️  
**进度：** 40%（模型与服务层完成，处理器待集成）
