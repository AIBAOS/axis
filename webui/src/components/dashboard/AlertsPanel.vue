<template>
  <div class="bg-white rounded-lg shadow-md overflow-hidden">
    <div class="px-4 py-3 bg-gray-50 border-b flex justify-between items-center">
      <h3 class="font-semibold text-gray-900">系统告警</h3>
      <span v-if="alerts.length > 0" class="px-2 py-0.5 text-xs font-medium rounded-full bg-red-100 text-red-800">
        {{ alerts.length }}
      </span>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="p-4 text-center text-gray-500">
      <svg class="animate-spin h-5 w-5 mx-auto text-gray-400" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
      </svg>
      <p class="mt-2 text-sm">加载中...</p>
    </div>

    <!-- 无告警 -->
    <div v-else-if="alerts.length === 0" class="p-6 text-center">
      <svg class="mx-auto h-10 w-10 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <p class="mt-2 text-sm text-gray-600">系统运行正常，无告警</p>
    </div>

    <!-- 告警列表 -->
    <div v-else class="divide-y divide-gray-100 max-h-64 overflow-y-auto">
      <div
        v-for="alert in alerts"
        :key="alert.id"
        class="p-4 hover:bg-gray-50 cursor-pointer"
        @click="$emit('click', alert)"
      >
        <div class="flex items-start space-x-3">
          <div :class="getSeverityClass(alert.severity)" class="w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0">
            <svg v-if="alert.severity === 'critical'" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
            <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-gray-900">{{ alert.title }}</p>
            <p class="text-sm text-gray-500 truncate">{{ alert.message }}</p>
            <p class="text-xs text-gray-400 mt-1">{{ formatTime(alert.created_at) }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部操作 -->
    <div v-if="alerts.length > 0" class="px-4 py-3 bg-gray-50 border-t">
      <router-link to="/logs" class="text-sm text-primary-600 hover:text-primary-700">
        查看全部日志 →
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '@/utils/api'

defineEmits<{
  click: [alert: any]
}>()

const loading = ref(true)
const alerts = ref<any[]>([])

// 加载告警
const loadAlerts = async () => {
  loading.value = true
  try {
    const response = await api.system.logs({ level: 'error', page_size: 5 })
    const data = response.data.data || response.data || []
    alerts.value = data.map((log: any) => ({
      id: log.id,
      severity: log.level === 'error' ? 'critical' : 'warning',
      title: log.source || '系统错误',
      message: log.message,
      created_at: log.created_at
    }))
  } catch (error) {
    console.error('Failed to load alerts:', error)
    alerts.value = []
  } finally {
    loading.value = false
  }
}

// 获取严重程度样式
const getSeverityClass = (severity: string) => {
  return severity === 'critical'
    ? 'bg-red-100 text-red-600'
    : 'bg-yellow-100 text-yellow-600'
}

// 格式化时间
const formatTime = (timestamp: number) => {
  if (!timestamp) return ''
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes} 分钟前`
  if (hours < 24) return `${hours} 小时前`
  return `${days} 天前`
}

onMounted(() => {
  loadAlerts()
})
</script>