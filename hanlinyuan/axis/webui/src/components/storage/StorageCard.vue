<template>
  <div class="card hover:shadow-lg transition-shadow">
    <!-- 标题栏 -->
    <div class="flex justify-between items-start mb-4">
      <div class="flex items-center space-x-3">
        <div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
          </svg>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900">{{ volume.name }}</h3>
          <p class="text-sm text-gray-500">{{ volume.path }}</p>
        </div>
      </div>
      <span
        :class="[
          'px-2 py-1 text-xs font-medium rounded-full',
          volume.status === 'active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
        ]"
      >
        {{ volume.status === 'active' ? '活跃' : '非活跃' }}
      </span>
    </div>

    <!-- 使用率进度条 -->
    <div class="mb-4">
      <div class="flex justify-between text-sm mb-1">
        <span class="text-gray-600">使用率</span>
        <span class="font-medium text-gray-900">{{ usagePercent }}%</span>
      </div>
      <div class="w-full bg-gray-200 rounded-full h-2">
        <div
          :class="[
            'h-2 rounded-full transition-all',
            usagePercent > 90 ? 'bg-red-500' : usagePercent > 70 ? 'bg-yellow-500' : 'bg-primary-500'
          ]"
          :style="{ width: usagePercent + '%' }"
        />
      </div>
    </div>

    <!-- 详细信息 -->
    <div class="grid grid-cols-2 gap-4 text-sm">
      <div>
        <p class="text-gray-500">总容量</p>
        <p class="font-medium text-gray-900">{{ formatSize(volume.total_bytes) }}</p>
      </div>
      <div>
        <p class="text-gray-500">已使用</p>
        <p class="font-medium text-gray-900">{{ formatSize(volume.used_bytes) }}</p>
      </div>
      <div>
        <p class="text-gray-500">可用空间</p>
        <p class="font-medium text-gray-900">{{ formatSize(volume.available_bytes) }}</p>
      </div>
      <div>
        <p class="text-gray-500">文件系统</p>
        <p class="font-medium text-gray-900">{{ volume.filesystem_type || 'N/A' }}</p>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex justify-end space-x-2 mt-4 pt-4 border-t">
      <button class="btn-secondary text-sm py-1.5 px-3">
        详情
      </button>
      <button class="btn-secondary text-sm py-1.5 px-3">
        编辑
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  volume: {
    id: string
    name: string
    path: string
    total_bytes: number
    used_bytes: number
    available_bytes: number
    status: string
    filesystem_type?: string
  }
}>()

// 计算使用率百分比
const usagePercent = computed(() => {
  if (!props.volume.total_bytes) return 0
  return Math.round((props.volume.used_bytes / props.volume.total_bytes) * 100)
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
