<template>
  <div class="px-4 py-6 sm:px-0">
    <!-- 页面标题 -->
    <div class="mb-6">
      <h2 class="text-2xl font-bold text-gray-900">💾 存储管理</h2>
      <p class="text-gray-600 mt-1">管理磁盘、存储池和卷</p>
    </div>

    <!-- 标签页切换 -->
    <div class="mb-6">
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
    </div>

    <!-- 磁盘列表 -->
    <div v-if="currentTab === 'disks'" class="space-y-6">
      <!-- 磁盘概览卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="px-4 py-5 sm:p-6">
            <dt class="text-sm font-medium text-gray-500 truncate">总磁盘数</dt>
            <dd class="mt-1 text-3xl font-semibold text-gray-900">{{ disks.length }}</dd>
          </div>
        </div>
        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="px-4 py-5 sm:p-6">
            <dt class="text-sm font-medium text-gray-500 truncate">健康磁盘</dt>
            <dd class="mt-1 text-3xl font-semibold text-green-600">{{ healthyDisks }}</dd>
          </div>
        </div>
        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="px-4 py-5 sm:p-6">
            <dt class="text-sm font-medium text-gray-500 truncate">总容量</dt>
            <dd class="mt-1 text-3xl font-semibold text-gray-900">{{ formatSize(totalCapacity) }}</dd>
          </div>
        </div>
      </div>

      <!-- 磁盘列表 -->
      <div class="bg-white shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">磁盘列表</h3>
          
          <div v-if="loading" class="text-center py-8">
            <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
            <p class="mt-2 text-gray-600">加载中...</p>
          </div>

          <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
            {{ error }}
          </div>

          <div v-else class="space-y-4">
            <div
              v-for="disk in disks"
              :key="disk.id"
              class="border border-gray-200 rounded-lg p-4"
            >
              <div class="flex items-center justify-between mb-3">
                <div class="flex items-center space-x-3">
                  <span class="text-2xl">💿</span>
                  <div>
                    <h4 class="text-lg font-medium text-gray-900">{{ disk.model }}</h4>
                    <p class="text-sm text-gray-500">{{ disk.device }} • {{ disk.serial }}</p>
                  </div>
                </div>
                <span
                  :class="{
                    'bg-green-100 text-green-800': disk.health === 'healthy',
                    'bg-yellow-100 text-yellow-800': disk.health === 'warning',
                    'bg-red-100 text-red-800': disk.health === 'critical',
                  }"
                  class="px-2 py-1 text-xs font-medium rounded-full"
                >
                  {{ disk.health === 'healthy' ? '健康' : disk.health === 'warning' ? '警告' : '严重' }}
                </span>
              </div>

              <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                <div>
                  <span class="text-gray-500">容量</span>
                  <p class="font-medium">{{ formatSize(disk.size) }}</p>
                </div>
                <div>
                  <span class="text-gray-500">已用</span>
                  <p class="font-medium">{{ formatSize(disk.used) }}</p>
                </div>
                <div>
                  <span class="text-gray-500">可用</span>
                  <p class="font-medium">{{ formatSize(disk.available) }}</p>
                </div>
                <div>
                  <span class="text-gray-500">使用率</span>
                  <p class="font-medium">{{ disk.usage_percent.toFixed(1) }}%</p>
                </div>
              </div>

              <div class="mt-3">
                <div class="w-full bg-gray-200 rounded-full h-2">
                  <div
                    :class="getUsageColor(disk.usage_percent)"
                    class="h-2 rounded-full transition-all"
                    :style="{ width: disk.usage_percent + '%' }"
                  ></div>
                </div>
              </div>

              <div class="mt-4 flex items-center justify-between">
                <div class="flex items-center space-x-4 text-sm text-gray-600">
                  <span>🌡️ 温度：{{ disk.temperature }}°C</span>
                  <span>状态：{{ disk.status }}</span>
                </div>
                <button
                  @click="viewDiskDetail(disk)"
                  class="text-primary-600 hover:text-primary-800 text-sm font-medium"
                >
                  查看详情 →
                </button>
              </div>
            </div>

            <div v-if="disks.length === 0" class="text-center py-8 text-gray-500">
              暂无磁盘信息
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 存储池管理 -->
    <div v-if="currentTab === 'pools'" class="space-y-6">
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-medium text-gray-900">存储池列表</h3>
        <button
          @click="showCreatePoolModal = true"
          class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
        >
          ➕ 创建存储池
        </button>
      </div>

      <div v-if="loading" class="text-center py-8">
        <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        <p class="mt-2 text-gray-600">加载中...</p>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div
          v-for="pool in pools"
          :key="pool.id"
          class="bg-white shadow rounded-lg p-6"
        >
          <div class="flex items-center justify-between mb-4">
            <h4 class="text-lg font-medium text-gray-900">{{ pool.name }}</h4>
            <span class="px-2 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800">
              {{ pool.raid_type || 'Basic' }}
            </span>
          </div>

          <div class="space-y-3">
            <div class="flex justify-between text-sm">
              <span class="text-gray-500">总容量</span>
              <span class="font-medium">{{ formatSize(pool.total_size) }}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-gray-500">已用</span>
              <span class="font-medium">{{ formatSize(pool.used_size) }}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-gray-500">可用</span>
              <span class="font-medium">{{ formatSize(pool.available_size) }}</span>
            </div>
          </div>

          <div class="mt-4">
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div
                class="bg-blue-600 h-2 rounded-full"
                :style="{ width: (pool.used_size / pool.total_size * 100) + '%' }"
              ></div>
            </div>
            <p class="text-right text-xs text-gray-500 mt-1">
              {{ ((pool.used_size / pool.total_size) * 100).toFixed(1) }}% 已用
            </p>
          </div>

          <div class="mt-4 flex items-center justify-between">
            <span class="text-sm text-gray-600">
              {{ pool.disk_count }} 个磁盘
            </span>
            <div class="flex space-x-2">
              <button
                @click="viewPoolDetail(pool)"
                class="text-primary-600 hover:text-primary-800 text-sm"
              >
                详情
              </button>
              <button
                @click="manageVolumes(pool)"
                class="text-green-600 hover:text-green-800 text-sm"
              >
                管理卷
              </button>
            </div>
          </div>
        </div>

        <div v-if="pools.length === 0" class="col-span-full text-center py-8 text-gray-500">
          暂无存储池，点击上方按钮创建
        </div>
      </div>
    </div>

    <!-- 卷管理 -->
    <div v-if="currentTab === 'volumes'" class="space-y-6">
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-medium text-gray-900">存储卷列表</h3>
        <button
          @click="showCreateVolumeModal = true"
          class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
        >
          ➕ 创建卷
        </button>
      </div>

      <div v-if="loading" class="text-center py-8">
        <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        <p class="mt-2 text-gray-600">加载中...</p>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="volume in volumes"
          :key="volume.id"
          class="bg-white shadow rounded-lg p-6"
        >
          <div class="flex items-center space-x-3 mb-4">
            <span class="text-3xl">📀</span>
            <div>
              <h4 class="text-lg font-medium text-gray-900">{{ volume.name }}</h4>
              <p class="text-sm text-gray-500">{{ volume.pool_name }}</p>
            </div>
          </div>

          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-500">容量</span>
              <span class="font-medium">{{ formatSize(volume.size) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500">已用</span>
              <span class="font-medium">{{ formatSize(volume.used) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500">使用率</span>
              <span class="font-medium">{{ volume.usage_percent.toFixed(1) }}%</span>
            </div>
          </div>

          <div class="mt-4">
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div
                :class="getUsageColor(volume.usage_percent)"
                class="h-2 rounded-full"
                :style="{ width: volume.usage_percent + '%' }"
              ></div>
            </div>
          </div>

          <div class="mt-4 flex items-center justify-between">
            <span
              :class="{
                'text-green-600': volume.status === 'healthy',
                'text-yellow-600': volume.status === 'warning',
                'text-red-600': volume.status === 'critical',
              }"
              class="text-sm font-medium"
            >
              {{ volume.status === 'healthy' ? '● 正常' : volume.status === 'warning' ? '● 警告' : '● 异常' }}
            </span>
            <div class="flex space-x-2">
              <button
                @click="expandVolume(volume)"
                class="text-blue-600 hover:text-blue-800 text-sm"
              >
                扩容
              </button>
              <button
                @click="deleteVolume(volume)"
                class="text-red-600 hover:text-red-800 text-sm"
              >
                删除
              </button>
            </div>
          </div>
        </div>

        <div v-if="volumes.length === 0" class="col-span-full text-center py-8 text-gray-500">
          暂无存储卷，点击上方按钮创建
        </div>
      </div>
    </div>

    <!-- 创建存储池模态框 -->
    <div v-if="showCreatePoolModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center">
      <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h3 class="text-lg font-medium text-gray-900 mb-4">创建存储池</h3>
        <form @submit.prevent="createPool" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">名称</label>
            <input
              v-model="newPool.name"
              type="text"
              required
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">RAID 类型</label>
            <select
              v-model="newPool.raid_type"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="basic">Basic (单盘)</option>
              <option value="raid0">RAID 0 (条带化)</option>
              <option value="raid1">RAID 1 (镜像)</option>
              <option value="raid5">RAID 5 (分布式奇偶校验)</option>
              <option value="raid6">RAID 6 (双奇偶校验)</option>
              <option value="raid10">RAID 10 (镜像 + 条带)</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">选择磁盘</label>
            <div class="space-y-2 max-h-48 overflow-y-auto border border-gray-300 rounded-md p-2">
              <label v-for="disk in availableDisks" :key="disk.id" class="flex items-center">
                <input
                  type="checkbox"
                  :value="disk.id"
                  v-model="newPool.disk_ids"
                  class="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span class="ml-2 text-sm text-gray-700">{{ disk.model }} ({{ formatSize(disk.size) }})</span>
              </label>
            </div>
          </div>
          <div class="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              @click="showCreatePoolModal = false"
              class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
            >
              取消
            </button>
            <button
              type="submit"
              :disabled="creating"
              class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
            >
              {{ creating ? '创建中...' : '创建' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import apiClient from '../api/client';

interface Disk {
  id: number;
  device: string;
  model: string;
  serial: string;
  size: number;
  used: number;
  available: number;
  usage_percent: number;
  status: string;
  health: string;
  temperature: number;
}

interface Pool {
  id: number;
  name: string;
  raid_type: string;
  total_size: number;
  used_size: number;
  available_size: number;
  disk_count: number;
  status: string;
}

interface Volume {
  id: number;
  name: string;
  pool_name: string;
  pool_id: number;
  size: number;
  used: number;
  usage_percent: number;
  status: string;
  created_at: number;
}

const tabs = [
  { id: 'disks', name: '📀 磁盘管理' },
  { id: 'pools', name: '🗄️ 存储池' },
  { id: 'volumes', name: '📁 存储卷' },
];

const currentTab = ref('disks');
const loading = ref(true);
const error = ref<string | null>(null);

const disks = ref<Disk[]>([]);
const pools = ref<Pool[]>([]);
const volumes = ref<Volume[]>([]);

// 模态框状态
const showCreatePoolModal = ref(false);
const showCreateVolumeModal = ref(false);
const creating = ref(false);

const newPool = ref({
  name: '',
  raid_type: 'basic',
  disk_ids: [] as number[],
});

// 计算属性
const healthyDisks = computed(() => {
  return disks.value.filter(d => d.health === 'healthy').length;
});

const totalCapacity = computed(() => {
  return disks.value.reduce((sum, d) => sum + d.size, 0);
});

const availableDisks = computed(() => {
  return disks.value.filter(d => d.status === 'available');
});

// 获取磁盘列表
const fetchDisks = async () => {
  try {
    const response = await apiClient.getStorageDisks();
    if (response.success && response.data) {
      disks.value = Array.isArray(response.data) ? response.data : [];
    }
  } catch (err) {
    console.error('Failed to fetch disks:', err);
  }
};

// 获取存储池列表
const fetchPools = async () => {
  try {
    const response = await apiClient.getStoragePools();
    if (response.success && response.data) {
      pools.value = Array.isArray(response.data) ? response.data : [];
    }
  } catch (err) {
    console.error('Failed to fetch pools:', err);
  }
};

// 获取卷列表
const fetchVolumes = async () => {
  try {
    const response = await apiClient.getStorageVolumes();
    if (response.success && response.data) {
      volumes.value = Array.isArray(response.data) ? response.data : [];
    }
  } catch (err) {
    console.error('Failed to fetch volumes:', err);
  }
};

// 刷新数据
const refreshData = async () => {
  loading.value = true;
  error.value = null;
  try {
    await Promise.all([
      fetchDisks(),
      fetchPools(),
      fetchVolumes(),
    ]);
  } catch (err) {
    error.value = '加载存储信息失败';
    console.error('Failed to refresh storage data:', err);
  } finally {
    loading.value = false;
  }
};

// 创建存储池
const createPool = async () => {
  creating.value = true;
  try {
    await apiClient.createStoragePool({
      name: newPool.value.name,
      raid_type: newPool.value.raid_type,
      disk_ids: newPool.value.disk_ids,
    });
    showCreatePoolModal.value = false;
    newPool.value = { name: '', raid_type: 'basic', disk_ids: [] };
    await fetchPools();
  } catch (err) {
    alert('创建存储池失败');
    console.error('Create pool failed:', err);
  } finally {
    creating.value = false;
  }
};

// 查看磁盘详情
const viewDiskDetail = (disk: Disk) => {
  // TODO: 实现磁盘详情页面
  console.log('View disk detail:', disk);
};

// 查看存储池详情
const viewPoolDetail = (pool: Pool) => {
  // TODO: 实现存储池详情页面
  console.log('View pool detail:', pool);
};

// 管理卷
const manageVolumes = (pool: Pool) => {
  currentTab.value = 'volumes';
  // TODO: 过滤显示该池的卷
  console.log('Manage volumes for pool:', pool);
};

// 扩容卷
const expandVolume = (volume: Volume) => {
  const newSize = prompt(`当前容量：${formatSize(volume.size)}\n请输入新容量 (GB):`);
  if (!newSize) return;
  // TODO: 实现扩容 API
  console.log('Expand volume:', volume, 'to', newSize);
};

// 删除卷
const deleteVolume = (volume: Volume) => {
  if (!confirm(`确定要删除卷 "${volume.name}" 吗？此操作不可恢复！`)) return;
  // TODO: 实现删除 API
  console.log('Delete volume:', volume);
};

// 工具函数
const formatSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const getUsageColor = (percent: number): string => {
  if (percent < 70) return 'bg-green-500';
  if (percent < 85) return 'bg-yellow-500';
  return 'bg-red-500';
};

onMounted(() => {
  refreshData();
});
</script>

<style scoped>
/* Storage view specific styles */
</style>
