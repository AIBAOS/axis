# Axis 架构设计

**版本：** 0.1.0  
**最后更新：** 2026-03-14  
**状态：** 草案

---

## 🎯 设计目标

### 核心原则

1. **性能优先** — 充分利用 Rust 零成本抽象和异步运行时优势
2. **可靠性** — 完善的错误处理、数据校验和故障恢复机制
3. **可扩展性** — 模块化设计，支持协议插件化和水平扩展
4. **易维护性** — 清晰的代码结构、完善的文档和测试覆盖

### 非目标（当前版本）

- ❌ 分布式存储（v0.3+）
- ❌ 多租户支持（v0.2+）
- ❌ 对象存储协议（S3 兼容）

---

## 🏗️ 系统架构

### 整体分层

```
┌─────────────────────────────────────────────────┐
│              客户端层 (Clients)                  │
│    HTTP / WebDAV / SMB / FTP / 移动端            │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│              API 层 (api/)                       │
│   路由分发 · 请求解析 · 响应序列化 · 认证授权     │
│              技术栈：axum + serde                │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│           网络模块 (network/)                    │
│   连接管理 · 协议解析 · TLS 加密 · 限流熔断       │
│              技术栈：tokio + rustls              │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│           存储模块 (storage/)                    │
│   文件系统抽象 · IO 操作 · 缓存 · 数据校验        │
│              技术栈：tokio::fs + crc32           │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│           物理存储 (Physical Storage)            │
│         本地磁盘 / RAID / 网络存储               │
└─────────────────────────────────────────────────┘
```

### 横向支撑模块

```
┌─────────────────────────────────────────────────┐
│           配置管理 (config/)                     │
│   配置文件加载 · 环境变量 · 热更新 · 默认值       │
│              技术栈：config crate                │
└─────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│           错误处理 (error/)                      │
│   统一错误类型 · 错误转换 · 错误码定义            │
│              技术栈：thiserror                   │
└─────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│           日志系统 (logging)                     │
│   结构化日志 · 链路追踪 · 性能分析               │
│              技术栈：tracing + tracing-subscriber│
└─────────────────────────────────────────────────┘
```

---

## 📦 模块详细设计

### 1. API 层 (`src/api/`)

**职责：** HTTP 请求处理、路由分发、业务逻辑编排

**核心组件：**

```rust
// api/mod.rs
pub mod routes;
pub mod handlers;
pub mod middleware;

// api/routes.rs
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/files/*path", get(handlers::get_file))
        .route("/files/*path", put(handlers::put_file))
        .route("/files/*path", delete(handlers::delete_file))
        .layer(middleware::auth_layer())
        .layer(middleware::logging_layer())
        .with_state(state)
}
```

**关键接口：**

| 接口 | 方法 | 描述 |
|------|------|------|
| `/health` | GET | 健康检查 |
| `/files/*path` | GET | 获取文件/目录列表 |
| `/files/*path` | PUT | 上传文件 |
| `/files/*path` | DELETE | 删除文件 |
| `/files/*path` | POST | 创建目录 |

### 2. 网络模块 (`src/network/`)

**职责：** 连接管理、协议解析、TLS 加密

**设计要点：**

- 基于 Tokio 异步运行时，支持高并发连接
- 连接池管理，避免频繁创建/销毁连接
- TLS 1.3 支持，使用 rustls 提供加密
- 请求限流和熔断机制，防止过载

```rust
// network/connection.rs
pub struct Connection {
    id: Uuid,
    stream: TcpStream,
    tls: Option<TlsStream>,
    created_at: Instant,
    last_activity: Instant,
}

impl Connection {
    pub async fn accept(stream: TcpStream) -> Result<Self> {
        // TLS 握手
        // 连接注册
        // 返回连接对象
    }
    
    pub async fn send(&mut self, data: &[u8]) -> Result<usize> {
        // 发送数据
    }
    
    pub async fn recv(&mut self, buf: &mut [u8]) -> Result<usize> {
        // 接收数据
    }
}
```

### 3. 存储模块 (`src/storage/`)

**职责：** 文件系统抽象、IO 操作、缓存管理

**核心抽象：**

```rust
// storage/mod.rs
pub trait StorageBackend: Send + Sync {
    async fn read(&self, path: &Path, offset: u64, len: u64) -> Result<Vec<u8>>;
    async fn write(&self, path: &Path, offset: u64, data: &[u8]) -> Result<usize>;
    async fn delete(&self, path: &Path) -> Result<()>;
    async fn metadata(&self, path: &Path) -> Result<FileMetadata>;
    async fn list(&self, path: &Path) -> Result<Vec<FileMetadata>>;
}

// storage/filesystem.rs
pub struct FileSystemBackend {
    root: PathBuf,
    cache: Arc<Cache>,
}

impl StorageBackend for FileSystemBackend {
    // 实现本地文件系统存储
}
```

**缓存策略：**

