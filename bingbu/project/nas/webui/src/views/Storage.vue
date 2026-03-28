<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        存储管理
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

    <!-- 存储概览卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <!-- 总容量 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-blue-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">总容量</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">
              {{ storageStats.totalCapacity || '--' }}
            </p>
          </div>
        </div>
      </div>

      <!-- 已用空间 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-yellow-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">已用空间</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">
              {{ storageStats.usedSpace || '--' }}
            </p>
          </div>
        </div>
      </div>

      <!-- 剩余空间 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-green-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">剩余空间</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">
              {{ storageStats.freeSpace || '--' }}
            </p>
          </div>
        </div>
      </div>

      <!-- 使用率 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md" :class="usageRateColor">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">使用率</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">
              {{ storageStats.usageRate || '--' }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- 存储使用进度条 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">存储使用情况</h3>
      <div class="relative pt-1">
        <div class="flex mb-2 items-center justify-between">
          <div>
            <span class="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-blue-600 bg-blue-200 dark:text-blue-200 dark:bg-blue-900">
              已用 {{ storageStats.usageRate || '--' }}
            </span>
          </div>
          <div class="text-right">
            <span class="text-xs font-semibold inline-block text-blue-600 dark:text-blue-400">
              {{ storageStats.usedSpace || '--' }} / {{ storageStats.totalCapacity || '--' }}
            </span>
          </div>
        </div>
        <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-blue-200 dark:bg-blue-900">
          <div :style="{ width: storageStats.usagePercent || '0%' }" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500 transition-all duration-500"></div>
        </div>
      </div>
    </div>

    <!-- 磁盘列表 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">物理磁盘</h3>
        <span class="text-sm text-gray-500 dark:text-gray-400">{{ disks.length }} 块磁盘</span>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="px-6 py-12 text-center">
        <svg class="animate-spin h-8 w-8 text-indigo-600 mx-auto" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">加载中...</p>
      </div>

      <!-- 磁盘列表 -->
      <div v-else-if="disks.length > 0" class="divide-y divide-gray-200 dark:divide-gray-700">
        <div v-for="disk in disks" :key="disk.id" class="px-6 py-4 hover:bg-gray-50 dark:hover:bg-gray-700">
          <div class="flex items-center justify-between">
            <div class="flex items-center">
              <span class="text-2xl mr-4">💿</span>
              <div>
                <p class="text-sm font-medium text-gray-900 dark:text-white">{{ disk.name }}</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">{{ disk.model || '--' }}</p>
              </div>
            </div>
            <div class="flex items-center space-x-6">
              <div class="text-right">
                <p class="text-sm font-medium text-gray-900 dark:text-white">{{ disk.capacity || '--' }}</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">容量</p>
              </div>
              <div class="text-right">
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="diskHealthClass(disk.health)">
                  {{ disk.health || '未知' }}
                </span>
              </div>
              <div class="text-right">
                <p class="text-sm text-gray-500 dark:text-gray-400">{{ disk.temperature || '--' }}°C</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">温度</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 空列表 -->
      <div v-else class="px-6 py-12 text-center">
        <p class="text-sm text-gray-500 dark:text-gray-400">暂无磁盘信息</p>
      </div>
    </div>

    <!-- 存储池列表 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">存储池</h3>
        <button class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded text-indigo-700 bg-indigo-100 hover:bg-indigo-200 dark:text-indigo-300 dark:bg-indigo-900 dark:hover:bg-indigo-800">
          + 新建存储池
        </button>
      </div>

      <!-- 存储池列表 -->
      <div v-if="storagePools.length > 0" class="divide-y divide-gray-200 dark:divide-gray-700">
        <div v-for="pool in storagePools" :key="pool.id" class="px-6 py-4 hover:bg-gray-50 dark:hover:bg-gray-700">
          <div class="flex items-center justify-between">
            <div class="flex items-center">
              <span class="text-2xl mr-4">🗄️</span>
              <div>
                <p class="text-sm font-medium text-gray-900 dark:text-white">{{ pool.name }}</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">{{ pool.type || 'RAID' }}</p>
              </div>
            </div>
            <div class="flex items-center space-x-6">
              <div class="text-right">
                <p class="text-sm font-medium text-gray-900 dark:text-white">{{ pool.capacity || '--' }}</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">总容量</p>
              </div>
              <div class="text-right">
                <p class="text-sm text-gray-500 dark:text-gray-400">{{ pool.diskCount || 0 }} 块磁盘</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">成员</p>
              </div>
              <div class="text-right">
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
                  正常
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 空列表 -->
      <div v-else class="px-6 py-12 text-center">
        <p class="text-sm text-gray-500 dark:text-gray-400">暂无存储池</p>
      </div>
    </div>

    <!-- 存储卷列表 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">存储卷</h3>
        <button class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded text-indigo-700 bg-indigo-100 hover:bg-indigo-200 dark:text-indigo-300 dark:bg-indigo-900 dark:hover:bg-indigo-800">
          + 新建存储卷
        </button>
      </div>

      <!-- 存储卷列表 -->
      <div v-if="storageVolumes.length > 0" class="divide-y divide-gray-200 dark:divide-gray-700">
        <div v-for="volume in storageVolumes" :key="volume.id" class="px-6 py-4 hover:bg-gray-50 dark:hover:bg-gray-700">
          <div class="flex items-center justify-between">
            <div class="flex items-center">
              <span class="text-2xl mr-4">📀</span>
              <div>
                <p class="text-sm font-medium text-gray-900 dark:text-white">{{ volume.name }}</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">{{ volume.path || '/' }}</p>
              </div>
            </div>
            <div class="flex items-center space-x-6">
              <div class="text-right">
                <p class="text-sm font-medium text-gray-900 dark:text-white">{{ volume.capacity || '--' }}</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">容量</p>
              </div>
              <div class="text-right">
                <p class="text-sm text-gray-500 dark:text-gray-400">{{ volume.usage || '--' }}</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">已用</p>
              </div>
              <div class="text-right">
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
                  正常
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 空列表 -->
      <div v-else class="px-6 py-12 text-center">
        <p class="text-sm text-gray-500 dark:text-gray-400">暂无存储卷</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'

const router = useRouter()

const loading = ref(false)

// 存储统计数据
const storageStats = ref({
  totalCapacity: '--',
  usedSpace: '--',
  freeSpace: '--',
  usageRate: '--',
  usagePercent: '0%'
})

// 使用率颜色
const usageRateColor = computed(() => {
  const percent = parseFloat(storageStats.value.usagePercent) || 0
  if (percent >= 90) return 'bg-red-500 text-white'
  if (percent >= 70) return 'bg-yellow-500 text-white'
  return 'bg-green-500 text-white'
})

// 磁盘列表
const disks = ref([])

// 存储池列表
const storagePools = ref([])

// 存储卷列表
const storageVolumes = ref([])

// 磁盘健康状态样式
const diskHealthClass = (health) => {
  switch (health) {
    case '良好':
    case '健康':
      return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
    case '警告':
      return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200'
    case '故障':
    case '损坏':
      return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'
    default:
      return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
  }
}

// 格式化容量
const formatCapacity = (bytes) => {
  if (!bytes && bytes !== 0) return '--'
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB']
  let unitIndex = 0
  let value = bytes
  
  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024
    unitIndex++
  }
  
  return `${value.toFixed(unitIndex === 0 ? 0 : 2)} ${units[unitIndex]}`
}

