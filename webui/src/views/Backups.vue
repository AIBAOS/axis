<template>
  <div class="px-4 py-6 sm:px-0">
    <div class="bg-white overflow-hidden shadow rounded-lg">
      <div class="px-4 py-5 sm:p-6">
        <h2 class="text-2xl font-bold text-gray-900 mb-4">📦 备份管理</h2>
        
        <div class="mb-6">
          <button
            @click="showCreateModal = true"
            class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
          >
            ➕ 创建备份任务
          </button>
        </div>

        <div v-if="loading" class="text-center py-8">
          <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
          <p class="mt-2 text-gray-600">加载中...</p>
        </div>

        <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
          {{ error }}
        </div>

        <div v-else class="space-y-4">
          <div v-for="backup in backups" :key="backup.id" class="border border-gray-200 rounded-lg p-4">
            <div class="flex items-center justify-between">
              <div>
                <h3 class="text-lg font-medium text-gray-900">{{ backup.name }}</h3>
                <p class="text-sm text-gray-500 mt-1">{{ backup.description || '无描述' }}</p>
              </div>
              <span
                :class="{
                  'bg-green-100 text-green-800': backup.status === 'idle',
                  'bg-blue-100 text-blue-800': backup.status === 'running',
                  'bg-yellow-100 text-yellow-800': backup.status === 'completed',
                  'bg-red-100 text-red-800': backup.status === 'failed',
                }"
                class="px-2 py-1 text-xs font-medium rounded-full"
              >
                {{ backup.status }}
              </span>
            </div>
            <div class="mt-3 grid grid-cols-2 gap-4 text-sm text-gray-600">
              <div>
                <span class="font-medium">源路径:</span> {{ backup.source_path }}
              </div>
              <div>
                <span class="font-medium">目标路径:</span> {{ backup.destination_path }}
              </div>
              <div>
                <span class="font-medium">类型:</span> {{ backup.backup_type }}
              </div>
              <div v-if="backup.schedule">
                <span class="font-medium">调度:</span> {{ backup.schedule }}
              </div>
            </div>
          </div>

          <div v-if="backups.length === 0" class="text-center py-8 text-gray-500">
            暂无备份任务
          </div>
        </div>
      </div>
    </div>

    <!-- Create Backup Modal -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center">
      <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h3 class="text-lg font-medium text-gray-900 mb-4">创建备份任务</h3>
        
        <form @submit.prevent="createBackup" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">名称</label>
            <input
              v-model="newBackup.name"
              type="text"
              required
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500"
            />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700">描述</label>
            <input
              v-model="newBackup.description"
              type="text"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500"
            />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700">源路径</label>
            <input
              v-model="newBackup.source_path"
              type="text"
              required
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500"
            />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700">目标路径</label>
            <input
              v-model="newBackup.destination"
              type="text"
              required
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500"
            />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700">备份类型</label>
            <select
              v-model="newBackup.backup_type"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500"
            >
              <option value="full">全量备份</option>
              <option value="incremental">增量备份</option>
            </select>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700">调度 (Cron 表达式，可选)</label>
            <input
              v-model="newBackup.schedule"
              type="text"
              placeholder="0 2 * * *"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500"
            />
          </div>

          <div class="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              @click="showCreateModal = false"
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
import { ref, onMounted } from 'vue';
import apiClient from '../api/client';

interface Backup {
  id: number;
  name: string;
  description: string;
  backup_type: string;
  source_path: string;
  destination_path: string;
  schedule: string | null;
  status: string;
  created_at: number;
  updated_at: number;
}

const backups = ref<Backup[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const showCreateModal = ref(false);
const creating = ref(false);

const newBackup = ref({
  name: '',
  description: '',
  source_path: '',
  destination: '',
  backup_type: 'full',
  schedule: '',
});

const fetchBackups = async () => {
  try {
    const response = await apiClient.getBackups();
    if (response.success && response.data) {
      backups.value = Array.isArray(response.data) ? response.data : [];
    }
  } catch (err) {
    error.value = '加载备份列表失败';
    console.error('Failed to fetch backups:', err);
  } finally {
    loading.value = false;
  }
};

const createBackup = async () => {
  creating.value = true;
  try {
    const response = await apiClient.createBackup(newBackup.value);
    if (response.success) {
      showCreateModal.value = false;
      await fetchBackups();
      // Reset form
      newBackup.value = {
        name: '',
        description: '',
        source_path: '',
        destination: '',
        backup_type: 'full',
        schedule: '',
      };
    }
  } catch (err) {
    alert('创建备份任务失败');
    console.error('Failed to create backup:', err);
  } finally {
    creating.value = false;
  }
};

onMounted(() => {
  fetchBackups();
});
</script>

<style scoped>
/* Backups view specific styles */
</style>
