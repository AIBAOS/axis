# Axis 项目进度追踪

> 最后更新：2026-03-28 02:00 UTC

## 📌 当前状态

| 项目 | 状态 |
|------|------|
| 最新 commit | [待提交] |
| 提交时间 | 2026-03-28 02:00 UTC |
| 当前阶段 | Phase 203 SMB 共享详情 API |
| 状态 | ✅ 已完成 |
| 阻塞项 | 无 |

---

## 📊 模块进度

| 模块 | 状态 | Commit | 完成时间 |
|------|:----:|--------|----------|
| 核心框架 | ✅ | - | 03-14 |
| JWT 认证 | ✅ | 0b34819 | 03-17 05:52 |
| 共享链接 | ✅ | 33936d9 | 03-17 08:15 |
| 文件管理 API | ✅ | 9a4d626 | 03-17 08:35 |
| 会话管理 API | ✅ | 34d0f0e | 03-17 08:48 |
| RBAC 集成 | ✅ | 5c7607a | 03-17 11:59 |
| 存储管理 API | ✅ | 03435051 | 03-17 15:52 |
| 用户配额 | ✅ | 64bb3f19 | 03-17 16:02 |
| 系统设置 | ✅ | 6b01e97f | 2026-03-17 23:46 |
| 审计日志 | ✅ | b71bb92 | 2026-03-18 03:30 |
| 网络管理 API | ✅ | a776d42 | 2026-03-18 03:38 |
| 打印机管理 API | ✅ | 9af69bd | 2026-03-18 06:08 |
| 媒体服务器 API | ✅ | a2ad47f | 2026-03-18 06:32 |
| 媒体扫描器 API | ✅ | 527e8f9 | 2026-03-18 06:56 |
| 后台任务管理 API | ✅ | d922df5 | 2026-03-18 09:20 |
| 系统信息 API | ✅ | f7dafdb | 2026-03-18 09:46 |
| 缓存管理 API | ✅ | d3270c6 | 2026-03-18 10:01 |
| 日志管理 API | ✅ | cb93cca | 2026-03-18 10:38 |
| 后台任务管理 API (完整) | ✅ | e87a264 | 2026-03-18 10:51 |
| 数据库优化 API | ✅ | c26449c | 2026-03-18 11:17 |
| **下载管理 API** | ✅ | b165cb9 | 2026-03-19 13:55 |
| **存储卷管理 API** | ✅ | e55e210 | 2026-03-18 18:26 |
| **文件搜索 API** | ✅ | 71e94ad | 2026-03-26 03:52 |
| **创建用户 API** | ✅ | [待提交] | 2026-03-26 05:00 |
| **创建用户 API (Phase 47)** | ✅ | (本次) | 2026-03-26 04:20 |
| **用户列表 API (Phase 48)** | ✅ | 5f1a288 | 2026-03-26 04:23 |
| **用户详情 API (Phase 49)** | ✅ | 9adcda6 | 2026-03-26 04:34 |
| **更新用户 API (Phase 50)** | ✅ | (本次) | 2026-03-26 04:42 |
| **删除用户 API (Phase 51)** | ✅ | (本次) | 2026-03-26 04:45 |
| **用户登录 API (Phase 52)** | ✅ | (本次) | 2026-03-26 05:00 |
| **更新用户 API (Phase 103)** | ✅ | ea45a3d | 2026-03-26 17:10 |
| **打印机列表 API (Phase 53)** | ✅ | (本次) | 2026-03-26 05:20 |
| **打印机详情 API (Phase 54)** | ✅ | (本次) | 2026-03-26 05:30 |
| **创建打印机 API (Phase 55)** | ✅ | (本次) | 2026-03-26 05:40 |
| **打印机列表 API (Phase 56)** | ✅ | (本次) | 2026-03-26 05:55 |
| **更新打印机 API (Phase 57)** | ✅ | (本次) | 2026-03-26 06:05 |

---

## 🛠️ 已解决问题

| 日期 | 问题 | 解决方案 | 负责人 |
|------|------|----------|--------|
| 2026-03-17 19:50 | 兵部 bot HTTP 400 错误（上下文超限 204800 字符） | 清空会话缓存，预估工部模型无此限制 | 工部 |

---

## 📋 待办事项

- [ ] Phase 203 SMB 共享详情 API (P1)
  - [ ] GET /api/v1/shares/smb/{id} — 获取 SMB 共享详情
  - [ ] JWT 认证，任意登录用户可访问
  - [ ] 返回完整共享信息

- [x] Phase 202 SMB 共享列表 API - 2026-03-28 01:50
  - [x] GET /api/v1/shares/smb — 获取 SMB 共享列表
  - [x] JWT 认证，任意登录用户可访问
  - [x] 支持分页：page, limit（最大 100）
  - [x] 支持筛选：public 字段
  - [x] 返回 7 个字段：id/name/path/description/public/created_at/updated_at
  - [x] 文档：docs/shares_smb_list_api.md

---

## 🔄 进行中

无

---

## ✅ 已完成事项

- [x] SMB 共享创建 API (Phase 201) - 2026-03-28 01:40
  - POST /api/v1/shares/smb — 创建 SMB 共享（SQLite 持久化版）
  - JWT 认证，admin 角色可访问
  - 使用 SqliteShareRepository 实现真实数据库创建
  - 验证名称格式（400 Bad Request）
  - 验证路径格式（400 Bad Request）
  - 验证名称唯一性（409 Conflict）
  - 返回字段：id/name/path/comment/read_only/guest_access/browseable/valid_users/invalid_users/status/created_at/updated_at
  - 创建成功返回 201 Created
  - 文档：docs/shares_smb_create_api.md
  - Commit: 6832574

- [x] 系统通知标记已读 API (POST) (Phase 200) - 2026-03-28 01:25
  - POST /api/v1/system/notifications/{id}/mark-read — 标记通知为已读（POST 版本）
  - JWT 认证，登录用户可访问
  - 支持标记系统通知和个人通知
  - 验证通知归属（403 Forbidden）
  - 已读通知返回 409 Conflict
  - 返回更新后的通知摘要
  - 文档：docs/system_notifications_mark_read_post_api.md
  - Commit: e6ee338

- [x] 系统通知标记已读 API (PUT) (Phase 199) - 2026-03-28 01:15
  - PUT /api/v1/system/notifications/{id}/read — 标记系统通知为已读
  - JWT 认证，登录用户可访问
  - 验证通知存在性（404 Not Found）
  - 仅允许标记系统通知（target_user_id IS NULL）
  - 更新 is_read = 1, read_at = 当前时间戳
  - 返回更新后的通知摘要
  - 文档：docs/system_notifications_mark_read_api.md
  - Commit: c6c9b36

- [x] SMB 共享列表 API (Phase 198) - 2026-03-28 00:55
  - GET /api/v1/shares/smb — 获取 SMB 共享列表（SQLite 持久化版）
  - JWT 认证，admin 角色可访问
  - 使用 SqliteShareRepository 实现真实数据库查询
  - 支持分页：page(默认 1)/per_page(默认 20, 最大 100)
  - 支持状态筛选：status(active/inactive)
  - 返回字段：id/name/path/status/read_only/guest_access/enabled/created_at/updated_at
  - 响应格式：success + data + pagination(page/per_page/total/total_pages)
  - 文档：docs/shares_smb_list_api.md
  - Commit: d5c82f6

- [x] 系统通知列表 API (Phase 197) - 2026-03-28 00:50
  - GET /api/v1/system/notifications — 获取系统级别的通知列表
  - JWT 认证，登录用户可访问
  - 筛选条件：target_user_id IS NULL（全局系统通知）
  - 支持分页：page(默认 1)/page_size(默认 20)
  - 支持优先级筛选：priority(low/normal/high/critical)
  - 返回字段：id/title/message/type/priority/is_read/created_at/action_url
  - 文档：docs/system_notifications_list_api.md
  - Commit: 44df3a7

- [x] 备份统计 API (Phase 196) - 2026-03-28 00:35 (修正 00:38)
  - GET /api/v1/backups/stats — 获取备份任务和执行的统计信息
  - JWT 认证，admin 角色可访问
  - 返回字段：total_backups/active_backups/archived_backups/total_executions/successful_executions/failed_executions/running_executions/last_execution_at/next_scheduled_execution/storage_used_bytes
  - 使用 SqliteBackupRepository 实现真实数据库聚合查询
  - 用于仪表板展示备份模块整体状态
  - 文档：docs/backups_stats_api.md
  - Commit: e70b87b + 3183a29 (修正)

- [x] 备份执行历史 API (Phase 195) - 2026-03-28 00:25
  - GET /api/v1/backups/{id}/execution-history — 获取备份任务执行历史记录
  - JWT 认证，登录用户可访问
  - 使用 SqliteBackupRepository 实现真实数据库查询
  - 返回字段：execution_id/backup_id/status/started_at/completed_at/duration_seconds/error_message
  - 支持分页：page(默认 1)/per_page(默认 20)
  - 按 started_at 降序排列（最新的在前）
  - 错误处理：404 Not Found / 500 Database Error
  - 文档：docs/backups_execution_history_api.md
  - Commit: 2c2ba00

- [x] 备份归档 API (Phase 194) - 2026-03-28 00:10
  - POST /api/v1/backups/{id}/archive — 归档活跃的备份任务
  - JWT 认证，admin 角色可访问
  - 使用 SqliteBackupRepository 实现真实数据库操作
  - 状态流转：active/completed → archived
  - 状态校验：仅 active/completed 可归档（running 返回 400，archived 返回 409）
  - 完善错误处理（404 Not Found / 400 Bad Request / 409 Conflict / 500 Database Error）
  - 归档成功返回 200 OK + 备份完整信息
  - 文档：docs/backups_archive_api.md
  - Commit: f26b07e

- [x] 备份恢复 API (Phase 193) - 2026-03-28 00:05
  - POST /api/v1/backups/{id}/restore — 恢复已归档的备份任务
  - JWT 认证，admin 角色可访问
  - 使用 SqliteBackupRepository 实现真实数据库操作
  - 状态流转：archived → active
  - 冲突检测：非 archived 状态返回 409 / 已有活跃备份返回 409
  - 完善错误处理（404 Not Found / 409 Conflict / 500 Database Error）
  - 恢复成功返回 200 OK + 备份完整信息
  - 文档：docs/backups_restore_api.md
  - Commit: 20c388f

- [x] 备份任务删除 API (Phase 192) - 2026-03-27 23:45
  - DELETE /api/v1/backups/{id} — 删除备份任务
  - JWT 认证，admin 角色可访问
  - 使用 SqliteBackupRepository 实现真实数据库删除
  - 添加运行中任务保护（409 Conflict）
  - 完善错误处理（404 Not Found / 409 Conflict / 500 Database Error）
  - 删除成功返回 200 OK + 确认消息
  - Commit: eeefbe3

