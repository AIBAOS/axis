<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">系统概览</h1>
          <p class="text-gray-600 mt-1">{{ systemInfo.hostname }} - 运行时间: {{ formatUptime(systemInfo.uptime_seconds) }}</p>
        </div>
        <div class="flex items-center space-x-3">
          <span class="text-sm text-gray-500">
            最后更新: {{ lastUpdate }}
          </span>
          <button
            @click="refreshAll"
            :disabled="loading"
            class="btn-secondary flex items-center space-x-2"
          >
            <svg :class="{'animate-spin': loading}" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>刷新</span>
          </button>
        </div>
      </div>

      <!-- 核心指标卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <!-- CPU 使用率 -->
        <MetricCard
          title="CPU 使用率"
          :value="resources.cpu.usage_percent"
          unit="percent"
          type="cpu"
          :show-progress="true"
          :progress-value="resources.cpu.usage_percent"
          :progress-max="100"
          progress-label="使用率"
          :sub-info="`负载: ${resources.cpu.load_1m?.toFixed(2) || '-'} / ${resources.cpu.core_count || 0} 核`"
        />

        <!-- 内存使用率 -->
        <MetricCard
          title="内存使用率"
          :value="resources.memory.usage_percent"
          unit="percent"
          type="memory"
          :show-progress="true"
          :progress-value="resources.memory.used_bytes"
          :progress-max="resources.memory.total_bytes"
          progress-label="已使用"
          :sub-info="`${formatBytes(resources.memory.used_bytes)} / ${formatBytes(resources.memory.total_bytes)}`"
        />

        <!-- 磁盘空间 -->
        <MetricCard
          title="磁盘空间"
          :value="diskUsagePercent"
          unit="percent"
          type="disk"
          :show-progress="true"
          :progress-value="diskUsagePercent"
          :progress-max="100"
          progress-label="已使用"
          :sub-info="`${disks.length} 个磁盘在线`"
        />

        <!-- 运行服务 -->
        <MetricCard
          title="网络吞吐"
          :value="networkThroughput"
          type="network"
          :sub-info="`↓ ${formatSpeed(resources.network_io?.rx_bytes_sec)} / ↑ ${formatSpeed(resources.network_io?.tx_bytes_sec)}`"
        />
      </div>

      <!-- 主内容区域 -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- 左侧：快速入口 -->
        <div class="lg:col-span-2 space-y-6">
          <!-- 快速入口 -->
          <div>
            <h2 class="text-lg font-semibold text-gray-900 mb-4">快速入口</h2>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
              <QuickLinkCard
                title="文件管理"
                description="浏览和管理文件"
                to="/files"
                type="files"
              />
              <QuickLinkCard
                title="存储管理"
                description="管理存储卷和磁盘"
                to="/storage"
                type="storage"
              />
              <QuickLinkCard
                title="共享管理"
                description="SMB/NFS/WebDAV 共享"
                to="/shares"
                type="shares"
              />
              <QuickLinkCard
                title="用户管理"
                description="管理用户和权限"
                to="/users"
                type="users"
              />
              <QuickLinkCard
                title="备份管理"
                description="备份任务和恢复"
                to="/backups"
                type="backups"
              />
              <QuickLinkCard
                title="系统日志"
                description="查看系统运行日志"
                to="/logs"
                type="logs"
              />
            </div>
          </div>

          <!-- 系统信息卡片 -->
          <div class="bg-white rounded-lg shadow-md p-6">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">系统信息</h2>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
              <div>
                <p class="text-gray-500">主机名</p>
                <p class="font-medium text-gray-900">{{ systemInfo.hostname || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">操作系统</p>
                <p class="font-medium text-gray-900">{{ systemInfo.os_version || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">CPU 型号</p>
                <p class="font-medium text-gray-900 truncate" :title="systemInfo.cpu_model">{{ systemInfo.cpu_model || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">内存容量</p>
                <p class="font-medium text-gray-900">{{ systemInfo.total_memory_gb || '-' }} GB</p>
              </div>
              <div>
                <p class="text-gray-500">内核版本</p>
                <p class="font-medium text-gray-900 truncate" :title="systemInfo.kernel_version">{{ systemInfo.kernel_version || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">CPU 核心数</p>
                <p class="font-medium text-gray-900">{{ systemInfo.cpu_cores || '-' }} 核</p>
              </div>
              <div>
                <p class="text-gray-500">启动时间</p>
                <p class="font-medium text-gray-900">{{ formatBootTime(systemInfo.boot_time) }}</p>
              </div>
              <div>
                <p class="text-gray-500">WebUI 版本</p>
                <p class="font-medium text-gray-900">{{ version }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- 右侧：告警面板 -->
        <div>
          <AlertsPanel @click="handleAlertClick" />
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import MetricCard from '@/components/dashboard/MetricCard.vue'
import QuickLinkCard from '@/components/dashboard/QuickLinkCard.vue'
import AlertsPanel from '@/components/dashboard/AlertsPanel.vue'
import { api } from '@/utils/api'

const router = useRouter()
const version = import.meta.env.VITE_APP_VERSION || '0.1.0'

// 状态
const loading = ref(false)
const lastUpdate = ref('-')
let refreshTimer: ReturnType<typeof setInterval> | null = null

// 系统信息
const systemInfo = ref({
  hostname: '-',
  os_version: '-',
  kernel_version: '-',
  cpu_model: '-',
  cpu_cores: 0,
  total_memory_gb: 0,
  uptime_seconds: 0,
  boot_time: 0
})

// 系统资源
const resources = ref({
  cpu: {
    usage_percent: 0,
    load_1m: 0,
    load_5m: 0,
    load_15m: 0,
    core_count: 0
  },
  memory: {
    total_bytes: 0,
    used_bytes: 0,
    available_bytes: 0,
    usage_percent: 0
  },
  disk_io: {
    read_bytes_sec: 0,
    write_bytes_sec: 0
  },
  network_io: {
    rx_bytes_sec: 0,
    tx_bytes_sec: 0
  }
})

// 磁盘信息
const disks = ref<any[]>([])

// 计算磁盘使用率
const diskUsagePercent = computed(() => {
  if (disks.value.length === 0) return 0
  const total = disks.value.reduce((sum, d) => sum + (d.size_bytes || 0), 0)
  const used = disks.value.reduce((sum, d) => sum + (d.used_bytes || 0), 0)
  return total > 0 ? Math.round(used / total * 100) : 0
})

// 计算网络吞吐
const networkThroughput = computed(() => {
  const rx = resources.value.network_io?.rx_bytes_sec || 0
  const tx = resources.value.network_io?.tx_bytes_sec || 0
  return `${formatSpeed(rx)} / ${formatSpeed(tx)}`
})

// 加载系统信息
const loadSystemInfo = async () => {
  try {
    const response = await api.system.info()
    if (response.data.success !== false) {
      systemInfo.value = response.data.data || response.data
    }
  } catch (error) {
    console.error('Failed to load system info:', error)
  }
}

// 加载系统资源
const loadResources = async () => {
  try {
    const response = await api.system.resources()
    if (response.data.success !== false) {
      resources.value = response.data.data || response.data
    }
  } catch (error) {
    console.error('Failed to load resources:', error)
  }
}

// 加载磁盘信息
const loadDisks = async () => {
  try {
    const response = await api.storage.getDisks()
    disks.value = response.data.disks || response.data || []
  } catch (error) {
    console.error('Failed to load disks:', error)
  }
}

// 刷新所有数据
const refreshAll = async () => {
  loading.value = true
  try {
    await Promise.all([
      loadSystemInfo(),
      loadResources(),
      loadDisks()
    ])
    lastUpdate.value = new Date().toLocaleTimeString('zh-CN')
  } finally {
    loading.value = false
  }
}

// 格式化字节
const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

// 格式化速度
const formatSpeed = (bytesPerSec: number) => {
  if (!bytesPerSec) return '0 B/s'
  return formatBytes(bytesPerSec) + '/s'
}

// 格式化运行时间
const formatUptime = (seconds: number) => {
  if (!seconds) return '-'
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)

  if (days > 0) return `${days} 天 ${hours} 小时`
  if (hours > 0) return `${hours} 小时 ${minutes} 分钟`
  return `${minutes} 分钟`
}

// 格式化启动时间
const formatBootTime = (timestamp: number) => {
  if (!timestamp) return '-'
  return new Date(timestamp * 1000).toLocaleDateString('zh-CN')
}

// 处理告警点击
const handleAlertClick = (alert: any) => {
  router.push('/logs')
}

// 生命周期
onMounted(() => {
  refreshAll()

  // 每 30 秒自动刷新
  refreshTimer = setInterval(() => {
    refreshAll()
  }, 30000)
})

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer)
  }
})
</script>