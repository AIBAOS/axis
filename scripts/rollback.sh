#!/bin/bash
#
# Axis NAS 回滚脚本
# 
# 用途：回滚到之前的版本（升级失败时使用）
#
# 使用方法：
#   sudo ./rollback.sh [选项] [备份路径]
#
# 选项：
#   -l, --list           列出可用回滚点
#   -h, --help           显示帮助信息
#
# 示例：
#   sudo ./rollback.sh                      # 回滚到最近的备份
#   sudo ./rollback.sh -l                   # 列出所有回滚点
#   sudo ./rollback.sh /var/backup/xxx      # 回滚到指定备份
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

# 显示帮助信息
show_help() {
    echo "Axis NAS 回滚脚本"
    echo ""
    echo "用法：$0 [选项] [备份路径]"
    echo ""
    echo "选项:"
    echo "  -l, --list           列出可用回滚点"
    echo "  -h, --help           显示帮助信息"
    echo ""
    echo "示例:"
    echo "  $0                              # 回滚到最近的备份"
    echo "  $0 -l                           # 列出所有回滚点"
    echo "  $0 /var/backup/axis/xxx         # 回滚到指定备份"
}

# 检查 root 权限
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log_error "请使用 sudo 运行此脚本"
        exit 1
    fi
}

# 列出可用回滚点
list_backups() {
    log_info "正在查找可用回滚点..."
    echo ""
    echo "========================================"
    echo "  可用回滚点"
    echo "========================================"
    echo ""
    
    if [ ! -d "$BACKUP_DIR" ]; then
        log_error "备份目录不存在：$BACKUP_DIR"
        exit 1
    fi
    
    local COUNT=0
    for backup in $(ls -t "$BACKUP_DIR"/axis_backup_* 2>/dev/null); do
        ((COUNT++))
        local DATE=$(echo "$backup" | grep -oP '\d{8}_\d{6}')
        echo "  $COUNT. $backup"
        echo "     时间：$DATE"
        
        # 显示备份清单（如果有）
        if [ -f "${backup}.txt" ]; then
            echo "     详情：$(head -1 ${backup}.txt)"
        fi
        echo ""
    done
    
    if [ $COUNT -eq 0 ]; then
        log_error "未找到备份文件"
        exit 1
    fi
    
    echo "========================================"
    echo "总计：$COUNT 个回滚点"
    echo "========================================"
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
    
    # 恢复配置文件（如果有）
    local CONFIG_BACKUP=$(echo "$BACKUP_PATH" | sed 's/axis_backup/config/')
    if [ -f "${CONFIG_BACKUP}.tar.gz" ]; then
        log_info "正在恢复配置文件..."
        tar -xzf "${CONFIG_BACKUP}.tar.gz" -C "$(dirname $AXIS_CONFIG_DIR)"
        log_success "配置文件已恢复"
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

# 解析命令行参数
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -l|--list)
                list_backups
                exit 0
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                BACKUP_PATH="$1"
                shift
                ;;
        esac
    done
}

# 主函数
main() {
    # 解析参数
    parse_args "$@"
    
    echo ""
    echo "========================================"
    echo "  Axis NAS 回滚脚本"
    echo "========================================"
    echo ""
    
    check_root
    
    # 如果没有指定备份路径，使用最近的备份
    if [ -z "$BACKUP_PATH" ]; then
        BACKUP_PATH=$(ls -t "$BACKUP_DIR"/axis_backup_* 2>/dev/null | head -n1)
        if [ -z "$BACKUP_PATH" ]; then
            log_error "未找到备份文件"
            log_info "使用 -l 参数列出可用回滚点"
            exit 1
        fi
        log_info "使用最近的备份：$BACKUP_PATH"
    fi
    
    # 确认回滚
    read -p "确认回滚到 $BACKUP_PATH？[Y/n] " CONFIRM
    if [[ "$CONFIRM" =~ ^[Nn]$ ]]; then
        log_info "回滚已取消"
        exit 0
    fi
    
    stop_service
    do_rollback "$BACKUP_PATH"
    start_service
    show_info "$BACKUP_PATH"
}

# 执行主函数
main "$@"
