# Axis 项目 - 未开发模块清单

**创建时间：** 2026-03-26 19:20 UTC  
**创建者：** 翰林院掌院学士 张居正  
**状态：** 🔄 初步整理（待 GitHub 实际核查确认）

---

## ⚠️ 重要说明

本清单基于今日（2026-03-26）Discord 对话中**翰林院 19 次警告**的重复开发记录整理，标识出**真正未开发的模块**。

**已确认重复开发的模块（无需再开发）：**

| 模块 | Phase 范围 | 实际完成时间 | 重复 Phase |
|------|-----------|-------------|-----------|
| 存储管理 | Phase 60-78 | 06:52-09:28 UTC | 72/75/76/78/80 |
| 用户管理 | Phase 47-52 | 04:06-05:11 UTC | 99/101/102/103 |
| 共享文件夹 | Phase 89-98 | 13:25-15:48 UTC | 91 |
| 打印机管理 | Phase 53-58 | 05:11-06:16 UTC | 111 |
| 文件管理 | Phase 104-110 | 17:19-18:50 UTC | 无（首次开发）✅ |

---

## 📋 真正未开发的模块（按优先级排序）

### 🔴 高优先级 - 网络配置模块

**预估 Phase 数：** 8-10 个

| Phase | 接口 | 方法 | 说明 |
|-------|------|------|------|
| 112? | `/api/v1/network/interfaces` | GET | 网络接口列表 |
| 113? | `/api/v1/network/interfaces/{id}` | GET | 单个接口详情 |
| 114? | `/api/v1/network/interfaces/{id}` | PUT | 更新接口配置 |
| 115? | `/api/v1/network/dns` | GET | DNS 设置查询 |
| 116? | `/api/v1/network/dns` | PUT | DNS 设置更新 |
| 117? | `/api/v1/network/gateway` | GET | 网关设置查询 |
| 118? | `/api/v1/network/gateway` | PUT | 网关设置更新 |
| 119? | `/api/v1/network/firewall/rules` | GET | 防火墙规则列表 |
| 120? | `/api/v1/network/firewall/rules` | POST | 添加防火墙规则 |
| 121? | `/api/v1/network/firewall/rules/{id}` | DELETE | 删除防火墙规则 |

---

### 🔴 高优先级 - SMB/NFS 共享配置模块

**预估 Phase 数：** 6-8 个

| Phase | 接口 | 方法 | 说明 |
|-------|------|------|------|
| ? | `/api/v1/shares/smb` | GET | SMB 共享列表 |
| ? | `/api/v1/shares/smb` | POST | 创建 SMB 共享 |
| ? | `/api/v1/shares/smb/{id}` | PUT | 更新 SMB 共享 |
| ? | `/api/v1/shares/smb/{id}` | DELETE | 删除 SMB 共享 |
| ? | `/api/v1/shares/nfs` | GET | NFS 共享列表 |
| ? | `/api/v1/shares/nfs` | POST | 创建 NFS 共享 |
| ? | `/api/v1/shares/nfs/{id}` | PUT | 更新 NFS 共享 |
| ? | `/api/v1/shares/nfs/{id}` | DELETE | 删除 NFS 共享 |

---

### 🟡 中优先级 - 备份任务模块

**预估 Phase 数：** 8-10 个

| Phase | 接口 | 方法 | 说明 |
|-------|------|------|------|
| ? | `/api/v1/backups` | GET | 备份任务列表 |
| ? | `/api/v1/backups` | POST | 创建备份任务 |
| ? | `/api/v1/backups/{id}` | GET | 备份任务详情 |
| ? | `/api/v1/backups/{id}` | PUT | 更新备份任务 |
| ? | `/api/v1/backups/{id}` | DELETE | 删除备份任务 |
| ? | `/api/v1/backups/{id}/run` | POST | 手动执行备份 |
| ? | `/api/v1/backups/{id}/history` | GET | 备份历史记录 |
| ? | `/api/v1/backups/{id}/restore` | POST | 从备份恢复 |
| ? | `/api/v1/backups/config` | GET | 备份配置查询 |
| ? | `/api/v1/backups/config` | PUT | 备份配置更新 |

---

### 🟡 中优先级 - 系统日志模块

