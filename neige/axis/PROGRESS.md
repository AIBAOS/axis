# Axis Project - 开发进度

## Phase 1 ✅ 核心框架
- [x] actix-web 服务器配置
- [x] SQLite + r2d2 连接池
- [x] 基础项目结构

## Phase 2 ✅ JWT 认证模块
- [x] JWT 模型定义（JwtConfig/JwtClaims/JwtToken）
- [x] 用户/角色模型
- [x] JWT 服务层（token 生成/验证、密码哈希）
- [x] 用户登录逻辑（handlers/auth.rs）
- [x] JWT 中间件（middleware/jwt_auth.rs）
- [x] 配置文件加载（config.toml）

## Phase 2.5 ✅ 共享链接
- [x] 共享链接生成 API

## Phase 3 ✅ 文件管理 API
- [x] POST /api/v1/files/upload - 文件上传
- [x] GET /api/v1/files/download/{id} - 文件下载
- [x] DELETE /api/v1/files/{id} - 文件删除
- [x] GET /api/v1/files/list - 文件列表
- [x] PUT /api/v1/files/rename - 文件重命名
- [x] PUT /api/v1/files/move - 文件移动
- [x] POST /api/v1/files/copy - 文件复制

## Phase 3.2 ✅ 会话管理
- [x] GET /api/v1/sessions/current - 当前会话
- [x] GET /api/v1/sessions/list - 会话列表
- [x] DELETE /api/v1/sessions/{id} - 删除会话

## Phase 3.3 ✅ RBAC 集成
- [x] 角色 CRUD API
- [x] 权限管理 API
- [x] 用户角色绑定

## Phase 46 ✅ 文件搜索 API
- [x] GET /api/v1/files/search - 文件搜索
  - [x] 模糊匹配文件名（q 参数）
  - [x] 搜索路径指定（path 参数）
  - [x] 递归搜索（recursive 参数，最大 10 层）
  - [x] 文件类型筛选（file_type: file/folder/image/video/document）
  - [x] 修改时间范围筛选（modified_after/before）
  - [x] 分页支持（limit/offset）
  - [x] 安全校验（防止目录遍历攻击）
  - [x] 无结果返回 200 空列表
  - [x] 参数错误返回 400 Bad Request

## Phase 102 ✅ 创建用户 API
- [x] POST /api/v1/users - 创建用户

## Phase 103 ✅ 更新用户 API
- [x] PUT /api/v1/users/{id} - 更新用户
  - [x] JWT 认证（仅 admin 角色）
  - [x] 邮箱格式校验
  - [x] 角色有效性校验
  - [x] 用户存在性校验（404）
  - [x] 返回更新后的用户信息

## Phase 104 ⏳ 待开发
- [ ] DELETE /api/v1/users/{id} - 删除用户

## Phase 225 ✅ 用户列表 API
- [x] GET /api/v1/users - 用户列表

## Phase 226 ✅ 用户详情 API
- [x] GET /api/v1/users/{id} - 用户详情
  - [x] JWT 认证（登录用户）
  - [x] 权限控制（admin 可查看任意用户，普通用户仅可查看自己）
  - [x] 用户存在性校验（404 Not Found）
  - [x] 返回完整用户信息（不含密码）

## Phase 227 ⏳ 待开发
- [ ] DELETE /api/v1/users/{id} - 删除用户

## Phase 129 ✅ 网络接口列表
- [x] GET /api/v1/network/interfaces - 接口列表

## Phase 131 ✅ 网络接口详情
- [x] GET /api/v1/network/interfaces/{id} - 接口详情

## Phase 132 ✅ 网络接口创建 API
- [x] POST /api/v1/network/interfaces - 创建网络接口
  - [x] JWT 认证（仅 admin 角色）
  - [x] 接口名称唯一性校验（409 Conflict）
  - [x] 接口类型验证（ethernet/wifi/bridge/vlan）
  - [x] IP 地址格式校验（IPv4）
  - [x] MAC 地址格式校验（XX:XX:XX:XX:XX:XX）
  - [x] 返回 201 Created + 完整接口信息

## Phase 133 ⏳ 待开发
- [ ] PUT /api/v1/network/interfaces/{id} - 更新网络接口
- [ ] DELETE /api/v1/network/interfaces/{id} - 删除网络接口

## Phase 201 ✅ 创建 SMB 共享
- [x] POST /api/v1/shares/smb - 创建 SMB 共享

## Phase 202 ✅ SMB 共享列表 API
- [x] GET /api/v1/shares/smb - 共享列表
  - [x] JWT 认证（任意登录用户）
  - [x] 分页支持（page/limit，最大 100）
  - [x] 筛选支持（public 字段）
  - [x] 返回完整共享信息
  - [x] 分页元数据（total/total_pages）

## Phase 203 ⏳ 待开发
- [ ] GET /api/v1/shares/smb/{id} - 共享详情
- [ ] PUT /api/v1/shares/smb/{id} - 更新共享
- [ ] DELETE /api/v1/shares/smb/{id} - 删除共享

---

## 待办事项

### Phase 4 前瞻
- [ ] 文件版本控制
- [ ] 回收站功能
- [ ] 操作审计日志
- [ ] 多租户支持

### Phase 4.1 性能优化
- [ ] 搜索结果缓存
- [ ] 数据库索引优化
- [ ] 异步搜索队列

---

**最后更新**: 2026-03-28 08:10 UTC
**版本**: v0.22.6
