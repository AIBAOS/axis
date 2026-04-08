#!/bin/bash
#
# Axis NAS 回滚脚本
#
# 用途：回滚到之前的版本（升级失败时使用）
#
# 使用方法：
#   sudo ./rollback.sh [备份路径]
#
# 示例：
#   sudo ./rollback.sh                      # 回滚到最近的备份
#   sudo ./rollback.sh /var/backup/axis/axis_backup_20260408_160000
#

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 配置变量
AXIS_HOME="/opt/axis"
AXIS_SERVICE="axis"
BACKUP_DIR="/var/backup/axis"

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

# 查找最近的备份
find_latest_backup() {
    log_info "正在查找最近的备份..."
    
    LATEST_BACKUP=$(ls -t "$BACKUP_DIR"/axis_backup_* 2>/dev/null | head -n1)
    
    if [ -z "$LATEST_BACKUP" ]; then
        log_error "未找到备份文件"
        exit 1
    fi
    
    log_info "找到备份：$LATEST_BACKUP"
    echo "$LATEST_BACKUP"
}

# 停止服务
stop_service() {
    log_info "正在停止 Axis NAS 服务..."
    systemctl stop $AXIS_SERVICE
    sleep 2
    log_success "服务已停止"
}

# 执行回滚
do_rollback() {
    local BACKUP_PATH=$1
    
    log_info "正在回滚到版本：$BACKUP_PATH"
    
    # 恢复二进制文件
    if [ -f "$BACKUP_PATH" ]; then
        cp "$BACKUP_PATH" "$AXIS_HOME/axis"
        chmod +x "$AXIS_HOME/axis"
        chown axis:axis "$AXIS_HOME/axis"
        log_success "二进制文件已恢复"
    else
        log_error "备份文件不存在：$BACKUP_PATH"
        exit 1
    fi
}

# 启动服务
start_service() {
    log_info "正在启动 Axis NAS 服务..."
    systemctl start $AXIS_SERVICE
    sleep 5
    
    if systemctl is-active --quiet $AXIS_SERVICE; then
        log_success "服务启动成功"
    else
        log_error "服务启动失败，请检查日志"
        exit 1
    fi
}

# 显示回滚信息
show_info() {
    local BACKUP_PATH=$1
    
    echo ""
    echo "========================================"
    echo -e "${GREEN}Axis NAS 回滚完成！${NC}"
    echo "========================================"
    echo ""
    echo "回滚版本：$BACKUP_PATH"
    echo "服务状态：$(systemctl is-active $AXIS_SERVICE)"
    echo ""
    echo "========================================"
}

# 主函数
main() {
    local BACKUP_PATH=$1
    
    echo ""
    echo "========================================"
    echo "  Axis NAS 回滚脚本"
    echo "========================================"
    echo ""
    
    check_root
    
    # 如果没有指定备份路径，使用最近的备份
    if [ -z "$BACKUP_PATH" ]; then
        BACKUP_PATH=$(find_latest_backup)
    fi
    
    stop_service
    do_rollback "$BACKUP_PATH"
    start_service
    show_info "$BACKUP_PATH"
}

# 执行主函数
main "$@"
