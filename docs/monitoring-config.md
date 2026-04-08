# Axis NAS 监控告警配置指南

**版本：** v1.0.0  
**更新时间：** 2026-04-08  
**状态：** 生产就绪

---

## 目录

1. [监控指标](#1-监控指标)
2. [告警阈值](#2-告警阈值)
3. [Prometheus 配置](#3-prometheus 配置)
4. [Grafana 仪表板](#4-grafana 仪表板)
5. [告警通知](#5-告警通知)
6. [日志收集](#6-日志收集)

---

## 1. 监控指标

### 1.1 系统指标

| 指标 | 说明 | 单位 |
|------|------|------|
| `node_cpu_usage` | CPU 使用率 | % |
| `node_memory_usage` | 内存使用率 | % |
| `node_disk_usage` | 磁盘使用率 | % |
| `node_network_rx` | 网络接收速率 | bytes/s |
| `node_network_tx` | 网络发送速率 | bytes/s |

### 1.2 应用指标

| 指标 | 说明 | 单位 |
|------|------|------|
| `axis_api_requests_total` | API 请求总数 | count |
| `axis_api_request_duration` | API 请求延迟 | ms |
| `axis_api_errors_total` | API 错误总数 | count |
| `axis_database_connections` | 数据库连接数 | count |
| `axis_active_sessions` | 活跃会话数 | count |

---

## 2. 告警阈值

### 2.1 严重告警（Critical）

| 指标 | 阈值 | 说明 |
|------|------|------|
| CPU 使用率 | > 90% (5 分钟) | CPU 负载过高 |
| 内存使用率 | > 95% (5 分钟) | 内存即将耗尽 |
| 磁盘使用率 | > 95% | 磁盘即将耗尽 |
| API 错误率 | > 10% (5 分钟) | 服务异常 |
| 服务宕机 | 服务停止 | 服务不可用 |

### 2.2 警告告警（Warning）

| 指标 | 阈值 | 说明 |
|------|------|------|
| CPU 使用率 | > 80% (5 分钟) | CPU 负载较高 |
| 内存使用率 | > 80% (5 分钟) | 内存使用较高 |
| 磁盘使用率 | > 90% | 磁盘空间不足 |
| API 响应时间 | P99 > 500ms | 响应缓慢 |
| API 错误率 | > 5% (5 分钟) | 错误率偏高 |

### 2.3 信息告警（Info）

| 指标 | 阈值 | 说明 |
|------|------|------|
| 磁盘使用率 | > 80% | 磁盘使用提醒 |
| API 响应时间 | P95 > 200ms | 响应时间提醒 |

---

## 3. Prometheus 配置

### 3.1 prometheus.yml

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

alerting:
  alertmanagers:
    - static_configs:
        - targets:
          - alertmanager:9093

rule_files:
  - "axis_alerts.yml"

scrape_configs:
  - job_name: 'axis'
    static_configs:
      - targets: ['axis:8080']
    metrics_path: '/metrics'
    
  - job_name: 'node'
    static_configs:
      - targets: ['node-exporter:9100']
```

### 3.2 axis_alerts.yml

```yaml
groups:
  - name: axis_alerts
    rules:
      # CPU 使用率告警
      - alert: HighCPUUsage
        expr: node_cpu_usage > 80
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "CPU 使用率过高"
          description: "CPU 使用率 {{ $value }}% 超过 80%"
      
      # 内存使用率告警
      - alert: HighMemoryUsage
        expr: node_memory_usage > 80
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "内存使用率过高"
          description: "内存使用率 {{ $value }}% 超过 80%"
      
      # 磁盘使用率告警
      - alert: HighDiskUsage
        expr: node_disk_usage > 90
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "磁盘使用率过高"
          description: "磁盘使用率 {{ $value }}% 超过 90%"
      
      # API 错误率告警
      - alert: HighAPIErrorRate
        expr: rate(axis_api_errors_total[5m]) / rate(axis_api_requests_total[5m]) > 0.05
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "API 错误率过高"
          description: "API 错误率 {{ $value }} 超过 5%"
      
      # 服务宕机告警
      - alert: AxisServiceDown
        expr: up{job="axis"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Axis 服务宕机"
          description: "Axis 服务已停止运行"
```

---

## 4. Grafana 仪表板

### 4.1 系统监控仪表板

**导入 ID：** 1860（Node Exporter Full）

**包含面板：**
- CPU 使用率（整体/每核心）
- 内存使用率
- 磁盘使用率
- 网络流量
- 系统负载

### 4.2 应用监控仪表板

**仪表板 JSON：**

```json
{
  "dashboard": {
    "title": "Axis NAS 监控",
    "panels": [
      {
        "title": "API 请求速率",
        "targets": [
          {
            "expr": "rate(axis_api_requests_total[5m])"
          }
        ]
      },
      {
        "title": "API 响应时间",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(axis_api_request_duration_bucket[5m]))"
          }
        ]
      },
      {
        "title": "API 错误率",
        "targets": [
          {
            "expr": "rate(axis_api_errors_total[5m])"
          }
        ]
      },
      {
        "title": "数据库连接数",
        "targets": [
          {
            "expr": "axis_database_connections"
          }
        ]
      },
      {
        "title": "活跃会话数",
        "targets": [
          {
            "expr": "axis_active_sessions"
          }
        ]
      }
    ]
  }
}
```

---

## 5. 告警通知

### 5.1 邮件通知

**Alertmanager 配置：**

```yaml
global:
  smtp_smarthost: 'smtp.example.com:587'
  smtp_from: 'alertmanager@example.com'
  smtp_auth_username: 'alertmanager@example.com'
  smtp_auth_password: 'password'

route:
  group_by: ['alertname']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 1h
  receiver: 'email-notifications'

receivers:
  - name: 'email-notifications'
    email_configs:
      - to: 'admin@example.com'
        send_resolved: true
```

### 5.2 钉钉通知

**Webhook 配置：**

```yaml
receivers:
  - name: 'dingtalk-notifications'
    webhook_configs:
      - url: 'https://oapi.dingtalk.com/robot/send?access_token=YOUR_TOKEN'
        send_resolved: true
```

### 5.3 短信通知

**配置示例：**

```yaml
receivers:
  - name: 'sms-notifications'
    webhook_configs:
      - url: 'https://sms-api.example.com/send'
        send_resolved: true
```

---

## 6. 日志收集

### 6.1 Fluentd 配置

```xml
<match axis.**>
  @type elasticsearch
  host elasticsearch
  port 9200
  logstash_format true
  logstash_prefix axis-logs
  <buffer>
    @type file
    path /var/log/fluentd/buffer
    flush_mode interval
    retry_type exponential_backoff
    flush_interval 5s
  </buffer>
</match>
```

### 6.2 日志级别配置

**config.toml：**

```toml
[logging]
level = "info"  # debug, info, warn, error
```

### 6.3 日志查看命令

```bash
# 查看实时日志
sudo journalctl -u axis -f

# 查看最近 100 行
sudo journalctl -u axis -n 100

# 查看特定级别日志
sudo journalctl -u axis -p err

# 查看特定时间范围
sudo journalctl -u axis --since "2026-04-08 00:00:00" --until "2026-04-08 23:59:59"
```

---

## 附录

### A. 监控架构

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Axis NAS  │────>│ Prometheus  │────>│   Grafana   │
│   :8080     │     │   :9090     │     │   :3000     │
└─────────────┘     └─────────────┘     └─────────────┘
                           │
                           v
                    ┌─────────────┐
                    │ Alertmanager│
                    │   :9093     │
                    └─────────────┘
                           │
                           v
                    ┌─────────────┐
                    │  通知渠道   │
                    │ 邮件/钉钉/短信│
                    └─────────────┘
```

### B. 快速开始

**1. 安装 Prometheus：**

```bash
docker run -d -p 9090:9090 \
  -v /path/to/prometheus.yml:/etc/prometheus/prometheus.yml \
  prom/prometheus
```

**2. 安装 Grafana：**

```bash
docker run -d -p 3000:3000 \
  -v grafana-storage:/var/lib/grafana \
  grafana/grafana
```

**3. 安装 Alertmanager：**

```bash
docker run -d -p 9093:9093 \
  -v /path/to/alertmanager.yml:/etc/alertmanager/alertmanager.yml \
  prom/alertmanager
```

### C. 联系方式

- 项目仓库：https://github.com/AIBAOS/axis
- 问题反馈：GitHub Issues
- 文档：/docs 目录

---

**文档版本：** v1.0.0  
**最后更新：** 2026-04-08  
**维护者：** 兵部
