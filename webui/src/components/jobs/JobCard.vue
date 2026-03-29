<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow border border-gray-200">
    <!-- 卡片头部 -->
    <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <!-- 任务类型图标 -->
        <div :class="typeIconClass" class="w-10 h-10 rounded-lg flex items-center justify-center">
          <component :is="typeIcon" class="w-5 h-5" />
        </div>
        <div>
          <h3 class="font-semibold text-gray-900">{{ job.name || job.document_name || `任务 #${job.id}` }}</h3>
          <p class="text-sm text-gray-500">{{ typeLabel }}</p>
        </div>
      </div>
      <!-- 状态标签 -->
      <span :class="statusClass" class="px-2.5 py-1 text-xs font-medium rounded-full">
        {{ statusLabel }}
      </span>
    </div>

    <!-- 卡片内容 -->
    <div class="px-4 py-3 space-y-2">
      <!-- 任务详情 -->
      <div v-if="job.command" class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        <code class="text-xs bg-gray-100 px-1 rounded">{{ job.command }}</code>
      </div>

      <!-- 打印任务详情 -->
      <div v-if="jobType === 'print' && job.pages" class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
        </svg>
        <span>{{ job.pages }} 页 × {{ job.copies || 1 }} 份</span>
      </div>

      <!-- 调度时间 -->
      <div v-if="job.schedule" class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span class="font-mono text-xs">{{ job.schedule }}</span>
      </div>

      <!-- 进度条（进行中任务） -->
      <div v-if="job.status === 'printing' || job.status === 'running'" class="mt-2">
        <div class="flex justify-between text-xs text-gray-500 mb-1">
          <span>进度</span>
          <span>{{ job.progress || 50 }}%</span>
        </div>
        <div class="w-full bg-gray-200 rounded-full h-1.5">
          <div
            class="bg-blue-500 h-1.5 rounded-full transition-all"
            :style="{ width: (job.progress || 50) + '%' }"
          ></div>
        </div>
      </div>

      <!-- 时间信息 -->
      <div class="flex items-center justify-between text-xs text-gray-400 pt-2 border-t border-gray-100">
        <span>
          {{ job.submitted_at || job.created_at ? `创建: ${formatTime(job.submitted_at || job.created_at)}` : '' }}
        </span>
        <span v-if="job.next_run">
          下次: {{ formatTime(job.next_run) }}
        </span>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="px-4 py-3 bg-gray-50 rounded-b-lg flex justify-end space-x-2">
      <button
        @click="$emit('detail', job)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
      >
        详情
      </button>
      <button
        v-if="canCancel"
        @click="$emit('cancel', job)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-red-600 hover:bg-red-50 rounded transition-colors"
      >
        取消
      </button>
      <button
        v-if="canRetry"
        @click="$emit('retry', job)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-green-600 hover:bg-green-50 rounded transition-colors"
      >
        重试
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, h } from 'vue'

const props = defineProps<{
  job: any
  jobType: 'print' | 'cron' | 'backup' | 'system'
}>()

defineEmits<{
  detail: [job: any]
  cancel: [job: any]
  retry: [job: any]
}>()

// 任务类型图标
const PrintIcon = () => h('svg', { class: 'w-5 h-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z' })
])

const CronIcon = () => h('svg', { class: 'w-5 h-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' })
])

const BackupIcon = () => h('svg', { class: 'w-5 h-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12' })
])

const SystemIcon = () => h('svg', { class: 'w-5 h-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }),
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' })
])

// 图标选择
const typeIcon = computed(() => {
  switch (props.jobType) {
    case 'print': return PrintIcon
    case 'cron': return CronIcon
    case 'backup': return BackupIcon
    case 'system': return SystemIcon
    default: return SystemIcon
  }
})

// 类型图标背景色
const typeIconClass = computed(() => {
  switch (props.jobType) {
    case 'print': return 'bg-purple-100 text-purple-600'
    case 'cron': return 'bg-orange-100 text-orange-600'
    case 'backup': return 'bg-green-100 text-green-600'
    case 'system': return 'bg-blue-100 text-blue-600'
    default: return 'bg-gray-100 text-gray-600'
  }
})

// 类型标签
const typeLabel = computed(() => {
  switch (props.jobType) {
    case 'print': return '打印任务'
    case 'cron': return '定时任务'
    case 'backup': return '备份任务'
    case 'system': return '系统任务'
    default: return '任务'
  }
})

// 状态样式
const statusClass = computed(() => {
  const status = props.job.status
  switch (status) {
    case 'pending':
    case 'queued':
      return 'bg-gray-100 text-gray-700'
    case 'printing':
    case 'running':
    case 'active':
      return 'bg-blue-100 text-blue-700'
    case 'completed':
    case 'success':
      return 'bg-green-100 text-green-700'
    case 'failed':
    case 'error':
      return 'bg-red-100 text-red-700'
    case 'canceled':
    case 'cancelled':
      return 'bg-yellow-100 text-yellow-700'
    case 'inactive':
      return 'bg-gray-100 text-gray-500'
    default:
      return 'bg-gray-100 text-gray-700'
  }
})

// 状态标签
const statusLabel = computed(() => {
  const status = props.job.status
  switch (status) {
    case 'pending':
    case 'queued':
      return '排队中'
    case 'printing':
    case 'running':
      return '进行中'
    case 'active':
      return '已激活'
    case 'completed':
    case 'success':
      return '已完成'
    case 'failed':
    case 'error':
      return '失败'
    case 'canceled':
    case 'cancelled':
      return '已取消'
    case 'inactive':
      return '未激活'
    default:
      return status
  }
})

// 是否可取消
const canCancel = computed(() => {
  const status = props.job.status
  return ['pending', 'queued', 'printing', 'running'].includes(status)
})

// 是否可重试
const canRetry = computed(() => {
  const status = props.job.status
  return ['failed', 'error', 'canceled', 'cancelled'].includes(status)
})

// 格式化时间
const formatTime = (timestamp: number | string) => {
  if (!timestamp) return '-'
  const date = typeof timestamp === 'number' 
    ? (timestamp > 9999999999 ? new Date(timestamp) : new Date(timestamp * 1000))
    : new Date(timestamp)
  return date.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>