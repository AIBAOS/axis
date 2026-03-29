<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow border border-gray-200">
    <!-- 标题栏 -->
    <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <!-- 磁盘类型图标 -->
        <div :class="typeIconClass" class="w-12 h-12 rounded-lg flex items-center justify-center">
          <component :is="typeIcon" class="w-6 h-6" />
        </div>
        <div>
          <h3 class="font-semibold text-gray-900">{{ disk.name }}</h3>
          <p class="text-sm text-gray-500 truncate" :title="disk.model">{{ disk.model || '未知型号' }}</p>
        </div>
      </div>
      <!-- 状态标签 -->
      <span :class="statusClass" class="px-2.5 py-1 text-xs font-medium rounded-full">
        {{ statusLabel }}
      </span>
    </div>

    <!-- 卡片内容 -->
    <div class="px-4 py-3 space-y-3">
      <!-- 容量 -->
      <div class="flex items-center justify-between">
        <span class="text-sm text-gray-600">容量</span>
        <span class="font-medium text-gray-900">{{ disk.size_human || formatSize(disk.size_bytes) }}</span>
      </div>

      <!-- 类型 -->
      <div class="flex items-center justify-between">
        <span class="text-sm text-gray-600">类型</span>
        <span class="text-sm text-gray-900">{{ diskTypeLabel }}</span>
      </div>

      <!-- 温度 -->
      <div class="flex items-center justify-between">
        <span class="text-sm text-gray-600">温度</span>
        <div class="flex items-center space-x-1">
          <span :class="temperatureClass" class="font-medium">
            {{ disk.temperature || '-' }}°C
          </span>
          <svg v-if="disk.temperature && disk.temperature > 45" class="w-4 h-4 text-red-500" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M12.395 2.553a1 1 0 00-1.45-.385c-.345.23-.614.558-.822.88-.214.33-.403.713-.57 1.116-.334.804-.614 1.768-.84 2.734a31.365 31.365 0 00-.613 3.58 2.64 2.64 0 01-.945-1.067c-.328-.68-.398-1.534-.398-2.654A1 1 0 005.05 6.05 6.981 6.981 0 003 11a7 7 0 1011.95-4.95c-.592-.591-.98-.985-1.348-1.467-.363-.476-.724-1.063-1.207-2.03zM12.12 15.12A3 3 0 017 13s.879.5 2.5.5c0-1 .5-4 1.25-4.5.5 1 .786 1.293 1.371 1.879A2.99 2.99 0 0113 13a2.99 2.99 0 01-.879 2.121z" clip-rule="evenodd" />
          </svg>
        </div>
      </div>

      <!-- 健康状态 -->
      <div class="flex items-center justify-between">
        <span class="text-sm text-gray-600">S.M.A.R.T.</span>
        <span :class="smartClass" class="px-2 py-0.5 text-xs rounded-full font-medium">
          {{ smartLabel }}
        </span>
      </div>

      <!-- 使用状态 -->
      <div v-if="disk.in_use !== undefined" class="flex items-center justify-between">
        <span class="text-sm text-gray-600">使用状态</span>
        <span :class="disk.in_use ? 'text-blue-600' : 'text-gray-500'" class="text-sm">
          {{ disk.in_use ? '使用中' : '空闲' }}
        </span>
      </div>

      <!-- 通电时间 -->
      <div v-if="disk.power_on_hours" class="flex items-center justify-between">
        <span class="text-sm text-gray-600">通电时间</span>
        <span class="text-sm text-gray-900">{{ formatPowerOnHours(disk.power_on_hours) }}</span>
      </div>
    </div>

    <!-- 详细信息 -->
    <div class="px-4 py-2 bg-gray-50 text-xs">
      <div class="grid grid-cols-2 gap-2">
        <div>
          <span class="text-gray-400">序列号:</span>
          <span class="ml-1 text-gray-600 font-mono truncate">{{ disk.serial_number || 'N/A' }}</span>
        </div>
        <div>
          <span class="text-gray-400">路径:</span>
          <span class="ml-1 text-gray-600 font-mono">{{ disk.path || 'N/A' }}</span>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="px-4 py-3 flex justify-end space-x-2">
      <button
        @click="$emit('smart', disk)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
      >
        S.M.A.R.T.
      </button>
      <button
        @click="$emit('detail', disk)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
      >
        详情
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, h } from 'vue'

