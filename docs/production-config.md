# Axis NAS 生产环境配置文档

**版本：** v1.0.0  
**更新时间：** 2026-04-08  
**状态：** 生产就绪

---

## 目录

1. [系统依赖](#1-系统依赖)
2. [环境变量清单](#2-环境变量清单)
3. [配置文件](#3-配置文件)
4. [数据库初始化](#4-数据库初始化)
5. [系统服务配置](#5-系统服务配置)
6. [安全配置](#6-安全配置)
7. [监控与日志](#7-监控与日志)
8. [上线检查清单](#8-上线检查清单)

---

## 1. 系统依赖

### 1.1 Rust 环境

| 依赖 | 版本要求 | 说明 |
|------|---------|------|
| Rust | >= 1.75.0 | 使用 stable 工具链 |
| Cargo | >= 1.75.0 | Rust 包管理器 |

**安装命令：**
```bash
# 安装 Rust（如未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 验证版本
rustc --version  # 应输出：rustc 1.75.0+
cargo --version  # 应输出：cargo 1.75.0+
```

### 1.2 系统库

| 库 | 说明 |
|----|------|
| libssl-dev | OpenSSL 开发库 |
| pkg-config | 编译配置工具 |
| gcc | C 编译器 |
| make | 构建工具 |

**安装命令（Debian/Ubuntu）：**
```bash
sudo apt-get update
sudo apt-get install -y libssl-dev pkg-config gcc make
```

**安装命令（CentOS/RHEL）：**
```bash
sudo yum install -y openssl-devel pkgconfig gcc make
```

### 1.3 可选依赖

| 依赖 | 用途 |
|------|------|
| PostgreSQL | 如使用 PostgreSQL 数据库 |
| systemd | 系统服务管理 |

---

## 2. 环境变量清单

### 2.1 核心环境变量

| 变量名 | 必填 | 默认值 | 说明 | 示例 |
|--------|------|--------|------|------|
| `JWT_SECRET_KEY` | ✅ | - | JWT 签名密钥（至少 32 字符） | `your-32-char-secret-key-here` |
| `DATABASE_PATH` | ✅ | `NAS.db` | SQLite 数据库文件路径 | `/var/lib/axis/NAS.db` |
| `SERVER_HOST` | ❌ | `0.0.0.0` | 服务器监听地址 | `0.0.0.0` |
| `SERVER_PORT` | ❌ | `8080` | 服务器监听端口 | `8080` |
| `RUST_LOG` | ❌ | `info` | 日志级别 | `info` |

### 2.2 生产环境配置示例

```bash
# /etc/axis/axis.env

# JWT 配置（必须修改为随机生成的密钥）
export JWT_SECRET_KEY="your-super-secret-32-character-key-here"

# 数据库配置
export DATABASE_PATH="/var/lib/axis/NAS.db"

# 服务器配置
export SERVER_HOST="0.0.0.0"
export SERVER_PORT="8080"

# 日志配置
export RUST_LOG="info"
```

### 2.3 JWT 密钥生成

```bash
# 生成随机 JWT 密钥（32 字符）
openssl rand -base64 32

# 或使用 Python
python3 -c "import secrets; print(secrets.token_urlsafe(32))"
```

---

## 3. 配置文件

### 3.1 配置文件位置

| 环境 | 路径 |
|------|------|
| 开发环境 | `config.toml`（项目根目录） |
| 生产环境 | `/etc/axis/config.toml` |

### 3.2 生产环境配置文件

```toml
# /etc/axis/config.toml

# JWT 配置
[jwt]
# 生产环境必须使用环境变量 JWT_SECRET_KEY
secret_key = ""  # 留空，从环境变量读取
issuer = "axis-nas"
audience = "axis-nas-users"
expiration_minutes = 60
refresh_enabled = false

# 数据库配置
[database]
path = "/var/lib/axis/NAS.db"
max_connections = 10

# 服务器配置
[server]
host = "0.0.0.0"
port = 8080
```

### 3.3 文件权限设置

```bash
# 创建配置目录
sudo mkdir -p /etc/axis
sudo mkdir -p /var/lib/axis
sudo mkdir -p /var/log/axis

# 设置配置文件权限
sudo chown -R axis:axis /etc/axis
sudo chmod 600 /etc/axis/config.toml
sudo chmod 600 /etc/axis/axis.env

# 设置数据目录权限
sudo chown -R axis:axis /var/lib/axis
sudo chmod 700 /var/lib/axis

# 设置日志目录权限
sudo chown -R axis:axis /var/log/axis
sudo chmod 755 /var/log/axis
```

---

## 4. 数据库初始化

### 4.1 SQLite 数据库初始化

Axis NAS 使用 SQLite 数据库，首次启动时自动创建表结构。

**自动初始化流程：**
1. 首次启动时，系统自动检测数据库文件是否存在
2. 如不存在，自动创建 `NAS.db` 文件
3. 自动创建所有必需表（users, roles, permissions, sessions 等）
4. 自动插入系统角色（admin, user, guest）

**手动初始化（可选）：**

```sql
-- 初始化脚本：scripts/init_db.sql

-- 1. 创建数据库文件
-- SQLite 会在首次执行时自动创建

-- 2. 应用性能优化 PRAGMA
PRAGMA journal_mode = WAL;
PRAGMA cache_size = -64000;
PRAGMA synchronous = NORMAL;
PRAGMA temp_store = MEMORY;
PRAGMA foreign_keys = ON;
PRAGMA auto_vacuum = INCREMENTAL;
PRAGMA busy_timeout = 5000;

-- 3. 验证表结构
.tables

-- 4. 验证系统角色
SELECT * FROM roles;
```

**执行初始化：**
```bash
# 方式 1：首次启动应用时自动初始化
./target/release/axis

# 方式 2：手动执行初始化脚本
sqlite3 /var/lib/axis/NAS.db < scripts/init_db.sql
```

### 4.2 数据库备份

```bash
#!/bin/bash
# scripts/backup_db.sh

DB_PATH="/var/lib/axis/NAS.db"
BACKUP_DIR="/var/backup/axis"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p $BACKUP_DIR

# 备份数据库
cp $DB_PATH $BACKUP_DIR/NAS_$DATE.db

# 压缩备份
gzip $BACKUP_DIR/NAS_$DATE.db

# 清理 30 天前的备份
find $BACKUP_DIR -name "NAS_*.db.gz" -mtime +30 -delete

echo "Backup completed: NAS_$DATE.db.gz"
```

**添加到 crontab：**
```bash
# 每天凌晨 2 点备份
0 2 * * * /path/to/scripts/backup_db.sh
```

### 4.3 数据库迁移

如需从旧版本升级：

```bash
# 1. 备份当前数据库
cp /var/lib/axis/NAS.db /var/lib/axis/NAS.db.backup

# 2. 停止服务
sudo systemctl stop axis

# 3. 运行迁移脚本（如有）
./target/release/axis-migrate

# 4. 启动服务
sudo systemctl start axis
```

---

## 5. 系统服务配置

### 5.1 systemd 服务配置

```ini
# /etc/systemd/system/axis.service

[Unit]
Description=Axis NAS Service
After=network.target

[Service]
Type=simple
User=axis
Group=axis
WorkingDirectory=/opt/axis
EnvironmentFile=/etc/axis/axis.env
ExecStart=/opt/axis/axis
Restart=on-failure
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=axis

# 安全限制
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/axis /var/log/axis

[Install]
WantedBy=multi-user.target
```

### 5.2 服务管理命令

```bash
# 重新加载 systemd 配置
sudo systemctl daemon-reload

# 启用服务（开机自启）
sudo systemctl enable axis

# 启动服务
sudo systemctl start axis

# 停止服务
sudo systemctl stop axis

# 重启服务
sudo systemctl restart axis

# 查看服务状态
sudo systemctl status axis

# 查看日志
sudo journalctl -u axis -f
```

### 5.3 创建系统用户

```bash
# 创建 axis 系统用户（无登录权限）
sudo useradd -r -s /bin/false -d /opt/axis axis

# 验证用户创建
id axis
```

---

## 6. 安全配置

### 6.1 防火墙配置

**UFW（Ubuntu）：**
```bash
# 启用防火墙
sudo ufw enable

# 允许 SSH
sudo ufw allow 22/tcp

# 允许 Axis NAS 端口
sudo ufw allow 8080/tcp

# 查看状态
sudo ufw status
```

**firewalld（CentOS）：**
```bash
# 启用防火墙
sudo systemctl enable firewalld
sudo systemctl start firewalld

# 允许服务
sudo firewall-cmd --permanent --add-service=ssh
sudo firewall-cmd --permanent --add-port=8080/tcp

# 重载配置
sudo firewall-cmd --reload
```

### 6.2 SSL/TLS 配置（可选）

如使用反向代理（Nginx）：

```nginx
# /etc/nginx/sites-available/axis

server {
    listen 443 ssl http2;
    server_name nas.example.com;

    ssl_certificate /etc/letsencrypt/live/nas.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/nas.example.com/privkey.pem;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

# HTTP 重定向到 HTTPS
server {
    listen 80;
    server_name nas.example.com;
    return 301 https://$server_name$request_uri;
}
```

### 6.3 安全加固建议

1. **定期更新系统：**
   ```bash
   sudo apt-get update && sudo apt-get upgrade -y
   ```

2. **限制 SSH 访问：**
   ```bash
   # /etc/ssh/sshd_config
   PermitRootLogin no
   PasswordAuthentication no
   ```

3. **启用失败登录保护：**
   ```bash
   sudo apt-get install fail2ban
   sudo systemctl enable fail2ban
   ```

4. **定期审计日志：**
   ```bash
   sudo journalctl -u axis --since "24 hours ago"
   ```

---

## 7. 监控与日志

### 7.1 日志配置

**日志位置：**
- systemd 日志：`journalctl -u axis`
- 应用日志：`/var/log/axis/`

**日志级别：**
| 级别 | 说明 | 使用场景 |
|------|------|---------|
| error | 错误日志 | 生产环境默认 |
| warn | 警告日志 | 开发/测试 |
| info | 信息日志 | 开发/调试 |
| debug | 调试日志 | 开发/调试 |

**配置日志级别：**
```bash
# 在 /etc/axis/axis.env 中设置
export RUST_LOG="info"
```

### 7.2 监控指标

**关键指标：**
| 指标 | 告警阈值 | 说明 |
|------|---------|------|
| CPU 使用率 | >80% | 持续 5 分钟 |
| 内存使用率 | >80% | 持续 5 分钟 |
| 磁盘使用率 | >90% | 立即告警 |
| API 响应时间 (P99) | >500ms | 持续 10 分钟 |
| 错误率 | >1% | 持续 5 分钟 |

**监控脚本示例：**
```bash
#!/bin/bash
# scripts/health_check.sh

# 检查服务状态
if ! systemctl is-active --quiet axis; then
    echo "CRITICAL: Axis service is down"
    exit 2
fi

# 检查 API 响应
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/api/v1/system/health)
if [ "$RESPONSE" != "200" ]; then
    echo "CRITICAL: Health check failed (HTTP $RESPONSE)"
    exit 2
fi

echo "OK: Axis is healthy"
exit 0
```

### 7.3 日志轮转

```bash
# /etc/logrotate.d/axis

/var/log/axis/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0640 axis axis
    postrotate
        systemctl reload axis
    endscript
}
```

---

## 8. 上线检查清单

### 8.1 部署前检查

- [ ] 系统依赖已安装（Rust、系统库）
- [ ] 系统用户已创建（axis 用户）
- [ ] 目录结构已创建（/etc/axis, /var/lib/axis, /var/log/axis）
- [ ] 配置文件已部署（config.toml, axis.env）
- [ ] JWT_SECRET_KEY 已设置为随机密钥
- [ ] 文件权限已正确设置
- [ ] 防火墙规则已配置
- [ ] systemd 服务已配置

### 8.2 功能验证

- [ ] 服务启动成功（systemctl status axis）
- [ ] 健康检查通过（curl http://localhost:8080/api/v1/system/health）
- [ ] 登录功能正常
- [ ] 文件上传/下载正常
- [ ] 数据库备份脚本已配置

### 8.3 安全验证

- [ ] JWT_SECRET_KEY 不是默认值
- [ ] 配置文件权限为 600
- [ ] 数据库文件权限为 700
- [ ] 防火墙仅开放必要端口
- [ ] 日志中无敏感信息泄露

### 8.4 监控验证

- [ ] 日志正常输出
- [ ] 监控脚本运行正常
- [ ] 告警通知配置正确
- [ ] 备份脚本运行正常

### 8.5 性能验证

- [ ] API 响应时间 <200ms（P99）
- [ ] 并发承载 >100 用户
- [ ] 内存使用 <1GB
- [ ] CPU 使用率 <80%

---

## 附录

### A. 故障排查

**服务无法启动：**
```bash
# 查看详细日志
sudo journalctl -u axis -n 100 --no-pager

# 检查配置文件语法
cat /etc/axis/config.toml

# 检查环境变量
cat /etc/axis/axis.env

# 检查端口占用
sudo lsof -i :8080
```

**数据库连接失败：**
```bash
# 检查数据库文件权限
ls -la /var/lib/axis/NAS.db

# 检查磁盘空间
df -h /var/lib/axis

# 尝试修复数据库
sqlite3 /var/lib/axis/NAS.db "PRAGMA integrity_check;"
```

### B. 联系方式

- 项目仓库：https://github.com/AIBAOS/axis
- 问题反馈：GitHub Issues
- 文档：/docs 目录

---

**文档版本：** v1.0.0  
**最后更新：** 2026-04-08  
**维护者：** 兵部