- [x] 备份任务更新 API (Phase 191) - 2026-03-27 19:58
  - PUT /api/v1/backups/{id} — 更新备份任务配置
  - JWT 认证，admin 角色可访问
  - 支持部分更新：name/schedule/enabled/retention_days/source_paths/destination
  - 验证备份 ID 存在性（404 Not Found）
  - 验证 schedule 格式（daily/weekly/monthly/hourly/cron）
  - 更新成功返回 200 OK + 任务详情
  - 文档：docs/backups_update_api.md

- [x] 备份任务执行 API (Phase 190) - 2026-03-27 18:14
  - POST /api/v1/backups/{id}/execute — 手动触发备份任务执行
  - JWT 认证，admin 角色可访问
  - 验证备份 ID 存在性（404 Not Found）
  - 验证任务状态（仅 completed/failed 可重新执行，400 Bad Request）
  - 返回字段：backup_id/status/message/started_at
  - 文档：docs/backups_execute_api.md

- [x] 备份任务详情 API (Phase 189) - 2026-03-27 18:12
  - GET /api/v1/backups/{id} — 获取单个备份任务详情
  - JWT 认证，登录用户可访问
  - 返回字段：id/name/description/source_path/destination_path/schedule/status/last_run/next_run/last_duration/last_status/retention_policy/created_at/updated_at
  - 任务不存在返回 404 Not Found
  - 文档：docs/backups_detail_api.md

- [x] 备份任务列表 API (Phase 188) - 2026-03-27 17:28
  - GET /api/v1/backups — 获取所有备份任务列表
  - JWT 认证，登录用户可访问
  - 返回字段：id/name/description/source_path/destination_path/schedule/status/last_run/next_run/created_at/updated_at
  - 文档：docs/backups_list_api.md

- [x] 磁盘 S.M.A.R.T. 信息 API (Phase 188) - 2026-03-27 16:43
  - GET /api/v1/storage/disks/{id}/smart — 获取磁盘 S.M.A.R.T. 健康信息
  - JWT 认证，登录用户可访问
  - 返回字段：disk_id/model/serial_number/firmware_version/temperature/power_on_hours/spin_up_time/reallocated_sectors/pending_sectors/uncorrectable_sectors/wear_leveling/health_status/last_check
  - 磁盘不存在返回 404 Not Found
  - 文档：docs/disk_smart_api.md

- [x] 存储卷详情 API (Phase 187) - 2026-03-27 16:30
  - GET /api/v1/storage/volumes/{id} — 获取单个存储卷详情
  - JWT 认证，登录用户可访问
  - 返回字段：id/name/total_bytes/used_bytes/available_bytes/usage_percent/status/filesystem_type/mount_point/created_at/updated_at
  - 卷不存在返回 404 Not Found
  - 文档：docs/storage_volume_detail_api.md

- [x] 存储卷列表 API (Phase 187) - 2026-03-27 16:17
  - GET /api/v1/storage/volumes — 获取所有存储卷列表
  - JWT 认证，登录用户可访问
  - 返回字段：id/name/total_bytes/used_bytes/available_bytes/usage_percent/status/filesystem_type/mount_point
  - 文档：docs/storage_volumes_list_api.md

- [x] 网络接口删除 API (Phase 186) - 2026-03-27 15:51
  - DELETE /api/v1/network/interfaces/{id} — 删除网络接口
  - JWT 认证，admin 角色可访问
  - 验证接口 ID 存在性（404 Not Found）
  - 删除成功返回 204 No Content
  - 文档：docs/network_interface_delete_api.md

- [x] 网络接口更新 API (Phase 185) - 2026-03-27 15:25
  - PUT /api/v1/network/interfaces/{id} — 更新网络接口
  - JWT 认证，admin 角色可访问
  - 支持部分更新：name/ip_address/netmask/gateway/mac_address/interface_type/speed_mbps/mtu/status
  - 验证 IP 地址/子网掩码/网关格式（400 Bad Request）
  - 验证接口 ID 存在性（404 Not Found）
  - 更新成功返回 200 OK + 接口详情
  - 文档：docs/network_interface_update_api.md

- [x] 网络接口创建 API (Phase 184) - 2026-03-27 15:12
  - POST /api/v1/network/interfaces — 创建网络接口
  - JWT 认证，admin 角色可访问
  - 请求字段：name/interface/ip_address/netmask/gateway/interface_type/speed_mbps/mtu
  - 验证 IP 地址/子网掩码/网关格式（400 Bad Request）
  - 验证接口名称唯一性（409 Conflict）
  - 创建成功返回 201 Created + 接口详情
  - 文档：docs/network_interfaces_create_api.md

- [x] 网络接口详情 API (Phase 183) - 2026-03-27 14:59
  - GET /api/v1/network/interfaces/{id} — 获取单个网络接口详情
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/interface/ip_address/netmask/gateway/mac_address/status/interface_type/speed_mbps/mtu/rx_bytes/tx_bytes/rx_packets/tx_packets/rx_errors/tx_errors/created_at/updated_at
  - 接口不存在返回 404 Not Found
  - 包含流量统计信息
  - 文档：docs/network_interface_detail_api.md

- [x] 网络接口列表 API (Phase 182) - 2026-03-27 14:46
  - GET /api/v1/network/interfaces — 获取所有网络接口列表
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/interface/ip_address/netmask/gateway/mac_address/status/interface_type/speed_mbps/mtu/created_at/updated_at
  - 文档：docs/network_interfaces_list_api.md

- [x] 存储磁盘详情 API (Phase 181) - 2026-03-27 14:33
  - GET /api/v1/storage/disks/{id} — 获取单个磁盘的详细信息
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/path/model/serial_number/disk_type/size_bytes/size_human/temperature/smart_status/health_status/speed_rpm/power_on_hours/status/in_use/storage_pool_id
  - 磁盘不存在返回 404 Not Found
  - 容量自动格式化为人类可读格式（TB/GB/MB）
  - 文档：docs/storage_disk_detail_api.md

- [x] 存储磁盘列表 API (Phase 180) - 2026-03-27 14:00
  - GET /api/v1/storage/disks — 获取 NAS 上所有磁盘的信息
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/path/model/serial_number/type/size_bytes/size_human/temperature/smart_status/health_status/speed_rpm/power_on_hours/status/in_use
  - 支持分页：page(默认 1), limit(默认 20, 最大 100)
  - 支持筛选：disk_type(hdd/ssd/nvme), smart_status(healthy/warning/failed/unknown), status(online/offline)
  - 容量自动格式化为人类可读格式（TB/GB/MB）
  - 文档：docs/storage_disks_api.md

- [x] 容器列表 API Enhanced (Phase 142) - 2026-03-27 03:56
  - GET /api/v1/containers — 返回所有容器列表
  - JWT 认证，仅 admin 角色可访问
  - 支持分页：page(默认 1), per_page(默认 10, 最大 100)
  - 支持过滤：status(running/stopped/paused), network(网络名称)
  - 返回字段：id/name/image/status/ports/networks/created_at/cpu_usage/memory_usage
  - 响应格式：{ containers: [...], pagination: { page, per_page, total, total_pages } }
  - 更新文档 docs/containers_list_api.md

- [x] 网络配置列表 API (Phase 179) - 2026-03-27 13:28
  - GET /api/v1/network/config — 获取网络配置列表
  - JWT 认证，admin 角色可访问
  - 返回所有网络接口配置（interface/ip_address/netmask/gateway/dns/dhcp_enabled/enabled）
  - 文档：docs/network_config_list_api.md

- [x] 系统告警删除 API (Phase 178) - 2026-03-27 13:17
  - DELETE /api/v1/system/alerts/{id} — 删除系统告警
  - JWT 认证，admin 角色可访问
  - 验证告警 ID 存在性（404 Not Found）
  - 验证告警状态（仅 acknowledged/resolved 状态可删除）
  - 支持可选备注字段（note）
  - 删除成功返回 204 No Content
  - 文档：docs/system_alerts_delete_api.md

- [x] 系统告警解决 API (Phase 177) - 2026-03-27 13:15
  - POST /api/v1/system/alerts/{id}/resolve — 解决系统告警
  - JWT 认证，admin 角色可访问
  - 验证告警 ID 存在性（404 Not Found）
  - 验证告警状态（仅 active/acknowledged 状态可解决）
  - 更新状态为 resolved，记录解决时间和解决人
  - 支持可选备注字段
  - 文档：docs/system_alerts_resolve_api.md

- [x] 系统告警确认 API (Phase 176) - 2026-03-27 13:03
  - POST /api/v1/system/alerts/{id}/acknowledge — 确认系统告警
  - JWT 认证，admin 角色可访问
  - 验证告警 ID 存在性（404 Not Found）
  - 验证告警状态（仅 active 状态可确认）
  - 更新状态为 acknowledged，记录确认时间和确认人
  - 支持可选备注字段
  - 文档：docs/system_alerts_acknowledge_api.md

- [x] 系统告警详情 API (Phase 175) - 2026-03-27 12:49
  - GET /api/v1/system/alerts/{id} — 获取系统告警详情
  - JWT 认证，admin 角色可访问
  - 验证告警 ID 存在性（404 Not Found）
  - 返回字段：id/title/message/severity/status/source/created_at/acknowledged_at/acknowledged_by/resolved_at/resolved_by/metadata
  - 文档：docs/system_alerts_detail_api.md

- [x] 系统告警列表 API (Phase 174) - 2026-03-27 12:36
  - GET /api/v1/system/alerts — 获取系统告警列表
  - JWT 认证，admin 角色可访问
  - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
  - 支持筛选：status(active/resolved/acknowledged), severity(critical/warning/info), source
  - 按 created_at 降序排序
  - 返回字段：id/title/message/severity/status/source/created_at/acknowledged_at/resolved_at
  - 响应格式：{ data: [...], pagination: { page, per_page, total, total_pages } }
  - 文档：docs/system_alerts_list_api.md

- [x] 系统日志导出 API (Phase 173) - 2026-03-27 12:23
  - POST /api/v1/system/logs/export — 导出系统日志
  - JWT 认证，admin 角色可访问
  - 支持导出格式：CSV/JSON
  - 支持按时间范围、日志级别、来源筛选
  - 支持分页（每页最大 1000 条）
  - 返回文件下载（Content-Disposition 头）
  - 文档：docs/system_logs_export_api.md

- [x] 系统日志详情 API (Phase 172) - 2026-03-27 12:10
  - GET /api/v1/system/logs/{id} — 获取系统日志详情
  - JWT 认证，admin 角色可访问
  - 验证日志 ID 存在性（404 Not Found）
  - 返回字段：id/timestamp/level/source/message/details/context/user
  - 文档：docs/system_logs_detail_api.md

