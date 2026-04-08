# Axis NAS Docker 镜像
# 版本：v1.0.0
# 更新时间：2026-04-08

# 构建阶段
FROM rust:1.75 AS builder

WORKDIR /app

# 复制 Cargo 配置文件
COPY Cargo.toml Cargo.lock ./

# 复制源码
COPY src ./src

# 编译发布版本
RUN cargo build --release

# 运行阶段
FROM debian:bullseye-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -r -s /bin/false axis

# 复制二进制文件
COPY --from=builder /app/target/release/axis /usr/local/bin/

# 创建数据目录
RUN mkdir -p /var/lib/axis /var/log/axis && \
    chown -R axis:axis /var/lib/axis /var/log/axis

# 暴露端口
EXPOSE 8080

# 数据卷
VOLUME ["/var/lib/axis"]

# 切换到非 root 用户
USER axis

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/v1/system/health || exit 1

# 启动命令
CMD ["axis"]
