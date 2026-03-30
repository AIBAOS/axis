# 第十一轮回归测试报告

**测试时间:** 2026-03-30 14:56 UTC  
**测试方式:** 代码审计 + 认证覆盖率验证  
**测试人:** 兵部

---

## 1. files_rename.rs 认证修复验证

| 函数 | 认证状态 | 验证结果 |
|------|:--------:|:--------:|
| rename_file | ✅ 已添加 | JWT Bearer token 验证 |
| move_file | ✅ 已添加 | 复用 rename_file 认证 |

**修复代码:**
```rust
let token = req.headers().get("Authorization")
    .and_then(|h| h.to_str().ok())
    .and_then(|s| s.strip_prefix("Bearer "))
    .ok_or_else(|| ErrorUnauthorized("Missing or invalid Authorization header"))?;

jwt_service.validate_token(token)
    .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;
```

---

## 2. 认证覆盖率统计

| 指标 | 数值 |
|------|:----:|
| 总 Handler 数 | 208 |
| 有认证 Handler | 195 |
| 无认证 Handler | 13 |
| 认证覆盖率 | **93%** |

---

## 3. 无认证 Handler 分析

### 合理无认证 (无需修复)

| 文件 | 原因 |
|------|------|
| `auth_login.rs` | 登录接口，提供 token |
| `mod.rs` | 模块文件，非 handler |
| `backups_archive.rs` | Stub 文件 (501) |
| `backups_delete.rs` | Stub 文件 (501) |
| `backups_detail.rs` | Stub 文件 (501) |
| `backups_execution_history.rs` | Stub 文件 (501) |
| `backups_restore.rs` | Stub 文件 (501) |
| `backups_stats.rs` | Stub 文件 (501) |
| `backups_update.rs` | Stub 文件 (501) |

### 需修复 (安全漏洞)

| 文件 | 函数数 | 严重度 |
|------|:------:|:------:|
| `logs_ex.rs` | 3 | 🟠 中等 |
| `rbac.rs` | 5 | 🔴 高危 |
| `scheduled_tasks.rs` | 7 | 🟠 中等 |
| `updates.rs` | 2 | 🟠 中等 |

---

## 4. 核心功能认证验证

| 功能 | Handler | 认证状态 |
|------|---------|:--------:|
| 文件上传 | files_upload.rs | ✅ |
| 文件下载 | files_download.rs | ✅ |
| 文件删除 | files_delete.rs | ✅ |
| 文件重命名 | files_rename.rs | ✅ 已修复 |
| 用户创建 | users_create.rs | ✅ |
| 用户删除 | users_delete.rs | ✅ |
| 存储池创建 | storage_pool_create.rs | ✅ |
| 存储池删除 | storage_pool_delete.rs | ✅ |

---

## 测试统计

| 项目 | 数量 |
|------|:----:|
| 验证修复项 | 1 |
| 认证覆盖率 | 93% |
| 发现新问题 | 4 个文件缺少认证 |

---

## 测试结论

**⚠️ 需关注**

- files_rename.rs 认证修复已验证 ✅
- 认证覆盖率 93%（4 个文件 17 个函数缺少认证）
- 建议：后续迭代修复 logs_ex.rs, rbac.rs, scheduled_tasks.rs, updates.rs