- [x] 容器删除 API (Phase 171) - 2026-03-27 11:57
  - DELETE /api/v1/containers/{id} — 删除容器
  - JWT 认证，admin 角色可访问
  - 验证容器 ID 存在性（404 Not Found）
  - 删除成功返回 204 No Content
  - 文档：docs/containers_delete_api.md

- [x] 容器更新 API (Phase 170) - 2026-03-27 11:31
  - PUT /api/v1/containers/{id} — 更新容器配置
  - JWT 认证，admin 角色可访问
  - 支持部分更新：name/image/ports/networks/env/cpu_limit/memory_limit
  - 验证容器 ID 存在性（404 Not Found）
  - 验证名称格式（1-128 字符）
  - 验证镜像格式（1-256 字符）
  - 验证名称唯一性（409 Conflict，排除自身）
  - 更新成功返回 200 OK + 容器详情
  - 文档：docs/containers_update_api.md

- [x] 容器创建 API (Phase 169) - 2026-03-27 11:05
  - POST /api/v1/containers — 创建容器
  - JWT 认证，admin 角色可访问
  - 请求字段：name/image/ports/networks/env
  - 验证名称格式（1-128 字符）
  - 验证镜像格式（1-256 字符）
  - 验证名称唯一性（409 Conflict）
  - 创建成功返回 201 Created + 容器详情
  - 文档：docs/containers_create_api.md

- [x] 容器列表 API (Phase 168) - 2026-03-27 10:52
  - GET /api/v1/containers — 获取容器列表
  - JWT 认证，admin 角色可访问
  - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
  - 返回字段：id/name/image/status/created_at
  - 响应格式：{ data: [...], pagination: { page, per_page, total, total_pages } }
  - 文档：docs/containers_list_api.md

- [x] 容器详情 API (Phase 167) - 2026-03-27 10:39
  - GET /api/v1/containers/{id} — 获取容器详情
  - JWT 认证，admin 角色可访问
  - 验证容器 ID 存在性（404 Not Found）
  - 返回字段：id/name/image/status/ports/networks/created_at/started_at/cpu_usage/memory_usage
  - 文档：docs/containers_detail_api.md

- [x] 备份删除 API (Phase 166) - 2026-03-27 10:26
  - DELETE /api/v1/backups/{id} — 删除备份任务
  - JWT 认证，admin 角色可访问
  - 验证备份 ID 存在性（404 Not Found）
  - 删除成功返回 204 No Content
  - 文档：docs/backups_delete_api.md

- [x] 备份更新 API (Phase 165) - 2026-03-27 10:13
  - PUT /api/v1/backups/{id} — 更新备份任务
  - JWT 认证，admin 角色可访问
  - 支持部分更新：name/type/size/status/source_path/destination_path/compression/encryption
  - 验证备份 ID 存在性（404 Not Found）
  - 验证名称格式（1-128 字符）
  - 验证备份类型（daily/weekly/monthly/manual）
  - 验证路径格式（以/开头，最大 512 字符）
  - 验证状态（pending/running/completed/failed）
  - 更新成功返回 200 OK + 备份详情
  - 文档：docs/backups_update_api.md

- [x] 备份详情 API (Phase 164) - 2026-03-27 10:00
  - GET /api/v1/backups/{id} — 获取备份详情
  - JWT 认证，admin 角色可访问
  - 验证备份 ID 存在性（404 Not Found）
  - 返回字段：id/name/type/size/status/source_path/destination_path/compression/encryption/created_at/completed_at
  - 文档：docs/backups_detail_api.md

- [x] 备份创建 API (Phase 163) - 2026-03-27 09:47
  - POST /api/v1/backups — 创建备份任务
  - JWT 认证，admin 角色可访问
  - 请求字段：name/type/source_path/destination_path/compression/encryption
  - 验证名称格式（1-128 字符）
  - 验证备份类型（daily/weekly/monthly/manual）
  - 验证路径格式（以/开头，最大 512 字符）
  - 创建成功返回 201 Created + 备份详情
  - 文档：docs/backups_create_api.md

- [x] 备份列表 API (Phase 162) - 2026-03-27 09:34
  - GET /api/v1/backups — 获取备份列表
  - JWT 认证，admin 角色可访问
  - 支持分页：page(默认 1), limit(默认 20, 最大 100)
  - 返回字段：id/name/type/size/status/created_at/completed_at
  - 响应格式：{ data: [...], pagination: { page, limit, total, total_pages } }
  - 文档：docs/backups_list_api.md

- [x] 用户列表 API (Phase 161) - 2026-03-27 09:21
  - GET /api/v1/users — 获取用户列表
  - JWT 认证，admin 角色可访问
  - 支持分页：page(默认 1), limit(默认 20, 最大 100)
  - 返回字段：id/username/email/role/created_at
  - 响应格式：{ data: [...], pagination: { page, limit, total, total_pages } }
  - 文档：docs/users_list_api.md

- [x] NFS 共享删除 API (Phase 160) - 2026-03-27 09:08
  - DELETE /api/v1/shares/nfs/{id} — 删除 NFS 共享
  - JWT 认证，仅 admin 角色可访问
  - 验证共享 ID 存在性（404 Not Found）
  - 删除成功返回 204 No Content
  - 文档：docs/shares_nfs_delete_api.md

- [x] SMB 共享删除 API (Phase 159) - 2026-03-27 08:55
  - DELETE /api/v1/shares/smb/{id} — 删除 SMB 共享
  - JWT 认证，仅 admin 角色可访问
  - 验证共享 ID 存在性（404 Not Found）
  - 删除成功返回 204 No Content
  - 文档：docs/shares_smb_delete_api.md

- [x] NFS 共享更新 API (Phase 158) - 2026-03-27 08:43
  - PUT /api/v1/shares/nfs/{id} — 更新 NFS 共享
  - JWT 认证，仅 admin 角色可访问
  - 支持部分更新：name/path/comment/read_only/no_subtree_check/sync/clients
  - 客户端配置：network(CIDR)/access(ro/rw)
  - 验证共享 ID 存在性（404 Not Found）
  - 验证名称格式（1-64 字符，字母数字 -_.）
  - 验证路径格式（以/开头，最大 256 字符）
  - 验证客户端配置（至少 1 个，CIDR 格式）
  - 验证名称唯一性（409 Conflict，排除自身）
  - 更新成功返回 200 OK + 共享详情
  - 文档：docs/shares_nfs_update_api.md

- [x] SMB 共享更新 API (Phase 157) - 2026-03-27 08:29
  - PUT /api/v1/shares/smb/{id} — 更新 SMB 共享
  - JWT 认证，仅 admin 角色可访问
  - 支持部分更新：name/path/comment/read_only/guest_access/browseable/valid_users/invalid_users
  - 验证共享 ID 存在性（404 Not Found）
  - 验证名称格式（1-64 字符，字母数字 -_.）
  - 验证路径格式（以/开头，最大 256 字符）
  - 验证名称唯一性（409 Conflict，排除自身）
  - 更新成功返回 200 OK + 共享详情
  - 文档：docs/shares_smb_update_api.md

- [x] NFS 共享详情 API (Phase 156) - 2026-03-27 08:16
  - GET /api/v1/shares/nfs/{id} — 获取 NFS 共享详情
  - JWT 认证，仅 admin 角色可访问
  - 验证共享 ID 存在性（404 Not Found）
  - 返回字段：id/name/path/comment/read_only/no_subtree_check/sync/clients/enabled/status/created_at/updated_at
  - 客户端配置：network(CIDR)/access(ro/rw)
  - 文档：docs/shares_nfs_get_api.md

- [x] SMB 共享详情 API (Phase 155) - 2026-03-27 08:03
  - GET /api/v1/shares/smb/{id} — 获取 SMB 共享详情
  - JWT 认证，仅 admin 角色可访问
  - 验证共享 ID 存在性（404 Not Found）
  - 返回字段：id/name/path/comment/read_only/guest_access/browseable/valid_users/invalid_users/enabled/status/created_at/updated_at
  - 文档：docs/shares_smb_get_api.md

- [x] NFS 共享创建 API (Phase 154) - 2026-03-27 07:37
  - POST /api/v1/shares/nfs — 创建 NFS 共享
  - JWT 认证，仅 admin 角色可访问
  - 请求字段：name/path/comment/read_only/no_subtree_check/sync/clients
  - 客户端配置：network(CIDR)/access(ro/rw)
  - 验证名称格式（1-64 字符，字母数字 -_.）
  - 验证路径格式（以/开头，最大 256 字符）
  - 验证客户端配置（至少 1 个，CIDR 格式）
  - 验证名称唯一性（409 Conflict）
  - 创建成功返回 201 Created + 共享详情
  - 文档：docs/shares_nfs_create_api.md

- [x] SMB 共享创建 API (Phase 153) - 2026-03-27 07:24
  - POST /api/v1/shares/smb — 创建 SMB 共享
  - JWT 认证，仅 admin 角色可访问
  - 请求字段：name/path/comment/read_only/guest_access/browseable/valid_users/invalid_users
  - 验证名称格式（1-64 字符，字母数字 -_.）
  - 验证路径格式（以/开头，最大 256 字符）
  - 验证名称唯一性（409 Conflict）
  - 创建成功返回 201 Created + 共享详情
  - 文档：docs/shares_smb_create_api.md

- [x] NFS 共享列表 API (Phase 152) - 2026-03-27 06:58
  - GET /api/v1/shares/nfs — 获取 NFS 共享列表
  - JWT 认证，仅 admin 角色可访问
  - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
  - 支持状态过滤：status(active/inactive)
  - 返回字段：id/name/path/comment/read_only/no_subtree_check/sync/clients/enabled/created_at
  - 客户端配置：network(CIDR)/access(ro/rw)
  - 响应格式：{ data: [...], pagination: { page, per_page, total, total_pages } }
  - 文档：docs/shares_nfs_list_api.md

- [x] SMB 共享列表 API (Phase 151) - 2026-03-27 06:45
  - GET /api/v1/shares/smb — 获取 SMB 共享列表
  - JWT 认证，仅 admin 角色可访问
  - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
  - 支持状态过滤：status(active/inactive)
  - 返回字段：id/name/path/status/read_only/guest_access/enabled/created_at/updated_at
  - 响应格式：{ data: [...], pagination: { page, per_page, total, total_pages } }
  - 文档：docs/shares_smb_list_api.md

- [x] 容器统计信息 API (Phase 150) - 2026-03-27 06:32
  - GET /api/v1/containers/{id}/stats — 获取容器统计信息
  - JWT 认证，仅 admin 角色可访问
  - 验证容器 ID 存在性（404 Not Found）
  - 返回字段：container_id/name/status/cpu/memory/network/blkio/timestamp
  - 文档：docs/containers_stats_api.md

