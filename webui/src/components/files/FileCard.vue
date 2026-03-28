<template>
  <div
    class="card hover:shadow-lg transition-shadow cursor-pointer group"
    @dblclick="$emit('open', file)"
  >
    <!-- 文件图标和名称 -->
    <div class="flex items-start space-x-3 mb-3">
      <div class="flex-shrink-0">
        <!-- 文件夹图标 -->
        <div v-if="file.is_folder" class="w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center">
          <svg class="w-8 h-8 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
        </div>
        <!-- 文件类型图标 -->
        <div v-else class="w-12 h-12 bg-gray-100 rounded-lg flex items-center justify-center">
          <span class="text-2xl">{{ fileIcon }}</span>
        </div>
      </div>
      <div class="flex-1 min-w-0">
        <h3 class="text-sm font-medium text-gray-900 truncate" :title="file.name">
          {{ file.name }}
        </h3>
        <p class="text-xs text-gray-500 mt-1">
          {{ formatFileSize(file.size_bytes) }}
        </p>
        <p class="text-xs text-gray-400 mt-1">
          {{ formatDate(file.created_at) }}
        </p>
      </div>
    </div>

    <!-- 操作按钮 (hover 时显示) -->
    <div class="flex justify-end space-x-2 pt-3 border-t opacity-0 group-hover:opacity-100 transition-opacity">
      <button
        v-if="!file.is_folder"
        @click.stop="$emit('download', file)"
        class="p-1.5 text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
        title="下载"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
        </svg>
      </button>
      <button
        @click.stop="$emit('open', file)"
        class="p-1.5 text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
        title="打开"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
        </svg>
      </button>
      <button
        @click.stop="$emit('rename', file)"
        class="p-1.5 text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
        title="重命名"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
        </svg>
      </button>
      <button
        @click.stop="$emit('delete', file)"
        class="p-1.5 text-gray-600 hover:text-red-600 hover:bg-red-50 rounded transition-colors"
        title="删除"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  file: {
    id: string
    name: string
    path: string
    size_bytes: number
    is_folder: boolean
    file_type?: string
    created_at: string
    updated_at?: string
  }
}>()

defineEmits<{
  open: [file: any]
  download: [file: any]
  delete: [file: any]
  rename: [file: any, newName: string]
}>()

// 文件图标映射
const fileIcon = computed(() => {
  const type = props.file.file_type
  switch (type) {
    case 'image':
      return '🖼️'
    case 'document':
      return '📄'
    case 'video':
      return '🎬'
    case 'audio':
      return '🎵'
    case 'archive':
      return '📦'
    default:
      return '📎'
  }
})

// 格式化文件大小
const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

// 格式化日期
const formatDate = (dateString: string): string => {
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
