#!/bin/bash
# Axis 项目自动巡检脚本
# 用法：./axis-inspect.sh

set -e

WORKSPACE="/home/node/.openclaw/workspace/silijian"
MEMORY_FILE="$WORKSPACE/memory/2026-03-17.md"
REPO_DIR="$WORKSPACE/bingbu/project/nas/axis"

echo "📊 Axis 项目自动巡检 ($(date -u '+%Y-%m-%d %H:%M UTC'))"
echo "========================================"

# 1. 获取 GitHub 最新提交
echo "📌 获取最新提交..."
cd "$REPO_DIR"
git fetch origin main 2>/dev/null || true
LATEST_COMMIT=$(git log origin/main -1 --format="%h" 2>/dev/null || echo "未知")
COMMIT_DATE=$(git log origin/main -1 --format="%ci" 2>/dev/null || echo "未知")
COMMIT_MSG=$(git log origin/main -1 --format="%s" 2>/dev/null || echo "未知")

echo "最新提交：$LATEST_COMMIT ($COMMIT_DATE)"
echo "提交信息：$COMMIT_MSG"

# 2. 读取 README 进度状态
echo ""
echo "📋 读取项目进度..."
if [ -f "$REPO_DIR/README.md" ]; then
    echo "README.md 存在，可解析进度"
else
    echo "⚠️ README.md 不存在"
fi

# 3. 输出检查报告
echo ""
echo "========================================"
echo "巡检完成。请根据以上信息更新 memory/2026-03-17.md"
echo ""
echo "最新状态摘要："
echo "- 最新 commit: $LATEST_COMMIT"
echo "- 提交时间：$COMMIT_DATE"
echo "- 提交内容：$COMMIT_MSG"
