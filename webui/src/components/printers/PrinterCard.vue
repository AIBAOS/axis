<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow border border-gray-200">
    <!-- 卡片头部 -->
    <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <div :class="statusIconClass" class="w-12 h-12 rounded-lg flex items-center justify-center">
          <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" />
          </svg>
        </div>
        <div>
          <div class="flex items-center space-x-2">
            <h3 class="font-semibold text-gray-900">{{ printer.name }}</h3>
            <span v-if="printer.is_default" class="px-1.5 py-0.5 text-xs bg-primary-100 text-primary-700 rounded">默认</span>
          </div>
          <p class="text-sm text-gray-500">{{ printer.model || '未知型号' }}</p>
        </div>
      </div>
      <span :class="statusClass" class="px-2.5 py-1 text-xs font-medium rounded-full flex items-center space-x-1">
        <span v-if="statusIcon" class="w-1.5 h-1.5 rounded-full" :class="statusDotClass"></span>
        <span>{{ statusLabel }}</span>
      </span>
    </div>

    <!-- 卡片内容 -->
    <div class="px-4 py-3 space-y-2">
      <div v-if="printer.ip_address" class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H9m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
        </svg>
        <span class="font-mono">{{ printer.ip_address }}{{ printer.port ? `:${printer.port}` : '' }}</span>
      </div>

      <div class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
        </svg>
        <span>{{ typeLabel }}</span>
      </div>

      <div v-if="printer.location" class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
        <span>{{ printer.location }}</span>
      </div>

      <!-- 能力标签 -->
      <div v-if="printer.capabilities" class="flex flex-wrap gap-1 pt-2">
        <span v-if="printer.capabilities.color" class="px-2 py-0.5 text-xs bg-blue-50 text-blue-600 rounded">彩色</span>
        <span v-if="printer.capabilities.duplex" class="px-2 py-0.5 text-xs bg-green-50 text-green-600 rounded">双面</span>
        <span v-if="printer.capabilities.scanning" class="px-2 py-0.5 text-xs bg-purple-50 text-purple-600 rounded">扫描</span>
        <span v-if="printer.capabilities.fax" class="px-2 py-0.5 text-xs bg-orange-50 text-orange-600 rounded">传真</span>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="px-4 py-3 bg-gray-50 rounded-b-lg flex justify-end space-x-2">
      <button
        @click="$emit('detail', printer)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
      >
        详情
      </button>
      <button
        @click="$emit('test-print', printer)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded transition-colors"
      >
        测试页
      </button>
      <button
        @click="$emit('edit', printer)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
      >
        编辑
      </button>
      <button
        @click="$emit('delete', printer)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-red-600 hover:bg-red-50 rounded transition-colors"
      >
        删除
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  printer: {
    id: number
    name: string
    model?: string
    type?: string
    status: string
    ip_address?: string
    port?: number
    location?: string
    is_default?: boolean
    capabilities?: {
      color?: boolean
      duplex?: boolean
      scanning?: boolean
      fax?: boolean
    }
  }
}>()

defineEmits<{
  detail: [printer: any]
  testPrint: [printer: any]
  edit: [printer: any]
  delete: [printer: any]
}>()

// 状态图标背景色
const statusIconClass = computed(() => {
  switch (props.printer.status) {
    case 'idle': return 'bg-green-100 text-green-600'
    case 'printing': return 'bg-blue-100 text-blue-600'
    case 'error':
    case 'out_of_paper':
    case 'paper_jam': return 'bg-red-100 text-red-600'
    case 'offline': return 'bg-gray-100 text-gray-500'
    case 'warning': return 'bg-yellow-100 text-yellow-600'
    default: return 'bg-gray-100 text-gray-500'
  }
})

// 状态标签样式
const statusClass = computed(() => {
  switch (props.printer.status) {
    case 'idle': return 'bg-green-100 text-green-700'
    case 'printing': return 'bg-blue-100 text-blue-700'
    case 'error':
    case 'out_of_paper':
    case 'paper_jam': return 'bg-red-100 text-red-700'
    case 'offline': return 'bg-gray-100 text-gray-700'
    case 'warning': return 'bg-yellow-100 text-yellow-700'
    default: return 'bg-gray-100 text-gray-700'
  }
})

// 状态点样式
const statusDotClass = computed(() => {
  switch (props.printer.status) {
    case 'idle': return 'bg-green-500'
    case 'printing': return 'bg-blue-500 animate-pulse'
    case 'error':
    case 'out_of_paper':
    case 'paper_jam': return 'bg-red-500'
    case 'offline': return 'bg-gray-400'
    case 'warning': return 'bg-yellow-500'
    default: return 'bg-gray-400'
  }
})

// 状态图标
const statusIcon = computed(() => {
  return ['printing', 'error', 'out_of_paper', 'paper_jam'].includes(props.printer.status)
})

// 状态标签
const statusLabel = computed(() => {
  switch (props.printer.status) {
    case 'idle': return '空闲'
    case 'printing': return '打印中'
    case 'error': return '错误'
    case 'out_of_paper': return '缺纸'
    case 'paper_jam': return '卡纸'
    case 'offline': return '离线'
    case 'warning': return '警告'
    default: return '未知'
  }
})

// 类型标签
const typeLabel = computed(() => {
  switch (props.printer.type) {
    case 'network': return '网络打印机'
    case 'usb': return 'USB 打印机'
    case 'virtual': return '虚拟打印机'
    default: return '打印机'
  }
})
</script>