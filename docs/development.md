# Axis 开发规范

**版本：** 0.1.0  
**生效日期：** 2026-03-14  
**适用范围：** 所有 Axis 项目贡献者

---

## 📋 代码风格

### Rust 代码规范

**格式化：**
```bash
# 提交前必须运行
cargo fmt
```

**代码检查：**
```bash
# 提交前必须通过
cargo clippy -- -D warnings
```

**命名约定：**

| 类型 | 规范 | 示例 |
|------|------|------|
| 包/crate | `kebab-case` | `axis-storage` |
| 模块/文件 | `snake_case` | `file_system.rs` |
| 类型/结构体 | `PascalCase` | `FileSystemBackend` |
| 函数/方法 | `snake_case` | `read_file` |
| 变量 | `snake_case` | `file_path` |
| 常量 | `SCREAMING_SNAKE_CASE` | `MAX_FILE_SIZE` |
| Trait | `PascalCase` | `StorageBackend` |
| 宏 | `snake_case!` | `ensure!` |

**代码组织：**

```rust
// 1. 模块声明
mod config;
mod error;
mod storage;

// 2. use 导入（按字母顺序）
use std::path::Path;
use tokio::fs::File;
use tracing::info;

// 3. 常量
const DEFAULT_PORT: u16 = 8080;

// 4. 静态变量
static VERSION: &str = env!("CARGO_PKG_VERSION");

// 5. 宏定义
macro_rules! ensure {
    // ...
}

// 6. 类型定义
pub struct Server {
    // 字段按字母顺序
    config: Config,
    port: u16,
}

// 7. 实现块
impl Server {
    // 方法按字母顺序
    pub async fn start(&self) -> Result<()> {
        // ...
    }
}

// 8. Trait 实现
impl Display for Server {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // ...
    }
}

// 9. 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_server_start() {
        // ...
    }
}
```

---

## 📝 注释规范

### 文档注释

**公共 API 必须有文档注释：**

```rust
/// 启动 HTTP 服务器
///
/// # 参数
/// * `config` - 服务器配置
///
/// # 返回
/// * `Ok(())` - 启动成功
/// * `Err(AxisError)` - 启动失败
///
/// # 示例
/// ```rust
/// let server = Server::new(config);
/// server.start().await?;
/// ```
///
/// # 错误
/// 可能返回以下错误：
/// - `AddressInUse` - 端口已被占用
/// - `PermissionDenied` - 权限不足
pub async fn start(&self, config: Config) -> Result<()> {
    // ...
}
```

### 内部注释

**复杂逻辑需要解释：**

```rust
// 使用双缓冲减少锁竞争
// 参考：https://en.wikipedia.org/wiki/Double_buffering
let mut buffer = self.buffer.write().await;
buffer.swap(&mut new_data);
```

**TODO 注释格式：**

```rust
// TODO(用户名): 描述待办事项 - 截止日期 (可选)
// TODO(zhangsan): 实现断点续传功能 - 2026-04-01
```

**FIXME 注释格式：**

```rust
// FIXME(用户名): 描述需要修复的问题 - 优先级 (HIGH/MEDIUM/LOW)
// FIXME(lisi): 内存泄漏问题 - HIGH
```

---

## 🧪 测试规范

### 测试组织

```rust
// src/storage/filesystem.rs

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    // 测试辅助函数
    fn create_temp_backend() -> (FileSystemBackend, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let backend = FileSystemBackend::new(temp_dir.path());
        (backend, temp_dir)
    }
    
    // 单元测试
    #[tokio::test]
    async fn test_write_and_read() {
        let (backend, _temp) = create_temp_backend();
        
        let data = b"hello world";
        let written = backend.write(Path::new("test.txt"), 0, data).await.unwrap();
        
        assert_eq!(written, 11);
        
        let read = backend.read(Path::new("test.txt"), 0, 11).await.unwrap();
        assert_eq!(read, data);
    }
    
    // 集成测试
    #[tokio::test]
    async fn test_concurrent_access() {
        // ...
    }
}
```

### 测试覆盖率要求

| 模块类型 | 最低覆盖率 |
|----------|-----------|
| 核心模块（storage/） | 80% |
| API 层（api/） | 70% |
| 工具模块 | 60% |

**生成覆盖率报告：**
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

---

## 📦 提交规范

### Git Commit Message

**格式：**
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type 类型：**

| 类型 | 描述 |
|------|------|
| `feat` | 新功能 |
| `fix` | Bug 修复 |
| `docs` | 文档更新 |
| `style` | 代码格式（不影响功能） |
| `refactor` | 重构（非新功能） |
| `test` | 测试相关 |
| `chore` | 构建/工具链 |
| `perf` | 性能优化 |
| `ci` | CI/CD 配置 |

**Scope 范围：**
- `api` - API 层
- `storage` - 存储模块
- `network` - 网络模块
- `config` - 配置管理
- `error` - 错误处理
- `deps` - 依赖更新
- `ci` - CI/CD

**示例：**

```bash
# 新功能
feat(storage): 添加文件缓存支持