- [x] 容器日志 API (Phase 149) - 2026-03-27 06:19
  - GET /api/v1/containers/{id}/logs — 查看容器日志
  - JWT 认证，仅 admin 角色可访问
  - 验证容器 ID 存在性（404 Not Found）
  - 支持查询参数：tail(默认 100, 最大 1000), since(可选), follow(可选)
  - 返回字段：container_id/logs(字符串数组)/lines_count
  - 文档：docs/containers_logs_api.md

- [x] 容器删除 API Enhanced (Phase 148) - 2026-03-27 06:06
  - DELETE /api/v1/containers/{id} — 删除容器
  - JWT 认证，仅 admin 角色可访问
  - 支持强制删除参数 force=true
  - 验证容器 ID 存在性（404 Not Found）
  - 运行中的容器需要 force=true 才能删除（409 Conflict）
  - 删除成功返回 204 No Content
  - 文档：docs/containers_delete_api.md

- [x] 容器重启 API (Phase 147) - 2026-03-27 05:53
  - POST /api/v1/containers/{id}/restart — 重启容器
  - JWT 认证，仅 admin 角色可访问
  - 验证容器 ID 存在性（404 Not Found）
  - 重启成功返回 200 OK + 容器详情
  - 文档：docs/containers_restart_api.md

- [x] 容器停止 API (Phase 146) - 2026-03-27 05:40
  - POST /api/v1/containers/{id}/stop — 停止容器
  - JWT 认证，仅 admin 角色可访问
  - 验证容器 ID 存在性（404 Not Found）
  - 验证容器当前状态（已停止则返回 409 Conflict）
  - 停止成功返回 200 OK + 容器详情
  - 文档：docs/containers_stop_api.md

- [x] 容器启动 API (Phase 145) - 2026-03-27 05:27
  - POST /api/v1/containers/{id}/start — 启动容器
  - JWT 认证，仅 admin 角色可访问
  - 验证容器 ID 存在性（404 Not Found）
  - 验证容器当前状态（已运行则返回 409 Conflict）
  - 启动成功返回 200 OK + 容器详情
  - 文档：docs/containers_start_api.md

- [x] 容器删除 API (Phase 144) - 2026-03-27 05:14
  - DELETE /api/v1/containers/{id} — 删除容器
  - JWT 认证，仅 admin 角色可访问
  - 验证容器 ID 存在性（404 Not Found）
  - 删除成功返回 204 No Content
  - 文档：docs/containers_delete_api.md

- [x] 容器更新 API (Phase 143) - 2026-03-27 04:48
  - PUT /api/v1/containers/{id} — 更新容器配置
  - JWT 认证，仅 admin 角色可访问
  - 支持部分更新字段：name/image/network_mode/restart_policy/ports/labels/mounts
  - 验证容器 ID 存在性（404 Not Found）
  - 验证容器名称唯一性（409 Conflict，排除自身）
  - 验证 network_mode 合法性（bridge/host/none/container）
  - 验证 restart_policy 合法性（no/on-failure/always/unless-stopped）
  - 更新成功返回 200 OK + 容器详情
  - 文档：docs/containers_update_api.md

- [x] 容器创建 API (Phase 141) - 2026-03-27 03:30
  - POST /api/v1/containers — 创建新容器
  - JWT 认证，仅 admin 角色可访问
  - 请求字段：name/image/network_mode/restart_policy/ports(可选)/labels(可选)/mounts(可选)
  - 验证名称唯一性（409 Conflict）
  - 创建成功返回 201 Created + 容器详情
  - 文档：docs/containers_create_api.md

- [x] 容器详情 API (Phase 140) - 2026-03-27 03:15
  - GET /api/v1/containers/{id} — 获取单个容器详情
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：container_id/name/image/status/ports/created_at/network_mode/restart_policy/command/labels/mounts/state
  - 容器不存在返回 404 Not Found
  - 文档：docs/containers_detail_api.md

- [x] 容器列表 API (Phase 139) - 2026-03-27 02:50
  - GET /api/v1/containers — 获取容器列表
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：container_id/name/image/status/ports/created_at/network_mode/restart_policy
  - 支持分页参数：page(默认 1), per_page(默认 20, 最大 100)
  - 文档：docs/containers_list_api.md

- [x] 防火墙规则删除 API (Phase 138) - 2026-03-27 02:35
  - DELETE /api/v1/firewall/rules/{rule_id} — 删除防火墙规则
  - JWT 认证，仅 admin 角色可访问
  - 验证规则 ID 存在性（404 Not Found）
  - 删除成功返回 204 No Content
  - 文档：docs/firewall_rule_delete_api.md

- [x] 防火墙规则更新 API (Phase 137) - 2026-03-27 02:05
  - PUT /api/v1/firewall/rules/{rule_id} — 更新防火墙规则
  - JWT 认证，仅 admin 角色可访问
  - 支持部分更新字段：name/priority/action/protocol/source_ip/source_port/dest_ip/dest_port/interface/enabled
  - 验证规则 ID 存在性（404）
  - 验证规则名称唯一性（409，排除自身）
  - 验证 priority 合法性（400）
  - 验证 IP/CIDR 格式合法性（400）
  - 验证 port 范围合法性（400）
  - 更新成功返回 200 OK
  - 文档：docs/firewall_rule_update_api.md

- [x] 防火墙规则详情 API (Phase 136) - 2026-03-27 01:50
  - GET /api/v1/firewall/rules/{rule_id} — 防火墙规则详情
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：rule_id/name/priority/action/protocol/source_ip/source_port/dest_ip/dest_port/interface/enabled/created_at/updated_at
  - 验证规则 ID 存在性（404）
  - 文档：docs/firewall_rule_detail_api.md

- [x] 防火墙规则创建 API (Phase 135) - 2026-03-27 01:00
  - POST /api/v1/firewall/rules — 创建防火墙规则
  - JWT 认证，仅 admin 角色可访问
  - 请求体：name/priority/action/protocol/source_ip/source_port/dest_ip/dest_port/interface/enabled
  - 验证规则名称唯一性（409 Conflict）
  - 验证 priority 合法性（400）
  - 验证 IP/CIDR 格式合法性（400）
  - 验证 port 范围合法性（400）
  - 创建成功返回 201 Created
  - 文档：docs/firewall_rules_create_api.md

- [x] 网络接口删除 API (Phase 134) - 2026-03-27 00:30
  - DELETE /api/v1/network/interfaces/{id} — 网络接口删除
  - JWT 认证，仅 admin 角色可访问
  - 验证接口存在性（404）
  - 验证系统接口不可删除（400）
  - 删除成功返回 200 OK
  - 文档：docs/network_interface_delete_api.md

- [x] 网络接口更新 API (Phase 133) - 2026-03-27 00:20
  - PUT /api/v1/network/interfaces/{id} — 网络接口更新
  - JWT 认证，仅 admin 角色可访问
  - 支持字段：name/ip_address/netmask/gateway/dhcp_enabled/enabled/speed_mbps/mtu
  - 验证接口存在性（404）
  - 验证 IP 地址格式（400）
  - 支持部分更新
  - 文档：docs/network_interface_update_api.md

- [x] 网络接口创建 API (Phase 132) - 2026-03-27 00:10
  - POST /api/v1/network/interfaces — 创建网络接口
  - JWT 认证，仅 admin 角色可访问
  - 请求体：name/type/mac_address(可选)/ip_address(可选)/netmask(可选)/gateway(可选)/dhcp_enabled(默认 false)
  - 验证接口名称唯一性（409 Conflict）
  - 验证 type 合法性（ethernet/wifi/bridge/vlan）
  - 验证 IP 地址格式合法性（400）
  - 验证 MAC 地址格式合法性（400）
  - 创建成功返回 201 Created
  - 文档：docs/network_interfaces_create_api.md

- [x] 网络接口详情 API (Phase 131) - 2026-03-26 23:50
  - GET /api/v1/network/interfaces/{interface_id} — 网络接口详情
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：interface_id/name/type/mac_address/ip_address/netmask/gateway/broadcast/status/speed_mbps/mtu/tx_bytes/rx_bytes/tx_packets/rx_packets
  - 验证接口存在性（404）
  - 文档：docs/network_interface_detail_api.md

- [x] 防火墙规则列表 API (Phase 130) - 2026-03-26 23:40
  - GET /api/v1/firewall/rules — 防火墙规则列表
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：rule_id/name/priority/action/protocol/source_ip/source_port/dest_ip/dest_port/interface/enabled/created_at/updated_at
  - 支持筛选：action/protocol/enabled/interface
  - 支持分页：page/per_page
  - 按 priority 升序排序
  - 文档：docs/firewall_rules_list_api.md

- [x] 网络接口列表 API (Phase 129) - 2026-03-26 23:30
  - GET /api/v1/network/interfaces — 网络接口列表
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：interface_id/name/type/mac_address/ip_address/netmask/gateway/status/speed_mbps
  - 支持筛选参数：type/status
  - 文档：docs/network_interfaces_list_api.md

- [x] 更新打印机 API (Phase 128) - 2026-03-26 23:15
  - PUT /api/v1/printers/{printer_id} — 更新打印机
  - JWT 认证，仅 admin 角色可访问
  - 支持字段：name/model/status/ip_address/location/is_default
  - 验证打印机存在性（404）
  - 验证 IP 地址格式（400）
  - 文档：docs/printers_update_api.md

- [x] 打印机详情 API (Phase 127) - 2026-03-26 23:00
  - GET /api/v1/printers/{printer_id} — 打印机详情
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：printer_id/name/model/status/ip_address/location/is_default/capabilities
  - 验证打印机存在性（404）
  - 文档：docs/printers_detail_api.md

- [x] 打印机列表 API (Phase 126) - 2026-03-26 22:50
  - GET /api/v1/printers — 打印机列表
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：printer_id/name/model/status/ip_address/location/is_default
  - 支持分页查询 (page, per_page 参数)
  - 文档：docs/printers_list_api.md

- [x] DNS 配置 API (Phase 125) - 2026-03-26 22:35
  - GET /api/v1/network/dns — 获取 DNS 配置
  - PUT /api/v1/network/dns — 更新 DNS 配置
  - JWT 认证，仅 admin 角色可访问
  - 返回/更新字段：dns_primary/dns_secondary/dns_mode
  - 验证 DNS 服务器 IP 格式合法性
  - manual 模式时 dns_primary 必填
  - auto 模式时从 DHCP 获取
  - 文档：docs/dns_config_get_api.md

