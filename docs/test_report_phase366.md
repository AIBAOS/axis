# 第十五轮测试报告 - 深度边界测试与异常输入

**测试时间:** 2026-03-30 16:53 UTC  
**测试方式:** 代码审计 + 安全检查  
**测试人:** 兵部

---

## 测试范围

1. 用户管理模块（创建/更新/删除边界）
2. 存储管理模块（卷/池名称边界、容量边界）
3. 文件操作模块（路径遍历、特殊字符）
4. 共享模块（SMB/NFS/WebDAV/FTP 配置边界）

---

## 测试结果

| 模块 | 测试用例数 | 通过 | 失败 | Bug 数 |
|------|:----------:|:----:|:----:|:------:|
| 深度边界测试 | 8 | 8 | 0 | 0 |
| 异常输入测试 | 6 | 6 | 0 | 0 |
| 安全测试 | 4 | 4 | 0 | 0 |
| **总计** | **18** | **18** | **0** | **0** |

---

## 详细测试用例

### 深度边界测试

| # | 测试场景 | 输入 | 预期结果 | 实际代码 | 状态 |
|:-:|----------|------|----------|----------|:----:|
| 1 | 文件大小上限 | 100MB+1 | 400 Bad Request | MAX_FILE_SIZE=100MB | ✅ |
| 2 | 文件大小下限 | 0 字节 | 400 Bad Request | MIN_FILE_SIZE=1 | ✅ |
| 3 | 分页上限 | per_page=1000 | 限制为100 | `.min(100)` | ✅ |
| 4 | 用户名下限 | 2字符 | 400 Bad Request | `.len() >= 3` | ✅ |
| 5 | 用户名上限 | 51字符 | 400 Bad Request | `.len() <= 50` | ✅ |
| 6 | Pool名称上限 | 101字符 | 400 Bad Request | `.len() <= 100` | ✅ |
| 7 | Volume名称上限 | 65字符 | 400 Bad Request | `.len() <= 64` | ✅ |
| 8 | 文件名上限 | 256字符 | 400 Bad Request | `.len() > 255` | ✅ |

### 异常输入测试

| # | 测试场景 | 输入 | 预期结果 | 实际代码 | 状态 |
|:-:|----------|------|----------|----------|:----:|
| 1 | 路径遍历 | `../etc/passwd` | 403 Forbidden | `contains("..")` | ✅ |
| 2 | null 字节 | `\0` | 400 Bad Request | `contains('\0')` | ✅ |
| 3 | 特殊字符用户名 | `test@user` | 400 Bad Request | 字符验证 | ✅ |
| 4 | 空必填字段 | username="" | 400 Bad Request | `is_empty()` | ✅ |
| 5 | 非法协议 | protocol="xxx" | 400 Bad Request | valid_protocols | ✅ |
| 6 | 非法状态 | status="invalid" | 400 Bad Request | valid_statuses | ✅ |

### 安全测试

| # | 测试场景 | 检查项 | 实际状态 | 状态 |
|:-:|----------|--------|----------|:----:|
| 1 | SQL 注入 | 参数化查询 | params![] 使用正确 | ✅ |
| 2 | XSS 攻击 | v-html/innerHTML | 未发现使用 | ✅ |
| 3 | 符号链接逃逸 | canonicalize | 已检查 starts_with | ✅ |
| 4 | 权限边界 | admin 检查 | is_admin() 检查 | ✅ |

---

## 安全检查详情

### SQL 注入防护

```rust
// 使用参数化查询
conn.execute("DELETE FROM user_roles WHERE user_id = ?1", params![user_id])?;
stmt.query_map(params![id], |row| ...)?
```

**结论:** ✅ 全部使用 params![] 参数化查询

### XSS 防护

```
检查 v-html/innerHTML/dangerouslySetInnerHTML: 未发现
```

**结论:** ✅ 无 XSS 风险

### 路径遍历防护

```rust
// files_browse.rs
if path.contains("..") {
    return 403 Forbidden;
}

if !canonical_target.starts_with(&canonical_base) {
    return 403 Forbidden;
}
```

**结论:** ✅ 路径遍历防护完善

---

## 发现的 Bug

**无**

---

## 测试结论

✅ **全部通过**

- 深度边界测试全部通过
- 异常输入测试全部通过
- 安全测试全部通过
- 发现 Bug 数: 0