- 实现 LRU 缓存策略
- 支持配置缓存大小
- 添加缓存命中率统计

Closes #45

# Bug 修复
fix(api): 修复大文件上传超时问题

- 增加超时时间配置
- 添加进度回调

Fixes #78

# 文档更新
docs: 更新 API 文档示例

# 依赖更新
chore(deps): 升级 tokio 到 1.35

# 重构
refactor(error): 统一错误处理逻辑

BREAKING CHANGE: AxisError 枚举结构变更
```

### Pull Request 流程

**1. 创建分支：**
```bash
git checkout -b feat/storage-cache
```

**2. 开发与提交：**
```bash
# 多次提交，遵循 commit message 规范
git add .
git commit -m "feat(storage): 添加缓存层"
```

**3. 推送与 PR：**
```bash
git push origin feat/storage-cache
# 在 GitHub 创建 Pull Request
```

**4. PR 模板：**
```markdown
## 变更说明
简要描述本次 PR 的目的

## 相关 Issue
Closes #123

## 测试计划
- [ ] 单元测试通过
- [ ] 集成测试通过
- [ ] 手动测试完成

## 检查清单
- [ ] 代码已格式化 (cargo fmt)
- [ ] Clippy 检查通过
- [ ] 文档已更新
- [ ] 测试覆盖率满足要求
```

**5. 代码审查：**
- 至少 1 名维护者批准
- 所有 CI 检查通过
- 无未解决的评论

**6. 合并：**
- 使用 Squash and Merge（保持历史整洁）
- 删除源分支

---

## 🔒 安全规范

### 敏感信息处理

**禁止硬编码敏感信息：**

```rust
// ❌ 错误示例
const API_KEY: &str = "sk-1234567890abcdef";

// ✅ 正确示例
let api_key = std::env::var("AXIS_API_KEY")
    .expect("AXIS_API_KEY must be set");
```

**配置文件脱敏：**

```toml
# config.example.toml (可提交到仓库)
[server]
api_key = "${AXIS_API_KEY}"  # 使用环境变量

# config.toml (不提交，加入 .gitignore)
[server]
api_key = "actual-key-here"
```

### 依赖安全

**定期审计：**
```bash
# 安装 cargo-audit
cargo install cargo-audit

# 运行审计
cargo audit

# 每日自动审计（CI/CD）
# .github/workflows/security.yml
```

**依赖更新策略：**
- 小版本更新：自动合并（依赖机器人）
- 大版本更新：人工审查后合并
- 安全更新：立即处理

---

## 📊 性能规范

### 基准测试

**关键路径必须有基准测试：**

```rust
// benches/storage_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_read_file(c: &mut Criterion) {
    let backend = create_test_backend();
    
    c.bench_function("read_1mb_file", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                backend.read(black_box(Path::new("test.bin")), 0, 1024 * 1024).await
            })
        })
    });
}

criterion_group!(benches, bench_read_file);
criterion_main!(benches);
```

**运行基准测试：**
```bash
cargo bench
```

### 性能指标

| 操作 | 目标延迟 (P99) |
|------|---------------|
| 小文件读取 (<1MB) | <10ms |
| 大文件读取 (>100MB) | <1s |
| 文件上传 | <100ms + 传输时间 |
| 目录列表 (<1000 项) | <50ms |

---

## 📚 文档规范

### 文档更新时机

**必须同步更新文档的场景：**
- 新增/修改 API 接口
- 变更配置选项
- 重构模块结构
- 修复重大 Bug

### 文档位置

| 文档类型 | 位置 |
|----------|------|
| README | `README.md` (根目录) |
| 架构设计 | `docs/architecture.md` |
| API 文档 | `docs/api.md` |
| 开发规范 | `docs/development.md` |
| 变更日志 | `CHANGELOG.md` |

### 文档风格

- 使用中文编写（技术术语保留英文）
- 代码示例必须可运行
- 截图/图表标注来源
- 最后更新日期置于文末

---

## 🚨 违规处理

**轻微违规（警告）：**
- 代码格式不规范
- 注释不完整
- 提交信息不规范

**中度违规（要求整改）：**
- 测试覆盖率不达标
- 文档未同步更新
- 未经审查直接合并

**严重违规（禁止合并）：**
- 硬编码敏感信息
- 绕过 CI/CD 检查
- 引入已知安全漏洞

---

## 📖 参考资料

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Pull Request 最佳实践](https://docs.github.com/en/pull-requests)

---

*本文档由翰林院编写，内阁审议通过后生效。修订需经内阁批准。*

*最后更新：2026-03-14*
