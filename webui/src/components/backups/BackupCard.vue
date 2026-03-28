<template>
  <div class="card hover:shadow-lg transition-shadow">
    <!-- 标题栏 -->
    <div class="flex justify-between items-start mb-4">
      <div class="flex items-center space-x-3">
        <div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H2a2 2 0 01-2-2V5a2 2 0 012-2h6" />
          </svg>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900">{{ backup.name }}</h3>
          <p class="text-sm text-gray-500">{{ backup.type === 'full' ? '全量备份' : '增量备份' }}</p>
        </div>
      </div>
      <span
        :class="[
          'px-2 py-1 text-xs font-medium rounded-full',
          statusClass
        ]"
      >
        {{ statusText }}
      </span>
    </div>

    <!-- 备份路径信息 -->
    <div class="space-y-2 mb-4">
      <div class="flex items-start space-x-2 text-sm">
        <svg class="w-4 h-4 text-gray-400 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14" />
        </svg>
        <div>
          <span class="text-gray-500">源路径:</span>
          <span class="ml-2 text-gray-900 font-mono text-xs">{{ backup.source_path }}</span>
        </div>
      </div>
      <div class="flex items-start space-x-2 text-sm">
        <svg class="w-4 h-4 text-gray-400 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
        </svg>
        <div>
          <span class="text-gray-500">目标路径:</span>
          <span class="ml-2 text-gray-900 font-mono text-xs">{{ backup.destination_path }}</span>
        </div>
      </div>
    </div>

    <!-- 计划任务信息 -->
    <div class="bg-gray-50 rounded-lg p-3 mb-4">
      <div class="flex items-center space-x-2 text-sm">
        <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span class="text-gray-500">计划:</span>
        <span class="ml-2 text-gray-900 font-medium">{{ scheduleText }}</span>
      </div>
      <div v-if="backup.next_run" class="flex items-center space-x-2 text-sm mt-2">
        <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        <span class="text-gray-500">下次执行:</span>
        <span class="ml-2 text-gray-900">{{ formatDateTime(backup.next_run) }}</span>
      </div>
    </div>

    <!-- 最后执行信息 -->
    <div v-if="backup.last_run" class="mb-4 text-sm">
      <div class="flex justify-between">
        <span class="text-gray-500">最后执行:</span>
        <span class="text-gray-900">{{ formatDateTime(backup.last_run) }}</span>
      </div>
      <div v-if="backup.last_status" class="flex justify-between mt-1">
        <span class="text-gray-500">最后状态:</span>
        <span :class="lastStatusClass">{{ lastStatusText }}</span>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex justify-end space-x-2 pt-4 border-t">
      <button
        @click="$emit('execute', backup)"
        class="btn-secondary text-sm py-1.5 px-3 flex items-center space-x-1"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span>执行</span>
      </button>
      <button
        @click="$emit('restore', backup)"
        class="btn-secondary text-sm py-1.5 px-3 flex items-center space-x-1"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
        </svg>
        <span>恢复</span>
      </button>
      <button
        @click="$emit('edit', backup)"
        class="btn-secondary text-sm py-1.5 px-3"
      >
        编辑
      </button>
      <button
        @click="$emit('delete', backup)"
        class="text-red-600 hover:text-red-900 text-sm py-1.5 px-3"
      >
        删除
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  backup: {
    id: string
    name: string
    type: string
    source_path: string
    destination_path: string
    schedule: string
    status: string
    last_run?: string
    last_status?: string
    next_run?: string
  }
}>()

defineEmits<{
  execute: [backup: any]
  restore: [backup: any]
  edit: [backup: any]
  delete: [backup: any]
}>()

// 状态样式
const statusClass = computed(() => {
  switch (props.backup.status) {
    case 'active':
      return 'bg-green-100 text-green-800'
    case 'inactive':
      return 'bg-gray-100 text-gray-800'
    case 'running':
      return 'bg-blue-100 text-blue-800'
    default:
      return 'bg-gray-100 text-gray-800'
  }
})

// 状态文本
const statusText = computed(() => {
  switch (props.backup.status) {
    case 'active':
      return '活跃'
    case 'inactive':
      return '非活跃'
    case 'running':
      return '运行中'
    default:
      return props.backup.status
  }
})

// 计划任务文本
const scheduleText = computed(() => {
  const schedule = props.backup.schedule
  if (!schedule) return '手动'
  
  const schedules: Record<string, string> = {
    'hourly': '每小时',
    'daily': '每天',
    'weekly': '每周',
    'monthly': '每月'
  }
  
  return schedules[schedule] || schedule
})

// 最后状态样式
const lastStatusClass = computed(() => {
  const status = props.backup.last_status?.toLowerCase()
  if (status === 'success' || status === 'completed') return 'text-green-600 font-medium'
  if (status === 'failed' || status === 'error') return 'text-red-600 font-medium'
  if (status === 'running') return 'text-blue-600 font-medium'
  return 'text-gray-600 font-medium'
})

// 最后状态文本
const lastStatusText = computed(() => {
  const status = props.backup.last_status
  if (!status) return '无'
  
  const statuses: Record<string, string> = {
    'success': '成功',
    'completed': '完成',
    'failed': '失败',
    'error': '错误',
    'running': '运行中'
  }
  
  return statuses[status.toLowerCase()] || status
})

// 格式化日期时间
const formatDateTime = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>
