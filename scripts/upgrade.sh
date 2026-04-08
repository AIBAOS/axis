#!/bin/bash
#
# Axis NAS 升级脚本
#
# 用途：升级 Axis NAS 到最新版本
#
# 使用方法：
#   sudo ./upgrade.sh [版本号]
#
# 示例：
#   sudo ./upgrade.sh          # 升级到最新版本
#   sudo ./upgrade.sh v1.1.0   # 升级到指定版本
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
AXIS_CONFIG_DIR="/etc/axis"
AXIS_DATA_DIR="/var/lib/axis"
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

# 停止服务
stop_service() {
    log_info "正在停止 Axis NAS 服务..."
    systemctl stop $AXIS_SERVICE
    sleep 2
    log_success "服务已停止"
}

# 获取当前版本
get_current_version() {
    if [ -f "$AXIS_HOME/axis" ]; then
        local VERSION=$("$AXIS_HOME/axis" --version 2>/dev/null || echo "unknown")
        echo "$VERSION"
    else
        echo "unknown"
    fi
}

# 获取最新版本
get_latest_version() {
    local VERSION=$(curl -s https://api.github.com/repos/AIBAOS/axis/releases/latest | grep '"tag_name"' | cut -d'"' -f4)
    if [ -z "$VERSION" ]; then
        echo "v1.0.0"
    else
        echo "$VERSION"
    fi
}

# 版本对比（返回 0 表示需要升级，1 表示已是最新）
compare_versions() {
    local CURRENT=$1
    local LATEST=$2
    
    if [ "$CURRENT" = "unknown" ]; then
        return 0
    fi
    
    if [ "$CURRENT" = "$LATEST" ]; then
        return 1
    fi
    
    return 0
}

# 备份当前版本
backup_current() {
    log_info "正在备份当前版本..."
    
    mkdir -p "$BACKUP_DIR"
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_PATH="$BACKUP_DIR/axis_backup_$TIMESTAMP"
    
    # 备份二进制文件
    if [ -f "$AXIS_HOME/axis" ]; then
        cp "$AXIS_HOME/axis" "$BACKUP_PATH"
        log_info "二进制文件已备份到：$BACKUP_PATH"
    fi
    
    # 备份数据库
    if [ -f "$AXIS_DATA_DIR/NAS.db" ]; then
        cp "$AXIS_DATA_DIR/NAS.db" "$BACKUP_DIR/NAS.db_$TIMESTAMP"
        log_info "数据库已备份到：$BACKUP_DIR/NAS.db_$TIMESTAMP"
    fi
    
    # 备份配置文件
    if [ -d "$AXIS_CONFIG_DIR" ]; then
        tar -czf "$BACKUP_DIR/config_$TIMESTAMP.tar.gz" -C "$(dirname $AXIS_CONFIG_DIR)" "$(basename $AXIS_CONFIG_DIR)"
        log_info "配置文件已备份到：$BACKUP_DIR/config_$TIMESTAMP.tar.gz"
    fi
    
    # 创建备份清单
    cat > "$BACKUP_DIR/backup_$TIMESTAMP.txt" << EOF
备份时间：$TIMESTAMP
当前版本：$(get_current_version)
目标版本：$TARGET_VERSION
二进制文件：$BACKUP_PATH
数据库：$BACKUP_DIR/NAS.db_$TIMESTAMP
配置文件：$BACKUP_DIR/config_$TIMESTAMP.tar.gz
EOF
    
    log_success "备份完成"
}

# 下载新版本
download_new_version() {
    local VERSION=$1
    
    if [ -z "$VERSION" ]; then
        log_info "正在获取最新版本..."
        VERSION=$(curl -s https://api.github.com/repos/AIBAOS/axis/releases/latest | grep '"tag_name"' | cut -d'"' -f4)
        
        if [ -z "$VERSION" ]; then
            log_error "无法获取最新版本"
            exit 1
        fi
    fi
    
    log_info "目标版本：$VERSION"
    
    BINARY_URL="https://github.com/AIBAOS/axis/releases/download/$VERSION/axis"
    
    log_info "正在下载新版本..."
    
    if command -v wget &>/dev/null; then
        wget -q -O "$AXIS_HOME/axis.new" "$BINARY_URL"
    elif command -v curl &>/dev/null; then
        curl -sL -o "$AXIS_HOME/axis.new" "$BINARY_URL"
    else
        log_error "未找到 wget 或 curl"
        exit 1
    fi
    
    chmod +x "$AXIS_HOME/axis.new"
    
    log_success "新版本下载完成"
}

# 替换二进制文件
replace_binary() {
    log_info "正在替换二进制文件..."
    
    mv "$AXIS_HOME/axis" "$AXIS_HOME/axis.old"
    mv "$AXIS_HOME/axis.new" "$AXIS_HOME/axis"
    
    chown axis:axis "$AXIS_HOME/axis"
    
    log_success "二进制文件已替换"
}

# 启动服务
start_service() {
    log_info "正在启动 Axis NAS 服务..."
    systemctl start $AXIS_SERVICE
    sleep 5
    
    if systemctl is-active --quiet $AXIS_SERVICE; then
        log_success "服务启动成功"
    else
        log_error "服务启动失败"
        rollback
        exit 1
    fi
}

# 回滚函数
rollback() {
    log_warning "正在回滚到之前的版本..."
    
    if [ -f "$AXIS_HOME/axis.old" ]; then
        mv "$AXIS_HOME/axis.old" "$AXIS_HOME/axis"
        systemctl start $AXIS_SERVICE
        log_success "回滚完成"
    else
        log_error "无法回滚，备份文件不存在"
    fi
}

# 清理旧版本
cleanup() {
    log_info "正在清理旧版本..."
    
    if [ -f "$AXIS_HOME/axis.old" ]; then
        rm -f "$AXIS_HOME/axis.old"
        log_info "旧版本文件已删除"
    fi
    
    # 清理 30 天前的备份
    find "$BACKUP_DIR" -name "axis_backup_*" -mtime +30 -delete
    find "$BACKUP_DIR" -name "NAS.db_*" -mtime +30 -delete
    find "$BACKUP_DIR" -name "config_*" -mtime +30 -delete
    
    log_success "清理完成"
}

# 显示升级信息
show_info() {
    local VERSION=$1
    
    echo ""
    echo "========================================"
    echo -e "${GREEN}Axis NAS 升级完成！${NC}"
    echo "========================================"
    echo ""
    echo "当前版本：$VERSION"
    echo "服务状态：$(systemctl is-active $AXIS_SERVICE)"
    echo ""
    echo "回滚命令："
    echo "  sudo $AXIS_HOME/rollback.sh"
    echo ""
    echo "========================================"
}

# 主函数
main() {
    local TARGET_VERSION=$1
    
    echo ""
    echo "========================================"
    echo "  Axis NAS 升级脚本"
    echo "========================================"
    echo ""
    
    check_root
    
    # 获取版本信息
    CURRENT_VERSION=$(get_current_version)
    if [ -z "$TARGET_VERSION" ]; then
        TARGET_VERSION=$(get_latest_version)
    fi
    
    echo "当前版本：$CURRENT_VERSION"
    echo "目标版本：$TARGET_VERSION"
    echo ""
    
    # 版本对比
    if [ "$CURRENT_VERSION" = "$TARGET_VERSION" ]; then
        log_success "已是最新版本，无需升级"
        exit 0
    fi
    
    # 确认升级
    read -p "确认升级到 $TARGET_VERSION？[Y/n] " CONFIRM
    if [[ "$CONFIRM" =~ ^[Nn]$ ]]; then
        log_info "升级已取消"
        exit 0
    fi
    
    stop_service
    backup_current
    download_new_version "$TARGET_VERSION"
    replace_binary
    start_service
    cleanup
    show_info "$TARGET_VERSION"
}

# 执行主函数
main "$@"
