#!/bin/bash
#
# Axis NAS 一键安装脚本
# 
# 用途：自动化部署 Axis NAS 到生产环境
# 系统要求：Ubuntu 20.04+ / Debian 10+ / CentOS 8+
#
# 使用方法：
#   curl -fsSL https://raw.githubusercontent.com/AIBAOS/axis/main/scripts/install.sh | sudo bash
#
# 或下载后执行：
#   chmod +x install.sh
#   sudo ./install.sh
#

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置变量
AXIS_USER="axis"
AXIS_GROUP="axis"
AXIS_HOME="/opt/axis"
AXIS_CONFIG_DIR="/etc/axis"
AXIS_DATA_DIR="/var/lib/axis"
AXIS_LOG_DIR="/var/log/axis"
AXIS_SERVICE="axis"
AXIS_PORT="8080"

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

# 检查是否以 root 运行
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log_error "请使用 sudo 运行此脚本"
        exit 1
    fi
    log_info "Root 权限检查通过"
}

# 检测操作系统
detect_os() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$ID
        VERSION=$VERSION_ID
    else
        log_error "无法检测操作系统版本"
        exit 1
    fi
    
    log_info "检测到操作系统：$OS $VERSION"
}

# 安装系统依赖
install_dependencies() {
    log_info "正在安装系统依赖..."
    
    case $OS in
        ubuntu|debian)
            apt-get update
            apt-get install -y curl wget openssl ca-certificates
            ;;
        centos|rhel|fedora)
            yum install -y curl wget openssl ca-certificates
            ;;
        *)
            log_warning "未知操作系统，尝试通用安装方法"
            ;;
    esac
    
    log_success "系统依赖安装完成"
}

# 创建系统用户
create_user() {
    log_info "正在创建系统用户：$AXIS_USER"
    
    if id "$AXIS_USER" &>/dev/null; then
        log_warning "用户 $AXIS_USER 已存在，跳过创建"
    else
        useradd -r -s /bin/false -d "$AXIS_HOME" -m "$AXIS_USER"
        log_success "系统用户 $AXIS_USER 创建成功"
    fi
}

# 创建目录结构
create_directories() {
    log_info "正在创建目录结构..."
    
    mkdir -p "$AXIS_HOME"
    mkdir -p "$AXIS_CONFIG_DIR"
    mkdir -p "$AXIS_DATA_DIR"
    mkdir -p "$AXIS_LOG_DIR"
    
    # 设置权限
    chown -R "$AXIS_USER:$AXIS_GROUP" "$AXIS_HOME"
    chown -R "$AXIS_USER:$AXIS_GROUP" "$AXIS_CONFIG_DIR"
    chown -R "$AXIS_USER:$AXIS_GROUP" "$AXIS_DATA_DIR"
    chown -R "$AXIS_USER:$AXIS_GROUP" "$AXIS_LOG_DIR"
    
    chmod 755 "$AXIS_HOME"
    chmod 700 "$AXIS_DATA_DIR"
    chmod 755 "$AXIS_LOG_DIR"
    
    log_success "目录结构创建完成"
}

# 生成 JWT 密钥
generate_jwt_secret() {
    log_info "正在生成 JWT 密钥..."
    
    JWT_SECRET=$(openssl rand -base64 32)
    
    log_success "JWT 密钥生成完成"
}

# 创建配置文件
create_config() {
    log_info "正在创建配置文件..."
    
    # 创建环境变量文件
    cat > "$AXIS_CONFIG_DIR/axis.env" << EOF
# Axis NAS 环境变量配置
# 生成时间：$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# JWT 配置
export JWT_SECRET_KEY="$JWT_SECRET"

# 数据库配置
export DATABASE_PATH="$AXIS_DATA_DIR/NAS.db"

# 服务器配置
export SERVER_HOST="0.0.0.0"
export SERVER_PORT="$AXIS_PORT"

# 日志配置
export RUST_LOG="info"
EOF
    
    chmod 600 "$AXIS_CONFIG_DIR/axis.env"
    chown "$AXIS_USER:$AXIS_GROUP" "$AXIS_CONFIG_DIR/axis.env"
    
    # 创建配置文件
    cat > "$AXIS_CONFIG_DIR/config.toml" << EOF
# Axis NAS 配置文件
# 生成时间：$(date -u +"%Y-%m-%dT%H:%M:%SZ")

[jwt]
# 生产环境使用环境变量 JWT_SECRET_KEY
secret_key = ""
issuer = "axis-nas"
audience = "axis-nas-users"
expiration_minutes = 60
refresh_enabled = false

[database]
path = "$AXIS_DATA_DIR/NAS.db"
max_connections = 10

[server]
host = "0.0.0.0"
port = $AXIS_PORT
EOF
    
    chmod 600 "$AXIS_CONFIG_DIR/config.toml"
    chown "$AXIS_USER:$AXIS_GROUP" "$AXIS_CONFIG_DIR/config.toml"
    
    log_success "配置文件创建完成"
}

