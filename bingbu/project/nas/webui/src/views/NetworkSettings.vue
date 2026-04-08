<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        网络配置
      </h1>
      <button
        @click="refreshData"
        class="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
      >
        <svg class="w-5 h-5 mr-2" :class="{ 'animate-spin': loading }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
        </svg>
        刷新
      </button>
    </div>

    <!-- 网络模式选择 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">网络模式</h3>
      <div class="space-y-3">
        <label class="flex items-center">
          <input
            v-model="networkMode"
            type="radio"
            value="dhcp"
            class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300"
          />
          <span class="ml-3 text-sm text-gray-700 dark:text-gray-300">
            DHCP（自动获取 IP）
          </span>
        </label>
        <label class="flex items-center">
          <input
            v-model="networkMode"
            type="radio"
            value="static"
            class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300"
          />
          <span class="ml-3 text-sm text-gray-700 dark:text-gray-300">
            静态 IP（手动配置）
          </span>
        </label>
      </div>
    </div>

    <!-- 网络配置表单 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">网络配置</h3>
      <form @submit.prevent="saveConfig" class="space-y-4">
        <!-- IP 地址 -->
        <div>
          <label for="ipAddress" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            IP 地址
          </label>
          <input
            id="ipAddress"
            v-model="formData.ipAddress"
            type="text"
            :disabled="networkMode === 'dhcp'"
            placeholder="192.168.1.100"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            :class="{ 'border-red-500': errors.ipAddress }"
          />
          <p v-if="errors.ipAddress" class="mt-1 text-sm text-red-600 dark:text-red-400">
            {{ errors.ipAddress }}
          </p>
        </div>

        <!-- 子网掩码 -->
        <div>
          <label for="subnetMask" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            子网掩码
          </label>
          <input
            id="subnetMask"
            v-model="formData.subnetMask"
            type="text"
            :disabled="networkMode === 'dhcp'"
            placeholder="255.255.255.0"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            :class="{ 'border-red-500': errors.subnetMask }"
          />
          <p v-if="errors.subnetMask" class="mt-1 text-sm text-red-600 dark:text-red-400">
            {{ errors.subnetMask }}
          </p>
        </div>

        <!-- 网关 -->
        <div>
          <label for="gateway" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            默认网关
          </label>
          <input
            id="gateway"
            v-model="formData.gateway"
            type="text"
            :disabled="networkMode === 'dhcp'"
            placeholder="192.168.1.1"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            :class="{ 'border-red-500': errors.gateway }"
          />
          <p v-if="errors.gateway" class="mt-1 text-sm text-red-600 dark:text-red-400">
            {{ errors.gateway }}
          </p>
        </div>

        <!-- DNS 服务器 -->
        <div>
          <label for="dnsServer" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            DNS 服务器
          </label>
          <input
            id="dnsServer"
            v-model="formData.dnsServer"
            type="text"
            :disabled="networkMode === 'dhcp'"
            placeholder="8.8.8.8"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            :class="{ 'border-red-500': errors.dnsServer }"
          />
          <p v-if="errors.dnsServer" class="mt-1 text-sm text-red-600 dark:text-red-400">
            {{ errors.dnsServer }}
          </p>
        </div>

        <!-- 备用 DNS -->
        <div>
          <label for="dnsServer2" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            备用 DNS
          </label>
          <input
            id="dnsServer2"
            v-model="formData.dnsServer2"
            type="text"
            :disabled="networkMode === 'dhcp'"
            placeholder="8.8.4.4"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
          />
        </div>

        <!-- 操作按钮 -->
        <div class="flex justify-end space-x-3 pt-4">
          <button
            type="button"
            @click="resetForm"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            重置
          </button>
          <button
            type="submit"
            :disabled="loading || !isFormValid"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ saving ? '保存中...' : '保存配置' }}
          </button>
        </div>
      </form>
    </div>

    <!-- 当前网络信息 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">当前网络信息</h3>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
          <span class="text-sm text-gray-500 dark:text-gray-400">MAC 地址</span>
          <span class="text-sm font-medium text-gray-900 dark:text-white">{{ networkInfo.mac || '--' }}</span>
        </div>
        <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
          <span class="text-sm text-gray-500 dark:text-gray-400">网络状态</span>
          <span class="text-sm font-medium" :class="networkInfo.status === 'connected' ? 'text-green-600 dark:text-green-400' : 'text-gray-600 dark:text-gray-400'">
            {{ networkInfo.status === 'connected' ? '● 已连接' : '○ 未连接' }}
          </span>
        </div>
        <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
          <span class="text-sm text-gray-500 dark:text-gray-400">连接速度</span>
          <span class="text-sm font-medium text-gray-900 dark:text-white">{{ networkInfo.speed || '--' }}</span>
        </div>
        <div class="flex justify-between items-center py-2">
          <span class="text-sm text-gray-500 dark:text-gray-400">网络模式</span>
          <span class="text-sm font-medium text-gray-900 dark:text-white">{{ networkMode === 'dhcp' ? 'DHCP' : '静态 IP' }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

const loading = ref(false)
const saving = ref(false)
const networkMode = ref('dhcp')

const formData = ref({
  ipAddress: '',
  subnetMask: '',
  gateway: '',
  dnsServer: '',
  dnsServer2: ''
})

const errors = ref({
  ipAddress: '',
  subnetMask: '',
  gateway: '',
  dnsServer: ''
})

const networkInfo = ref({
  mac: '',
  status: 'disconnected',
  speed: ''
})

// IP 地址格式验证
const validateIP = (ip) => {
  if (!ip) return 'IP 地址不能为空'
  const parts = ip.split('.')
  if (parts.length !== 4) return 'IP 地址格式错误（应为 x.x.x.x）'
  for (const part of parts) {
    const num = parseInt(part, 10)
    if (isNaN(num) || num < 0 || num > 255) return 'IP 地址每段应为 0-255'
  }
  return ''
}

// 表单验证
const validateForm = () => {
  errors.value = {
    ipAddress: '',
    subnetMask: '',
    gateway: '',
    dnsServer: ''
  }
  
  let isValid = true
  
  if (networkMode.value === 'static') {
    errors.value.ipAddress = validateIP(formData.value.ipAddress)
    if (errors.value.ipAddress) isValid = false
    
    errors.value.subnetMask = validateIP(formData.value.subnetMask)
    if (errors.value.subnetMask) isValid = false
    
    errors.value.gateway = validateIP(formData.value.gateway)
    if (errors.value.gateway) isValid = false
    
    if (formData.value.dnsServer) {
      errors.value.dnsServer = validateIP(formData.value.dnsServer)
      if (errors.value.dnsServer) isValid = false
    }
  }
  
  return isValid
}

// 表单是否有效
const isFormValid = computed(() => {
  if (networkMode.value === 'dhcp') return true
  return validateForm()
})

// 刷新数据
const refreshData = async () => {
  loading.value = true
  try {
    // TODO: 调用后端 API 获取网络配置
    // const response = await apiClient.get('/network/config')
    // formData.value = response.data
    
    // 模拟数据
    formData.value = {
      ipAddress: '192.168.1.100',
      subnetMask: '255.255.255.0',
      gateway: '192.168.1.1',
      dnsServer: '8.8.8.8',
      dnsServer2: '8.8.4.4'
    }
    
    networkInfo.value = {
      mac: '00:1A:2B:3C:4D:5E',
      status: 'connected',
      speed: '1000 Mbps'
    }
  } catch (error) {
    console.error('Failed to load network config:', error)
  } finally {
    loading.value = false
  }
}

// 保存配置
const saveConfig = async () => {
  if (!validateForm()) return
  
  saving.value = true
  try {
    // TODO: 调用后端 API 保存网络配置
    // await apiClient.put('/network/config', formData.value)
    
    // 模拟保存
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    alert('网络配置已保存')
  } catch (error) {
    console.error('Failed to save network config:', error)
    alert('保存失败，请重试')
  } finally {
    saving.value = false
  }
}

// 重置表单
const resetForm = () => {
  refreshData()
}

// 页面加载时获取配置
onMounted(() => {
  refreshData()
})
</script>
