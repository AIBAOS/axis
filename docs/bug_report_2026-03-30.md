# Bug 报告 - 2026-03-30 主动测试

## Bug #45: users_create.rs 缺少用户名长度验证

**严重度:** 🟠 中等

**问题描述:**
- `users.rs` 有用户名长度验证 (3-50 字符)
- `users_create.rs` 只检查 `username.is_empty()`，缺少长度验证

**影响:**
- 可能创建超长用户名或过短用户名（1-2字符）

**对比:**
```rust
// users.rs (正确)
if payload.username.len() < 3 || payload.username.len() > 50 {
    return Ok(HttpResponse::BadRequest()...);
}

// users_create.rs (缺少)
if username.is_empty() {  // 只检查空，没有长度验证
    return Ok(HttpResponse::BadRequest()...);
}
```

**修复方案:**
在 `users_create.rs` 添加用户名长度验证 (3-50 字符)

---

## Bug #46: storage_pool_create.rs name 缺少特殊字符验证

**严重度:** 🟠 中等

**问题描述:**
- 只检查 `name.is_empty()` 和 `name.len() > 100`
- 没有检查特殊字符（路径遍历字符、控制字符）

**影响:**
- 可能创建包含 `..`、`/`、`\` 等危险字符的存储池名称
- 可能导致路径遍历或显示问题

**修复方案:**
参考 `shares_ftp_create.rs` 的 `validate_share_name` 函数，添加名称验证

---

## Bug #47: storage_volume_create.rs name 缺少验证

**严重度:** 🟠 中等

**问题描述:**
- 只检查 `name.is_empty()`
- 没有长度限制和特殊字符验证

**影响:**
- 可能创建超长卷名或包含特殊字符的卷名

**修复方案:**
添加长度限制 (1-64 字符) 和特殊字符验证

---

## 潜在问题（非 Bug）

### P1: MAX_FILE_SIZE = 100MB
- 文件上传限制为 100MB
- 对于 >1GB 文件会被拒绝
- **评估:** 这是合理的产品限制，非 bug

### P2: ALLOWED_EXTENSIONS 只有 10 种
- 只允许 txt, pdf, doc, docx, xls, xlsx, jpg, jpeg, png, gif
- **评估:** 这是安全限制，防止上传可执行文件，非 bug

---

## 测试结论

| 项目 | 结果 |
|------|------|
| Files API 边界测试 | ✅ 验证完善 |
| Storage API 异常输入 | ⚠️ 发现 Bug #46, #47 |
| Users API 安全测试 | ⚠️ 发现 Bug #45 |
| Toast 系统联调 | ✅ Phase 353 已统一 |

**需要修复:** Bug #45, #46, #47