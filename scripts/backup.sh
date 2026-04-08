#!/bin/bash
#
# Axis NAS 备份脚本
#
# 用途：备份数据库和配置文件
#
# 使用方法：
#   sudo ./backup.sh [备份目录]
#
# 示例：
#   sudo ./backup.sh                    # 备份到默认目录
#   sudo ./backup.sh /mnt/backup/axis   # 备份到指定目录
#

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 配置变量
AXIS_DATA_DIR="/var/lib/axis"
AXIS_CONFIG_DIR="/etc/axis"
DEFAULT_BACKUP_DIR="/var/backup/axis"

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查 root 权限
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log_error "请使用 sudo 运行此脚本"
        exit 1
    fi
}

# 创建备份目录
create_backup_dir() {
    local BACKUP_DIR=$1
    
    if [ ! -d "$BACKUP_DIR" ]; then
        mkdir -p "$BACKUP_DIR"
        log_info "备份目录已创建：$BACKUP_DIR"
    fi
}

# 备份数据库
backup_database() {
    local BACKUP_DIR=$1
    local TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    
    log_info "正在备份数据库..."
    
    if [ -f "$AXIS_DATA_DIR/NAS.db" ]; then
        # 使用 WAL 模式备份（如果启用）
        if [ -f "$AXIS_DATA_DIR/NAS.db-wal" ]; then
            # 检查点操作，确保 WAL 文件合并到主数据库
            sqlite3 "$AXIS_DATA_DIR/NAS.db" "PRAGMA wal_checkpoint(TRUNCATE);" 2>/dev/null || true
        fi
        
        cp "$AXIS_DATA_DIR/NAS.db" "$BACKUP_DIR/NAS.db_$TIMESTAMP"
        
        # 压缩备份
        gzip "$BACKUP_DIR/NAS.db_$TIMESTAMP"
        
        log_success "数据库已备份：$BACKUP_DIR/NAS.db_$TIMESTAMP.gz"
    else
        log_warning "数据库文件不存在"
    fi
}

# 备份配置文件
backup_config() {
    local BACKUP_DIR=$1
    local TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    
    log_info "正在备份配置文件..."
    
    if [ -d "$AXIS_CONFIG_DIR" ]; then
        tar -czf "$BACKUP_DIR/config_$TIMESTAMP.tar.gz" -C "$(dirname $AXIS_CONFIG_DIR)" "$(basename $AXIS_CONFIG_DIR)"
        log_success "配置文件已备份：$BACKUP_DIR/config_$TIMESTAMP.tar.gz"
    else
        log_warning "配置目录不存在"
    fi
}

# 清理旧备份
cleanup_old_backups() {
    local BACKUP_DIR=$1
    local KEEP_DAYS=${2:-30}
    
    log_info "正在清理 $KEEP_DAYS 天前的备份..."
    
    find "$BACKUP_DIR" -name "NAS.db_*.gz" -mtime +$KEEP_DAYS -delete
    find "$BACKUP_DIR" -name "config_*.tar.gz" -mtime +$KEEP_DAYS -delete
    
    log_success "旧备份已清理"
}

# 显示备份信息
show_info() {
    local BACKUP_DIR=$1
    
    echo ""
    echo "========================================"
    echo -e "${GREEN}Axis NAS 备份完成！${NC}"
    echo "========================================"
    echo ""
    echo "备份目录：$BACKUP_DIR"
    echo ""
    echo "备份文件："
    ls -lh "$BACKUP_DIR"/*.* 2>/dev/null | tail -10
    echo ""
    echo "========================================"
}

# 主函数
main() {
    local BACKUP_DIR=${1:-$DEFAULT_BACKUP_DIR}
    local KEEP_DAYS=${2:-30}
    local TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    
    echo ""
    echo "========================================"
    echo "  Axis NAS 备份脚本"
    echo "  备份目录：$BACKUP_DIR"
    echo "========================================"
    echo ""
    
    check_root
    create_backup_dir "$BACKUP_DIR"
    backup_database "$BACKUP_DIR"
    backup_config "$BACKUP_DIR"
    cleanup_old_backups "$BACKUP_DIR" "$KEEP_DAYS"
    show_info "$BACKUP_DIR"
}

# 执行主函数
main "$@"
