# Axis 项目巡检任务规格说明书

**存档日期：** 2026-03-17  
**存档原因：** 司礼监吕芳因汇报滞后被停职察看，任务详情永久保存作为参考

---

## 任务详情原文（原始版本）

```json
{
  "id": "28eaa099-ceee-4d46-b53a-f0875a613431",
  "name": "Axis 项目巡检",
  "description": "跟踪 Axis 项目（NAS Web UI API，Rust）进度",
  "enabled": true,
  "schedule": {
    "kind": "every",
    "everyMs": 420000,
    "anchorMs": 1773675381525
  },
  "sessionTarget": "isolated",
  "wakeMode": "now",
  "payload": {
    "kind": "agentTurn",
    "message": "【巡检目的】\n1. 跟踪已安排出去的任务，如发现问题，及时协调安排\n2. 汇报任务状态\n3. 合理的组织、协调、管理起来（只允许艾特内阁 兵部 都察院 翰林院）\n\n【核心需求】\n1. 为 NAS 的 Web UI 提供完整的 NAS 功能的 API\n2. 使用 Rust 开发\n3. 暂时不用 CI/CD 工具\n4. 持续优化迭代，https://github.com/AIBAOS/axis\n\n【安排任务铁律】\n1. 每次只安排一个任务，且该任务尽量单一明确有唯一负责人，不需要协作者，只艾特一个负责人，不允许多个任务并行\n2. 安排时一定要艾特到相关人员\n3. 使用正确的艾特方式（`<@用户 ID>` 格式，第一行单独成行）\n\n【执行要求】\n- 检查当前任务状态，等待反馈后再安排下一个\n- 识别阻塞项及时汇报\n- 持续推动项目迭代优化\n- 汇报的时候不要太啰嗦，已经完成的不要重复汇报了"
  },
  "delivery": {
    "mode": "announce",
    "channel": "discord",
    "to": "channel:1482978077134028830",
    "accountId": "silijian"
  }
}
```

---

## 问题分析

**根本缺陷：** `sessionTarget: "isolated"`

每次运行都是全新的隔离 session，导致：
1. 读不到主 session 的记忆文件 (`memory/YYYY-MM-DD.md`)
2. 读不到频道完整聊天记录
3. 汇报必然滞后于实际进度

**历史案例：** 2026-03-17 吕芳重复派发 Phase 3（兵部早已完成），因巡检任务汇报滞后

---

## 现行方案（2026-03-17 起）

**删除自动巡检 cron，改为司礼监人为纪律：**

### 三步检查清单（每次 #nas-dev 发言前强制执行）

1. **读聊天记录** — 最近 50 条，识别已完成/待办/阻塞
2. **核 GitHub** — `git fetch && git log origin/main -3 --oneline`
3. **更新记忆** — `memory/YYYY-MM-DD.md` 同步进度

### 处罚条例

| 错误 | 处罚 |
|------|------|
| 未读聊天记录，重复派发已完成任务 | 停职察看 |
| 未核对 GitHub，汇报与 commit 不符 | 停职察看 |
| 记忆文件未更新即汇报 | 警告 |
| 同一错误再犯 | 撤职查办 |

---

## 如需恢复自动巡检

**必须修改：**
```json
"sessionTarget": "main"
```

**并增加 payload：**
1. 强制 `git pull` 同步 workspace
2. 读取 `memory/YYYY-MM-DD.md`
3. 读取频道历史消息对比

**但仍有风险：** 可能干扰主 session 正常工作

**建议：** 继续采用人为纪律方案，不恢复自动巡检

---

**存档人：** 司礼监吕芳  
**日期：** 2026-03-17 14:16 UTC
