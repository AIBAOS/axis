# 第八十五轮主动测试报告 - Bug #78-#90批量发现

## 测试概要
- 测试范围：网络管理及相关模块分页参数边界测试
- 测试项数：15 | 通过：2 | Bug：13
- 编译状态：待验证
- 系统状态：🔴 发现13个分页参数边界Bug

## 发现Bug清单（批量问题）

### Bug类型：`per_page.unwrap_or(20)` 缺少 `.max(1)` 保护

| Bug ID | 文件 | 行号 | 问题 |
|--------|------|------|------|
| #78 | printers_list.rs | 87 | `unwrap_or(20)` 无 `.max(1)` |
| #79 | shares_ftp_list.rs | 70 | `unwrap_or(20).min(100)` 无 `.max(1)` |
| #80 | shares_list.rs | 91 | `unwrap_or(20).min(100)` 无 `.max(1)` |
| #81 | shares_nfs_list.rs | 80 | `unwrap_or(20).min(100)` 无 `.max(1)` |
| #82 | shares_webdav_list.rs | 69 | `unwrap_or(20).min(100)` 无 `.max(1)` |
| #83 | storage_pools_list.rs | 86 | `unwrap_or(20).min(100)` 无 `.max(1)` |
| #84 | storage_volume_snapshots_list.rs | 89 | `unwrap_or(20).min(100)` 无 `.max(1)` |
| #85 | storage_volumes.rs | 90 | `unwrap_or(20).min(100)` 无 `.max(1)` |
| #86 | system_alerts_list.rs | 79 | `unwrap_or(20).min(100)` 无 `.max(1)` |
| #87 | system_notifications_list.rs | 87 | `unwrap_or(20)` 无 `.max(1)` |
| #88 | updates.rs | 78 | `unwrap_or(20)` 无 `.max(1)` |
| #89 | usb_devices.rs | 56 | `unwrap_or(20)` 无 `.max(1)` |
| #90 | users_list.rs | 70 | `unwrap_or(20).min(100)` 无 `.max(1)` |

### 问题影响
- 用户传入 `per_page=0` 会导致：
  - 计算 `offset = (page - 1) * 0` → 永远为 0
  - 数据库查询 `LIMIT 0` → 返回空结果集
  - 分页计算异常

### 修复方案
所有handlers统一添加 `.max(1)`：
```rust
let per_page = query.per_page.unwrap_or(20).max(1).min(100);
```

## 已验证无Bug模块
- firewall_rules.rs: ✅ `.max(1).min(100)` 已修复
- containers_list.rs: ✅ `.max(1)` 已修复
- backups_list.rs: ✅ `.max(1).min(100)` 已修复

---

**测试时间**：2026-04-10 19:20 UTC
**测试工程师**：兵部于谦 🏹