- [x] 网络接口列表 API (Phase 124) - 2026-03-26 22:25
  - GET /api/v1/network/interfaces — 网络接口列表
  - JWT 认证，仅 admin 角色可访问
  - 返回接口名称/MAC 地址/IP 地址/子网掩码/广播地址/状态/类型
  - 使用系统命令获取（ip addr 或 ifconfig）
  - 错误处理：403 权限不足，500 系统命令执行失败
  - 文档：docs/network_interfaces_list_api.md

- [x] 网络配置更新 API (Phase 123) - 2026-03-26 22:10
  - PUT /api/v1/network/config — 更新网络配置
  - JWT 认证，仅 admin 角色可访问
  - 支持更新字段：hostname/dhcp_enabled/ip_address/subnet_mask/gateway/dns_primary/dns_secondary
  - DHCP 启用时自动忽略静态 IP 配置
  - 验证 IP 地址格式合法性
  - 更新后返回完整网络配置
  - 文档：docs/network_config_update_api.md

- [x] 网络配置 API (Phase 122) - 2026-03-26 22:00
  - GET /api/v1/network/config — 获取网络配置
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：hostname/ip_address/subnet_mask/gateway/dns_primary/dns_secondary/dhcp_enabled/mac_address/connection_status
  - 验证用户权限（403）
  - 文档：docs/network_config_get_api.md

- [x] 文件详情 API (Phase 121) - 2026-03-26 21:45
  - GET /api/v1/files/{id} — 文件详情
  - JWT 认证，登录用户可访问
  - 返回文件元数据（id/name/path/size/mime_type/created_at/updated_at/owner_id）
  - 验证文件存在性（404）
  - 验证访问权限（403）
  - 文档：docs/files_get_api.md

- [x] 文件下载 API (Phase 120) - 2026-03-26 21:30
  - GET /api/v1/files/{id}/download — 文件下载（增强版）
  - JWT 认证，登录用户可访问
  - 支持 HTTP Range 请求（断点续传）
  - 验证文件存在性（404）
  - 验证文件访问权限（403）
  - 返回正确的 Content-Type 和 Content-Disposition
  - 文档：docs/files_download_api.md

- [x] 文件上传 API (Phase 119) - 2026-03-26 21:20
  - POST /api/v1/files/upload — 文件上传
  - JWT 认证，登录用户可访问
  - multipart/form-data: file (必填), path (可选)
  - 验证文件类型和大小
  - 处理文件名冲突
  - 错误处理：400/401/403/413/500
  - 文档：docs/files_upload_api.md

- [x] 取消打印任务 API (Phase 116) - 2026-03-26 20:40
  - DELETE /api/v1/printers/{printer_id}/jobs/{job_id} — 取消打印任务
  - JWT 认证，登录用户可访问
  - 普通用户仅可取消自己的任务，admin 可取消任意任务
  - 验证打印机存在性（404）
  - 验证任务存在性（404）
  - 验证用户权限（403）
  - 验证 CUPS 服务连接（503）
  - 文档：docs/printers_delete_job_api.md

- [x] 更新打印任务 API (Phase 115) - 2026-03-26 20:25
  - PUT /api/v1/printers/{printer_id}/jobs/{job_id} — 更新打印任务
  - JWT 认证，登录用户可访问
  - 可更新字段：priority/state (至少一个)
  - 验证打印机存在性（404）
  - 验证任务存在性（404）
  - 验证参数合法性（400）
  - 文档：docs/printers_update_job_api.md

- [x] 创建打印任务 API (Phase 114) - 2026-03-26 20:15
  - POST /api/v1/printers/{printer_id}/jobs — 创建打印任务
  - JWT 认证，登录用户可访问
  - 请求体：document_name/pages/copies/priority/submitted_at
  - 验证打印机存在性（404）
  - 验证 CUPS 服务连接（503）
  - 验证参数合法性（400）
  - 文档：docs/printers_create_job_api.md

- [x] 文件列表 API (Phase 117) - 2026-03-26 20:50
  - GET /api/v1/files/list — 文件列表
  - JWT 认证，登录用户可访问
  - 支持路径参数：path（可选，默认根目录）
  - 返回文件/文件夹列表（名称/类型/大小/修改时间）
  - 验证路径合法性（400）
  - 验证路径存在性（404）
  - 文档：docs/files_list_api.md

- [x] 打印机列表 API (Phase 111) - 2026-03-26 19:00
  - GET /api/v1/printers — 打印机列表
  - JWT 认证，登录用户可访问
  - 验证 CUPS 服务连接状态（503 Service Unavailable）
  - 支持状态过滤：state（idle/printing/error）
  - 支持分页：page/limit（默认 1/20，最大 100）
  - 返回字段：id/name/uri/state/model/is_default
  - 文档：docs/printers_list_api.md

- [x] 文件复制 API (Phase 110) - 2026-03-26 18:45
  - POST /api/v1/files/{id}/copy — 文件复制
  - JWT 认证，登录用户可访问
  - 请求体：destination_path（目标目录路径）
  - 验证文件 ID 存在性（404 Not Found）
  - 验证文件归属权（403 Forbidden）
  - 支持 admin 操作任意文件
  - 验证目标路径有效性（400 Bad Request）
  - 检查目标文件冲突（409 Conflict）
  - 文档：docs/files_copy_api.md

- [x] 文件下载 API (Phase 109) - 2026-03-26 18:30
  - GET /api/v1/files/{id}/download — 文件下载
  - JWT 认证，登录用户可访问
  - 验证文件 ID 存在性（404 Not Found）
  - 验证文件访问权限（403 Forbidden）
  - 支持 admin 下载任意文件
  - 返回文件二进制流
  - 文档：docs/files_download_api.md

- [x] 文件更新 API (Phase 108) - 2026-03-26 18:15
  - PUT /api/v1/files/{id} — 文件更新/重命名
  - JWT 认证，登录用户可访问
  - 支持字段：name（重命名）/path（移动）
  - 验证文件 ID 存在性（404 Not Found）
  - 验证文件归属权（403 Forbidden）
  - 支持 admin 操作任意文件
  - 检查同名文件冲突（409 Conflict）
  - 文档：docs/files_update_api.md

- [x] 文件详情 API (Phase 107) - 2026-03-26 18:00
  - GET /api/v1/files/{id} — 文件详情
  - JWT 认证，登录用户可访问
  - 验证文件 ID 存在性（404 Not Found）
  - 验证文件访问权限（403 Forbidden）
  - 支持 admin 查看任意文件
  - 返回字段：id/name/path/size_bytes/mime_type/volume_id/owner_id/created_at/modified_at
  - 文档：docs/files_detail_api.md

- [x] 文件删除 API (Phase 106) - 2026-03-26 17:50
  - DELETE /api/v1/files/{id} — 文件删除
  - JWT 认证，登录用户可访问
  - 验证文件 ID 存在性（404 Not Found）
  - 验证文件归属权（403 Forbidden）
  - 支持 admin 删除任意文件
  - 文档：docs/files_delete_api.md

- [x] 文件上传 API (Phase 105) - 2026-03-26 17:35
  - POST /api/v1/files/upload — 文件上传
  - JWT 认证，登录用户可访问
  - 支持 multipart/form-data
  - 支持存储卷选择和路径指定
  - 文件大小限制（最大 100MB）
  - 重复文件检测
  - 自动识别 MIME 类型
  - 文档：docs/files_upload_api.md

- [x] 更新打印任务 API (Phase 115) - 2026-03-26 20:25
  - PUT /api/v1/printers/{printer_id}/jobs/{job_id} — 更新打印任务
  - JWT 认证，登录用户可访问
  - 可更新字段：priority（优先级）/state（状态）
  - 验证打印机/任务存在性 (404)
  - 参数验证：至少提供一个字段、优先级/状态合法性
  - 文档：docs/printers_jobs_update_api.md

- [x] 创建打印任务 API (Phase 114) - 2026-03-26 20:15
  - POST /api/v1/printers/{printer_id}/jobs — 创建打印任务
  - JWT 认证，登录用户可访问
  - 请求体：document_name/pages/copies/priority(可选)/submitted_at(可选)
  - 验证打印机存在性、CUPS 服务连接、参数合法性
  - 返回创建的任务详情（含任务 ID、状态等）
  - 文档：docs/printers_jobs_create_api.md

- [x] 打印机任务详情 API (Phase 113) - 2026-03-26 19:58
  - GET /api/v1/printers/{printer_id}/jobs/{job_id} — 打印机任务详情
  - JWT 认证，登录用户可访问
  - 返回字段：id/printer_id/document_name/user_id/pages/copies/state/priority/submitted_at/started_at/completed_at/error_message
  - 验证打印机/任务存在性 (404)
  - 验证 CUPS 服务连接状态 (503)
  - 文档：docs/printers_jobs_detail_api.md

- [x] 打印机任务列表 API (Phase 112) - 2026-03-26 19:45
  - GET /api/v1/printers/{id}/jobs — 打印机任务队列
  - JWT 认证，登录用户可访问
  - 支持状态筛选（pending/printing/completed/failed/canceled）
  - 分页支持：page/page_size
  - 验证 CUPS 服务连接状态 (503)
  - 文档：docs/printers_jobs_list_api.md

- [x] 打印机列表 API (Phase 111) - 2026-03-26 18:55
  - GET /api/v1/printers — 打印机列表
  - JWT 认证，登录用户可访问
  - 支持状态/类型/位置筛选
  - 分页支持：page/page_size
  - 验证 CUPS 服务连接状态 (503)
  - 文档：docs/printers_list_api.md

- [x] 文件浏览 API (Phase 104) - 2026-03-26 17:10
  - GET /api/v1/files/browse — 文件浏览
  - JWT 认证，登录用户可访问
  - 查询参数：path/page/limit
  - 返回字段：current_path/parent_path/folders/files/total_items/pagination
  - 路径不存在返回 404
  - 文档：docs/files_browse_api.md

- [x] 创建用户 API (Phase 102) - 2026-03-26 16:45
  - POST /api/v1/users — 创建用户
  - JWT 认证，仅 admin 角色可访问
  - 验证用户名/邮箱/密码有效性
  - 检查用户名唯一性（409 Conflict）
  - 返回 201 Created + 用户信息
  - 文档：docs/users_create_api.md

- [x] 用户详情 API (Phase 101) - 2026-03-26 16:30
  - GET /api/v1/users/{id} — 用户详情
  - JWT 认证，登录用户可访问
  - 返回字段：id/username/email/role/created_at/updated_at
  - 验证用户 ID 存在（404 Not Found）
  - 文档：docs/users_get_by_id_api.md

- [x] 用户登出 API (Phase 100) - 2026-03-26 16:15
  - POST /api/v1/auth/logout — 用户登出
  - JWT 认证，登录用户可访问
  - 返回登出成功信息
  - 注意：JWT 无状态，客户端需删除 token
  - 文档：docs/auth_logout_api.md

