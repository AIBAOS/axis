<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        网络管理
      </h1>
      <button
        @click="saveAll"
        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
      >
        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4"></path>
        </svg>
        保存全部
      </button>
    </div>

    <!-- 网络状态概览 -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">网络状态</p>
            <p :class="networkStatus.online ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'" class="text-2xl font-semibold">
              {{ networkStatus.online ? '在线' : '离线' }}
            </p>
          </div>
          <svg class="w-12 h-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.236-3.905 14.141 0M1.394 9.393c5.857-5.857 15.355-5.857 21.213 0"></path>
          </svg>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">IP 地址</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ networkStatus.ip }}</p>
          </div>
          <svg class="w-12 h-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"></path>
          </svg>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">下载速度</p>
            <p class="text-2xl font-semibold text-green-600 dark:text-green-400">{{ networkStatus.downloadSpeed }}</p>
          </div>
          <svg class="w-12 h-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
          </svg>
        </div>
      </div>
    </div>

    <!-- 网络接口配置 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h2 class="text-lg font-medium text-gray-900 dark:text-white mb-4">网络接口配置</h2>
      <div class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">IP 地址</label>
            <input
              v-model="networkConfig.ip"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">子网掩码</label>
            <input
              v-model="networkConfig.netmask"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">网关</label>
            <input
              v-model="networkConfig.gateway"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">DHCP</label>
            <select
              v-model="networkConfig.dhcp"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option :value="true">启用</option>
              <option :value="false">禁用</option>
            </select>
          </div>
        </div>
      </div>
    </div>

    <!-- DNS 设置 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h2 class="text-lg font-medium text-gray-900 dark:text-white mb-4">DNS 设置</h2>
      <div class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">首选 DNS</label>
            <input
              v-model="dnsConfig.primary"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">备用 DNS</label>
            <input
              v-model="dnsConfig.secondary"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- 防火墙规则 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-medium text-gray-900 dark:text-white">防火墙规则</h2>
        <button
          @click="addFirewallRule"
          class="inline-flex items-center px-3 py-1.5 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
        >
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
          </svg>
          添加规则
        </button>
      </div>
      <div class="space-y-3">
        <div
          v-for="(rule, index) in firewallRules"
          :key="index"
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
        >
          <div class="flex-1">
            <div class="flex items-center space-x-3">
              <span :class="rule.action === 'allow' ? 'text-green-600' : 'text-red-600'" class="text-sm font-medium">
                {{ rule.action === 'allow' ? '允许' : '拒绝' }}
              </span>
              <span class="text-sm text-gray-900 dark:text-white">{{ rule.port }} 端口</span>
              <span class="text-xs text-gray-500 dark:text-gray-400">{{ rule.protocol }}</span>
            </div>
          </div>
          <button
            @click="removeFirewallRule(index)"
            class="text-red-600 hover:text-red-700"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- 端口映射 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-medium text-gray-900 dark:text-white">端口映射</h2>
        <button
          @click="addPortMapping"
          class="inline-flex items-center px-3 py-1.5 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
        >
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
          </svg>
          添加映射
        </button>
      </div>
      <div class="space-y-3">
        <div
          v-for="(mapping, index) in portMappings"
          :key="index"
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
        >
          <div class="flex-1">
            <div class="flex items-center space-x-3">
              <span class="text-sm text-gray-900 dark:text-white">{{ mapping.externalPort }} → {{ mapping.internalPort }}</span>
              <span class="text-xs text-gray-500 dark:text-gray-400">{{ mapping.protocol }}</span>
            </div>
          </div>
          <button
            @click="removePortMapping(index)"
            class="text-red-600 hover:text-red-700"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- 权限配置 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h2 class="text-lg font-medium text-gray-900 dark:text-white mb-4">权限配置</h2>
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">访客权限</label>
          <div class="space-y-2">
            <label class="flex items-center">
              <input
                v-model="permissions.guest.read"
                type="checkbox"
                class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">只读访问</span>
            </label>
            <label class="flex items-center">
              <input
                v-model="permissions.guest.write"
                type="checkbox"
                class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">写入权限</span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- 共享开关 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h2 class="text-lg font-medium text-gray-900 dark:text-white mb-4">共享服务</h2>
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-gray-900 dark:text-white">SMB 共享</p>
            <p class="text-xs text-gray-500 dark:text-gray-400">Windows 文件共享</p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              v-model="shares.smb"
              type="checkbox"
              class="sr-only peer"
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
          </label>
        </div>
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-gray-900 dark:text-white">NFS 共享</p>
            <p class="text-xs text-gray-500 dark:text-gray-400">Linux 文件共享</p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              v-model="shares.nfs"
              type="checkbox"
              class="sr-only peer"
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
          </label>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useToast } from '../composables/useToast'

const toast = useToast()

const networkStatus = ref({
  online: true,
  ip: '192.168.1.100',
  downloadSpeed: '100 Mbps'
})

const networkConfig = ref({
  ip: '192.168.1.100',
  netmask: '255.255.255.0',
  gateway: '192.168.1.1',
  dhcp: false
})

const dnsConfig = ref({
  primary: '8.8.8.8',
  secondary: '8.8.4.4'
})

const firewallRules = ref([
  { action: 'allow', port: 80, protocol: 'TCP' },
  { action: 'allow', port: 443, protocol: 'TCP' },
  { action: 'allow', port: 22, protocol: 'TCP' }
])

const portMappings = ref([
  { externalPort: 8080, internalPort: 80, protocol: 'TCP' },
  { externalPort: 443, internalPort: 443, protocol: 'TCP' }
])

const permissions = ref({
  guest: {
    read: true,
    write: false
  }
})

const shares = ref({
  smb: true,
  nfs: true
})

const addFirewallRule = () => {
  firewallRules.value.push({ action: 'allow', port: 0, protocol: 'TCP' })
  toast.success('防火墙规则已添加')
}

const removeFirewallRule = (index) => {
  firewallRules.value.splice(index, 1)
  toast.success('防火墙规则已删除')
}

const addPortMapping = () => {
  portMappings.value.push({ externalPort: 0, internalPort: 0, protocol: 'TCP' })
  toast.success('端口映射已添加')
}

const removePortMapping = (index) => {
  portMappings.value.splice(index, 1)
  toast.success('端口映射已删除')
}

const saveAll = () => {
  // 模拟保存
  toast.success('全部设置已保存')
}

onMounted(() => {
  // 加载配置
})
</script>
