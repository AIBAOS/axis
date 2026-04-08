# Axis NAS 备份策略配置指南

**版本：** v1.0.0  
**更新时间：** 2026-04-08  
**状态：** 生产就绪

---

## 目录

1. [备份策略](#1-备份策略)
2. [数据库备份](#2-数据库备份)
3. [文件备份](#3-文件备份)
4. [备份验证](#4-备份验证)
5. [灾难恢复](#5-灾难恢复)
6. [备份脚本](#6-备份脚本)

---

## 1. 备份策略

### 1.1 备份类型

| 类型 | 频率 | 保留期 | 说明 |
|------|------|--------|------|
| 完全备份 | 每周日 02:00 | 4 周 | 完整备份所有数据 |
| 增量备份 | 每日 02:00 | 7 天 | 仅备份变更数据 |
| 实时备份 | 持续 | 24 小时 | 实时同步重要数据 |

### 1.2 备份内容

| 数据类型 | 备份方式 | 优先级 |
|---------|---------|--------|
| 数据库 (NAS.db) | 完全 + 增量 | 🔴 高 |
| 用户上传文件 | 完全 + 增量 | 🔴 高 |
| 配置文件 | 完全备份 | 🟡 中 |
| 日志文件 | 不备份 | ⚪ 低 |

### 1.3 备份保留策略

**3-2-1 备份原则：**
- 保留 3 份备份副本
- 使用 2 种不同存储介质
- 1 份异地备份

**保留期：**
- 每日备份：保留 7 天
- 每周备份：保留 4 周
- 每月备份：保留 12 个月

---

## 2. 数据库备份

### 2.1 SQLite 备份命令

```bash
# 备份数据库
cp /var/lib/axis/NAS.db /backup/NAS.db.$(date +%Y%m%d_%H%M%S)

# 压缩备份
tar -czf /backup/NAS.db.$(date +%Y%m%d_%H%M%S).tar.gz /var/lib/axis/NAS.db

# 验证备份
tar -tzf /backup/NAS.db.$(date +%Y%m%d_%H%M%S).tar.gz
```

### 2.2 自动化备份脚本

**backup-database.sh：**

```bash
#!/bin/bash

BACKUP_DIR="/backup/database"
DATE=$(date +%Y%m%d_%H%M%S)
DB_PATH="/var/lib/axis/NAS.db"

# 创建备份目录
mkdir -p $BACKUP_DIR

# 备份数据库
cp $DB_PATH $BACKUP_DIR/NAS.db.$DATE

# 压缩备份
cd $BACKUP_DIR
tar -czf NAS.db.$DATE.tar.gz NAS.db.$DATE
rm NAS.db.$DATE

# 清理 7 天前的备份
find $BACKUP_DIR -name "NAS.db.*.tar.gz" -mtime +7 -delete

echo "数据库备份完成：$BACKUP_DIR/NAS.db.$DATE.tar.gz"
```

### 2.3 定时任务配置

**crontab 配置：**

```bash
# 每日凌晨 2 点备份数据库
0 2 * * * /opt/axis/scripts/backup-database.sh

# 每周日凌晨 1 点完全备份
0 1 * * 0 /opt/axis/scripts/backup-full.sh
```

---

## 3. 文件备份

### 3.1 文件备份命令

```bash
# 使用 rsync 备份
rsync -avz /var/lib/axis/files/ /backup/files/

# 使用 tar 备份
tar -czf /backup/files.$(date +%Y%m%d_%H%M%S).tar.gz /var/lib/axis/files/
```

### 3.2 自动化备份脚本

**backup-files.sh：**

```bash
#!/bin/bash

BACKUP_DIR="/backup/files"
DATE=$(date +%Y%m%d_%H%M%S)
SOURCE_DIR="/var/lib/axis/files"

# 创建备份目录
mkdir -p $BACKUP_DIR

# 使用 rsync 增量备份
rsync -avz --delete $SOURCE_DIR/ $BACKUP_DIR/current/

# 创建快照
cp -al $BACKUP_DIR/current $BACKUP_DIR/snapshot.$DATE

# 清理 7 天前的快照
find $BACKUP_DIR -name "snapshot.*" -mtime +7 -exec rm -rf {} \;

echo "文件备份完成：$BACKUP_DIR/snapshot.$DATE"
```

---

## 4. 备份验证

### 4.1 备份完整性验证

```bash
#!/bin/bash

BACKUP_FILE=$1

# 验证 tar.gz 文件
if tar -tzf $BACKUP_FILE > /dev/null 2>&1; then
    echo "✅ 备份文件完整：$BACKUP_FILE"
else
    echo "❌ 备份文件损坏：$BACKUP_FILE"
    exit 1
fi

# 验证数据库文件
if [ -f "$BACKUP_FILE" ]; then
    # SQLite 完整性检查
    sqlite3 $BACKUP_FILE "PRAGMA integrity_check;"
fi
```

### 4.2 恢复测试

**定期恢复测试（每月）：**

```bash
#!/bin/bash

# 1. 准备测试环境
TEST_DB="/tmp/test_NAS.db"
BACKUP_FILE="/backup/database/NAS.db.latest.tar.gz"

# 2. 解压备份
tar -xzf $BACKUP_FILE -C /tmp/

# 3. 验证数据库
sqlite3 $TEST_DB "PRAGMA integrity_check;"

# 4. 清理测试环境
rm -f $TEST_DB

echo "恢复测试完成"
```

---

## 5. 灾难恢复

### 5.1 恢复流程

**1. 停止服务：**

```bash
sudo systemctl stop axis
```

**2. 恢复数据库：**

```bash
# 解压备份
tar -xzf /backup/NAS.db.20260408_020000.tar.gz -C /var/lib/axis/

# 设置权限
sudo chown axis:axis /var/lib/axis/NAS.db
sudo chmod 600 /var/lib/axis/NAS.db
```

**3. 恢复文件：**

```bash
# 恢复文件备份
rsync -avz /backup/files/current/ /var/lib/axis/files/

# 设置权限
sudo chown -R axis:axis /var/lib/axis/files
```

**4. 启动服务：**

```bash
sudo systemctl start axis
sudo systemctl status axis
```

**5. 验证恢复：**

```bash
# 检查服务状态
sudo systemctl status axis

# 检查 API
curl http://localhost:8080/api/v1/system/health

# 检查 WebUI
curl http://localhost:8080
```

### 5.2 灾难恢复计划

| 场景 | 恢复时间目标 (RTO) | 恢复点目标 (RPO) |
|------|------------------|----------------|
| 数据库损坏 | < 1 小时 | < 24 小时 |
| 文件丢失 | < 2 小时 | < 24 小时 |
| 服务器宕机 | < 4 小时 | < 24 小时 |
| 数据中心灾难 | < 24 小时 | < 7 天 |

---

## 6. 备份脚本

### 6.1 完全备份脚本

**backup-full.sh：**

```bash
#!/bin/bash

BACKUP_DIR="/backup/full"
DATE=$(date +%Y%m%d_%H%M%S)

# 创建备份目录
mkdir -p $BACKUP_DIR

# 备份数据库
cp /var/lib/axis/NAS.db $BACKUP_DIR/NAS.db.$DATE
tar -czf $BACKUP_DIR/NAS.db.$DATE.tar.gz $BACKUP_DIR/NAS.db.$DATE
rm $BACKUP_DIR/NAS.db.$DATE

# 备份文件
tar -czf $BACKUP_DIR/files.$DATE.tar.gz /var/lib/axis/files/

# 备份配置
tar -czf $BACKUP_DIR/config.$DATE.tar.gz /etc/axis/

# 清理 4 周前的备份
find $BACKUP_DIR -name "*.tar.gz" -mtime +28 -delete

echo "完全备份完成：$BACKUP_DIR"
```

### 6.2 增量备份脚本

**backup-incremental.sh：**

```bash
#!/bin/bash

BACKUP_DIR="/backup/incremental"
DATE=$(date +%Y%m%d_%H%M%S)
REFERENCE="/backup/incremental/.reference"

# 创建备份目录
mkdir -p $BACKUP_DIR

# 增量备份数据库
if [ -f "$REFERENCE" ]; then
    rsync -avz --link-dest=$REFERENCE /var/lib/axis/NAS.db $BACKUP_DIR/NAS.db.$DATE/
else
    cp -r /var/lib/axis/NAS.db $BACKUP_DIR/NAS.db.$DATE/
fi

# 更新引用
rm -f $REFERENCE
ln -s $BACKUP_DIR/NAS.db.$DATE $REFERENCE

# 清理 7 天前的备份
find $BACKUP_DIR -maxdepth 1 -name "NAS.db.*" -mtime +7 -exec rm -rf {} \;

echo "增量备份完成：$BACKUP_DIR/NAS.db.$DATE"
```

### 6.3 备份验证脚本

**verify-backup.sh：**

```bash
#!/bin/bash

BACKUP_DIR="/backup"
ERROR_COUNT=0

echo "开始验证备份文件..."

# 验证数据库备份
for file in $BACKUP_DIR/database/*.tar.gz; do
    if tar -tzf "$file" > /dev/null 2>&1; then
        echo "✅ $file"
    else
        echo "❌ $file"
        ((ERROR_COUNT++))
    fi
done

# 验证文件备份
for file in $BACKUP_DIR/files/*.tar.gz; do
    if tar -tzf "$file" > /dev/null 2>&1; then
        echo "✅ $file"
    else
        echo "❌ $file"
        ((ERROR_COUNT++))
    fi
done

echo ""
if [ $ERROR_COUNT -eq 0 ]; then
    echo "✅ 所有备份文件验证通过"
    exit 0
else
    echo "❌ $ERROR_COUNT 个备份文件验证失败"
    exit 1
fi
```

---

## 附录

### A. 备份存储建议

| 存储类型 | 容量建议 | 性能要求 | 成本 |
|---------|---------|---------|------|
| 本地 SSD | 系统盘 2 倍 | 高 | 高 |
| NAS | 系统盘 5 倍 | 中 | 中 |
| 云存储 | 系统盘 3 倍 | 低 | 低 |

### B. 备份监控指标

| 指标 | 阈值 | 告警级别 |
|------|------|---------|
| 备份失败 | 1 次 | 🔴 严重 |
| 备份延迟 | > 2 小时 | 🟡 警告 |
| 备份大小异常 | < 50% 或 > 200% | 🟡 警告 |
| 备份验证失败 | 1 次 | 🔴 严重 |

### C. 联系方式

- 项目仓库：https://github.com/AIBAOS/axis
- 问题反馈：GitHub Issues
- 文档：/docs 目录

---

**文档版本：** v1.0.0  
**最后更新：** 2026-04-08  
**维护者：** 兵部
