<template>
  <div class="card hover:shadow-lg transition-shadow">
    <!-- 标题栏 -->
    <div class="flex justify-between items-start mb-4">
      <div class="flex items-center space-x-3">
        <div class="w-10 h-10 bg-gray-100 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
          </svg>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900">{{ disk.name }}</h3>
          <p class="text-sm text-gray-500">{{ disk.model || '未知型号' }}</p>
        </div>
      </div>
      <span
        :class="[
          'px-2 py-1 text-xs font-medium rounded-full',
          disk.status === 'online' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
        ]"
      >
        {{ disk.status === 'online' ? '在线' : '离线' }}
      </span>
    </div>

    <!-- 详细信息 -->
    <div class="grid grid-cols-2 gap-4 text-sm mb-4">
      <div>
        <p class="text-gray-500">类型</p>
        <p class="font-medium text-gray-900">{{ diskTypeText }}</p>
      </div>
      <div>
        <p class="text-gray-500">容量</p>
        <p class="font-medium text-gray-900">{{ formatSize(disk.size_bytes) }}</p>
      </div>
      <div>
        <p class="text-gray-500">温度</p>
        <p :class="temperatureClass">{{ disk.temperature }}°C</p>
      </div>
      <div>
        <p class="text-gray-500">健康状态</p>
        <p :class="healthClass">{{ disk.smart_status || '未知' }}</p>
      </div>
    </div>

    <!-- 附加信息 -->
    <div class="bg-gray-50 rounded-lg p-3 text-sm">
      <div class="grid grid-cols-2 gap-2">
        <div>
          <span class="text-gray-500">序列号:</span>
          <span class="ml-2 text-gray-900 font-mono text-xs">{{ disk.serial_number || 'N/A' }}</span>
        </div>
        <div>
          <span class="text-gray-500">路径:</span>
          <span class="ml-2 text-gray-900 font-mono text-xs">{{ disk.path || 'N/A' }}</span>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex justify-end space-x-2 mt-4 pt-4 border-t">
      <button class="btn-secondary text-sm py-1.5 px-3">
        S.M.A.R.T.
      </button>
      <button class="btn-secondary text-sm py-1.5 px-3">
        详情
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  disk: {
    id: string
    name: string
    model?: string
    serial_number?: string
    disk_type?: string
    size_bytes: number
    temperature?: number
    smart_status?: string
    health_status?: string
    status: string
    path?: string
  }
}>()

// 磁盘类型文本
const diskTypeText = computed(() => {
  const type = props.disk.disk_type?.toLowerCase()
  switch (type) {
    case 'hdd':
      return '机械硬盘'
    case 'ssd':
      return '固态硬盘'
    case 'nvme':
      return 'NVMe SSD'
    default:
      return props.disk.disk_type || '未知'
  }
})

// 温度样式
const temperatureClass = computed(() => {
  const temp = props.disk.temperature
  if (!temp) return 'font-medium text-gray-900'
  if (temp > 50) return 'font-medium text-red-600'
  if (temp > 40) return 'font-medium text-yellow-600'
  return 'font-medium text-green-600'
})

// 健康状态样式
const healthClass = computed(() => {
  const status = props.disk.smart_status?.toLowerCase()
  if (status === 'healthy' || status === 'good') return 'font-medium text-green-600'
  if (status === 'warning' || status === 'caution') return 'font-medium text-yellow-600'
  if (status === 'failed' || status === 'bad') return 'font-medium text-red-600'
  return 'font-medium text-gray-900'
})

// 格式化文件大小
const formatSize = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}
</script>
