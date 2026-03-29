<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div>
        <h1 class="text-2xl font-bold text-gray-900">存储管理</h1>
        <p class="text-gray-600 mt-1">管理存储卷、存储池和物理磁盘</p>
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
              'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm'
            ]"
          >
            {{ tab.name }}
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

      <!-- 存储卷列表 -->
      <div v-else-if="currentTab === 'volumes'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h2 class="text-lg font-semibold text-gray-900">存储卷</h2>
          <button class="btn-primary flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            <span>创建存储卷</span>
          </button>
        </div>

        <div v-if="volumes.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
          </svg>
          <p class="mt-4 text-gray-600">暂无存储卷</p>
          <p class="mt-2 text-sm text-gray-500">创建第一个存储卷开始使用</p>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <StorageCard
            v-for="volume in volumes"
            :key="volume.id"
            :volume="volume"
          />
        </div>
      </div>

      <!-- 存储池列表 -->
      <div v-else-if="currentTab === 'pools'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h2 class="text-lg font-semibold text-gray-900">存储池</h2>
          <button class="btn-primary flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            <span>创建存储池</span>
          </button>
        </div>

        <div v-if="pools.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
          </svg>
          <p class="mt-4 text-gray-600">暂无存储池</p>
          <p class="mt-2 text-sm text-gray-500">创建第一个存储池开始使用</p>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <StoragePoolCard
            v-for="pool in pools"
            :key="pool.id"
            :pool="pool"
          />
        </div>
      </div>

      <!-- 磁盘列表 -->
      <div v-else-if="currentTab === 'disks'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h2 class="text-lg font-semibold text-gray-900">物理磁盘</h2>
          <button class="btn-secondary flex items-center space-x-2" @click="loadDisks">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>刷新</span>
          </button>
        </div>

        <div v-if="disks.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
          </svg>
          <p class="mt-4 text-gray-600">暂无磁盘信息</p>
        </div>

        <div v-else class="space-y-4">
          <DiskCard
            v-for="disk in disks"
            :key="disk.id"
            :disk="disk"
          />
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import StorageCard from '@/components/storage/StorageCard.vue'
import StoragePoolCard from '@/components/storage/StoragePoolCard.vue'
import DiskCard from '@/components/storage/DiskCard.vue'
import { api } from '@/utils/api'

// 选项卡
const tabs = [
  { id: 'volumes', name: '存储卷' },
  { id: 'pools', name: '存储池' },
  { id: 'disks', name: '物理磁盘' }
]

const currentTab = ref('volumes')
const loading = ref(true)

// 数据
const volumes = ref<any[]>([])
const pools = ref<any[]>([])
const disks = ref<any[]>([])

// 加载存储卷
const loadVolumes = async () => {
  try {
    const response = await api.storage.getVolumes()
    volumes.value = response.data.volumes || []
  } catch (error) {
    console.error('Failed to load volumes:', error)
  }
}

// 加载存储池
const loadPools = async () => {
  try {
    const response = await api.storage.getPools()
    pools.value = response.data.pools || []
  } catch (error) {
    console.error('Failed to load pools:', error)
  }
}

// 加载磁盘
const loadDisks = async () => {
  try {
    const response = await api.storage.getDisks()
    disks.value = response.data.disks || []
  } catch (error) {
    console.error('Failed to load disks:', error)
  }
}

// 加载所有数据
const loadAll = async () => {
  loading.value = true
  await Promise.all([loadVolumes(), loadPools(), loadDisks()])
  loading.value = false
}

// 生命周期
onMounted(() => {
  loadAll()
})
</script>
