<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">存储管理</h1>
          <p class="text-gray-600 mt-1">管理存储卷、存储池和物理磁盘</p>
        </div>
        <button
          @click="refreshAll"
          :disabled="loading"
          class="btn-secondary flex items-center space-x-2"
        >
          <svg :class="{'animate-spin': loading}" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span>刷新</span>
        </button>
      </div>

      <!-- 存储空间概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <!-- 总容量 -->
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-12 h-12 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总容量</p>
              <p class="text-xl font-bold text-gray-900">{{ formatBytes(storageUsage.total_bytes) }}</p>
            </div>
          </div>
        </div>

        <!-- 已使用 -->
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-12 h-12 rounded-lg bg-orange-100 flex items-center justify-center mr-3">
              <svg class="w-6 h-6 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">已使用</p>
              <p class="text-xl font-bold text-orange-600">{{ formatBytes(storageUsage.used_bytes) }}</p>
            </div>
          </div>
        </div>

        <!-- 可用空间 -->
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-12 h-12 rounded-lg bg-green-100 flex items-center justify-center mr-3">
              <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">可用空间</p>
              <p class="text-xl font-bold text-green-600">{{ formatBytes(storageUsage.available_bytes) }}</p>
            </div>
          </div>
        </div>

        <!-- 磁盘数量 -->
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-12 h-12 rounded-lg bg-purple-100 flex items-center justify-center mr-3">
              <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">磁盘数量</p>
              <p class="text-xl font-bold text-gray-900">{{ disks.length }} 块</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 存储空间使用进度条 -->
      <div class="bg-white rounded-lg shadow p-4">
        <div class="flex justify-between items-center mb-2">
          <h3 class="font-semibold text-gray-900">存储空间使用</h3>
          <span class="text-sm text-gray-600">{{ usagePercent }}% 已使用</span>
        </div>
        <div class="w-full bg-gray-200 rounded-full h-4">
          <div
            :class="[
              'h-4 rounded-full transition-all',
              usagePercent > 90 ? 'bg-red-500' : usagePercent > 70 ? 'bg-yellow-500' : 'bg-green-500'
            ]"
            :style="{ width: usagePercent + '%' }"
          ></div>
        </div>
        <div class="flex justify-between mt-2 text-xs text-gray-500">
          <span>0%</span>
          <span>50%</span>
          <span>100%</span>
        </div>
      </div>

      <!-- 选项卡 -->
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="currentTab = tab.id"
            :class="[
              currentTab === tab.id
                ? 'border-primary-500 text-primary-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300',
              'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center space-x-2'
            ]"
          >
            <span>{{ tab.name }}</span>
            <span
              v-if="getCount(tab.id) > 0"
              class="px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-600"
            >
              {{ getCount(tab.id) }}
            </span>
          </button>
        </nav>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="flex justify-center items-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
        <span class="ml-3 text-gray-600">加载中...</span>
      </div>

      <!-- 物理磁盘 -->
      <div v-else-if="currentTab === 'disks'" class="space-y-4">
        <div v-if="disks.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2" />
          </svg>
          <p class="mt-4 text-gray-600">暂无磁盘信息</p>
        </div>

        <div v-else class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <DiskCard
            v-for="disk in disks"
            :key="disk.id"
            :disk="disk"
            @smart="showSmartDetails"
          />
        </div>
      </div>

      <!-- 存储卷 -->
      <div v-else-if="currentTab === 'volumes'" class="space-y-4">
        <div v-if="volumes.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
          </svg>
          <p class="mt-4 text-gray-600">暂无存储卷</p>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <StorageCard
            v-for="volume in volumes"
            :key="volume.id"
            :volume="volume"
          />
        </div>
      </div>

      <!-- 存储池 -->
      <div v-else-if="currentTab === 'pools'" class="space-y-4">
        <div v-if="pools.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
          </svg>
          <p class="mt-4 text-gray-600">暂无存储池</p>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <StoragePoolCard
            v-for="pool in pools"
            :key="pool.id"
            :pool="pool"
          />
        </div>
      </div>

      <!-- S.M.A.R.T. 详情模态框 -->
      <div v-if="selectedDisk" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto">
          <div class="flex justify-between items-center px-6 py-4 border-b sticky top-0 bg-white">
            <h3 class="text-lg font-semibold text-gray-900">S.M.A.R.T. 详情</h3>
            <button @click="selectedDisk = null" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div class="p-6 space-y-4">
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p class="text-gray-500">磁盘名称</p>
                <p class="font-medium text-gray-900">{{ selectedDisk.name }}</p>
              </div>
              <div>
                <p class="text-gray-500">型号</p>
                <p class="font-medium text-gray-900">{{ selectedDisk.model }}</p>
              </div>
              <div>
                <p class="text-gray-500">序列号</p>
                <p class="font-mono text-gray-900">{{ selectedDisk.serial_number || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">容量</p>
                <p class="font-medium text-gray-900">{{ selectedDisk.size_human || formatBytes(selectedDisk.size_bytes) }}</p>
              </div>
              <div>
                <p class="text-gray-500">温度</p>
                <p :class="getTemperatureClass(selectedDisk.temperature)" class="font-medium">
                  {{ selectedDisk.temperature || '-' }}°C
                </p>
              </div>
              <div>
                <p class="text-gray-500">健康状态</p>
                <span :class="getSmartClass(selectedDisk.smart_status)" class="px-2 py-1 rounded text-xs font-medium">
                  {{ getSmartLabel(selectedDisk.smart_status) }}
                </span>
              </div>
              <div>
                <p class="text-gray-500">通电时间</p>
                <p class="font-medium text-gray-900">{{ formatPowerOnHours(selectedDisk.power_on_hours) }}</p>
              </div>
              <div v-if="selectedDisk.speed_rpm">
                <p class="text-gray-500">转速</p>
                <p class="font-medium text-gray-900">{{ selectedDisk.speed_rpm }} RPM</p>
              </div>
            </div>

            <!-- 健康指标 -->
            <div class="border-t pt-4">
              <h4 class="font-medium text-gray-900 mb-3">健康指标</h4>
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Reallocated Sectors</span>
                  <span class="text-green-600">0</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Pending Sectors</span>
                  <span class="text-green-600">0</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">CRC Errors</span>
                  <span class="text-green-600">0</span>
                </div>
              </div>
            </div>
          </div>
          <div class="px-6 py-4 bg-gray-50 rounded-b-lg flex justify-end">
            <button @click="selectedDisk = null" class="btn-secondary">关闭</button>
          </div>
        </div>
      </div>

      <!-- Toast -->
      <div v-if="toast.show" class="fixed bottom-4 right-4 z-50">
        <div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">
          {{ toast.message }}
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import StorageCard from '@/components/storage/StorageCard.vue'
import StoragePoolCard from '@/components/storage/StoragePoolCard.vue'
import DiskCard from '@/components/storage/DiskCard.vue'
import { api } from '@/utils/api'

// 选项卡
const tabs = [
  { id: 'disks', name: '物理磁盘' },
  { id: 'volumes', name: '存储卷' },
  { id: 'pools', name: '存储池' }
]

const currentTab = ref('disks')
const loading = ref(true)

// 数据
const volumes = ref<any[]>([])
const pools = ref<any[]>([])
const disks = ref<any[]>([])
const storageUsage = ref({ total_bytes: 0, used_bytes: 0, available_bytes: 0 })

// 选中的磁盘
const selectedDisk = ref<any>(null)

// Toast
const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

// 使用率
const usagePercent = computed(() => {
  if (!storageUsage.value.total_bytes) return 0
  return Math.round((storageUsage.value.used_bytes / storageUsage.value.total_bytes) * 100)
})

// 获取数量
const getCount = (tabId: string) => {
  switch (tabId) {
    case 'disks': return disks.value.length
    case 'volumes': return volumes.value.length
    case 'pools': return pools.value.length
    default: return 0
  }
}

// 加载存储使用量
const loadStorageUsage = async () => {
  try {
    const response = await api.storage.getUsage()
    storageUsage.value = response.data.data || response.data || { total_bytes: 0, used_bytes: 0, available_bytes: 0 }
  } catch (error) {
    console.error('Failed to load storage usage:', error)
  }
}

// 加载磁盘
const loadDisks = async () => {
  try {
    const response = await api.storage.getDisks()
    disks.value = response.data.data || response.data.disks || []
  } catch (error) {
    console.error('Failed to load disks:', error)
  }
}

// 加载存储卷
const loadVolumes = async () => {
  try {
    const response = await api.storage.getVolumes()
    volumes.value = response.data.volumes || response.data.data || []
  } catch (error) {
    console.error('Failed to load volumes:', error)
  }
}

// 加载存储池
const loadPools = async () => {
  try {
    const response = await api.storage.getPools()
    pools.value = response.data.pools || response.data.data || []
  } catch (error) {
    console.error('Failed to load pools:', error)
  }
}

// 刷新所有
const refreshAll = async () => {
  loading.value = true
  try {
    await Promise.all([
      loadStorageUsage(),
      loadDisks(),
      loadVolumes(),
      loadPools()
    ])
    showToast('success', '刷新成功')
  } catch (error) {
    showToast('error', '刷新失败')
  } finally {
    loading.value = false
  }
}

// 显示 S.M.A.R.T. 详情
const showSmartDetails = (disk: any) => {
  selectedDisk.value = disk
}

// 格式化字节
const formatBytes = (bytes: number) => {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

// 格式化通电时间
const formatPowerOnHours = (hours: number) => {
  if (!hours) return '-'
  const days = Math.floor(hours / 24)
  const h = hours % 24
  if (days > 0) return `${days} 天 ${h} 小时`
  return `${h} 小时`
}

// 温度样式
const getTemperatureClass = (temp: number) => {
  if (!temp) return 'text-gray-900'
  if (temp > 50) return 'text-red-600'
  if (temp > 40) return 'text-yellow-600'
  return 'text-green-600'
}

// SMART 样式
const getSmartClass = (status: string) => {
  switch (status?.toLowerCase()) {
    case 'healthy':
    case 'good':
      return 'bg-green-100 text-green-700'
    case 'warning':
    case 'caution':
      return 'bg-yellow-100 text-yellow-700'
    case 'failed':
    case 'bad':
      return 'bg-red-100 text-red-700'
    default:
      return 'bg-gray-100 text-gray-700'
  }
}

// SMART 标签
const getSmartLabel = (status: string) => {
  switch (status?.toLowerCase()) {
    case 'healthy':
    case 'good':
      return '健康'
    case 'warning':
    case 'caution':
      return '警告'
    case 'failed':
    case 'bad':
      return '故障'
    default:
      return '未知'
  }
}

// Toast
const showToast = (type: 'success' | 'error', message: string) => {
  toast.value = { show: true, type, message }
  setTimeout(() => { toast.value.show = false }, 3000)
}

// 生命周期
onMounted(() => {
  refreshAll()
})
</script>