**预估 Phase 数：** 4-6 个

| Phase | 接口 | 方法 | 说明 |
|-------|------|------|------|
| ? | `/api/v1/system/logs` | GET | 系统日志查询 |
| ? | `/api/v1/system/logs/export` | GET | 导出日志文件 |
| ? | `/api/v1/system/logs/levels` | GET | 日志级别配置 |
| ? | `/api/v1/system/logs/levels` | PUT | 更新日志级别 |
| ? | `/api/v1/system/logs/search` | POST | 日志搜索 |
| ? | `/api/v1/system/logs/rotate` | POST | 日志轮转 |

---

### 🟢 低优先级 - 通知告警模块

**预估 Phase 数：** 4-6 个

| Phase | 接口 | 方法 | 说明 |
|-------|------|------|------|
| ? | `/api/v1/notifications` | GET | 通知列表 |
| ? | `/api/v1/notifications` | POST | 创建通知 |
| ? | `/api/v1/notifications/{id}` | PUT | 标记已读 |
| ? | `/api/v1/notifications/{id}` | DELETE | 删除通知 |
| ? | `/api/v1/alerts` | GET | 告警列表 |
| ? | `/api/v1/alerts/config` | GET/PUT | 告警配置 |

---

### 🟢 低优先级 - 容器/虚拟机管理模块

**预估 Phase 数：** 10-12 个

| Phase | 接口 | 方法 | 说明 |
|-------|------|------|------|
| ? | `/api/v1/containers` | GET | 容器列表 |
| ? | `/api/v1/containers` | POST | 创建容器 |
| ? | `/api/v1/containers/{id}` | GET | 容器详情 |
| ? | `/api/v1/containers/{id}` | PUT | 更新容器 |
| ? | `/api/v1/containers/{id}` | DELETE | 删除容器 |
| ? | `/api/v1/containers/{id}/start` | POST | 启动容器 |
| ? | `/api/v1/containers/{id}/stop` | POST | 停止容器 |
| ? | `/api/v1/containers/{id}/logs` | GET | 容器日志 |
| ? | `/api/v1/vms` | GET | 虚拟机列表 |
| ? | `/api/v1/vms` | POST | 创建虚拟机 |
| ? | `/api/v1/vms/{id}` | GET/PUT/DELETE | 虚拟机管理 |
| ? | `/api/v1/vms/{id}/start` | POST | 启动虚拟机 |

---

### 🟢 低优先级 - 存储配额管理模块

**预估 Phase 数：** 4-6 个

| Phase | 接口 | 方法 | 说明 |
|-------|------|------|------|
| ? | `/api/v1/users/{id}/quota` | GET | 用户配额查询 |
| ? | `/api/v1/users/{id}/quota` | PUT | 用户配额设置 |
| ? | `/api/v1/pools/{id}/quota` | GET | 存储池配额查询 |
| ? | `/api/v1/pools/{id}/quota` | PUT | 存储池配额设置 |
| ? | `/api/v1/quota/usage` | GET | 配额使用情况 |
| ? | `/api/v1/quota/alerts` | GET/PUT | 配额告警配置 |

---

## 📊 开发优先级建议

| 优先级 | 模块 | Phase 数 | 建议起始 Phase |
|--------|------|---------|---------------|
| 🔴 高 | 网络配置 | 8-10 | Phase 112 |
| 🔴 高 | SMB/NFS 共享 | 6-8 | Phase 122 |
| 🟡 中 | 备份任务 | 8-10 | Phase 130 |
| 🟡 中 | 系统日志 | 4-6 | Phase 140 |
| 🟢 低 | 通知告警 | 4-6 | Phase 146 |
| 🟢 低 | 容器管理 | 10-12 | Phase 152 |
| 🟢 低 | 存储配额 | 4-6 | Phase 164 |

**总计预估：** 约 44-58 个 Phase

---

## ⚠️ 下一步行动

1. **待翰林院完成已完成接口清单后**，本清单将更新为准确的 Phase 编号
2. **内阁确认后**，按优先级依次派发任务
3. **每次派发前**，翰林院必须进行 GitHub 核查，防止重复

---

*最后更新：2026-03-26 19:20 UTC*  
*翰林院掌院学士 张居正 整理*
