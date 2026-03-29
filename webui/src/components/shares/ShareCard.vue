<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow duration-200 border border-gray-200">
    <!-- 卡片头部 -->
    <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <!-- 协议图标 -->
        <div :class="protocolIconClass" class="w-10 h-10 rounded-lg flex items-center justify-center">
          <component :is="protocolIcon" class="w-5 h-5" />
        </div>
        <div>
          <h3 class="font-semibold text-gray-900">{{ share.name }}</h3>
          <p class="text-sm text-gray-500">{{ share.path }}</p>
        </div>
      </div>
      <!-- 状态标签 -->
      <span :class="statusClass" class="px-2 py-1 text-xs font-medium rounded-full">
        {{ share.status || 'active' }}
      </span>
    </div>

    <!-- 卡片内容 -->
    <div class="px-4 py-3 space-y-2">
      <!-- 描述 -->
      <p v-if="share.description" class="text-sm text-gray-600 line-clamp-2">
        {{ share.description }}
      </p>

      <!-- 协议信息 -->
      <div class="flex items-center text-sm text-gray-500">
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.156-3.905 14.06 0M1.394 9.393c5.583-5.587 14.629-5.587 20.212 0" />
        </svg>
        <span>协议: {{ protocolLabel }}</span>
      </div>

      <!-- 公开/私有 -->
      <div class="flex items-center text-sm text-gray-500">
        <svg v-if="share.public" class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h-1.5a2.5 2.5 0 00-2.5-2.5V3.935" />
        </svg>
        <svg v-else class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
        </svg>
        <span>{{ share.public ? '公开访问' : '私有访问' }}</span>
      </div>

      <!-- NFS 特有信息：客户端 -->
      <div v-if="protocol === 'nfs' && share.clients?.length" class="text-sm text-gray-500">
        <span class="font-medium">客户端:</span>
        <span class="ml-1">{{ share.clients.map(c => c.network).join(', ') }}</span>
      </div>

      <!-- SMB 特有信息 -->
      <div v-if="protocol === 'smb'" class="flex items-center text-sm text-gray-500">
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
        </svg>
        <span>{{ share.guest_access ? '允许访客' : '禁止访客' }}</span>
        <span class="mx-2">|</span>
        <span>{{ share.read_only ? '只读' : '读写' }}</span>
      </div>

      <!-- 创建时间 -->
      <div class="text-xs text-gray-400 pt-2 border-t border-gray-100">
        创建于 {{ formatDate(share.created_at) }}
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="px-4 py-3 bg-gray-50 rounded-b-lg flex justify-end space-x-2">
      <button
        @click="$emit('edit', share)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors"
      >
        编辑
      </button>
      <button
        @click="$emit('delete', share)"
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-red-600 hover:bg-red-50 rounded transition-colors"
      >
        删除
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, h } from 'vue'

const props = defineProps<{
  share: any
  protocol: 'smb' | 'nfs' | 'webdav' | 'ftp'
}>()

const emit = defineEmits<{
  edit: [share: any]
  delete: [share: any]
}>()

// 协议图标组件
const SmbIcon = () => h('svg', { class: 'w-5 h-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4' })
])

const NfsIcon = () => h('svg', { class: 'w-5 h-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.156-3.905 14.06 0M1.394 9.393c5.583-5.587 14.629-5.587 20.212 0' })
])

const WebdavIcon = () => h('svg', { class: 'w-5 h-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H9m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9-9a9 9 0 019 9' })
])

const FtpIcon = () => h('svg', { class: 'w-5 h-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12' })
])

// 协议图标
const protocolIcon = computed(() => {
  switch (props.protocol) {
    case 'smb': return SmbIcon
    case 'nfs': return NfsIcon
    case 'webdav': return WebdavIcon
    case 'ftp': return FtpIcon
    default: return SmbIcon
  }
})

// 协议图标背景色
const protocolIconClass = computed(() => {
  switch (props.protocol) {
    case 'smb': return 'bg-blue-100 text-blue-600'
    case 'nfs': return 'bg-green-100 text-green-600'
    case 'webdav': return 'bg-purple-100 text-purple-600'
    case 'ftp': return 'bg-orange-100 text-orange-600'
    default: return 'bg-gray-100 text-gray-600'
  }
})

// 协议名称
const protocolLabel = computed(() => {
  switch (props.protocol) {
    case 'smb': return 'SMB/CIFS'
    case 'nfs': return 'NFS'
    case 'webdav': return 'WebDAV'
    case 'ftp': return 'FTP'
    default: return props.protocol.toUpperCase()
  }
})

// 状态样式
const statusClass = computed(() => {
  const status = props.share.status || 'active'
  switch (status) {
    case 'active': return 'bg-green-100 text-green-800'
    case 'inactive': return 'bg-gray-100 text-gray-800'
    case 'error': return 'bg-red-100 text-red-800'
    default: return 'bg-gray-100 text-gray-800'
  }
})

// 格式化日期
const formatDate = (timestamp: number | string) => {
  if (!timestamp) return '未知'
  const date = typeof timestamp === 'number' ? new Date(timestamp * 1000) : new Date(timestamp)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}
</script>