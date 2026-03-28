# TOOLS.md - Local Notes

Skills define _how_ tools work. This file is for _your_ specifics — the stuff that's unique to your setup.

## What Goes Here

Things like:

- Camera names and locations
- SSH hosts and aliases
- Preferred voices for TTS
- Speaker/room names
- Device nicknames
- Anything environment-specific

## Examples

```markdown
### Cameras

- living-room → Main area, 180° wide angle
- front-door → Entrance, motion-triggered

### SSH

- home-server → 192.168.1.100, user: admin

### TTS

- Preferred voice: "Nova" (warm, slightly British)
- Default speaker: Kitchen HomePod
```

## Why Separate?

Skills are shared. Your setup is yours. Keeping them apart means you can update skills without losing your notes, and share skills without leaking your infrastructure.

---

Add whatever helps you do your job. This is your cheat sheet.

## GitHub

- **用户:** Qliangw
- **组织:** AIBAOS
- **权限:** admin
- **Token:** 已配置 (环境变量 GITHUB_TOKEN)

## Discord

- **Guild ID:** 1480548143908126720
- **nas-dev 线程 ID:** 1482978077134028830 (tasks 频道下)

## 朝廷架构 - 三省六部 Discord ID 名册

### 📌 艾特格式说明

**Discord 艾特格式：** `<@用户 ID>`

例如：`<@1481836163718189216>` 会显示为 @内阁

**复制即用：** 直接复制下面的完整 `<@ID>` 格式粘贴到消息中即可艾特

---

### 👑 主子

| 称呼 | Discord ID | 艾特格式 |
|------|-----------|---------|
| 飞玄真君 | 924854368472289280 | `<@924854368472289280>` |

---

### 🏛️ 三省 (决策/监察/文档)

| 部门 | Discord ID | 艾特格式 | 职责 |
|------|-----------|---------|------|
| 内阁 | 1481836163718189216 | `<@1481836163718189216>` | 战略决策、方案审议、全局规划 |
| 都察院 | 1481843512675926130 | `<@1481843512675926130>` | 监察审计、代码审查、质量把控 |
| 翰林院 | 1481843996094496798 | `<@1481843996094496798>` | 学术研究、知识整理、文档撰写 |

---

### ⚔️ 六部 (执行部门)

| 部门 | Discord ID | 艾特格式 | 职责 |
|------|-----------|---------|------|
| 兵部 | 1480545234621435984 | `<@1480545234621435984>` | 软件工程、系统架构、编码开发 |
| 户部 | 1480549863438422156 | `<@1480549863438422156>` | 财务预算、电商运营、成本分析 |
| 礼部 | 1480550785816199358 | `<@1480550785816199358>` | 品牌营销、内容创作、宣传推广 |
| 工部 | 1480551447673311313 | `<@1480551447673311313>` | DevOps、服务器运维、部署监控 |
| 吏部 | 1480551970077937757 | `<@1480551970077937757>` | 项目管理、创业孵化、人事协调 |
| 刑部 | 1480552316602945718 | `<@1480552316602945718>` | 法务合规、知识产权、合同审核 |

---

### 📋 快速派发模板

```
<@1480545234621435984>

【角色】xxx
【任务】xxx
【背景】xxx
【要求】xxx
【格式】xxx
```

---

### ⚠️ 派发铁律

1. **每次只 @一个部门** — 禁止同时 @多人
2. **必须等反馈再派下一个** — 顺序执行
3. **找人做事一定要艾特** — 不能只发消息
4. **内阁审批单独提请** — 一次一个事项

---

### 🚨 艾特格式铁律（内阁审定！）

**✅ 正确格式：**

```
<@1481836163718189216>

【请内阁裁决】...
```

**❌ 错误格式：**

```
@内阁 <@1481836163718189216>  ← 前面不能有任何文字！

【请内阁裁决】...
@内阁 <@1481836163718189216>  ← 放在后面或中间都不对！

```<@1481836163718189216>```  ← 不能放代码块！
```

**铁律：**
- **艾特必须在消息第一行**
- **单独成行**
- **前面不能有任何文字（包括 @部门）**
- **不得嵌入正文中间**
- **不得放在代码块里**
- **不得用 markdown 包裹**

**记忆口诀：一行标签二行空，三行正文记心中。**

**内阁盯着，下次再错，打回重发！**

---

**此名册永久保存，新 session 启动后必读！**
