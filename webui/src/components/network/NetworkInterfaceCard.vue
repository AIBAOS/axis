<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow border border-gray-200">
    <!-- 卡片头部 -->
    <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <div :class="statusIconClass" class="w-12 h-12 rounded-lg flex items-center justify-center">
          <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path v-if="isWireless" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.156-3.905 14.06 0M1.394 9.393c5.583-5.587 14.629-5.587 20.212 0" />
            <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2" />
          </svg>
        </div>
        <div>
          <h3 class="font-semibold text-gray-900">{{ iface.name || iface.interface }}</h3>
          <p class="text-sm text-gray-500">{{ iface.interface }}</p>
        </div>
      </div>
      <span :class="statusClass" class="px-2.5 py-1 text-xs font-medium rounded-full flex items-center space-x-1">
        <span class="w-1.5 h-1.5 rounded-full" :class="statusDotClass"></span>
        <span>{{ statusLabel }}</span>
      </span>
    </div>

    <!-- 卡片内容 -->
    <div class="px-4 py-3 space-y-2">
      <!-- IP 地址 -->
      <div class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H9m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9-9a9 9 0 019 9" />
        </svg>
        <span class="font-mono">{{ iface.ip_address || '未分配' }}</span>
        <span v-if="iface.netmask" class="ml-2 text-gray-400">/ {{ iface.netmask }}</span>
      </div>

      <!-- MAC 地址 -->
      <div class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
        </svg>
        <span class="font-mono text-xs">{{ iface.mac_address || '-' }}</span>
      </div>

      <!-- 速度 -->
      <div v-if="iface.speed_mbps" class="flex items-center text-sm text-gray-600">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
        <span>{{ iface.speed_mbps }} Mbps</span>
      </div>

      <!-- DHCP 状态 -->
      <div class="flex items-center text-sm">
        <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span :class="iface.dhcp_enabled ? 'text-blue-600' : 'text-gray-600'">{{ iface.dhcp_enabled ? 'DHCP' : '静态 IP' }}</span>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="px-4 py-3 bg-gray-50 rounded-b-lg flex justify-end space-x-2">
      <button @click="$emit('edit', iface)" class="px-3 py-1.5 text-sm text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded transition-colors">配置</button>
      <button @click="$emit('test', iface)" class="px-3 py-1.5 text-sm text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded transition-colors">测试</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  interface: {
    id: number
    name?: string
    interface: string
    ip_address?: string
    netmask?: string
    gateway?: string
    mac_address?: string
    status: string
    interface_type?: string
    speed_mbps?: number
    mtu?: number
    dhcp_enabled?: boolean
  }
}>()

defineEmits<{
  edit: [iface: any]
  test: [iface: any]
}>()

const iface = computed(() => props.interface)

const isWireless = computed(() => props.interface.interface_type === 'wireless' || props.interface.interface?.startsWith('wlan'))

const statusIconClass = computed(() => {
  return props.interface.status === 'up' ? 'bg-green-100 text-green-600' : 'bg-gray-100 text-gray-500'
})

const statusClass = computed(() => {
  return props.interface.status === 'up' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'
})

const statusDotClass = computed(() => {
  return props.interface.status === 'up' ? 'bg-green-500' : 'bg-gray-400'
})

const statusLabel = computed(() => {
  return props.interface.status === 'up' ? '在线' : '离线'
})
</script>