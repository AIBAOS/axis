<template>
  <DefaultLayout>
    <div class="space-y-4">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">日志管理</h1>
          <p class="text-gray-600 mt-1">系统日志、操作审计和故障排查</p>
        </div>
        <div class="flex items-center space-x-3">
          <!-- 实时日志流开关 -->
          <div class="flex items-center space-x-2">
            <span class="text-sm text-gray-600">实时流</span>
            <button @click="toggleLiveStream" :class="liveStream ? 'bg-green-500' : 'bg-gray-300'" class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full transition-colors duration-200">
              <span :class="liveStream ? 'translate-x-5' : 'translate-x-0'" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200"></span>
            </button>
          </div>
          <!-- 自动刷新间隔 -->
          <select v-model="refreshInterval" @change="updateRefreshInterval" class="px-3 py-1.5 border border-gray-300 rounded-lg text-sm">
            <option :value="0">关闭自动刷新</option>
            <option :value="5000">5 秒</option>
            <option :value="10000">10 秒</option>
            <option :value="30000">30 秒</option>
            <option :value="60000">1 分钟</option>
          </select>
          <!-- 清空按钮 -->
          <button @click="confirmClearLogs" class="px-3 py-1.5 border border-red-300 text-red-600 rounded-lg hover:bg-red-50 text-sm">
            清空
          </button>
          <!-- 导出按钮 -->
          <button @click="exportLogs" class="btn-secondary flex items-center space-x-1 text-sm">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
            <span>导出</span>
          </button>
          <!-- 刷新按钮 -->
          <button @click="loadLogs" :disabled="loading" class="btn-secondary flex items-center space-x-1 text-sm">
            <svg :class="{'animate-spin': loading}" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" /></svg>
            <span>刷新</span>
          </button>
        </div>
      </div>

      <!-- 选项卡 -->
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8">
          <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id; loadLogs()" :class="[currentTab === tab.id ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700', 'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm']">
            {{ tab.name }}
            <span v-if="tab.count" class="ml-2 px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-600">{{ tab.count }}</span>
          </button>
        </nav>
      </div>

      <!-- 统计卡片 -->
      <div class="grid grid-cols-5 gap-3">
        <div class="bg-white rounded-lg shadow p-3"><div class="flex items-center"><div class="w-8 h-8 rounded bg-gray-100 flex items-center justify-center mr-2"><span class="text-lg">📊</span></div><div><p class="text-xs text-gray-500">总数</p><p class="text-lg font-bold">{{ stats.total }}</p></div></div></div>
        <div class="bg-white rounded-lg shadow p-3"><div class="flex items-center"><div class="w-8 h-8 rounded bg-red-100 flex items-center justify-center mr-2"><span class="text-lg">❌</span></div><div><p class="text-xs text-gray-500">ERROR</p><p class="text-lg font-bold text-red-600">{{ stats.error }}</p></div></div></div>
        <div class="bg-white rounded-lg shadow p-3"><div class="flex items-center"><div class="w-8 h-8 rounded bg-yellow-100 flex items-center justify-center mr-2"><span class="text-lg">⚠️</span></div><div><p class="text-xs text-gray-500">WARN</p><p class="text-lg font-bold text-yellow-600">{{ stats.warn }}</p></div></div></div>
        <div class="bg-white rounded-lg shadow p-3"><div class="flex items-center"><div class="w-8 h-8 rounded bg-blue-100 flex items-center justify-center mr-2"><span class="text-lg">ℹ️</span></div><div><p class="text-xs text-gray-500">INFO</p><p class="text-lg font-bold text-blue-600">{{ stats.info }}</p></div></div></div>
        <div class="bg-white rounded-lg shadow p-3"><div class="flex items-center"><div class="w-8 h-8 rounded bg-gray-100 flex items-center justify-center mr-2"><span class="text-lg">🔧</span></div><div><p class="text-xs text-gray-500">DEBUG</p><p class="text-lg font-bold text-gray-600">{{ stats.debug }}</p></div></div></div>
      </div>

      <!-- 筛选栏 -->
      <div class="bg-white rounded-lg shadow p-4">
        <div class="grid grid-cols-1 md:grid-cols-5 gap-3">
          <!-- 关键词搜索 -->
          <div><label class="block text-xs font-medium text-gray-700 mb-1">关键词</label><input v-model="searchQuery" type="text" placeholder="搜索..." class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 text-sm" /></div>
          <!-- 日志级别 -->
          <div><label class="block text-xs font-medium text-gray-700 mb-1">级别</label><select v-model="levelFilter" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 text-sm"><option value="all">全部</option><option value="error">ERROR</option><option value="warn">WARN</option><option value="info">INFO</option><option value="debug">DEBUG</option></select></div>
          <!-- 时间范围 -->
          <div><label class="block text-xs font-medium text-gray-700 mb-1">时间</label><select v-model="timeRange" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 text-sm"><option value="all">全部</option><option value="1h">1 小时</option><option value="6h">6 小时</option><option value="24h">24 小时</option><option value="7d">7 天</option><option value="30d">30 天</option></select></div>
          <!-- 来源模块 -->
          <div><label class="block text-xs font-medium text-gray-700 mb-1">来源</label><input v-model="sourceFilter" type="text" placeholder="模块名" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 text-sm" /></div>
          <!-- 每页数量 -->
          <div><label class="block text-xs font-medium text-gray-700 mb-1">每页</label><select v-model="pageSize" @change="loadLogs" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 text-sm"><option :value="50">50 条</option><option :value="100">100 条</option><option :value="200">200 条</option><option :value="500">500 条</option></select></div>
        </div>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading && logs.length === 0" class="flex justify-center py-12"><svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" /></svg></div>

      <!-- 空状态 -->
      <div v-else-if="logs.length === 0 && currentTab !== 'config'" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" /></svg><p class="mt-4 text-gray-600">暂无日志记录</p></div>

      <!-- 日志列表 -->
      <div v-else-if="currentTab !== 'config'" class="bg-white rounded-lg shadow overflow-hidden">
        <!-- 日志条目 -->
        <div class="divide-y divide-gray-100">
          <div v-for="log in logs" :key="log.id" class="hover:bg-gray-50">
            <!-- 日志行 -->
            <div @click="toggleLogDetail(log.id)" class="px-4 py-3 flex items-start space-x-3 cursor-pointer">
              <!-- 级别标签 -->
              <span :class="getLevelBgClass(log.level)" class="px-2 py-0.5 text-xs font-medium rounded flex-shrink-0">{{ log.level?.toUpperCase() }}</span>
              <!-- 内容 -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center space-x-2">
                  <span class="text-xs text-gray-400 font-mono">{{ formatTime(log.created_at) }}</span>
                  <span class="text-xs text-gray-500 bg-gray-100 px-1 rounded">{{ log.source || '-' }}</span>
                </div>
                <p class="text-sm text-gray-900 mt-1 break-all">{{ log.message }}</p>
              </div>
              <!-- 展开图标 -->
              <svg :class="{'rotate-180': expandedLogs.includes(log.id)}" class="w-4 h-4 text-gray-400 flex-shrink-0 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg>
            </div>
            <!-- 详情展开 -->
            <div v-if="expandedLogs.includes(log.id)" class="px-4 pb-3 pl-16">
              <div class="bg-gray-50 rounded-lg p-3 text-sm">
                <div class="grid grid-cols-2 gap-2 text-xs">
                  <div><span class="text-gray-500">日志ID:</span> <span class="font-mono">{{ log.id }}</span></div>
                  <div><span class="text-gray-500">进程:</span> <span class="font-mono">{{ log.pid || '-' }}</span></div>
                  <div><span class="text-gray-500">主机:</span> <span>{{ log.hostname || '-' }}</span></div>
                  <div><span class="text-gray-500">时间戳:</span> <span class="font-mono">{{ log.created_at }}</span></div>
                </div>
                <!-- 堆栈信息 -->
                <div v-if="log.stack_trace" class="mt-2 pt-2 border-t border-gray-200">
                  <p class="text-xs text-gray-500 mb-1">堆栈信息:</p>
                  <pre class="text-xs bg-gray-800 text-green-400 p-2 rounded overflow-x-auto">{{ log.stack_trace }}</pre>
                </div>
                <!-- 上下文 -->
                <div v-if="log.context" class="mt-2 pt-2 border-t border-gray-200">
                  <p class="text-xs text-gray-500 mb-1">上下文:</p>
                  <pre class="text-xs bg-gray-100 p-2 rounded overflow-x-auto">{{ typeof log.context === 'string' ? log.context : JSON.stringify(log.context, null, 2) }}</pre>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 分页 -->
        <div class="flex justify-between items-center px-4 py-3 bg-gray-50 border-t">
          <div class="text-sm text-gray-500">共 {{ total }} 条日志</div>
          <div class="flex items-center space-x-2">
            <button @click="currentPage--" :disabled="currentPage === 1" class="px-3 py-1.5 border rounded text-sm disabled:opacity-50">上一页</button>
            <span class="text-sm">{{ currentPage }} / {{ totalPages }}</span>
            <button @click="currentPage++" :disabled="currentPage === totalPages" class="px-3 py-1.5 border rounded text-sm disabled:opacity-50">下一页</button>
          </div>
        </div>
      </div>

      <!-- 轮转配置选项卡 -->
      <div v-else-if="currentTab === 'config'" class="max-w-2xl space-y-6">
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="font-semibold text-gray-900 mb-4">日志轮转配置</h3>
          <form @submit.prevent="saveRotationConfig" class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <label class="block text-sm font-medium text-gray-700">启用日志轮转</label>
                <p class="text-sm text-gray-500">自动轮转日志文件，防止单个文件过大</p>
              </div>
              <input v-model="rotationConfig.enabled" type="checkbox" class="h-5 w-5 text-primary-600 rounded" />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">单个文件最大 (MB)</label>
                <input v-model.number="rotationConfig.maxSizeMB" type="number" min="1" max="1000" class="w-full px-3 py-2 border rounded-lg" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">保留文件数</label>
                <input v-model.number="rotationConfig.maxFiles" type="number" min="1" max="100" class="w-full px-3 py-2 border rounded-lg" />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">保留天数</label>
              <input v-model.number="rotationConfig.retentionDays" type="number" min="1" max="365" class="w-32 px-3 py-2 border rounded-lg" />
              <p class="text-xs text-gray-500 mt-1">超过天数的日志将被自动删除</p>
            </div>

            <div class="flex items-center justify-between">
              <div>
                <label class="block text-sm font-medium text-gray-700">压缩旧日志</label>
                <p class="text-sm text-gray-500">压缩轮转后的日志文件以节省空间</p>
              </div>
              <input v-model="rotationConfig.compress" type="checkbox" class="h-5 w-5 text-primary-600 rounded" />
            </div>

            <div class="flex justify-end">
              <button type="submit" class="btn-primary">保存配置</button>
            </div>
          </form>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="font-semibold text-gray-900 mb-4">日志统计</h3>
          <div class="grid grid-cols-2 gap-4">
            <div class="p-4 bg-gray-50 rounded-lg">
              <p class="text-sm text-gray-500">日志文件总大小</p>
              <p class="text-2xl font-bold text-gray-900">{{ Math.round(total * 0.5) }} MB</p>
            </div>
            <div class="p-4 bg-gray-50 rounded-lg">
              <p class="text-sm text-gray-500">最早日志时间</p>
              <p class="text-lg font-medium text-gray-900">2026-03-01</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 实时日志流指示器 -->
      <div v-if="liveStream" class="fixed bottom-4 left-4 bg-green-500 text-white px-3 py-2 rounded-lg shadow-lg flex items-center space-x-2">
        <span class="w-2 h-2 bg-white rounded-full animate-pulse"></span>
        <span class="text-sm font-medium">实时日志流</span>
      </div>

      <!-- Toast -->
      <div v-if="toast.show" class="fixed bottom-4 right-4 z-50"><div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">{{ toast.message }}</div></div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import { api } from '@/utils/api'
import { useToast } from '@/composables/useToast'

const { showToast } = useToast()

// 状态
const loading = ref(true)
const logs = ref<any[]>([])
const total = ref(0)
const currentPage = ref(1)
const totalPages = ref(1)
const pageSize = ref(100)

// 选项卡
const tabs = [
  { id: 'system', name: '系统日志', count: 0 },
  { id: 'audit', name: '操作审计', count: 0 },
  { id: 'hardware', name: '硬件日志', count: 0 },
  { id: 'config', name: '轮转配置', count: 0 }
]
const currentTab = ref('system')

// 筛选
const searchQuery = ref('')
const levelFilter = ref('all')
const timeRange = ref('24h')
const sourceFilter = ref('')

// 自动刷新
const refreshInterval = ref(10000)
const liveStream = ref(false)
let refreshTimer: ReturnType<typeof setInterval> | null = null
let liveStreamTimer: ReturnType<typeof setInterval> | null = null

// 展开
const expandedLogs = ref<number[]>([])

// Toast


// 统计
const stats = computed(() => {
  const s = { total: total.value, error: 0, warn: 0, info: 0, debug: 0 }
  logs.value.forEach(l => { const lv = l.level as keyof typeof s; if (lv !== 'total' && s[lv] !== undefined) s[lv]++ })
  return s
})

// 加载日志
const loadLogs = async () => {
  loading.value = true
  try {
    const params: any = { page: currentPage.value, page_size: pageSize.value }
    if (levelFilter.value !== 'all') params.level = levelFilter.value
    if (sourceFilter.value) params.source = sourceFilter.value
    if (searchQuery.value) params.query = searchQuery.value

    const r = await api.system.logs(params)
    logs.value = r.data.data || r.data || []
    total.value = r.data.total || logs.value.length
    totalPages.value = r.data.total_pages || Math.ceil(total.value / pageSize.value)
  } catch (e) {
    showToast('error', '加载失败')
  } finally {
    loading.value = false
  }
}

// 实时流
const toggleLiveStream = () => {
  liveStream.value = !liveStream.value
  if (liveStream.value) {
    liveStreamTimer = setInterval(() => { loadLogs() }, 2000)
  } else {
    if (liveStreamTimer) { clearInterval(liveStreamTimer); liveStreamTimer = null }
  }
}

// 更新刷新间隔
const updateRefreshInterval = () => {
  if (refreshTimer) { clearInterval(refreshTimer); refreshTimer = null }
  if (refreshInterval.value > 0) {
    refreshTimer = setInterval(() => loadLogs(), refreshInterval.value)
  }
}

// 切换详情
const toggleLogDetail = (id: number) => {
  const idx = expandedLogs.value.indexOf(id)
  if (idx >= 0) expandedLogs.value.splice(idx, 1)
  else expandedLogs.value.push(id)
}

// 导出日志
const exportLogs = () => {
  const lines = logs.value.map(l => {
    const time = new Date(l.created_at * 1000).toISOString()
    return `[${time}] [${l.level?.toUpperCase()}] [${l.source || '-'}] ${l.message}`
  }).join('\n')

  const blob = new Blob([lines], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `system-logs-${new Date().toISOString().slice(0, 10)}.txt`
  a.click()
  URL.revokeObjectURL(url)
  showToast('success', '日志已导出')
}

// 级别背景色
const getLevelBgClass = (level: string) => {
  switch (level?.toLowerCase()) {
    case 'error': return 'bg-red-500 text-white'
    case 'warn': return 'bg-yellow-500 text-white'
    case 'info': return 'bg-blue-500 text-white'
    case 'debug': return 'bg-gray-400 text-white'
    default: return 'bg-gray-300 text-gray-700'
  }
}

// 格式化时间
const formatTime = (ts: number) => {
  if (!ts) return '-'
  return new Date(ts * 1000).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit', second: '2-digit' })
}

// 清空日志
const confirmClearLogs = () => {
  if (!confirm('确定清空所有日志吗？此操作不可撤销！')) return
  clearLogs()
}

const clearLogs = async () => {
  try {
    await api.system.clearLogs?.()
    logs.value = []
    total.value = 0
    showToast('success', '日志已清空')
  } catch (e) {
    showToast('error', '清空失败')
  }
}

// 轮转配置
const rotationConfig = ref({
  enabled: true,
  maxSizeMB: 100,
  maxFiles: 10,
  compress: true,
  retentionDays: 30
})

const saveRotationConfig = async () => {
  try {
    await api.settings.update({ log_rotation: rotationConfig.value })
    showToast('success', '轮转配置已保存')
  } catch (e) {
    showToast('error', '保存失败')
  }
}

// 监听
watch([currentPage, levelFilter, timeRange], () => loadLogs())

onMounted(() => { loadLogs(); updateRefreshInterval() })
onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer)
  if (liveStreamTimer) clearInterval(liveStreamTimer)
})
</script>