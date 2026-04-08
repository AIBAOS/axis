#!/bin/bash
#
# Axis NAS 恢复脚本
# 
# 用途：从备份恢复数据库和配置文件
#
# 使用方法：
#   sudo ./restore.sh [选项] <备份文件>
#
# 选项：
#   -h, --help           显示帮助信息
#
# 示例：
#   sudo ./restore.sh /var/backup/axis/NAS.db_20260408_160000.gz
#   sudo ./restore.sh /var/backup/axis/config_20260408_160000.tar.gz
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
AXIS_SERVICE="axis"

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
    echo "Axis NAS 恢复脚本"
    echo ""
    echo "用法：$0 [选项] <备份文件>"
    echo ""
    echo "选项:"
    echo "  -h, --help           显示帮助信息"
    echo ""
    echo "示例:"
    echo "  $0 /var/backup/axis/NAS.db_20260408_160000.gz"
    echo "  $0 /var/backup/axis/config_20260408_160000.tar.gz"
}

# 检查 root 权限
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log_error "请使用 sudo 运行此脚本"
        exit 1
    fi
}

# 停止服务
stop_service() {
    log_info "正在停止 Axis NAS 服务..."
    systemctl stop $AXIS_SERVICE
    sleep 2
    log_success "服务已停止"
}

# 恢复数据库
restore_database() {
    local BACKUP_FILE=$1
    
    log_info "正在恢复数据库：$BACKUP_FILE"
    
    # 备份当前数据库
    if [ -f "$AXIS_DATA_DIR/NAS.db" ]; then
        TIMESTAMP=$(date +%Y%m%d_%H%M%S)
        cp "$AXIS_DATA_DIR/NAS.db" "$AXIS_DATA_DIR/NAS.db.backup_$TIMESTAMP"
        log_info "当前数据库已备份：$AXIS_DATA_DIR/NAS.db.backup_$TIMESTAMP"
    fi
    
    # 解压并恢复
    if [[ "$BACKUP_FILE" == *.gz ]]; then
        gunzip -c "$BACKUP_FILE" > "$AXIS_DATA_DIR/NAS.db"
    else
        cp "$BACKUP_FILE" "$AXIS_DATA_DIR/NAS.db"
    fi
    
    chown axis:axis "$AXIS_DATA_DIR/NAS.db"
    chmod 600 "$AXIS_DATA_DIR/NAS.db"
    
    log_success "数据库已恢复"
}

# 恢复配置文件
restore_config() {
    local BACKUP_FILE=$1
    
    log_info "正在恢复配置文件：$BACKUP_FILE"
    
    # 备份当前配置
    if [ -d "$AXIS_CONFIG_DIR" ]; then
        TIMESTAMP=$(date +%Y%m%d_%H%M%S)
        mv "$AXIS_CONFIG_DIR" "$AXIS_CONFIG_DIR.backup_$TIMESTAMP"
        log_info "当前配置已备份：$AXIS_CONFIG_DIR.backup_$TIMESTAMP"
    fi
    
    # 解压并恢复
    mkdir -p "$(dirname $AXIS_CONFIG_DIR)"
    tar -xzf "$BACKUP_FILE" -C "$(dirname $AXIS_CONFIG_DIR)"
    
    chown -R axis:axis "$AXIS_CONFIG_DIR"
    chmod 600 "$AXIS_CONFIG_DIR"/*
    
    log_success "配置文件已恢复"
}

# 启动服务
start_service() {
    log_info "正在启动 Axis NAS 服务..."
    systemctl start $AXIS_SERVICE
    sleep 5
    
    # DEPLOY-3 修复：验证服务状态
    local MAX_RETRIES=3
    local RETRY_COUNT=0
    
    while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
        if systemctl is-active --quiet $AXIS_SERVICE; then
            # 服务运行中，进一步验证 API 可访问性
            if command -v curl &>/dev/null; then
                RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:8080/api/v1/system/health" 2>/dev/null || echo "000")
                if [ "$RESPONSE" = "200" ]; then
                    log_success "服务启动成功，健康检查通过 (HTTP $RESPONSE)"
                    return 0
                else
                    log_warning "服务运行但 API 不可用 (HTTP $RESPONSE)，尝试重启..."
                fi
            else
                log_success "服务启动成功"
                return 0
            fi
        else
            log_warning "服务未运行，尝试重启 ($RETRY_COUNT/$MAX_RETRIES)..."
        fi
        
        RETRY_COUNT=$((RETRY_COUNT + 1))
        systemctl restart $AXIS_SERVICE
        sleep 5
    done
    
    log_error "服务启动失败，已尝试 $MAX_RETRIES 次，请检查日志：journalctl -u axis"
    exit 1
}

# 显示恢复信息
show_info() {
    local BACKUP_FILE=$1
    
    echo ""
    echo "========================================"
    echo -e "${GREEN}Axis NAS 恢复完成！${NC}"
    echo "========================================"
    echo ""
    echo "恢复文件：$BACKUP_FILE"
    echo "服务状态：$(systemctl is-active $AXIS_SERVICE)"
    echo ""
    echo "========================================"
}

# 主函数
main() {
    local BACKUP_FILE=$1
    
    echo ""
    echo "========================================"
    echo "  Axis NAS 恢复脚本"
    echo "========================================"
    echo ""
    
    if [ -z "$BACKUP_FILE" ]; then
        log_error "请指定备份文件路径"
        echo "用法：sudo $0 <备份文件>"
        echo "示例：sudo $0 /var/backup/axis/NAS.db_20260408_160000.gz"
        exit 1
    fi
    
    if [ ! -f "$BACKUP_FILE" ]; then
        log_error "备份文件不存在：$BACKUP_FILE"
        exit 1
    fi
    
    check_root
    stop_service
    
    # 根据文件类型选择恢复方式
    if [[ "$BACKUP_FILE" == *NAS.db* ]]; then
        restore_database "$BACKUP_FILE"
    elif [[ "$BACKUP_FILE" == *config* ]]; then
        restore_config "$BACKUP_FILE"
    else
        log_error "未知的备份文件类型"
        exit 1
    fi
    
    start_service
    show_info "$BACKUP_FILE"
}

# 执行主函数
main "$@"