# 安装 systemd 服务
install_service() {
    log_info "正在安装 systemd 服务..."
    
    cat > /etc/systemd/system/$AXIS_SERVICE.service << EOF
[Unit]
Description=Axis NAS Service
After=network.target

[Service]
Type=simple
User=$AXIS_USER
Group=$AXIS_GROUP
WorkingDirectory=$AXIS_HOME
EnvironmentFile=$AXIS_CONFIG_DIR/axis.env
ExecStart=$AXIS_HOME/axis
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
ReadWritePaths=$AXIS_DATA_DIR $AXIS_LOG_DIR

[Install]
WantedBy=multi-user.target
EOF
    
    systemctl daemon-reload
    systemctl enable $AXIS_SERVICE
    
    log_success "systemd 服务安装完成"
}

# 配置防火墙
configure_firewall() {
    log_info "正在配置防火墙..."
    
    if command -v ufw &>/dev/null; then
        ufw allow $AXIS_PORT/tcp
        log_info "UFW: 已开放端口 $AXIS_PORT"
    elif command -v firewall-cmd &>/dev/null; then
        firewall-cmd --permanent --add-port=$AXIS_PORT/tcp
        firewall-cmd --reload
        log_info "firewalld: 已开放端口 $AXIS_PORT"
    else
        log_warning "未检测到防火墙工具，请手动配置端口 $AXIS_PORT"
    fi
}

# 下载 Axis 二进制文件
download_binary() {
    log_info "正在下载 Axis NAS 二进制文件..."
    
    # 从 GitHub Releases 下载最新版本
    LATEST_VERSION=$(curl -s https://api.github.com/repos/AIBAOS/axis/releases/latest | grep '"tag_name"' | cut -d'"' -f4)
    
    if [ -z "$LATEST_VERSION" ]; then
        log_warning "无法获取最新版本，使用默认版本"
        LATEST_VERSION="v1.0.0"
    fi
    
    log_info "最新版本：$LATEST_VERSION"
    
    # 下载二进制文件（根据实际情况调整 URL）
    BINARY_URL="https://github.com/AIBAOS/axis/releases/download/$LATEST_VERSION/axis"
    
    if command -v wget &>/dev/null; then
        wget -q -O "$AXIS_HOME/axis" "$BINARY_URL" || {
            log_warning "下载失败，请手动放置二进制文件到 $AXIS_HOME/axis"
            return 1
        }
    elif command -v curl &>/dev/null; then
        curl -sL -o "$AXIS_HOME/axis" "$BINARY_URL" || {
            log_warning "下载失败，请手动放置二进制文件到 $AXIS_HOME/axis"
            return 1
        }
    else
        log_error "未找到 wget 或 curl，无法下载二进制文件"
        return 1
    fi
    
    chmod +x "$AXIS_HOME/axis"
    chown "$AXIS_USER:$AXIS_GROUP" "$AXIS_HOME/axis"
    
    log_success "二进制文件下载完成"
}

# 启动服务
start_service() {
    log_info "正在启动 Axis NAS 服务..."
    
    systemctl start $AXIS_SERVICE
    
    # 等待服务启动
    sleep 5
    
    if systemctl is-active --quiet $AXIS_SERVICE; then
        log_success "Axis NAS 服务启动成功"
    else
        log_error "Axis NAS 服务启动失败，请检查日志：journalctl -u axis"
        exit 1
    fi
}

# 显示安装信息
show_info() {
    echo ""
    echo "========================================"
    echo -e "${GREEN}Axis NAS 安装完成！${NC}"
    echo "========================================"
    echo ""
    echo "服务状态：$(systemctl is-active $AXIS_SERVICE)"
    echo "监听端口：$AXIS_PORT"
    echo "配置文件：$AXIS_CONFIG_DIR/config.toml"
    echo "数据目录：$AXIS_DATA_DIR"
    echo "日志文件：journalctl -u axis"
    echo ""
    echo "管理命令："
    echo "  启动服务：sudo systemctl start $AXIS_SERVICE"
    echo "  停止服务：sudo systemctl stop $AXIS_SERVICE"
    echo "  重启服务：sudo systemctl restart $AXIS_SERVICE"
    echo "  查看状态：sudo systemctl status $AXIS_SERVICE"
    echo "  查看日志：sudo journalctl -u axis -f"
    echo ""
    echo "访问地址：http://<服务器 IP>:$AXIS_PORT"
    echo ""
    echo "========================================"
    echo -e "${YELLOW}安全提示：${NC}"
    echo "1. 请修改默认端口（如需）"
    echo "2. 请配置防火墙规则"
    echo "3. 请定期备份数据库"
    echo "4. 请查看文档：docs/production-config.md"
    echo "========================================"
}

# 主函数
main() {
    echo ""
    echo "========================================"
    echo "  Axis NAS 一键安装脚本"
    echo "  版本：v1.0.0"
    echo "========================================"
    echo ""
    
    check_root
    detect_os
    install_dependencies
    create_user
    create_directories
    generate_jwt_secret
    create_config
    download_binary
    install_service
    configure_firewall
    start_service
    show_info
}

# 执行主函数
main "$@"