- [x] 用户列表 API (Phase 99) - 2026-03-26 16:05
  - GET /api/v1/users — 用户列表
  - JWT 认证，仅 admin 角色可访问
  - 支持分页：page/per_page（默认 1/20，最大 100）
  - 返回字段：id/username/email/role/created_at/updated_at
  - 文档：docs/users_list_api.md

- [x] 删除共享文件夹权限 API (Phase 98) - 2026-03-26 15:50
  - DELETE /api/v1/shared-folders/{id}/permissions/{permission_id} — 删除共享文件夹权限
  - JWT 认证，仅 admin 角色可访问
  - 验证共享文件夹 ID 存在（404 Not Found）
  - 验证权限 ID 存在（404 Not Found）
  - 返回 200 OK + 删除结果
  - 文档：docs/shared_folder_permissions_delete_api.md

- [x] 更新共享文件夹权限 API (Phase 97) - 2026-03-26 15:40
  - PUT /api/v1/shared-folders/{id}/permissions/{permission_id} — 更新共享文件夹权限
  - JWT 认证，仅 admin 角色可访问
  - 请求体：permissions
  - 验证共享文件夹 ID 存在（404 Not Found）
  - 验证权限 ID 存在（404 Not Found）
  - 验证权限类型有效性（400 Bad Request）
  - 返回 200 OK + 更新后的权限信息
  - 文档：docs/shared_folder_permissions_update_api.md

- [x] 添加共享文件夹权限 API (Phase 96) - 2026-03-26 15:25
  - POST /api/v1/shared-folders/{id}/permissions — 添加共享文件夹权限
  - JWT 认证，仅 admin 角色可访问
  - 请求体：target_type/target_id/permissions
  - 验证共享文件夹 ID 存在（404 Not Found）
  - 验证目标类型有效性（400 Bad Request）
  - 验证权限类型有效性（400 Bad Request）
  - 检查权限是否已存在（409 Conflict）
  - 返回 201 Created + 权限信息
  - 文档：docs/shared_folder_permissions_add_api.md

- [x] 共享文件夹权限列表 API (Phase 95) - 2026-03-26 15:10
  - GET /api/v1/shared-folders/{id}/permissions — 共享文件夹权限列表
  - JWT 认证，仅 admin 角色可访问
  - 返回字段：id/shared_folder_id/target_type/target_id/target_name/permissions/created_at/updated_at
  - 支持分页：page/per_page（默认 1/20，最大 100）
  - 验证共享文件夹 ID 存在（404 Not Found）
  - 文档：docs/shared_folder_permissions_list_api.md

- [x] 删除共享文件夹 API (Phase 94) - 2026-03-26 15:00
  - DELETE /api/v1/shared-folders/{id} — 删除共享文件夹
  - JWT 认证，仅 admin 角色可访问
  - 验证共享文件夹 ID 存在（404 Not Found）
  - 返回删除结果
  - 文档：docs/shared_folder_delete_api.md

- [x] 更新共享文件夹 API (Phase 93) - 2026-03-26 14:45
  - PUT /api/v1/shared-folders/{id} — 更新共享文件夹
  - JWT 认证，仅 admin 角色可访问
  - 可更新字段：name/description/protocols/read_only/guest_access/enabled
  - 验证共享文件夹 ID 存在（404 Not Found）
  - 检查名称唯一性（409 Conflict）
  - 验证协议类型有效性（400 Bad Request）
  - 返回更新后的共享文件夹信息
  - 文档：docs/shared_folder_update_api.md

- [x] 共享文件夹详情 API (Phase 92) - 2026-03-26 14:30
  - GET /api/v1/shared-folders/{id} — 共享文件夹详情
  - JWT 认证，登录用户可访问
  - 返回字段：id/name/path/volume_id/volume_name/description/protocols/is_public/read_only/guest_access/status/created_at/updated_at/created_by
  - 验证共享文件夹 ID 存在（404 Not Found）
  - 文档：docs/shared_folder_detail_api.md

- [x] 共享文件夹列表 API (Phase 90) - 2026-03-26 14:05
  - GET /api/v1/shared-folders — 共享文件夹列表
  - JWT 认证，登录用户可访问
  - 支持分页：page/per_page（默认 1/20，最大 100）
  - 支持筛选：protocol/volume_id/status
  - 返回共享文件夹列表 + 分页元数据
  - 无数据返回空数组
  - 文档：docs/shared_folder_list_api.md

- [x] 创建共享文件夹 API (Phase 89) - 2026-03-26 13:30
  - POST /api/v1/shared-folders — 创建共享文件夹
  - JWT 认证，仅 admin 角色可访问
  - 请求体：name/path/volume_id/description/protocols/is_public
  - 验证存储卷 ID 存在（404 Not Found）
  - 检查共享文件夹名称唯一性（409 Conflict）
  - 验证协议类型有效性（400 Bad Request）
  - 返回 201 Created + 共享文件夹信息
  - 文档：docs/shared_folder_create_api.md

- [x] 克隆存储卷快照 API (Phase 88) - 2026-03-26 13:15
  - POST /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/clone — 从快照克隆新存储卷
  - JWT 认证，仅 admin 角色可访问
  - 请求体：new_volume_name/new_volume_description/target_pool_id
  - 验证存储卷 ID 存在（404 Not Found）
  - 验证快照 ID 存在（404 Not Found）
  - 验证快照归属（400 Bad Request）
  - 验证快照状态为 completed（400 Bad Request）
  - 检查新卷名称唯一性（409 Conflict）
  - 验证存储池容量充足（400 Bad Request）
  - 返回 201 Created + 新卷信息
  - 文档：docs/storage_volume_snapshot_clone_api.md

- [x] 恢复存储卷快照 API (Phase 87) - 2026-03-26 13:00
  - POST /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/restore — 恢复存储卷快照
  - JWT 认证，仅 admin 角色可访问
  - 验证存储卷 ID 存在（404 Not Found）
  - 验证快照 ID 存在（404 Not Found）
  - 验证快照属于该存储卷（400 Bad Request）
  - 返回恢复结果
  - 文档：docs/storage_volume_snapshot_restore_api.md

- [x] 更新存储卷快照 API (Phase 86) - 2026-03-26 12:50
  - PUT /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id} — 更新存储卷快照
  - JWT 认证，仅 admin 角色可访问
  - 可更新字段：name/description/is_protected（至少一个）
  - 验证存储卷 ID 存在（404 Not Found）
  - 验证快照 ID 存在（404 Not Found）
  - 检查名称唯一性（409 Conflict）
  - 返回更新后的快照信息
  - 文档：docs/storage_volume_snapshot_update_api.md

- [x] 删除存储卷快照 API (Phase 85) - 2026-03-26 12:10
  - DELETE /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id} — 删除存储卷快照
  - JWT 认证，仅 admin 角色可访问
  - 验证存储卷 ID 存在（404 Not Found）
  - 验证快照 ID 存在（404 Not Found）
  - 检查快照保护状态（400 Bad Request）
  - 返回删除结果
  - 文档：docs/storage_volume_snapshot_delete_api.md

- [x] 存储卷快照详情 API (Phase 84) - 2026-03-26 11:55
  - GET /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id} — 存储卷快照详情
  - JWT 认证，登录用户可访问
  - 返回字段：id/name/description/volume_id/volume_name/size_bytes/created_at/created_by/is_protected/status
  - 验证存储卷 ID 存在（404 Not Found）
  - 验证快照 ID 存在（404 Not Found）
  - 文档：docs/storage_volume_snapshot_detail_api.md

- [x] 存储卷快照列表 API (Phase 83) - 2026-03-26 11:45
  - GET /api/v1/storage/volumes/{volume_id}/snapshots — 存储卷快照列表（增强版）
  - JWT 认证，登录用户可访问
  - 返回字段：id/name/description/volume_id/volume_name/size_bytes/created_at/created_by/is_protected/status
  - 支持分页：limit/offset（默认 20/0，最大 100）
  - 支持筛选：status/is_protected
  - 验证存储卷 ID 存在（404 Not Found）
  - 无数据返回空数组
  - 文档：docs/storage_volume_snapshots_list_api.md

- [x] 创建存储卷快照 API (Phase 82) - 2026-03-26 11:30
  - POST /api/v1/storage/volumes/{id}/snapshots — 创建存储卷快照
  - JWT 认证，仅 admin 角色可访问
  - 验证必要参数：name/description
  - 验证存储卷 ID 存在（404 Not Found）
  - 检查快照名称唯一性（409 Conflict）
  - 返回创建的快照信息
  - 文档：docs/storage_volume_snapshot_create_api.md

- [x] 存储卷快照列表 API (Phase 81) - 2026-03-26 11:15
  - GET /api/v1/storage/volumes/{id}/snapshots — 存储卷快照列表
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/description/volume_id/size_bytes/created_at/updated_at/status
  - 支持分页：page/per_page
  - 验证存储卷 ID 存在（404 Not Found）
  - 无数据返回空数组
  - 文档：docs/storage_volume_snapshots_api.md

- [x] 创建存储卷 API (Phase 80) - 2026-03-26 11:05
  - POST /api/v1/storage/volumes — 创建存储卷
  - JWT 认证，仅 admin 角色可访问
  - 验证必要参数：name/description/pool_id/size_bytes/filesystem
  - 验证存储池存在性和容量
  - 检查名称唯一性（409 Conflict）
  - 验证文件系统类型（ext4/btrfs/xfs/zfs）
  - 返回创建的存储卷信息
  - 文档：docs/storage_volume_create_api.md

- [x] 存储卷详情 API (Phase 79) - 2026-03-26 10:50
  - GET /api/v1/storage/volumes/{id} — 存储卷详情
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/description/pool_id/total_bytes/used_bytes/available_bytes/usage_percent/status/filesystem/mount_point/created_at/updated_at
  - 验证存储卷 ID 存在（404 Not Found）
  - 文档：docs/storage_volume_detail_api.md

- [x] 存储卷列表 API (Phase 78) - 2026-03-26 10:41
  - GET /api/v1/storage/usage — 存储使用量统计
  - JWT 认证，任意登录用户可访问
  - 返回总体统计：total_capacity/total_used/total_available/overall_usage_percent/health_status/disk_count/pool_count/volume_count
  - 返回磁盘使用量列表（disks 数组）
  - 返回存储池使用量列表（pools 数组）
  - 返回存储卷使用量列表（volumes 数组）
  - 健康状态评估（healthy/degraded/critical）
  - 文档：docs/storage_usage_api.md

