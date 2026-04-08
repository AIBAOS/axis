# Axis NAS 生产环境部署指南

**版本：** v1.0.0  
**更新时间：** 2026-04-08  
**状态：** 生产就绪

---

## 目录

1. [系统要求](#1-系统要求)
2. [一键安装](#2-一键安装)
3. [手动安装](#3-手动安装)
4. [Docker 部署](#4-docker 部署)
5. [配置说明](#5-配置说明)
6. [服务管理](#6-服务管理)
7. [验证部署](#7-验证部署)
8. [故障排查](#8-故障排查)

---

## 1. 系统要求

### 1.1 硬件要求

| 配置 | 最低要求 | 推荐配置 |
|------|---------|---------|
| CPU | 2 核心 | 4 核心+ |
| 内存 | 2 GB | 4 GB+ |
| 磁盘 | 20 GB | 100 GB+ |
| 网络 | 100 Mbps | 1 Gbps+ |

### 1.2 操作系统

**支持的操作系统：**
- Ubuntu 20.04+ LTS
- Debian 10+
- CentOS 8+
- Rocky Linux 8+

### 1.3 系统依赖

| 依赖 | 版本要求 | 说明 |
|------|---------|------|
| Rust | >= 1.75.0 | Rust 工具链 |
| Cargo | >= 1.75.0 | Rust 包管理器 |
| libssl-dev | 最新 | OpenSSL 开发库 |
| pkg-config | 最新 | 编译配置工具 |
| gcc | 最新 | C 编译器 |
| make | 最新 | 构建工具 |

---

## 2. 一键安装

### 2.1 执行安装脚本

```bash
# 方式 1：直接执行（推荐）
curl -fsSL https://raw.githubusercontent.com/AIBAOS/axis/main/scripts/install.sh | sudo bash

# 方式 2：下载后执行
wget https://raw.githubusercontent.com/AIBAOS/axis/main/scripts/install.sh
chmod +x install.sh
sudo ./install.sh
```

### 2.2 安装过程

安装脚本将自动执行以下操作：

1. ✅ 检查系统依赖
2. ✅ 创建系统用户（axis）
3. ✅ 创建目录结构
4. ✅ 生成 JWT 密钥
5. ✅ 创建配置文件
6. ✅ 安装 systemd 服务
7. ✅ 配置防火墙
8. ✅ 启动服务

### 2.3 安装后验证

```bash
# 检查服务状态
sudo systemctl status axis

# 检查监听端口
sudo ss -tlnp | grep 8080

# 访问 WebUI
# http://<服务器 IP>:8080
```

---

## 3. 手动安装

### 3.1 安装系统依赖

**Ubuntu/Debian：**
```bash
sudo apt-get update
sudo apt-get install -y curl wget openssl ca-certificates
```

**CentOS/RHEL：**
```bash
sudo yum install -y curl wget openssl ca-certificates
```

### 3.2 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version  # 验证安装
```

### 3.3 编译源码

```bash
# 克隆仓库
git clone https://github.com/AIBAOS/axis.git
cd axis

# 编译发布版本
cargo build --release

# 复制二进制文件
sudo cp target/release/axis /usr/local/bin/
```

### 3.4 创建配置文件

```bash
# 创建配置目录
sudo mkdir -p /etc/axis
sudo mkdir -p /var/lib/axis
sudo mkdir -p /var/log/axis

# 创建配置文件
sudo tee /etc/axis/config.toml << EOF
[jwt]
secret_key = "$(openssl rand -base64 32)"
issuer = "axis-nas"
audience = "axis-nas-users"
expiration_minutes = 60
refresh_enabled = false

[database]
path = "/var/lib/axis/NAS.db"
max_connections = 10

[server]
host = "0.0.0.0"
port = 8080
EOF

# 设置权限
sudo chmod 600 /etc/axis/config.toml
sudo chown -R axis:axis /etc/axis /var/lib/axis /var/log/axis
```

### 3.5 创建 systemd 服务

```bash
sudo tee /etc/systemd/system/axis.service << EOF
[Unit]
Description=Axis NAS Service
After=network.target

[Service]
Type=simple
User=axis
Group=axis
WorkingDirectory=/opt/axis
EnvironmentFile=/etc/axis/axis.env
ExecStart=/usr/local/bin/axis
Restart=on-failure
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=axis

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable axis
sudo systemctl start axis
```

---

## 4. Docker 部署

### 4.1 Dockerfile

```dockerfile
FROM rust:1.75 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/axis /usr/local/bin/

EXPOSE 8080

VOLUME ["/var/lib/axis"]

CMD ["axis"]
```

### 4.2 docker-compose.yml

```yaml
version: '3.8'

services:
  axis:
    image: axis-nas:latest
    container_name: axis-nas
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - axis-data:/var/lib/axis
      - axis-config:/etc/axis
    environment:
      - JWT_SECRET_KEY=your-secret-key-here
      - DATABASE_PATH=/var/lib/axis/NAS.db
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/v1/system/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  axis-data:
  axis-config:
```

### 4.3 启动 Docker

```bash
# 构建镜像
docker-compose build

# 启动服务
docker-compose up -d

# 查看日志
docker-compose logs -f axis
```

---

## 5. 配置说明

### 5.1 环境变量

| 变量名 | 必填 | 默认值 | 说明 |
|--------|------|--------|------|
| `JWT_SECRET_KEY` | ✅ | - | JWT 签名密钥（至少 32 字符） |
| `DATABASE_PATH` | ✅ | `NAS.db` | SQLite 数据库路径 |
| `SERVER_HOST` | ❌ | `0.0.0.0` | 服务器监听地址 |
| `SERVER_PORT` | ❌ | `8080` | 服务器监听端口 |
| `RUST_LOG` | ❌ | `info` | 日志级别 |

### 5.2 配置文件

**config.toml 示例：**
```toml
[jwt]
secret_key = "your-super-secret-32-character-key"
issuer = "axis-nas"
audience = "axis-nas-users"
expiration_minutes = 60
refresh_enabled = false

[database]
path = "/var/lib/axis/NAS.db"
max_connections = 10

[server]
host = "0.0.0.0"
port = 8080
```

---

## 6. 服务管理

### 6.1 systemctl 命令

```bash
# 启动服务
sudo systemctl start axis

# 停止服务
sudo systemctl stop axis

# 重启服务
sudo systemctl restart axis

# 查看状态
sudo systemctl status axis

# 开机自启
sudo systemctl enable axis

# 禁用自启
sudo systemctl disable axis
```

### 6.2 查看日志

```bash
# 查看实时日志
sudo journalctl -u axis -f

# 查看最近 100 行
sudo journalctl -u axis -n 100

# 查看特定时间范围
sudo journalctl -u axis --since "2026-04-08 00:00:00"
```

---

## 7. 验证部署

### 7.1 健康检查

```bash
# API 健康检查
curl http://localhost:8080/api/v1/system/health

# 预期输出：
# {"success": true, "data": {...}}
```

### 7.2 功能验证

| 验证项 | 命令 | 预期结果 |
|--------|------|---------|
| 服务状态 | `systemctl status axis` | active (running) |
| 端口监听 | `ss -tlnp \| grep 8080` | LISTEN |
| WebUI 访问 | `http://<IP>:8080` | 登录页面 |
| API 响应 | `curl http://localhost:8080/api/v1/system/health` | 200 OK |

### 7.3 部署检查清单

- [ ] 服务正常运行
- [ ] 端口 8080 监听
- [ ] WebUI 可访问
- [ ] API 健康检查通过
- [ ] 日志正常输出
- [ ] 防火墙规则配置
- [ ] 开机自启配置
- [ ] 备份脚本配置

---

## 8. 故障排查

### 8.1 服务无法启动

```bash
# 查看详细错误
sudo journalctl -u axis -n 50 --no-pager

# 检查配置文件
sudo cat /etc/axis/config.toml

# 检查端口占用
sudo lsof -i :8080
```

### 8.2 数据库连接失败

```bash
# 检查数据库文件
ls -la /var/lib/axis/NAS.db

# 检查文件权限
sudo chown axis:axis /var/lib/axis/NAS.db
sudo chmod 600 /var/lib/axis/NAS.db
```

### 8.3 内存/CPU 过高

```bash
# 查看资源使用
top -p $(pgrep axis)

# 重启服务
sudo systemctl restart axis

# 检查日志
sudo journalctl -u axis --since "10 minutes ago"
```

---

## 附录

### A. 相关文件

| 文件 | 路径 | 说明 |
|------|------|------|
| 配置文件 | `/etc/axis/config.toml` | 主配置文件 |
| 环境变量 | `/etc/axis/axis.env` | 环境变量文件 |
| 数据库 | `/var/lib/axis/NAS.db` | SQLite 数据库 |
| 日志文件 | `journalctl -u axis` | 系统日志 |
| 服务文件 | `/etc/systemd/system/axis.service` | systemd 服务 |

### B. 默认端口

| 服务 | 端口 | 协议 |
|------|------|------|
| HTTP API | 8080 | TCP |
| HTTPS (可选) | 8443 | TCP |

### C. 联系方式

- 项目仓库：https://github.com/AIBAOS/axis
- 问题反馈：GitHub Issues
- 文档：/docs 目录

---

**文档版本：** v1.0.0  
**最后更新：** 2026-04-08  
**维护者：** 兵部