const props = defineProps<{
  disk: {
    id: string | number
    name: string
    model?: string
    serial_number?: string
    type?: string
    disk_type?: string
    size_bytes: number
    size_human?: string
    temperature?: number
    smart_status?: string
    health_status?: string
    status?: string
    path?: string
    in_use?: boolean
    power_on_hours?: number
    speed_rpm?: number
  }
}>()

defineEmits<{
  smart: [disk: any]
  detail: [disk: any]
}>()

// 磁盘类型图标
const HddIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2' })
])

const SsdIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z' })
])

const NvmeIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 10V3L4 14h7v7l9-11h-7z' })
])

// 类型图标
const typeIcon = computed(() => {
  const type = (props.disk.type || props.disk.disk_type || '').toLowerCase()
  if (type === 'nvme') return NvmeIcon
  if (type === 'ssd') return SsdIcon
  return HddIcon
})

// 类型图标背景色
const typeIconClass = computed(() => {
  const type = (props.disk.type || props.disk.disk_type || '').toLowerCase()
  if (type === 'nvme') return 'bg-purple-100 text-purple-600'
  if (type === 'ssd') return 'bg-blue-100 text-blue-600'
  return 'bg-gray-100 text-gray-600'
})

// 类型标签
const diskTypeLabel = computed(() => {
  const type = (props.disk.type || props.disk.disk_type || '').toLowerCase()
  if (type === 'nvme') return 'NVMe SSD'
  if (type === 'ssd') return '固态硬盘'
  if (type === 'hdd') return '机械硬盘'
  return type.toUpperCase() || '未知'
})

// 状态样式
const statusClass = computed(() => {
  const status = props.disk.status?.toLowerCase()
  if (status === 'online') return 'bg-green-100 text-green-700'
  if (status === 'offline') return 'bg-gray-100 text-gray-700'
  if (status === 'error') return 'bg-red-100 text-red-700'
  return 'bg-gray-100 text-gray-700'
})

// 状态标签
const statusLabel = computed(() => {
  const status = props.disk.status?.toLowerCase()
  if (status === 'online') return '在线'
  if (status === 'offline') return '离线'
  if (status === 'error') return '错误'
  return status || '未知'
})

// 温度样式
const temperatureClass = computed(() => {
  const temp = props.disk.temperature
  if (!temp) return 'text-gray-900'
  if (temp > 50) return 'text-red-600'
  if (temp > 40) return 'text-yellow-600'
  return 'text-green-600'
})

// SMART 样式
const smartClass = computed(() => {
  const status = (props.disk.smart_status || props.disk.health_status || '').toLowerCase()
  if (['healthy', 'good'].includes(status)) return 'bg-green-100 text-green-700'
  if (['warning', 'caution'].includes(status)) return 'bg-yellow-100 text-yellow-700'
  if (['failed', 'bad'].includes(status)) return 'bg-red-100 text-red-700'
  return 'bg-gray-100 text-gray-700'
})

// SMART 标签
const smartLabel = computed(() => {
  const status = (props.disk.smart_status || props.disk.health_status || '').toLowerCase()
  if (['healthy', 'good'].includes(status)) return '健康'
  if (['warning', 'caution'].includes(status)) return '警告'
  if (['failed', 'bad'].includes(status)) return '故障'
  return '未知'
})

// 格式化大小
const formatSize = (bytes: number) => {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

// 格式化通电时间
const formatPowerOnHours = (hours: number) => {
  if (!hours) return '-'
  const days = Math.floor(hours / 24)
  const h = hours % 24
  if (days > 365) {
    const years = Math.floor(days / 365)
    const d = days % 365
    return `${years} 年 ${d} 天`
  }
  if (days > 0) return `${days} 天 ${h} 时`
  return `${h} 小时`
}
</script>