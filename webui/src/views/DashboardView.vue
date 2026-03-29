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
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
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
        <MetricCard
          title="系统温度"
          :value="systemTemperature"
          unit="celsius"
          type="temperature"
          :sub-info="temperatureStatus"
        />
        <MetricCard
          title="磁盘空间"
          :value="diskUsagePercent"
          unit="percent"
          type="disk"
          :show-progress="true"
          :progress-value="diskUsagePercent"
          :progress-max="100"
          progress-label="已使用"
          :sub-info="healthStatus"
        />
        <MetricCard
          title="网络吞吐"
          :value="networkThroughput"
          type="network"
          :sub-info="`↓ ${formatSpeed(resources.network_io?.rx_bytes_sec)} / ↑ ${formatSpeed(resources.network_io?.tx_bytes_sec)}`"
        />
      </div>

      <!-- 主内容区域 -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- 左侧 -->
        <div class="lg:col-span-2 space-y-6">
          <!-- 网络流量图表 -->
          <NetworkChart
            :rx-history="rxHistory"
            :tx-history="txHistory"
            :current-rx="resources.network_io?.rx_bytes_sec || 0"
            :current-tx="resources.network_io?.tx_bytes_sec || 0"
            :total-rx="networkStats.totalRx"
            :total-tx="networkStats.totalTx"
          />

          <!-- 快速入口 -->
          <div>
            <h2 class="text-lg font-semibold text-gray-900 mb-4">快速入口</h2>
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
              <QuickLinkCard
                title="文件"
                description="文件管理"
                to="/files"
                type="files"
              />
              <QuickLinkCard
                title="存储"
                description="磁盘管理"
                to="/storage"
                type="storage"
              />
              <QuickLinkCard
                title="共享"
                description="共享管理"
                to="/shares"
                type="shares"
              />
              <QuickLinkCard
                title="打印机"
                description="打印机管理"
                to="/printers"
                type="default"
              />
              <QuickLinkCard
                title="任务"
                description="任务队列"
                to="/jobs"
                type="default"
              />
              <QuickLinkCard
                title="用户"
                description="用户管理"
                to="/users"
                type="users"
              />
              <QuickLinkCard
                title="备份"
                description="备份管理"
                to="/backups"
                type="backups"
              />
              <QuickLinkCard
                title="日志"
                description="系统日志"
                to="/logs"
                type="logs"
              />
            </div>
          </div>

          <!-- 最近活动 -->
          <div class="bg-white rounded-lg shadow-md overflow-hidden">
            <div class="px-4 py-3 border-b flex justify-between items-center">
              <h3 class="font-semibold text-gray-900">最近活动</h3>
              <router-link to="/logs" class="text-sm text-primary-600 hover:text-primary-700">查看全部</router-link>
            </div>
            <div v-if="recentLogs.length === 0" class="p-6 text-center text-gray-500">
              暂无活动记录
            </div>
            <div v-else class="divide-y divide-gray-100">
              <div
                v-for="log in recentLogs.slice(0, 5)"
                :key="log.id"
                class="px-4 py-3 hover:bg-gray-50"
              >
                <div class="flex items-start space-x-3">
                  <span :class="getLogLevelClass(log.level)" class="px-2 py-0.5 text-xs rounded">
                    {{ log.level?.toUpperCase() }}
                  </span>
                  <div class="flex-1 min-w-0">
                    <p class="text-sm text-gray-900 truncate">{{ log.message }}</p>
                    <p class="text-xs text-gray-400">{{ log.source }} · {{ formatLogTime(log.created_at) }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 系统信息 -->
          <div class="bg-white rounded-lg shadow-md p-4">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">系统信息</h2>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
              <div>
                <p class="text-gray-500">主机名</p>
                <p class="font-medium text-gray-900 truncate">{{ systemInfo.hostname || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">操作系统</p>
                <p class="font-medium text-gray-900 truncate">{{ systemInfo.os_version || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">CPU</p>
                <p class="font-medium text-gray-900 truncate">{{ systemInfo.cpu_cores || '-' }} 核</p>
              </div>
              <div>
                <p class="text-gray-500">内存</p>
                <p class="font-medium text-gray-900">{{ systemInfo.total_memory_gb || '-' }} GB</p>
              </div>
              <div>
                <p class="text-gray-500">内核版本</p>
                <p class="font-medium text-gray-900 truncate">{{ systemInfo.kernel_version || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">启动时间</p>
                <p class="font-medium text-gray-900">{{ formatBootTime(systemInfo.boot_time) }}</p>
              </div>
              <div>
                <p class="text-gray-500">磁盘数量</p>
                <p class="font-medium text-gray-900">{{ disks.length }} 块</p>
              </div>
              <div>
                <p class="text-gray-500">存储健康</p>
                <p class="font-medium" :class="healthColor">{{ healthStatus }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- 右侧 -->
        <div class="space-y-6">
          <!-- 快速操作 -->
          <QuickActions />

          <!-- 告警面板 -->
          <AlertsPanel @click="handleAlertClick" />

          <!-- 服务状态概览 -->
          <div class="bg-white rounded-lg shadow-md p-4">
            <h3 class="font-semibold text-gray-900 mb-4">服务状态</h3>
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600">存储服务</span>
                <span class="px-2 py-0.5 text-xs rounded bg-green-100 text-green-700">正常</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600">网络服务</span>
                <span class="px-2 py-0.5 text-xs rounded bg-green-100 text-green-700">正常</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600">打印服务</span>
                <span :class="printerStats.total > 0 ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'" class="px-2 py-0.5 text-xs rounded">
                  {{ printerStats.total > 0 ? '正常' : '无设备' }}
                </span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600">备份服务</span>
                <span :class="backupStats.active > 0 ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'" class="px-2 py-0.5 text-xs rounded">
                  {{ backupStats.active > 0 ? '运行中' : '待配置' }}
                </span>
              </div>
            </div>
          </div>
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
import NetworkChart from '@/components/dashboard/NetworkChart.vue'
import QuickActions from '@/components/dashboard/QuickActions.vue'
import { api } from '@/utils/api'

const router = useRouter()

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
  cpu: { usage_percent: 0, load_1m: 0, load_5m: 0, load_15m: 0, core_count: 0 },
  memory: { total_bytes: 0, used_bytes: 0, available_bytes: 0, usage_percent: 0 },
  disk_io: { read_bytes_sec: 0, write_bytes_sec: 0 },
  network_io: { rx_bytes_sec: 0, tx_bytes_sec: 0 }
})

// 网络流量历史
const rxHistory = ref<number[]>([])
const txHistory = ref<number[]>([])
const networkStats = ref({ totalRx: 0, totalTx: 0 })

// 磁盘和存储
const disks = ref<any[]>([])
const storageHealth = ref('healthy')

// 打印机统计
const printerStats = ref({ total: 0, idle: 0, printing: 0, offline: 0, error: 0 })

// 备份统计
const backupStats = ref({ active: 0, lastBackup: '', storageUsed: '' })

// 最近日志
const recentLogs = ref<any[]>([])

// 计算属性 - 系统温度
const systemTemperature = computed(() => {
  if (disks.value.length === 0) return 0
  const temps = disks.value
    .filter(d => d.temperature && d.temperature > 0)
    .map(d => d.temperature)
  if (temps.length === 0) return 0
  return Math.round(temps.reduce((a, b) => a + b, 0) / temps.length)
})

const temperatureStatus = computed(() => {
  const temp = systemTemperature.value
  if (temp === 0) return '暂无数据'
  if (temp < 40) return '正常'
  if (temp < 60) return '偏高'
  return '过热警告'
})

// 计算属性 - 磁盘使用率
const diskUsagePercent = computed(() => {
  if (disks.value.length === 0) return 0
  const total = disks.value.reduce((sum, d) => sum + (d.size_bytes || 0), 0)
  const used = disks.value.reduce((sum, d) => sum + (d.used_bytes || 0), 0)
  return total > 0 ? Math.round(used / total * 100) : 0
})

// 计算属性 - 健康状态
const healthStatus = computed(() => {
  const smartWarnings = disks.value.filter(d => d.smart_status === 'warning').length
  const smartErrors = disks.value.filter(d => d.smart_status === 'failed' || d.smart_status === 'error').length
  
  if (smartErrors > 0) return '异常'
  if (smartWarnings > 0) return '警告'
  return '健康'
})

const healthColor = computed(() => {
  const status = healthStatus.value
  if (status === '异常') return 'text-red-600'
  if (status === '警告') return 'text-yellow-600'
  return 'text-green-600'
})

// 计算属性 - 网络吞吐
const networkThroughput = computed(() => {
  const rx = resources.value.network_io?.rx_bytes_sec || 0
  const tx = resources.value.network_io?.tx_bytes_sec || 0
  return `${formatSpeed(rx)} / ${formatSpeed(tx)}`
})

// 加载函数
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

const loadResources = async () => {
  try {
    const response = await api.system.resources()
    if (response.data.success !== false) {
      const data = response.data.data || response.data
      resources.value = data
      
      // 更新网络流量历史
      rxHistory.value.push(data.network_io?.rx_bytes_sec || 0)
      txHistory.value.push(data.network_io?.tx_bytes_sec || 0)
      
      // 保持最近 60 个数据点（30秒刷新 * 60 = 30分钟）
      if (rxHistory.value.length > 60) {
        rxHistory.value = rxHistory.value.slice(-60)
        txHistory.value = txHistory.value.slice(-60)
      }
      
      // 更新总计
      networkStats.value.totalRx += data.network_io?.rx_bytes_sec || 0
      networkStats.value.totalTx += data.network_io?.tx_bytes_sec || 0
    }
  } catch (error) {
    console.error('Failed to load resources:', error)
  }
}

const loadDisks = async () => {
  try {
    const response = await api.storage.getDisks()
    disks.value = response.data.disks || response.data || []
    
    // 获取存储健康状态
    const usageResponse = await api.storage.getUsage()
    if (usageResponse.data.health_status) {
      storageHealth.value = usageResponse.data.health_status
    }
  } catch (error) {
    console.error('Failed to load disks:', error)
  }
}

const loadPrinters = async () => {
  try {
    const response = await api.printers.list()
    const printers = response.data.data || response.data || []
    printerStats.value = {
      total: printers.length,
      idle: printers.filter((p: any) => p.status === 'idle').length,
      printing: printers.filter((p: any) => p.status === 'printing').length,
      offline: printers.filter((p: any) => p.status === 'offline').length,
      error: printers.filter((p: any) => p.status === 'error').length
    }
  } catch (error) {
    console.error('Failed to load printers:', error)
  }
}

const loadBackups = async () => {
  try {
    const response = await api.backups.list()
    const backups = response.data.backups || response.data || []
    const active = backups.filter((b: any) => b.status === 'active' || b.status === 'running').length
    backupStats.value = {
      active,
      lastBackup: backups[0]?.created_at ? formatLogTime(backups[0].created_at) : '暂无',
      storageUsed: '-'
    }
  } catch (error) {
    console.error('Failed to load backups:', error)
  }
}

const loadRecentLogs = async () => {
  try {
    const response = await api.system.logs({ page_size: 10 })
    recentLogs.value = response.data.data || response.data || []
  } catch (error) {
    console.error('Failed to load logs:', error)
  }
}

// 刷新所有
const refreshAll = async () => {
  loading.value = true
  try {
    await Promise.all([
      loadSystemInfo(),
      loadResources(),
      loadDisks(),
      loadPrinters(),
      loadBackups(),
      loadRecentLogs()
    ])
    lastUpdate.value = new Date().toLocaleTimeString('zh-CN')
  } finally {
    loading.value = false
  }
}

// 辅助函数
const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

const formatSpeed = (bytesPerSec: number) => {
  if (!bytesPerSec) return '0 B/s'
  return formatBytes(bytesPerSec) + '/s'
}

const formatUptime = (seconds: number) => {
  if (!seconds) return '-'
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  if (days > 0) return `${days} 天 ${hours} 小时`
  if (hours > 0) return `${hours} 小时 ${minutes} 分钟`
  return `${minutes} 分钟`
}

const formatBootTime = (timestamp: number) => {
  if (!timestamp) return '-'
  const date = new Date(timestamp * 1000)
  return date.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}

const formatLogTime = (timestamp: number | string) => {
  if (!timestamp) return '-'
  const date = typeof timestamp === 'number'
    ? (timestamp > 9999999999 ? new Date(timestamp) : new Date(timestamp * 1000))
    : new Date(timestamp)
  return date.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}

const getLogLevelClass = (level: string) => {
  switch (level) {
    case 'error': return 'bg-red-100 text-red-700'
    case 'warn': return 'bg-yellow-100 text-yellow-700'
    case 'info': return 'bg-blue-100 text-blue-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

const handleAlertClick = () => {
  router.push('/logs')
}

// 生命周期
onMounted(() => {
  refreshAll()
  refreshTimer = setInterval(() => refreshAll(), 30000)
})

onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer)
})
</script>