#!/bin/bash
#
# Axis NAS 部署验证脚本
# 
# 用途：验证 Axis NAS 部署是否成功
#
# 使用方法：
#   sudo ./deploy-verify.sh
#

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 配置变量
AXIS_SERVICE="axis"
AXIS_PORT="8080"
AXIS_HOST="localhost"

# 计数器
PASS_COUNT=0
FAIL_COUNT=0

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((PASS_COUNT++))
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((FAIL_COUNT++))
}

# 检查服务状态
check_service_status() {
    log_info "检查服务状态..."
    
    if systemctl is-active --quiet $AXIS_SERVICE 2>/dev/null; then
        log_success "服务状态：运行中"
        return 0
    else
        log_error "服务状态：未运行"
        return 1
    fi
}

# 检查端口监听
check_port_listening() {
    log_info "检查端口监听：$AXIS_PORT"
    
    if command -v ss &>/dev/null; then
        if ss -tln | grep -q ":$AXIS_PORT "; then
            log_success "端口监听：$AXIS_PORT 正常"
            return 0
        fi
    elif command -v netstat &>/dev/null; then
        if netstat -tln | grep -q ":$AXIS_PORT "; then
            log_success "端口监听：$AXIS_PORT 正常"
            return 0
        fi
    fi
    
    log_error "端口监听：$AXIS_PORT 未监听"
    return 1
}

# 检查 API 健康端点
check_api_health() {
    log_info "检查 API 健康端点..."
    
    if command -v curl &>/dev/null; then
        RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" "http://$AXIS_HOST:$AXIS_PORT/api/v1/system/health" 2>/dev/null || echo "000")
        
        if [ "$RESPONSE" = "200" ]; then
            log_success "API 健康检查：HTTP $RESPONSE"
            return 0
        else
            log_error "API 健康检查：HTTP $RESPONSE"
            return 1
        fi
    else
        log_warning "curl 命令不可用，跳过 API 健康检查"
        return 0
    fi
}

# 检查配置文件
check_config_files() {
    log_info "检查配置文件..."
    
    if [ -f "/etc/axis/config.toml" ]; then
        log_success "配置文件：/etc/axis/config.toml 存在"
    else
        log_error "配置文件：/etc/axis/config.toml 不存在"
        return 1
    fi
    
    if [ -f "/etc/axis/axis.env" ]; then
        log_success "环境变量文件：/etc/axis/axis.env 存在"
    else
        log_warning "环境变量文件：/etc/axis/axis.env 不存在"
    fi
    
    return 0
}

# 检查数据目录
check_data_directories() {
    log_info "检查数据目录..."
    
    if [ -d "/var/lib/axis" ]; then
        log_success "数据目录：/var/lib/axis 存在"
    else
        log_error "数据目录：/var/lib/axis 不存在"
        return 1
    fi
    
    if [ -d "/var/log/axis" ]; then
        log_success "日志目录：/var/log/axis 存在"
    else
        log_warning "日志目录：/var/log/axis 不存在"
    fi
    
    return 0
}

# 检查数据库文件
check_database() {
    log_info "检查数据库文件..."
    
    if [ -f "/var/lib/axis/NAS.db" ]; then
        SIZE=$(du -h "/var/lib/axis/NAS.db" | cut -f1)
        log_success "数据库文件：存在 ($SIZE)"
        return 0
    else
        log_warning "数据库文件：不存在（首次启动会自动创建）"
        return 0
    fi
}

# 检查磁盘空间
check_disk_space() {
    log_info "检查磁盘空间..."
    
    THRESHOLD=90
    USAGE=$(df /var/lib/axis | tail -1 | awk '{print $5}' | sed 's/%//')
    
    if [ "$USAGE" -lt "$THRESHOLD" ]; then
        log_success "磁盘使用率：${USAGE}% (< ${THRESHOLD}%)"
        return 0
    else
        log_error "磁盘使用率：${USAGE}% (>= ${THRESHOLD}%)"
        return 1
    fi
}

# 检查系统资源
check_system_resources() {
    log_info "检查系统资源..."
    
    if command -v free &>/dev/null; then
        TOTAL=$(free -m | grep Mem | awk '{print $2}')
        USED=$(free -m | grep Mem | awk '{print $3}')
        PERCENT=$((USED * 100 / TOTAL))
        
        if [ "$PERCENT" -lt 80 ]; then
            log_success "内存使用率：${PERCENT}% (< 80%)"
        else
            log_warning "内存使用率：${PERCENT}% (>= 80%)"
        fi
    fi
    
    return 0
}

# 显示验证报告
show_report() {
    echo ""
    echo "========================================"
    echo "  Axis NAS 部署验证报告"
    echo "========================================"
    echo ""
    echo "通过：$PASS_COUNT"
    echo "失败：$FAIL_COUNT"
    echo ""
    
    if [ $FAIL_COUNT -eq 0 ]; then
        echo -e "${GREEN}状态：✅ 部署成功${NC}"
        echo ""
        echo "访问地址：http://<服务器 IP>:$AXIS_PORT"
        echo ""
    else
        echo -e "${RED}状态：❌ 部署失败${NC}"
        echo ""
        echo "请检查失败项并重新部署"
        echo ""
    fi
    
    echo "========================================"
}

# 主函数
main() {
    echo ""
    echo "========================================"
    echo "  Axis NAS 部署验证"
    echo "========================================"
    echo ""
    
    check_service_status || true
    check_port_listening || true
    check_api_health || true
    check_config_files || true
    check_data_directories || true
    check_database || true
    check_disk_space || true
    check_system_resources || true
    
    show_report
    
    if [ $FAIL_COUNT -gt 0 ]; then
        exit 1
    fi
    
    exit 0
}

# 执行主函数
main "$@"