// 加载存储数据
const loadStorageData = async () => {
  loading.value = true
  
  try {
    // 加载存储使用统计
    const usageResponse = await apiClient.get('/storage/usage')
    if (usageResponse.data.success) {
      const data = usageResponse.data.data
      storageStats.value = {
        totalCapacity: formatCapacity(data.total_bytes) || '--',
        usedSpace: formatCapacity(data.used_bytes) || '--',
        freeSpace: formatCapacity(data.free_bytes) || '--',
        usageRate: `${data.usage_percent?.toFixed(1) || 0}%`,
        usagePercent: `${data.usage_percent || 0}%`
      }
    }
    
    // 加载磁盘列表
    const disksResponse = await apiClient.get('/storage/disks')
    if (disksResponse.data.success) {
      disks.value = disksResponse.data.data.map(disk => ({
        id: disk.id,
        name: disk.name || disk.device || '--',
        model: disk.model || '--',
        capacity: formatCapacity(disk.capacity) || '--',
        health: disk.health || '良好',
        temperature: disk.temperature || '--'
      }))
    }
    
    // 加载存储池列表
    const poolsResponse = await apiClient.get('/storage/pools')
    if (poolsResponse.data.success) {
      storagePools.value = poolsResponse.data.data.map(pool => ({
        id: pool.id,
        name: pool.name || '--',
        type: pool.type || 'RAID',
        capacity: formatCapacity(pool.capacity) || '--',
        diskCount: pool.disk_count || 0
      }))
    }
    
    // 加载存储卷列表
    const volumesResponse = await apiClient.get('/storage/volumes')
    if (volumesResponse.data.success) {
      storageVolumes.value = volumesResponse.data.data.map(volume => ({
        id: volume.id,
        name: volume.name || '--',
        path: volume.path || '/',
        capacity: formatCapacity(volume.capacity) || '--',
        usage: formatCapacity(volume.used) || '--'
      }))
    }
  } catch (error) {
    console.error('Failed to load storage data:', error)
    
    // 使用模拟数据（API 未就绪时）
    storageStats.value = {
      totalCapacity: '8.0 TB',
      usedSpace: '3.2 TB',
      freeSpace: '4.8 TB',
      usageRate: '40.0%',
      usagePercent: '40%'
    }
    
    disks.value = [
      { id: 1, name: 'Disk 1', model: 'WD80EFZX', capacity: '8.0 TB', health: '良好', temperature: 35 },
      { id: 2, name: 'Disk 2', model: 'WD80EFZX', capacity: '8.0 TB', health: '良好', temperature: 36 },
      { id: 3, name: 'Disk 3', model: 'WD80EFZX', capacity: '8.0 TB', health: '良好', temperature: 34 },
      { id: 4, name: 'Disk 4', model: 'WD80EFZX', capacity: '8.0 TB', health: '良好', temperature: 35 }
    ]
    
    storagePools.value = [
      { id: 1, name: '存储池 1', type: 'RAID 5', capacity: '21.8 TB', diskCount: 3 }
    ]
    
    storageVolumes.value = [
      { id: 1, name: '卷 1', path: '/volume1', capacity: '21.8 TB', usage: '8.7 TB' }
    ]
  } finally {
    loading.value = false
  }
}

// 刷新数据
const refreshData = () => {
  loadStorageData()
}

// 页面加载时检查登录状态并加载数据
onMounted(() => {
  const token = localStorage.getItem('jwt_token')
  if (!token) {
    router.push('/login')
    return
  }
  
  loadStorageData()
})
</script>
