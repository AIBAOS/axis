<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">系统日志</h1>
          <p class="text-gray-600 mt-1">查看系统运行日志和错误信息</p>
        </div>
        <div class="flex items-center space-x-3">
          <!-- 自动刷新开关 -->
          <div class="flex items-center space-x-2">
            <input
              v-model="autoRefresh"
              type="checkbox"
              id="autoRefresh"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
            <label for="autoRefresh" class="text-sm text-gray-600">自动刷新</label>
          </div>
          <!-- 手动刷新按钮 -->
          <button
            @click="loadLogs"
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

      <!-- 筛选栏 -->
      <div class="bg-white rounded-lg shadow p-4">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
          <!-- 关键词搜索 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">关键词搜索</label>
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索消息内容..."
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              @input="handleSearch"
            />
          </div>

          <!-- 日志级别 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">日志级别</label>
            <select
              v-model="levelFilter"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              @change="handleFilter"
            >
              <option value="all">全部级别</option>
              <option value="error">ERROR</option>
              <option value="warn">WARN</option>
              <option value="info">INFO</option>
              <option value="debug">DEBUG</option>
            </select>
          </div>

          <!-- 时间范围 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">时间范围</label>
            <select
              v-model="timeRange"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              @change="handleFilter"
            >
              <option value="all">全部时间</option>
              <option value="1h">最近 1 小时</option>
              <option value="6h">最近 6 小时</option>
              <option value="24h">最近 24 小时</option>
              <option value="7d">最近 7 天</option>
              <option value="30d">最近 30 天</option>
            </select>
          </div>

          <!-- 来源模块 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">来源模块</label>
            <input
              v-model="sourceFilter"
              type="text"
              placeholder="如: system, auth..."
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              @input="handleSearch"
            />
          </div>
        </div>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading && logs.length === 0" class="flex justify-center items-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
        <span class="ml-3 text-gray-600">加载中...</span>
      </div>

      <!-- 空数据提示 -->
      <div v-else-if="logs.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <p class="mt-4 text-gray-600">暂无日志记录</p>
        <p class="mt-2 text-sm text-gray-500">系统运行日志将显示在这里</p>
      </div>

      <!-- 日志列表 -->
      <div v-else class="bg-white rounded-lg shadow overflow-hidden">
        <!-- 表头 -->
        <div class="grid grid-cols-12 gap-4 px-4 py-3 bg-gray-50 border-b font-medium text-sm text-gray-700">
          <div class="col-span-2">时间</div>
          <div class="col-span-1">级别</div>
          <div class="col-span-2">模块</div>
          <div class="col-span-7">消息</div>
        </div>

        <!-- 日志行 -->
        <div
          v-for="log in filteredLogs"
          :key="log.id"
          class="grid grid-cols-12 gap-4 px-4 py-3 border-b hover:bg-gray-50 text-sm"
        >
          <div class="col-span-2 text-gray-500">
            {{ formatTime(log.created_at) }}
          </div>
          <div class="col-span-1">
            <span :class="getLevelClass(log.level)" class="px-2 py-1 text-xs font-medium rounded-full">
              {{ log.level.toUpperCase() }}
            </span>
          </div>
          <div class="col-span-2 text-gray-600 font-mono text-xs">
            {{ log.source || '-' }}
          </div>
          <div class="col-span-7 text-gray-900 break-all">
            {{ log.message }}
          </div>
        </div>

        <!-- 分页 -->
        <div class="flex justify-between items-center px-4 py-3 bg-gray-50 border-t">
          <div class="text-sm text-gray-500">
            共 {{ total }} 条日志，当前第 {{ currentPage }} 页
          </div>
          <div class="flex items-center space-x-2">
            <button
              @click="currentPage--"
              :disabled="currentPage === 1"
              class="px-3 py-1.5 border rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-100 text-sm"
            >
              上一页
            </button>
            <span class="text-sm text-gray-600">
              {{ currentPage }} / {{ totalPages }}
            </span>
            <button
              @click="currentPage++"
              :disabled="currentPage === totalPages"
              class="px-3 py-1.5 border rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-100 text-sm"
            >
              下一页
            </button>
            <!-- 每页数量选择 -->
            <select
              v-model="pageSize"
              class="ml-4 px-3 py-1.5 border border-gray-300 rounded-lg text-sm"
              @change="handlePageSizeChange"
            >
              <option :value="20">20条/页</option>
              <option :value="50">50条/页</option>
              <option :value="100">100条/页</option>
            </select>
          </div>
        </div>
      </div>

      <!-- 统计信息 -->
      <div class="grid grid-cols-4 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">错误</p>
              <p class="text-xl font-bold text-gray-900">{{ stats.error }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-yellow-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">警告</p>
              <p class="text-xl font-bold text-gray-900">{{ stats.warn }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">信息</p>
              <p class="text-xl font-bold text-gray-900">{{ stats.info }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">调试</p>
              <p class="text-xl font-bold text-gray-900">{{ stats.debug }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import { api } from '@/utils/api'

// 状态
const loading = ref(false)
const logs = ref<any[]>([])
const total = ref(0)
const currentPage = ref(1)
const totalPages = ref(1)
const pageSize = ref(20)
const autoRefresh = ref(false)
let refreshTimer: ReturnType<typeof setInterval> | null = null

// 筛选条件
const searchQuery = ref('')
const levelFilter = ref('all')
const timeRange = ref('all')
const sourceFilter = ref('')

// 统计信息
const stats = ref({
  error: 0,
  warn: 0,
  info: 0,
  debug: 0
})

// 筛选后的日志
const filteredLogs = computed(() => {
  let result = logs.value

  // 搜索筛选（客户端过滤）
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(log =>
      log.message?.toLowerCase().includes(query) ||
      log.source?.toLowerCase().includes(query)
    )
  }

  // 来源筛选
  if (sourceFilter.value) {
    const source = sourceFilter.value.toLowerCase()
    result = result.filter(log =>
      log.source?.toLowerCase().includes(source)
    )
  }

  return result
})

// 获取日志级别样式
const getLevelClass = (level: string) => {
  switch (level.toLowerCase()) {
    case 'error':
      return 'bg-red-100 text-red-800'
    case 'warn':
      return 'bg-yellow-100 text-yellow-800'
    case 'info':
      return 'bg-blue-100 text-blue-800'
    case 'debug':
      return 'bg-gray-100 text-gray-800'
    default:
      return 'bg-gray-100 text-gray-800'
  }
}

// 格式化时间
const formatTime = (timestamp: number) => {
  if (!timestamp) return '-'
  const date = new Date(timestamp * 1000)
  return date.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 计算时间范围起始时间
const getTimeRangeStart = () => {
  if (timeRange.value === 'all') return undefined

  const now = Math.floor(Date.now() / 1000)
  const ranges: Record<string, number> = {
    '1h': 3600,
    '6h': 21600,
    '24h': 86400,
    '7d': 604800,
    '30d': 2592000
  }

  const offset = ranges[timeRange.value]
  return offset ? now - offset : undefined
}

// 加载日志
const loadLogs = async () => {
  loading.value = true
  try {
    const params: any = {
      page: currentPage.value,
      page_size: pageSize.value
    }

    // 级别筛选
    if (levelFilter.value !== 'all') {
      params.level = levelFilter.value
    }

    const response = await api.system.logs(params)

    if (response.data.success !== false) {
      logs.value = response.data.data || response.data || []
      total.value = response.data.total || logs.value.length
      totalPages.value = response.data.total_pages || Math.ceil(total.value / pageSize.value)

      // 更新统计
      updateStats()
    }
  } catch (error) {
    console.error('Failed to load logs:', error)
  } finally {
    loading.value = false
  }
}

// 更新统计信息
const updateStats = () => {
  stats.value = {
    error: logs.value.filter(l => l.level === 'error').length,
    warn: logs.value.filter(l => l.level === 'warn').length,
    info: logs.value.filter(l => l.level === 'info').length,
    debug: logs.value.filter(l => l.level === 'debug').length
  }
}

// 搜索处理
let searchTimer: ReturnType<typeof setTimeout> | null = null
const handleSearch = () => {
  if (searchTimer) clearTimeout(searchTimer)
  searchTimer = setTimeout(() => {
    currentPage.value = 1
    loadLogs()
  }, 300)
}

// 筛选处理
const handleFilter = () => {
  currentPage.value = 1
  loadLogs()
}

// 每页数量变化
const handlePageSizeChange = () => {
  currentPage.value = 1
  loadLogs()
}

// 监听页码变化
watch(currentPage, () => {
  loadLogs()
})

// 监听自动刷新
watch(autoRefresh, (enabled) => {
  if (enabled) {
    refreshTimer = setInterval(() => {
      loadLogs()
    }, 10000) // 每 10 秒刷新
  } else {
    if (refreshTimer) {
      clearInterval(refreshTimer)
      refreshTimer = null
    }
  }
})

// 生命周期
onMounted(() => {
  loadLogs()
})

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer)
  }
  if (searchTimer) {
    clearTimeout(searchTimer)
  }
})
</script>