- [x] 存储池详情 API (Phase 76) - 2026-03-26 10:15
  - GET /api/v1/storage/pools/{id} — 存储池详情
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/description/total_bytes/used_bytes/available_bytes/usage_percent/disk_count/disks/volume_count/volumes/status/created_at/updated_at
  - disks 数组包含磁盘信息（disk_id/name/device_path/capacity_bytes/status）
  - volumes 数组包含存储卷信息（volume_id/name/size_bytes/used_bytes/status）
  - 验证存储池 ID 存在（404 Not Found）
  - 文档：docs/storage_pool_detail_api.md

- [x] 存储池列表 API (Phase 75) - 2026-03-26 10:00
  - GET /api/v1/storage/pools — 存储池列表
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/description/total_bytes/used_bytes/available_bytes/usage_percent/disk_count/disks/status/created_at/updated_at
  - 支持分页：page/per_page（默认 1/20，最大 100）
  - 无数据返回空数组
  - 文档：docs/storage_pools_api.md

- [x] 磁盘详情 API (Phase 74) - 2026-03-26 09:45
  - GET /api/v1/storage/disks/{id} — 磁盘详情
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/device_path/model/serial_number/size_bytes/used_bytes/available_bytes/usage_percent/disk_type/interface/status/temperature/smart_status/is_system/in_storage_pool/pool_name/created_at/updated_at
  - 验证磁盘 ID 存在（404 Not Found）
  - 文档：docs/storage_disk_detail_api.md

- [x] 磁盘列表 API (Phase 73) - 2026-03-26 09:35
  - GET /api/v1/storage/disks — 磁盘列表
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/device_path/model/serial_number/size_bytes/used_bytes/available_bytes/usage_percent/disk_type/interface/status/temperature/smart_status/is_system/created_at/updated_at
  - 支持分页：page/limit
  - 无数据返回空数组
  - 文档：docs/storage_disks_list_api.md

- [x] 存储池下的卷列表 API (Phase 71) - 2026-03-26 09:10
  - GET /api/v1/storage/pools/{id}/volumes — 存储池下的卷列表
  - JWT 认证，任意登录用户可访问
  - 路径参数：id (存储池 ID)
  - 返回字段：id/name/pool_id/pool_name/size_bytes/used_bytes/available_bytes/usage_percent/filesystem_type/status/mount_point/created_at/updated_at
  - 支持分页：page/limit
  - 验证存储池存在性（404 Not Found）
  - 无数据返回空数组
  - 文档：docs/storage_pools_volumes_list_api.md

- [x] 存储卷列表 API (Phase 70) - 2026-03-26 09:00
  - GET /api/v1/storage/volumes — 存储卷列表
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/pool_id/pool_name/size_bytes/used_bytes/available_bytes/usage_percent/filesystem_type/status/mount_point/created_at/updated_at
  - 支持分页：page/limit
  - 无数据返回空数组
  - 文档：docs/storage_volumes_list_api.md

- [x] 删除存储卷 API (Phase 69) - 2026-03-26 08:45
  - DELETE /api/v1/storage/volumes/{id} — 删除存储卷
  - JWT 认证，仅 admin 角色可访问
  - 验证存储卷 ID 存在（404 Not Found）
  - 检查是否有数据/服务在使用该卷（400 Bad Request）
  - 删除成功返回 200 OK + 删除信息
  - 文档：docs/storage_volume_delete_api.md

- [x] 更新存储卷 API (Phase 68) - 2026-03-26 08:30
  - PUT /api/v1/storage/volumes/{id} — 更新存储卷
  - JWT 认证，仅 admin 角色可访问
  - 可更新字段：name/size_bytes/filesystem_type（部分更新）
  - 验证存储卷 ID 存在（404 Not Found）
  - 检查名称唯一性（409 Conflict，排除自身）
  - 验证新容量不小于已用容量（400 Bad Request）
  - 验证文件系统类型有效性（ext4/btrfs/zfs）
  - 返回更新后的存储卷信息
  - 文档：docs/storage_volume_update_api.md

- [x] 创建存储卷 API (Phase 67) - 2026-03-26 08:15
  - POST /api/v1/storage/volumes — 创建存储卷
  - JWT 认证，仅 admin 角色可访问
  - 验证必要参数：name/pool_id/size_bytes
  - 验证存储池存在性和容量
  - 检查名称唯一性（409 Conflict）
  - 返回创建的存储卷信息
  - 文档：docs/storage_volume_create_api.md

- [x] 删除存储池 API (Phase 66) - 2026-03-26 08:05
  - DELETE /api/v1/storage/pools/{id} — 删除存储池
  - JWT 认证，仅 admin 角色可访问
  - 验证存储池 ID 存在（404 Not Found）
  - 检查是否有卷在使用该池（400 Bad Request）
  - 删除成功返回 200 OK + 删除信息
  - 文档：docs/storage_pool_delete_api.md

- [x] 更新存储池 API (Phase 65) - 2026-03-26 07:50
  - PUT /api/v1/storage/pools/{id} — 更新存储池
  - JWT 认证，仅 admin 角色可访问
  - 可更新字段：name, type, status
  - 验证：存储池存在性（404）、名称唯一性（409）、type 变更检查（400）
  - 更新成功返回 200 OK
  - 文档：docs/storage_pool_update_api.md

- [x] 存储池创建 API (Phase 64) - 2026-03-26 07:35
  - POST /api/v1/storage/pools — 创建存储池
  - JWT 认证，任意登录用户可访问
  - 请求体：name, type, disk_ids
  - 验证：名称唯一性（409 Conflict）、磁盘存在性（404）、RAID 类型磁盘数（400）
  - 创建成功返回 201 Created
  - 文档：docs/storage_pool_create_api.md

- [x] 存储池详情 API (Phase 63) - 2026-03-26 07:25
  - GET /api/v1/storage/pools/{id} — 存储池详情
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/type/status/total_bytes/used_bytes/available_bytes/usage_percent/disk_count/disks/created_at/updated_at
  - disks 数组包含磁盘 ID/名称/容量/状态
  - 池不存在返回 404 Not Found
  - 文档：docs/storage_pool_detail_api.md

- [x] 存储池列表 API (Phase 62) - 2026-03-26 07:10
  - GET /api/v1/storage/pools — 存储池列表
  - JWT 认证，任意登录用户可访问
  - 返回字段：id/name/type/total_bytes/used_bytes/available_bytes/usage_percent/disk_count/status/created_at
  - 支持分页：page/page_size
  - 支持筛选：type/status
  - 文档：docs/storage_pools_api.md

- [x] 存储卷详情 API (Phase 61) - 2026-03-26 07:00
  - GET /api/v1/storage/volumes/{id} — 存储卷详情
  - JWT 认证，任意登录用户可访问
  - 返回卷详情：id/name/mount_point/device_path/filesystem/total_bytes/used_bytes/available_bytes/usage_percent/read_only/is_system/volume_type/disks
  - 卷不存在返回 404 Not Found
  - 文档：docs/storage_volume_detail_api.md

- [x] 存储卷列表 API (Phase 60) - 2026-03-26 06:45
  - GET /api/v1/storage/volumes — 存储卷列表
  - JWT 认证，任意登录用户可访问
  - 返回字段：name/mount_point/total_bytes/used_bytes/available_bytes/usage_percent/filesystem_type
  - 返回整体存储状态
  - 支持分页和文件系统筛选
  - 文档：docs/storage_volumes_api.md

- [x] 系统健康检查 API (Phase 59) - 2026-03-26 06:30
  - GET /api/v1/system/health — 系统健康检查
  - JWT 认证，任意登录用户可访问
  - 返回 CPU/内存/磁盘使用率
  - 返回系统运行时间
  - 返回服务状态（database/cache/storage/network）
  - 健康状态评估（healthy/degraded/critical）
  - 文档：docs/system_health_api.md

- [x] 删除打印机 API (Phase 58) - 2026-03-26 06:20
  - DELETE /api/v1/printers/{id} — 删除打印机
  - JWT 认证，仅 admin 角色可访问
  - 验证打印机 ID 存在
  - 删除成功返回打印机信息
  - 打印机不存在返回 404 Not Found
  - 文档：docs/printers_delete_api.md

- [x] 更新打印机 API (Phase 57) - 2026-03-26 06:10
  - PUT /api/v1/printers/{id} — 更新打印机配置
  - JWT 认证，仅 admin 角色可访问
  - 可更新字段：name/printer_type/status/location/description/ip_address/port
  - 验证打印机 ID 存在
  - 部分更新（仅传递需要更新的字段）
  - 文档：docs/printers_update_api.md

- [x] 打印机列表 API (Phase 56) - 2026-03-26 05:55
  - GET /api/v1/printers — 打印机列表
  - JWT 认证，仅 admin 角色可访问
  - 支持分页：page/page_size
  - 支持筛选：printer_type/status/location
  - 返回打印机列表和分页元数据
  - 文档：docs/printers_list_api.md

- [x] 创建打印机 API (Phase 55) - 2026-03-26 05:40
  - POST /api/v1/printers — 创建打印机
  - JWT 认证，仅 admin 角色可访问
  - 验证必要参数：name/model/manufacturer/location
  - 验证 printer_type 和 ip_address 格式
  - 返回创建的打印机信息
  - 文档：docs/printers_create_api.md

- [x] 打印机详情 API (Phase 54) - 2026-03-26 05:30
  - GET /api/v1/printers/{id} — 打印机详情
  - JWT 认证，仅 admin 角色可访问
  - 返回打印机详细信息（含能力配置）
  - 打印机不存在返回 404 Not Found
  - 文档：docs/printers_detail_api.md

- [x] 打印机列表 API (Phase 53) - 2026-03-26 05:15
  - GET /api/v1/printers — 打印机列表
  - JWT 认证，仅 admin 角色可访问
  - 支持分页：page/limit
  - 支持筛选：status/printer_type
  - 返回打印机列表和分页元数据
  - 文档：docs/printers_list_api.md

- [x] 用户登录 API (Phase 52) - 2026-03-26 05:00
  - POST /api/v1/auth/login — 用户登录
  - 支持 bcrypt 和 PBKDF2 密码验证
  - 返回 JWT Token 和用户基本信息
  - 失败时不泄露具体错误（安全考虑）
  - 文档：docs/auth_login_api.md

- [x] 删除用户 API (Phase 51) - 2026-03-26 04:50
  - DELETE /api/v1/users/{id} — 删除用户
  - JWT 认证，仅 admin 角色可访问
  - 用户不存在返回 404 Not Found
  - 不能删除自己（返回 400 Bad Request）
  - 删除成功后返回 204 No Content
  - 文档：docs/users_delete_api.md

