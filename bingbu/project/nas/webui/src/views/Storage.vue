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
        <button @click="showCreatePoolModal = true" class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded text-indigo-700 bg-indigo-100 hover:bg-indigo-200 dark:text-indigo-300 dark:bg-indigo-900 dark:hover:bg-indigo-800">
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
        <button @click="showCreateVolumeModal = true" class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded text-indigo-700 bg-indigo-100 hover:bg-indigo-200 dark:text-indigo-300 dark:bg-indigo-900 dark:hover:bg-indigo-800">
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

    <!-- 新建存储池模态框 -->
    <div v-if="showCreatePoolModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 overflow-y-auto">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 my-8">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">新建存储池</h3>
        </div>
        <div class="px-6 py-4 space-y-4 max-h-[60vh] overflow-y-auto">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              存储池名称 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="poolForm.name"
              @input="validatePoolName"
              type="text"
              required
              :class="poolFormErrors.name ? 'border-red-500' : 'border-gray-300'"
              class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
            <p v-if="poolFormErrors.name" class="mt-1 text-sm text-red-600">{{ poolFormErrors.name }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              RAID 类型
            </label>
            <select
              v-model="poolForm.type"
              class="w-full px-3 py-2 border border-gray-300 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="basic">Basic</option>
              <option value="raid0">RAID 0</option>
              <option value="raid1">RAID 1</option>
              <option value="raid5">RAID 5</option>
              <option value="raid10">RAID 10</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              选择磁盘
            </label>
            <div class="space-y-2">
              <div v-for="disk in availableDisks" :key="disk.id" class="flex items-center">
                <input
                  v-model="poolForm.disks"
                  :value="disk.id"
                  type="checkbox"
                  class="h-4 w-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                />
                <label class="ml-2 text-sm text-gray-700 dark:text-gray-300">
                  {{ disk.name }} ({{ disk.capacity }})
                </label>
              </div>
            </div>
            <p v-if="poolFormErrors.disks" class="mt-1 text-sm text-red-600">{{ poolFormErrors.disks }}</p>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="closeCreatePoolModal"
            :disabled="creatingPool"
            class="px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
          >
            取消
          </button>
          <button
            @click="createStoragePool"
            :disabled="creatingPool || !isPoolFormValid"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <svg v-if="creatingPool" class="animate-spin -ml-1 mr-2 h-4 w-4 inline" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            {{ creatingPool ? '创建中...' : '创建' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 新建存储卷模态框 -->
    <div v-if="showCreateVolumeModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 overflow-y-auto">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 my-8">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">新建存储卷</h3>
        </div>
        <div class="px-6 py-4 space-y-4 max-h-[60vh] overflow-y-auto">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              存储卷名称 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="volumeForm.name"
              @input="validateVolumeName"
              type="text"
              required
              :class="volumeFormErrors.name ? 'border-red-500' : 'border-gray-300'"
              class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
            <p v-if="volumeFormErrors.name" class="mt-1 text-sm text-red-600">{{ volumeFormErrors.name }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              存储池 <span class="text-red-500">*</span>
            </label>
            <select
              v-model="volumeForm.poolId"
              :class="volumeFormErrors.poolId ? 'border-red-500' : 'border-gray-300'"
              class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="">请选择存储池</option>
              <option v-for="pool in storagePools" :key="pool.id" :value="pool.id">
                {{ pool.name }} ({{ pool.capacity }})
              </option>
            </select>
            <p v-if="volumeFormErrors.poolId" class="mt-1 text-sm text-red-600">{{ volumeFormErrors.poolId }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              容量 (GB) <span class="text-red-500">*</span>
            </label>
            <input
              v-model.number="volumeForm.capacity"
              @input="validateVolumeCapacity"
              type="number"
              min="1"
              required
              :class="volumeFormErrors.capacity ? 'border-red-500' : 'border-gray-300'"
              class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
            <p v-if="volumeFormErrors.capacity" class="mt-1 text-sm text-red-600">{{ volumeFormErrors.capacity }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              文件系统 <span class="text-red-500">*</span>
            </label>
            <select
              v-model="volumeForm.filesystem"
              @change="validateVolumeFilesystem"
              :class="volumeFormErrors.filesystem ? 'border-red-500' : 'border-gray-300'"
              class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="">请选择文件系统</option>
              <option value="ext4">ext4</option>
              <option value="xfs">XFS</option>
              <option value="btrfs">Btrfs</option>
            </select>
            <p v-if="volumeFormErrors.filesystem" class="mt-1 text-sm text-red-600">{{ volumeFormErrors.filesystem }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              挂载点路径 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="volumeForm.mountPoint"
              @blur="validateVolumeMountPoint"
              type="text"
              required
              placeholder="/mnt/volume1"
              :class="volumeFormErrors.mountPoint ? 'border-red-500' : 'border-gray-300'"
              class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
            <p v-if="volumeFormErrors.mountPoint" class="mt-1 text-sm text-red-600">{{ volumeFormErrors.mountPoint }}</p>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="closeCreateVolumeModal"
            :disabled="creatingVolume"
            class="px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
          >
            取消
          </button>
          <button
            @click="createStorageVolume"
            :disabled="creatingVolume || !isVolumeFormValid"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <svg v-if="creatingVolume" class="animate-spin -ml-1 mr-2 h-4 w-4 inline" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            {{ creatingVolume ? '创建中...' : '创建' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'
import { useToast } from '../composables/useToast'

const router = useRouter()
const toast = useToast()

const loading = ref(false)

// 模态框状态
const showCreatePoolModal = ref(false)
const showCreateVolumeModal = ref(false)
const creatingPool = ref(false)
const creatingVolume = ref(false)

// 存储池表单
const poolForm = ref({
  name: '',
  type: 'basic',
  disks: []
})

const poolFormErrors = ref({
  name: '',
  disks: ''
})

// 存储卷表单
const volumeForm = ref({
  name: '',
  poolId: '',
  capacity: null,
  filesystem: '',
  mountPoint: ''
})

const volumeFormErrors = ref({
  name: '',
  poolId: '',
  capacity: '',
  filesystem: '',
  mountPoint: ''
})

// 磁盘名称验证：2-50 字符，禁止特殊字符
const validatePoolName = () => {
  const name = poolForm.value.name.trim()
  
  if (!name) {
    poolFormErrors.value.name = '存储池名称不能为空'
    return false
  }
  
  if (name.length < 2) {
    poolFormErrors.value.name = '存储池名称至少 2 个字符'
    return false
  }
  
  if (name.length > 50) {
    poolFormErrors.value.name = '存储池名称最多 50 个字符'
    return false
  }
  
  // 只允许字母、数字、下划线和连字符
  const validPattern = /^[a-zA-Z0-9_-]+$/
  if (!validPattern.test(name)) {
    poolFormErrors.value.name = '存储池名称只能包含字母、数字、下划线和连字符'
    return false
  }
  
  poolFormErrors.value.name = ''
  return true
}

// 存储卷名称验证：2-50 字符，禁止特殊字符
const validateVolumeName = () => {
  const name = volumeForm.value.name.trim()
  
  if (!name) {
    volumeFormErrors.value.name = '存储卷名称不能为空'
    return false
  }
  
  if (name.length < 2) {
    volumeFormErrors.value.name = '存储卷名称至少 2 个字符'
    return false
  }
  
  if (name.length > 50) {
    volumeFormErrors.value.name = '存储卷名称最多 50 个字符'
    return false
  }
  
  // 只允许字母、数字、下划线和连字符
  const validPattern = /^[a-zA-Z0-9_-]+$/
  if (!validPattern.test(name)) {
    volumeFormErrors.value.name = '存储卷名称只能包含字母、数字、下划线和连字符'
    return false
  }
  
  volumeFormErrors.value.name = ''
  return true
}

// 存储卷容量验证
const validateVolumeCapacity = () => {
  const capacity = volumeForm.value.capacity
  
  if (!capacity || capacity <= 0) {
    volumeFormErrors.value.capacity = '容量必须大于 0'
    return false
  }
  
  if (capacity > 1048576) { // 最大 1PB
    volumeFormErrors.value.capacity = '容量不能超过 1PB'
    return false
  }
  
  volumeFormErrors.value.capacity = ''
  return true
}

// 文件系统验证
const validateVolumeFilesystem = () => {
  const filesystem = volumeForm.value.filesystem
  const validFilesystems = ['ext4', 'xfs', 'btrfs']
  
  if (!filesystem) {
    volumeFormErrors.value.filesystem = '请选择文件系统类型'
    return false
  }
  
  if (!validFilesystems.includes(filesystem)) {
    volumeFormErrors.value.filesystem = '文件系统类型无效'
    return false
  }
  
  volumeFormErrors.value.filesystem = ''
  return true
}

// 挂载点路径验证
const validateVolumeMountPoint = () => {
  const mountPoint = volumeForm.value.mountPoint.trim()
  
  if (!mountPoint) {
    volumeFormErrors.value.mountPoint = '挂载点路径不能为空'
    return false
  }
  
  // 必须以 / 开头
  if (!mountPoint.startsWith('/')) {
    volumeFormErrors.value.mountPoint = '挂载点路径必须以 / 开头'
    return false
  }
  
  // 不能包含非法字符
  const invalidChars = /[<>:"'|]/
  if (invalidChars.test(mountPoint)) {
    volumeFormErrors.value.mountPoint = '挂载点路径包含非法字符'
    return false
  }
  
  // 不能以 /.. 开头
  if (mountPoint.startsWith('/..')) {
    volumeFormErrors.value.mountPoint = '挂载点路径不能包含 ..'
    return false
  }
  
  volumeFormErrors.value.mountPoint = ''
  return true
}

// 存储池表单有效性
const isPoolFormValid = computed(() => {
  const nameValid = validatePoolName()
  const disksValid = poolForm.value.disks.length > 0
  
  if (!disksValid) {
    poolFormErrors.value.disks = '请至少选择一块磁盘'
  } else {
    poolFormErrors.value.disks = ''
  }
  
  return nameValid && disksValid
})

// 存储卷表单有效性
const isVolumeFormValid = computed(() => {
  const nameValid = validateVolumeName()
  const poolIdValid = !!volumeForm.value.poolId
  const capacityValid = validateVolumeCapacity()
  const filesystemValid = validateVolumeFilesystem()
  const mountPointValid = validateVolumeMountPoint()
  
  if (!poolIdValid) {
    volumeFormErrors.value.poolId = '请选择存储池'
  } else {
    volumeFormErrors.value.poolId = ''
  }
  
  return nameValid && poolIdValid && capacityValid && filesystemValid && mountPointValid
})

// 可用磁盘列表
const availableDisks = computed(() => {
  return disks.value.filter(disk => disk.health === '良好' || disk.health === '健康')
})

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

// 关闭存储池创建模态框
const closeCreatePoolModal = () => {
  showCreatePoolModal.value = false
  poolForm.value = {
    name: '',
    type: 'basic',
    disks: []
  }
  poolFormErrors.value = {
    name: '',
    disks: ''
  }
}

// 关闭存储卷创建模态框
const closeCreateVolumeModal = () => {
  showCreateVolumeModal.value = false
  volumeForm.value = {
    name: '',
    poolId: '',
    capacity: null,
    filesystem: '',
    mountPoint: ''
  }
  volumeFormErrors.value = {
    name: '',
    poolId: '',
    capacity: '',
    filesystem: '',
    mountPoint: ''
  }
}

// 创建存储池
const createStoragePool = async () => {
  if (!isPoolFormValid.value) {
    toast.error('请修正表单错误')
    return
  }
  
  creatingPool.value = true
  
  try {
    const payload = {
      name: poolForm.value.name.trim(),
      type: poolForm.value.type,
      disk_ids: poolForm.value.disks
    }
    
    const response = await apiClient.post('/storage/pools', payload)
    
    if (response.data.success) {
      toast.success('存储池创建成功')
      closeCreatePoolModal()
      loadStorageData()
    } else {
      toast.error(response.data.error || '创建失败')
    }
  } catch (error) {
    console.error('Failed to create storage pool:', error)
    toast.error('创建失败：' + (error.response?.data?.error || '未知错误'))
  } finally {
    creatingPool.value = false
  }
}

// 创建存储卷
const createStorageVolume = async () => {
  if (!isVolumeFormValid.value) {
    toast.error('请修正表单错误')
    return
  }
  
  creatingVolume.value = true
  
  try {
    const payload = {
      name: volumeForm.value.name.trim(),
      pool_id: volumeForm.value.poolId,
      capacity: volumeForm.value.capacity * 1024 * 1024 * 1024, // GB 转字节
      filesystem: volumeForm.value.filesystem,
      mount_point: volumeForm.value.mountPoint.trim()
    }
    
    const response = await apiClient.post('/storage/volumes', payload)
    
    if (response.data.success) {
      toast.success('存储卷创建成功')
      closeCreateVolumeModal()
      loadStorageData()
    } else {
      toast.error(response.data.error || '创建失败')
    }
  } catch (error) {
    console.error('Failed to create storage volume:', error)
    toast.error('创建失败：' + (error.response?.data?.error || '未知错误'))
  } finally {
    creatingVolume.value = false
  }
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