- LRU 缓存热点文件元数据
- 写回（write-back）策略减少磁盘 IO
- 缓存一致性通过版本号保证

### 4. 配置管理 (`src/config.rs`)

**配置来源（优先级从高到低）：**

1. 命令行参数
2. 环境变量（`AXIS_*` 前缀）
3. 配置文件（`config.toml` / `config.yaml`）
4. 默认值

**配置结构：**

```toml
# config.toml 示例
[server]
host = "0.0.0.0"
port = 8080
tls_enabled = false
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"

[storage]
backend = "filesystem"
root_path = "/data/axis"
cache_size_mb = 1024

[logging]
level = "info"
format = "json"
output = "stdout"
```

### 5. 错误处理 (`src/error.rs`)

**错误分类：**

```rust
// error.rs
#[derive(thiserror::Error, Debug)]
pub enum AxisError {
    // 存储层错误
    #[error("文件未找到：{0}")]
    NotFound(String),
    
    #[error("权限不足：{0}")]
    PermissionDenied(String),
    
    #[error("磁盘空间不足")]
    NoSpaceLeft,
    
    // 网络层错误
    #[error("连接超时")]
    ConnectionTimeout,
    
    #[error("TLS 握手失败：{0}")]
    TlsError(#[from] rustls::Error),
    
    // API 层错误
    #[error("无效请求：{0}")]
    BadRequest(String),
    
    #[error("认证失败")]
    Unauthorized,
    
    // 内部错误
    #[error("内部错误：{0}")]
    Internal(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = std::result::Result<T, AxisError>;
```

**HTTP 状态码映射：**

| AxisError | HTTP 状态码 |
|-----------|------------|
| NotFound | 404 |
| PermissionDenied | 403 |
| NoSpaceLeft | 507 |
| BadRequest | 400 |
| Unauthorized | 401 |
| Internal | 500 |

---

## 🔐 安全设计

### 认证机制

- JWT Token 认证（v0.2+）
- 支持 Basic Auth（开发环境）
- API Key 认证（服务间调用）

### 授权模型

- 基于角色的访问控制（RBAC）
- 文件/目录级权限
- 读/写/执行/删除细粒度控制

### 数据安全

- TLS 1.3 传输加密
- 文件 checksum 校验（CRC32/SHA256）
- 敏感配置加密存储

---

## 📊 性能优化

### 异步 IO

- 全链路基于 Tokio 异步运行时
- 非阻塞文件 IO（`tokio::fs`）
- 连接池复用

### 缓存策略

```rust
// 多层缓存架构
L1: 内存缓存 (元数据，热点文件)
L2: SSD 缓存 (频繁访问文件)
L3: 磁盘存储 (全量数据)
```

### 并发控制

- 读写锁（`tokio::sync::RwLock`）
- 细粒度锁（文件级而非全局锁）
- 无锁数据结构（where possible）

---

## 🔄 扩展点

### 协议插件化

```rust
// 协议 trait
pub trait ProtocolHandler: Send + Sync {
    fn name(&self) -> &'static str;
    async fn handle_request(&self, ctx: RequestContext) -> Result<Response>;
}

// 注册协议
pub struct ProtocolRegistry {
    handlers: HashMap<String, Box<dyn ProtocolHandler>>,
}

impl ProtocolRegistry {
    pub fn register(&mut self, handler: Box<dyn ProtocolHandler>) {
        self.handlers.insert(handler.name().to_string(), handler);
    }
}
```

**内置协议：**
- HTTP/1.1 (v0.1)
- HTTP/2 (v0.2)
- WebDAV (v0.2)
- SMB (v0.3)

### 存储后端插件化

```rust
// 支持多种存储后端
pub enum StorageBackendType {
    FileSystem,
    S3Compatible,
    Distributed, // v0.3+
}
```

---

## 📈 监控与可观测性

### 指标收集

- 请求延迟（P50/P95/P99）
- QPS/TPS
- 错误率
- 缓存命中率
- 磁盘 IO 吞吐

### 链路追踪

- OpenTelemetry 集成
- 请求 ID 贯穿全链路
- 分布式追踪（v0.3+）

### 日志规范

```rust
// 结构化日志示例
tracing::info!(
    target: "axis::api",
    request_id = %request_id,
    method = %method,
    path = %path,
    status = %status,
    duration_ms = %duration,
    "请求完成"
);
```

---

## 🚧 技术债务

当前版本已知问题：

1. ❌ 缺少完整的错误恢复机制
2. ❌ 未实现断点续传
3. ❌ 缺少压力测试基准
4. ❌ 文档覆盖不足 60%

---

## 📚 参考资料

- [Tokio 异步编程指南](https://tokio.rs/tokio/tutorial)
- [Axum 官方文档](https://docs.rs/axum)
- [Rust 错误处理最佳实践](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [tracing 可观测性框架](https://tracing.rs)

---

*本文档由翰林院编写，内阁审议通过后生效。*
