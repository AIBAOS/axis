<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow border border-gray-200">
    <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <div :class="statusIconClass" class="w-10 h-10 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg>
        </div>
        <div><h3 class="font-semibold text-gray-900">{{ pool.name }}</h3><p class="text-sm text-gray-500">{{ raidLabel }}</p></div>
      </div>
      <span :class="statusClass" class="px-2 py-0.5 text-xs rounded-full">{{ statusLabel }}</span>
    </div>

    <div class="px-4 py-3 space-y-3">
      <div class="flex items-center justify-between text-sm"><span class="text-gray-500">容量</span><span class="font-medium text-gray-900">{{ formatBytes(pool.size_bytes) }}</span></div>
      <div class="flex items-center justify-between text-sm"><span class="text-gray-500">已用</span><span class="font-medium text-gray-900">{{ formatBytes(pool.used_bytes) }}</span></div>
      <div><div class="flex justify-between text-sm mb-1"><span class="text-gray-500">使用率</span><span class="font-medium">{{ usagePercent }}%</span></div><div class="w-full bg-gray-200 rounded-full h-2"><div :class="usagePercent > 90 ? 'bg-red-500' : usagePercent > 70 ? 'bg-yellow-500' : 'bg-green-500'" class="h-2 rounded-full" :style="{ width: usagePercent + '%' }"></div></div></div>
      <div class="flex items-center text-sm text-gray-600"><svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2" /></svg><span>{{ pool.disk_ids?.length || 0 }} 块磁盘</span></div>
    </div>

    <div class="px-4 py-3 bg-gray-50 rounded-b-lg flex justify-end space-x-2">
      <button @click="$emit('edit', pool)" class="px-3 py-1.5 text-sm text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded">配置</button>
      <button @click="$emit('delete', pool)" class="px-3 py-1.5 text-sm text-gray-600 hover:text-red-600 hover:bg-red-50 rounded">删除</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{ pool: { id: number; name: string; raid_level?: string; size_bytes: number; used_bytes: number; status?: string; disk_ids?: number[] } }>()
defineEmits<{ edit: [pool: any]; delete: [pool: any] }>()

const usagePercent = computed(() => props.pool.size_bytes > 0 ? Math.round(props.pool.used_bytes / props.pool.size_bytes * 100) : 0)
const raidLabel = computed(() => ({ single: '单盘', raid0: 'RAID 0', raid1: 'RAID 1', raid5: 'RAID 5', raid10: 'RAID 10' }[props.pool.raid_level] || props.pool.raid_level?.toUpperCase() || '单盘'))
const statusClass = computed(() => props.pool.status === 'online' ? 'bg-green-100 text-green-700' : props.pool.status === 'degraded' ? 'bg-yellow-100 text-yellow-700' : 'bg-red-100 text-red-700')
const statusLabel = computed(() => props.pool.status === 'online' ? '在线' : props.pool.status === 'degraded' ? '降级' : '离线')
const statusIconClass = computed(() => props.pool.status === 'online' ? 'bg-green-100 text-green-600' : props.pool.status === 'degraded' ? 'bg-yellow-100 text-yellow-600' : 'bg-red-100 text-red-600')

const formatBytes = (b: number) => { if (!b) return '0 B'; const k = 1024; const s = ['B', 'KB', 'MB', 'GB', 'TB']; const i = Math.floor(Math.log(b) / Math.log(k)); return (b / Math.pow(k, i)).toFixed(1) + ' ' + s[i] }
</script>