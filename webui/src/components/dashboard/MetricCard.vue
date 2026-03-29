<template>
  <div class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center space-x-3">
        <div :class="iconBgClass" class="w-12 h-12 rounded-lg flex items-center justify-center">
          <component :is="icon" class="w-6 h-6" :class="iconClass" />
        </div>
        <div>
          <h3 class="text-sm font-medium text-gray-500">{{ title }}</h3>
          <p class="text-2xl font-bold text-gray-900">{{ formattedValue }}</p>
        </div>
      </div>
      <div v-if="trend !== null" :class="trendClass">
        <svg v-if="trend > 0" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" />
        </svg>
        <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
        </svg>
        <span>{{ Math.abs(trend) }}%</span>
      </div>
    </div>

    <!-- 进度条（可选） -->
    <div v-if="showProgress" class="mt-4">
      <div class="flex justify-between text-sm text-gray-500 mb-1">
        <span>{{ progressLabel }}</span>
        <span>{{ progressPercent }}%</span>
      </div>
      <div class="w-full bg-gray-200 rounded-full h-2">
        <div
          :class="progressBarClass"
          class="h-2 rounded-full transition-all duration-500"
          :style="{ width: `${progressPercent}%` }"
        ></div>
      </div>
    </div>

    <!-- 子信息 -->
    <div v-if="subInfo" class="mt-4 text-sm text-gray-500">
      {{ subInfo }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, h } from 'vue'

const props = defineProps<{
  title: string
  value: number | string
  unit?: string
  type?: 'cpu' | 'memory' | 'disk' | 'network' | 'services' | 'temperature' | 'default'
  trend?: number | null
  showProgress?: boolean
  progressValue?: number
  progressMax?: number
  progressLabel?: string
  subInfo?: string
}>()

// 格式化值
const formattedValue = computed(() => {
  if (typeof props.value === 'string') return props.value
  if (props.unit === 'bytes') {
    return formatBytes(props.value)
  }
  if (props.unit === 'percent') {
    return `${props.value.toFixed(1)}%`
  }
  if (props.unit === 'celsius') {
    return `${props.value}°C`
  }
  return props.value.toString()
})

// 进度百分比
const progressPercent = computed(() => {
  if (!props.progressMax) return 0
  return Math.min(100, Math.round((props.progressValue || 0) / props.progressMax * 100))
})

// 图标组件
const CpuIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z' })
])

const MemoryIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10' })
])

const DiskIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4' })
])

const NetworkIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.156-3.905 14.06 0M1.394 9.393c5.583-5.587 14.629-5.587 20.212 0' })
])

const ServicesIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01' })
])

const TemperatureIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z' })
])

// 图标选择
const icon = computed(() => {
  switch (props.type) {
    case 'cpu': return CpuIcon
    case 'memory': return MemoryIcon
    case 'disk': return DiskIcon
    case 'network': return NetworkIcon
    case 'services': return ServicesIcon
    case 'temperature': return TemperatureIcon
    default: return CpuIcon
  }
})

// 图标背景色
const iconBgClass = computed(() => {
  switch (props.type) {
    case 'cpu': return 'bg-blue-100'
    case 'memory': return 'bg-purple-100'
    case 'disk': return 'bg-green-100'
    case 'network': return 'bg-orange-100'
    case 'services': return 'bg-indigo-100'
    case 'temperature': return 'bg-red-100'
    default: return 'bg-gray-100'
  }
})

// 图标颜色
const iconClass = computed(() => {
  switch (props.type) {
    case 'cpu': return 'text-blue-600'
    case 'memory': return 'text-purple-600'
    case 'disk': return 'text-green-600'
    case 'network': return 'text-orange-600'
    case 'services': return 'text-indigo-600'
    case 'temperature': return 'text-red-600'
    default: return 'text-gray-600'
  }
})

// 趋势样式
const trendClass = computed(() => {
  if (props.trend === null) return ''
  return props.trend > 0
    ? 'flex items-center text-red-500 text-sm'
    : 'flex items-center text-green-500 text-sm'
})

// 进度条颜色
const progressBarClass = computed(() => {
  const percent = progressPercent.value
  if (percent >= 90) return 'bg-red-500'
  if (percent >= 70) return 'bg-yellow-500'
  return 'bg-green-500'
})

// 格式化字节
const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}
</script>