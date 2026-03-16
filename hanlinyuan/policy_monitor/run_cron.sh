#!/bin/bash
# 政策监控系统 - 定时任务脚本
# 翰林院编制 - 每 8 小时执行一次

cd /home/node/.openclaw/workspace/hanlinyuan/policy_monitor

# 确保 PATH 包含 pip 用户目录
export PATH="/home/node/.local/bin:$PATH"

# 运行爬虫
python3 crawler_v2.py >> logs/cron_$(date +%Y%m%d).log 2>&1

echo "政策监控抓取完成 - $(date)"
