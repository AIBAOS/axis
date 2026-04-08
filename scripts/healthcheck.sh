#!/bin/bash
#
# Axis NAS 健康检查脚本
#
# 用途：检查 Axis NAS 服务健康状态
#
# 使用方法：
#   ./healthcheck.sh [选项]
#
# 选项：
#   --verbose    显示详细信息
#   --json       以 JSON 格式输出
#   --port       指定端口（默认 8080）
#
# 退出码：
#   0 - 健康
#   1 - 警告
#   2 - 严重错误
#

set -e

# 配置变量
AXIS_SERVICE="axis"
AXIS_PORT="8080"
AXIS_HOST="localhost"
VERBOSE=false
JSON_OUTPUT=false

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 解析参数
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose)
            VERBOSE=true
            shift
            ;;
        --json)
            JSON_OUTPUT=true
            shift
            ;;
        --port)
            AXIS_PORT="$2"
            shift 2
            ;;
        --host)
            AXIS_HOST="$2"
            shift 2
            ;;
        *)
            echo "未知选项：$1"
            echo "用法：$0 [--verbose] [--json] [--port PORT] [--host HOST]"
            exit 1
            ;;
    esac
done

# 检查结果
CHECKS_PASSED=0
CHECKS_FAILED=0
CHECKS_WARNING=0

# 日志函数
log_info() {
    if [ "$VERBOSE" = true ]; then
        echo -e "${BLUE}[INFO]${NC} $1"
    fi
}

log_success() {
    if [ "$VERBOSE" = true ] || [ "$JSON_OUTPUT" = false ]; then
        echo -e "${GREEN}[OK]${NC} $1"
    fi
    ((CHECKS_PASSED++))
}

log_warning() {
    if [ "$VERBOSE" = true ] || [ "$JSON_OUTPUT" = false ]; then
        echo -e "${YELLOW}[WARNING]${NC} $1"
    fi
    ((CHECKS_WARNING++))
}

log_error() {
    if [ "$VERBOSE" = true ] || [ "$JSON_OUTPUT" = false ]; then
        echo -e "${RED}[FAILED]${NC} $1"
    fi
    ((CHECKS_FAILED++))
}

# 检查服务状态
check_service_status() {
    log_info "检查服务状态..."
    
    if systemctl is-active --quiet $AXIS_SERVICE; then
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
    
    RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" "http://$AXIS_HOST:$AXIS_PORT/api/v1/system/health" 2>/dev/null || echo "000")
    
    if [ "$RESPONSE" = "200" ]; then
        log_success "API 健康检查：HTTP $RESPONSE"
        return 0
    else
        log_error "API 健康检查：HTTP $RESPONSE"
        return 1
    fi
}

# 检查数据库文件
check_database() {
    log_info "检查数据库文件..."
    
    DB_PATH="/var/lib/axis/NAS.db"
    
    if [ -f "$DB_PATH" ]; then
        SIZE=$(du -h "$DB_PATH" | cut -f1)
        log_success "数据库文件：存在 ($SIZE)"
        return 0
    else
        log_warning "数据库文件：不存在 ($DB_PATH)"
        return 1
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

# 检查内存使用
check_memory() {
    log_info "检查内存使用..."
    
    if command -v free &>/dev/null; then
        TOTAL=$(free -m | grep Mem | awk '{print $2}')
        USED=$(free -m | grep Mem | awk '{print $3}')
        PERCENT=$((USED * 100 / TOTAL))
        
        if [ "$PERCENT" -lt 80 ]; then
            log_success "内存使用率：${PERCENT}% (< 80%)"
            return 0
        else
            log_warning "内存使用率：${PERCENT}% (>= 80%)"
            return 1
        fi
    else
        log_info "内存检查：跳过（free 命令不可用）"
        return 0
    fi
}

# 检查日志文件
check_logs() {
    log_info "检查日志文件..."
    
    if journalctl -u $AXIS_SERVICE -n 1 &>/dev/null; then
        # 检查最近是否有错误日志
        ERROR_COUNT=$(journalctl -u $AXIS_SERVICE --since "5 minutes ago" | grep -i "error" | wc -l)
        
        if [ "$ERROR_COUNT" -eq 0 ]; then
            log_success "日志检查：无错误"
            return 0
        else
            log_warning "日志检查：发现 $ERROR_COUNT 个错误（最近 5 分钟）"
            return 1
        fi
    else
        log_info "日志检查：跳过（journalctl 不可用）"
        return 0
    fi
}

# 输出 JSON 结果
output_json() {
    cat << EOF
{
  "status": "$( [ $CHECKS_FAILED -eq 0 ] && echo "healthy" || echo "unhealthy" )",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "checks": {
    "passed": $CHECKS_PASSED,
    "failed": $CHECKS_FAILED,
    "warning": $CHECKS_WARNING
  },
  "service": "$AXIS_SERVICE",
  "port": $AXIS_PORT
}
EOF
}

# 输出摘要
output_summary() {
    echo ""
    echo "========================================"
    echo "  Axis NAS 健康检查摘要"
    echo "========================================"
    echo ""
    
    if [ $CHECKS_FAILED -eq 0 ]; then
        echo -e "${GREEN}状态：健康${NC}"
    elif [ $CHECKS_WARNING -gt 0 ]; then
        echo -e "${YELLOW}状态：警告${NC}"
    else
        echo -e "${RED}状态：严重错误${NC}"
    fi
    
    echo ""
    echo "检查项："
    echo -e "  通过：${GREEN}$CHECKS_PASSED${NC}"
    echo -e "  警告：${YELLOW}$CHECKS_WARNING${NC}"
    echo -e "  失败：${RED}$CHECKS_FAILED${NC}"
    echo ""
    echo "========================================"
}

# 主函数
main() {
    echo ""
    echo "========================================"
    echo "  Axis NAS 健康检查"
    echo "  服务：$AXIS_SERVICE"
    echo "  端口：$AXIS_PORT"
    echo "========================================"
    echo ""
    
    # 执行检查
    check_service_status || true
    check_port_listening || true
    check_api_health || true
    check_database || true
    check_disk_space || true
    check_memory || true
    check_logs || true
    
    # 输出结果
    if [ "$JSON_OUTPUT" = true ]; then
        output_json
    else
        output_summary
    fi
    
    # 返回退出码
    if [ $CHECKS_FAILED -gt 0 ]; then
        exit 2
    elif [ $CHECKS_WARNING -gt 0 ]; then
        exit 1
    else
        exit 0
    fi
}

# 执行主函数
main "$@"
