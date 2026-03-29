<template>
  <div class="card hover:shadow-lg transition-shadow">
    <!-- 标题栏 -->
    <div class="flex justify-between items-start mb-4">
      <div class="flex items-center space-x-3">
        <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
          </svg>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900">{{ pool.name }}</h3>
          <p class="text-sm text-gray-500">{{ pool.type || '标准池' }}</p>
        </div>
      </div>
      <span
        :class="[
          'px-2 py-1 text-xs font-medium rounded-full',
          pool.status === 'healthy' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'
        ]"
      >
        {{ pool.status === 'healthy' ? '健康' : '警告' }}
      </span>
    </div>

    <!-- 磁盘数量 -->
    <div class="mb-4">
      <div class="flex items-center space-x-2 text-sm text-gray-600">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
        <span>{{ pool.disk_count || 0 }} 块磁盘</span>
      </div>
    </div>

    <!-- 详细信息 -->
    <div class="grid grid-cols-2 gap-4 text-sm">
      <div>
        <p class="text-gray-500">总容量</p>
        <p class="font-medium text-gray-900">{{ formatSize(pool.total_bytes) }}</p>
      </div>
      <div>
        <p class="text-gray-500">已使用</p>
        <p class="font-medium text-gray-900">{{ formatSize(pool.used_bytes) }}</p>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex justify-end space-x-2 mt-4 pt-4 border-t">
      <button class="btn-secondary text-sm py-1.5 px-3">
        详情
      </button>
      <button class="btn-secondary text-sm py-1.5 px-3">
        管理磁盘
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  pool: {
    id: string
    name: string
    type?: string
    status: string
    total_bytes: number
    used_bytes: number
    disk_count?: number
  }
}>()

// 格式化文件大小
const formatSize = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}
</script>