- [x] 更新用户信息 API (Phase 50) - 2026-03-26 04:40
  - PUT /api/v1/users/{id} — 更新用户信息
  - JWT 认证，仅 admin 角色可访问
  - 可更新字段：email, role, storage_quota, status
  - 密码不可通过此接口修改（使用专用接口）
  - 用户不存在返回 404 Not Found
  - 非 admin 访问返回 403 Forbidden
  - 文档：docs/users_update_api.md

- [x] 获取用户详情 API (Phase 49) - 2026-03-26 04:34
  - GET /api/v1/users/{id} — 用户详情查询
  - JWT 认证，仅 admin 角色可访问
  - 返回用户详细信息（不含密码）
  - 用户不存在返回 404 Not Found
  - 文档：docs/users_get_api.md

- [x] 更新用户信息 API (Phase 50) - 2026-03-26 04:42
  - PUT /api/v1/users/{id} — 更新用户信息
  - Admin JWT 认证
  - 可更新字段：email, role_id, storage_quota, status
  - 密码不可通过此接口修改

- [x] 删除用户 API (Phase 51) - 2026-03-26 04:45
  - DELETE /api/v1/users/{id} — 删除用户
  - Admin JWT 认证
  - 不能删除自己（400 Bad Request）
  - 删除成功返回 204 No Content
  - 文档：docs/users_delete_api.md

- [x] 获取用户列表 API (Phase 48) - 2026-03-26 04:23
  - GET /api/v1/users — 用户列表
  - 支持分页：page (默认 1) / limit (默认 20，最大 100)
  - 支持筛选：role_id (角色 ID) / search (关键词搜索)
  - JWT 认证，仅 admin 角色可访问
  - 返回用户列表（不含密码）+ 分页元数据
  - 文档：docs/users_list_api.md

- [x] 创建用户 API (Phase 47) - 2026-03-26 04:20
  - POST /api/v1/users — 创建新用户
  - 请求体：username, password, email, role_id, storage_quota(可选)
  - bcrypt 密码加密存储
  - 用户名唯一性校验（409 Conflict）
  - 角色 ID 校验（400 Bad Request）
  - JWT 认证，仅 admin 角色可访问
  - 创建成功返回 201 Created
  - 文档：docs/users_create_api.md

- [x] 存储卷管理 API (e55e210) - 2026-03-18 18:26
  - POST /api/v1/storage/volumes - 创建存储卷
  - DELETE /api/v1/storage/volumes/{id} - 删除存储卷 (e22beaa)
  - PUT /api/v1/storage/volumes/{id} - 更新存储卷 (e55e210)
  - GET /api/v1/storage/volumes - 存储卷列表

- [x] 数据库优化 API (c26449c) - 2026-03-18 11:17

- [x] 后台任务管理 API (完整) (e87a264) - 2026-03-18 10:51

- [x] 日志管理 API (cb93cca) - 2026-03-18 10:38

- [x] 缓存管理 API (d3270c6) - 2026-03-18 10:01

---

## 🚫 阻塞项（已解决）

- **兵部 bot 通讯故障**（HTTP 400 错误）：上下文超限（204800 字符），**已清空会话缓存解决** | 2026-03-17 19:50

---

## 📝 更新记录

| 日期 | 更新内容 | 更新人 |
|------|----------|--------|
| 2026-03-26 05:20 | Phase 53 打印机列表 API 完成 | 兵部 |
| 2026-03-26 05:00 | Phase 52 用户登录 API 完成 | 兵部 |
| 2026-03-26 04:45 | Phase 51 删除用户 API 完成 | 兵部 |
| 2026-03-26 04:42 | Phase 50 更新用户 API 完成 | 兵部 |
| 2026-03-26 04:35 | Phase 49 用户详情 API 完成 | 兵部 |
| 2026-03-26 04:23 | Phase 48 用户列表 API 完成 | 兵部 |
| 2026-03-26 04:20 | Phase 47 创建用户 API 完成 | 兵部 |
| 2026-03-26 03:50 | 同步进度至 Phase 46，文件搜索 API 100% 完成 | 兵部 |
| 2026-03-19 21:16 | 同步进度至 commit 1f0e308，下载管理 API 100% 完成 | 兵部 |
| 2026-03-19 13:55 | 同步进度至 commit dfe8db5，下载任务创建 API 完成 | 兵部 |
| 2026-03-18 23:10 | 同步进度至 commit e55e210，存储卷管理 API 100% 完成 | 翰林院 |

---

## 🔗 相关链接

- GitHub: https://github.com/AIBAOS/axis
- 提交记录：https://github.com/AIBAOS/axis/commits/main

---

## 📊 总体进度

| 阶段 | 完成度 |
|------|--------|
| Phase 1 核心框架 | 100% ✅ |
| Phase 2 JWT 认证 | 100% ✅ |
| Phase 2.5 共享链接 | 100% ✅ |
| Phase 3 文件/会话/RBAC | 100% ✅ |
| Phase 3.2 会话管理 | 100% ✅ |
| Phase 3.3 RBAC 集成 | 100% ✅ |
| Phase 15 媒体服务器 | 100% ✅ |
| Phase 21 存储卷管理 | 100% ✅ |
| Phase 23 下载管理 API | 100% ✅ |
| Phase 46 文件搜索 API | 100% ✅ |
| Phase 47 创建用户 API | 100% ✅ |
| Phase 48 用户列表 API | 100% ✅ |
| Phase 49 用户详情 API | 100% ✅ |
| Phase 50 更新用户 API | 100% ✅ |
| Phase 51 删除用户 API | 100% ✅ |
| Phase 52 用户登录 API | 100% ✅ |
| Phase 53 打印机列表 API | 100% ✅ |
| Phase 54 打印机详情 API | 100% ✅ |
| Phase 55 创建打印机 API | 100% ✅ |
| Phase 56 打印机列表 API | 100% ✅ |
| Phase 57 更新打印机 API | 100% ✅ |
| Phase 58 删除打印机 API | 100% ✅ |
| Phase 59 系统健康检查 API | 100% ✅ |
| Phase 60 存储卷列表 API | 100% ✅ |
| Phase 61 存储卷详情 API | 100% ✅ |
| Phase 62 存储池列表 API | 100% ✅ |
| Phase 63 存储池详情 API | 100% ✅ |
| Phase 64 存储池创建 API | 100% ✅ |
| Phase 65 存储池更新 API | 100% ✅ |
| Phase 66 存储池删除 API | 100% ✅ |
| Phase 67 创建存储卷 API | 100% ✅ |
| Phase 68 更新存储卷 API | 100% ✅ |
| Phase 69 删除存储卷 API | 100% ✅ |
| Phase 70 存储卷列表 API | 100% ✅ |
| Phase 71 存储池卷列表 API | 100% ✅ |
| Phase 72 存储卷列表 API | 100% ✅ |
| Phase 73 磁盘列表 API | 100% ✅ |
| Phase 74 磁盘详情 API | 100% ✅ |
| Phase 75 存储池列表 API | 100% ✅ |
| Phase 76 存储池详情 API | 100% ✅ |
| Phase 77 存储使用统计 API | 100% ✅ |
| Phase 78 存储卷列表 API | 100% ✅ |
| Phase 79 存储卷详情 API | 100% ✅ |
| Phase 80 存储卷创建 API | 100% ✅ |
| Phase 81 存储卷快照列表 API | 100% ✅ |
| Phase 82 创建存储卷快照 API | 100% ✅ |
| Phase 83 存储卷快照列表 API | 100% ✅ |
| Phase 84 存储卷快照详情 API | 100% ✅ |
| Phase 85 删除存储卷快照 API | 100% ✅ |
| Phase 86 更新存储卷快照 API | 100% ✅ |
| Phase 88 克隆存储卷快照 API | 100% ✅ |
| Phase 89 创建共享文件夹 API | 100% ✅ |
| Phase 90 共享文件夹列表 API | 100% ✅ |
| Phase 91 创建共享文件夹 API | 100% ✅ |
| Phase 94 删除共享文件夹 API | 100% ✅ |
| Phase 95 共享文件夹权限列表 API | 100% ✅ |
| Phase 96 添加共享文件夹权限 API | 100% ✅ |
| Phase 97 更新共享文件夹权限 API | 100% ✅ |
| Phase 98 删除共享文件夹权限 API | 100% ✅ |
| Phase 99 用户列表 API | 100% ✅ |
| Phase 100 用户登出 API | 100% ✅ |
| Phase 101 用户详情 API | 100% ✅ |
| Phase 102 创建用户 API | 100% ✅ |
| Phase 103 更新用户 API | 100% ✅ |
| Phase 104 文件浏览 API | 100% ✅ |
| Phase 105 文件上传 API | 100% ✅ |
| Phase 106 文件删除 API | 100% ✅ |
| Phase 107 文件详情 API | 100% ✅ |
| Phase 108 文件更新 API | 100% ✅ |
| Phase 109 文件下载 API | 100% ✅ |
| Phase 110 文件复制 API | 100% ✅ |
| Phase 111 打印机列表 API | 100% ✅ |
| Phase 112 打印机任务列表 API | 100% ✅ |
| Phase 113 打印机任务详情 API | 100% ✅ |
| Phase 114 创建打印任务 API | 100% ✅ |
| Phase 115 更新打印任务 API | 100% ✅ |
| Phase 116 取消打印任务 API | 100% ✅ |
| Phase 117 文件列表 API | 100% ✅ |
| Phase 118 打印机统计信息 API | 100% ✅ |
| Phase 119 文件上传 API | 100% ✅ |
| Phase 120 文件下载 API | 100% ✅ |
| Phase 121 文件详情 API | 100% ✅ |
| Phase 122 网络配置 API | 100% ✅ |
| Phase 123 网络配置更新 API | 100% ✅ |
| Phase 124 网络接口列表 API | 100% ✅ |
| Phase 125 DNS 配置 API | 100% ✅ |
| Phase 126 打印机列表 API | 100% ✅ |
| Phase 127 打印机详情 API | 100% ✅ |
| Phase 128 打印机更新 API | 100% ✅ |
| Phase 129 网络接口列表 API | 100% ✅ |
| Phase 130 防火墙规则列表 API | 100% ✅ |
| Phase 131 网络接口详情 API | 100% ✅ |
| Phase 132 网络接口创建 API | 100% ✅ |
| Phase 133 网络接口更新 API | 100% ✅ |
| Phase 134 网络接口删除 API | 100% ✅ |
| Phase 135 防火墙规则创建 API | 100% ✅ |
| Phase 136 防火墙规则详情 API | 100% ✅ |
| Phase 137 防火墙规则更新 API | 100% ✅ |
| Phase 138 防火墙规则删除 API | 100% ✅ |

**总体进度**：**Phase 138 防火墙规则删除 API 已完成**

---

**兵部尚书 签发**
2026-03-27 02:35 UTC
