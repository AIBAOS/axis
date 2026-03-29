<template>
  <router-link
    :to="to"
    class="bg-white rounded-lg shadow-md p-5 hover:shadow-lg transition-all hover:scale-105 cursor-pointer block"
  >
    <div class="flex items-center space-x-4">
      <div :class="iconBgClass" class="w-12 h-12 rounded-lg flex items-center justify-center flex-shrink-0">
        <component :is="icon" class="w-6 h-6" :class="iconClass" />
      </div>
      <div class="flex-1 min-w-0">
        <h3 class="font-semibold text-gray-900 truncate">{{ title }}</h3>
        <p class="text-sm text-gray-500 truncate">{{ description }}</p>
      </div>
      <svg class="w-5 h-5 text-gray-400 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </div>
  </router-link>
</template>

<script setup lang="ts">
import { computed, h } from 'vue'

const props = defineProps<{
  title: string
  description: string
  to: string
  type?: 'files' | 'storage' | 'shares' | 'users' | 'backups' | 'logs' | 'settings' | 'default'
}>()

// 图标组件
const FilesIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z' })
])

const StorageIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4' })
])

const SharesIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z' })
])

const UsersIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })
])

const BackupsIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12' })
])

const LogsIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })
])

const SettingsIcon = () => h('svg', { class: 'w-6 h-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }),
  h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' })
])

// 图标选择
const icon = computed(() => {
  switch (props.type) {
    case 'files': return FilesIcon
    case 'storage': return StorageIcon
    case 'shares': return SharesIcon
    case 'users': return UsersIcon
    case 'backups': return BackupsIcon
    case 'logs': return LogsIcon
    case 'settings': return SettingsIcon
    default: return FilesIcon
  }
})

// 图标背景色
const iconBgClass = computed(() => {
  switch (props.type) {
    case 'files': return 'bg-blue-100'
    case 'storage': return 'bg-green-100'
    case 'shares': return 'bg-purple-100'
    case 'users': return 'bg-indigo-100'
    case 'backups': return 'bg-yellow-100'
    case 'logs': return 'bg-orange-100'
    case 'settings': return 'bg-gray-100'
    default: return 'bg-gray-100'
  }
})

// 图标颜色
const iconClass = computed(() => {
  switch (props.type) {
    case 'files': return 'text-blue-600'
    case 'storage': return 'text-green-600'
    case 'shares': return 'text-purple-600'
    case 'users': return 'text-indigo-600'
    case 'backups': return 'text-yellow-600'
    case 'logs': return 'text-orange-600'
    case 'settings': return 'text-gray-600'
    default: return 'text-gray-600'
  }
})
</script>