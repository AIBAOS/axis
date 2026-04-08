# Axis NAS 部署脚本

本目录包含 Axis NAS 的自动化部署和维护脚本。

## 脚本清单

| 脚本 | 用途 | 使用方法 |
|------|------|---------|
| `install.sh` | 一键安装 | `sudo ./install.sh` |
| `upgrade.sh` | 升级版本 | `sudo ./upgrade.sh [版本号]` |
| `rollback.sh` | 回滚版本 | `sudo ./rollback.sh [备份路径]` |
| `backup.sh` | 备份数据 | `sudo ./backup.sh [备份目录]` |
| `restore.sh` | 恢复数据 | `sudo ./restore.sh <备份文件>` |

## 快速开始

### 一键安装

```bash
# 方式 1：直接执行（推荐）
curl -fsSL https://raw.githubusercontent.com/AIBAOS/axis/main/scripts/install.sh | sudo bash

# 方式 2：下载后执行
wget https://raw.githubusercontent.com/AIBAOS/axis/main/scripts/install.sh
chmod +x install.sh
sudo ./install.sh
```

### 升级到最新版本

```bash
sudo ./upgrade.sh
```

### 升级到指定版本

```bash
sudo ./upgrade.sh v1.1.0
```

### 回滚到之前的版本

```bash
# 回滚到最近的备份
sudo ./rollback.sh

# 回滚到指定备份
sudo ./rollback.sh /var/backup/axis/axis_backup_20260408_160000
```

### 备份数据

```bash
# 备份到默认目录
sudo ./backup.sh

# 备份到指定目录
sudo ./backup.sh /mnt/backup/axis

# 备份并保留 7 天
sudo ./backup.sh /mnt/backup/axis 7
```

### 恢复数据

```bash
# 恢复数据库
sudo ./restore.sh /var/backup/axis/NAS.db_20260408_160000.gz

# 恢复配置文件
sudo ./restore.sh /var/backup/axis/config_20260408_160000.tar.gz
```

## 定时备份

将备份脚本添加到 crontab：

```bash
# 编辑 crontab
sudo crontab -e

# 添加以下行（每天凌晨 2 点备份）
0 2 * * * /opt/axis/scripts/backup.sh /var/backup/axis
```

## 故障排查

### 安装失败

1. 检查系统依赖：
   ```bash
   # Ubuntu/Debian
   sudo apt-get install curl wget openssl ca-certificates
   
   # CentOS/RHEL
   sudo yum install curl wget openssl ca-certificates
   ```

2. 检查端口占用：
   ```bash
   sudo lsof -i :8080
   ```

3. 查看安装日志：
   ```bash
   sudo journalctl -u axis -n 100
   ```

### 升级失败

升级脚本会自动回滚到之前的版本。如果自动回滚失败，手动执行：

```bash
sudo ./rollback.sh
```

### 恢复失败

1. 确保服务已停止：
   ```bash
   sudo systemctl stop axis
   ```

2. 检查备份文件完整性：
   ```bash
   # 检查数据库备份
   gunzip -t /var/backup/axis/NAS.db_*.gz
   
   # 检查配置备份
   tar -tzf /var/backup/axis/config_*.tar.gz
   ```

3. 手动恢复：
   ```bash
   # 恢复数据库
   gunzip -c /var/backup/axis/NAS.db_20260408_160000.gz > /var/lib/axis/NAS.db
   
   # 恢复配置
   tar -xzf /var/backup/axis/config_20260408_160000.tar.gz -C /etc/
   ```

## 安全提示

1. **备份密钥**：定期备份 `/etc/axis/axis.env` 文件，特别是 `JWT_SECRET_KEY`

2. **备份验证**：定期测试恢复流程，确保备份可用

3. **离线备份**：将备份文件复制到离线存储设备

4. **权限检查**：确保脚本只有 root 可执行：
   ```bash
   sudo chmod 700 /opt/axis/scripts/*.sh
   ```

## 相关文档

- [生产环境配置文档](../docs/production-config.md)
- [部署文档](../docs/deployment.md)
- [运维手册](../docs/operations.md)

## 支持与反馈

如有问题，请提交 GitHub Issues：
https://github.com/AIBAOS/axis/issues

---

**版本：** v1.0.0  
**最后更新：** 2026-04-08  
**维护者：** 兵部
