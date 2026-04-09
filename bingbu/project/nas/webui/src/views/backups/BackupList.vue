<template>
  <div class="space-y-6">
    <!-- 页面标题和操作栏 -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        备份管理
      </h1>
      <button
        @click="handleCreate"
        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
      >
        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
        </svg>
        创建备份
      </button>
    </div>

    <!-- 统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-blue-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">总备份数</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ stats.total }}</p>
          </div>
        </div>
      </div>

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
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">成功</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ stats.success }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-yellow-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">进行中</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ stats.processing }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-red-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">失败</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ stats.failed }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和筛选栏 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <!-- 搜索框 -->
        <div class="md:col-span-2">
          <FormInput
            v-model="searchQuery"
            placeholder="搜索备份名称..."
            label="搜索"
            :hide-label="true"
          />
        </div>

        <!-- 类型筛选 -->
        <div>
          <select
            v-model="typeFilter"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          >
            <option value="">全部类型</option>
            <option value="full">全量备份</option>
            <option value="incremental">增量备份</option>
          </select>
        </div>

        <!-- 状态筛选 -->
        <div>
          <select
            v-model="statusFilter"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          >
            <option value="">全部状态</option>
            <option value="success">成功</option>
            <option value="processing">进行中</option>
            <option value="failed">失败</option>
          </select>
        </div>
      </div>
    </div>

    <!-- 备份列表 -->
    <div v-if="loading" class="space-y-4">
      <SkeletonLoader v-for="i in 5" :key="i" :lines="3" has-title />
    </div>

    <div v-else-if="filteredBackups.length === 0" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-12 text-center">
      <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"></path>
      </svg>
      <p class="text-lg font-medium text-gray-900 dark:text-white mb-2">暂无备份</p>
      <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">创建备份开始使用</p>
      <button
        @click="handleCreate"
        class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
      >
        创建备份
      </button>
    </div>

    <div v-else class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead class="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">名称</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">类型</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">状态</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">大小</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">时间</th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">操作</th>
            </tr>
          </thead>
          <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
            <tr v-for="backup in filteredBackups" :key="backup.id" class="hover:bg-gray-50 dark:hover:bg-gray-700">
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm font-medium text-gray-900 dark:text-white">{{ backup.name }}</div>
                <div class="text-sm text-gray-500 dark:text-gray-400">{{ backup.description }}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full" :class="typeClasses[backup.type]">
                  {{ typeLabels[backup.type] }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full" :class="statusClasses[backup.status]">
                  {{ statusLabels[backup.status] }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                {{ backup.size }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                {{ backup.createdAt }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <button
                  v-if="backup.status === 'success'"
                  @click="handleRestore(backup)"
                  class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300 mr-3"
                >
                  恢复
                </button>
                <button
                  @click="handleDelete(backup)"
                  class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300"
                >
                  删除
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      v-if="showDeleteConfirm"
      v-model="showDeleteConfirm"
      title="确认删除"
      :message="deleteMessage"
      type="warning"
      confirm-text="删除"
      confirm-button-color="red"
      @confirm="confirmDelete"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import FormInput from '../../components/FormInput.vue'
import SkeletonLoader from '../../components/SkeletonLoader.vue'
import ConfirmDialog from '../../components/ConfirmDialog.vue'
import { useToast } from '../../composables/useToast'

const toast = useToast()

const loading = ref(true)
const backups = ref([])
const searchQuery = ref('')
const typeFilter = ref('')
const statusFilter = ref('')

const showDeleteConfirm = ref(false)
const backupToDelete = ref(null)

const stats = ref({
  total: 0,
  success: 0,
  processing: 0,
  failed: 0
})

const typeClasses = {
  full: 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400',
  incremental: 'bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-400'
}

const typeLabels = {
  full: '全量备份',
  incremental: '增量备份'
}

const statusClasses = {
  success: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400',
  processing: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400',
  failed: 'bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-400'
}

const statusLabels = {
  success: '成功',
  processing: '进行中',
  failed: '失败'
}

const filteredBackups = computed(() => {
  return backups.value.filter(backup => {
    const matchSearch = backup.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                       backup.description.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchType = !typeFilter.value || backup.type === typeFilter.value
    const matchStatus = !statusFilter.value || backup.status === statusFilter.value
    return matchSearch && matchType && matchStatus
  })
})

const deleteMessage = computed(() => {
  return backupToDelete.value ? `确定要删除备份 "${backupToDelete.value.name}" 吗？此操作不可恢复。` : ''
})

const loadBackups = async () => {
  loading.value = true
  try {
    // TODO: 调用 API 获取备份列表
    // const response = await apiClient.get('/api/v1/backups')
    // backups.value = response.data.data
    
    // 模拟数据
    backups.value = [
      {
        id: 1,
        name: '系统备份 2026-04-09',
        description: '每日系统备份',
        type: 'full',
        status: 'success',
        size: '2.5 GB',
        createdAt: '2026-04-09 01:00'
      },
      {
        id: 2,
        name: '数据备份 2026-04-09',
        description: '增量数据备份',
        type: 'incremental',
        status: 'success',
        size: '512 MB',
        createdAt: '2026-04-09 02:00'
      },
      {
        id: 3,
        name: '系统备份 2026-04-08',
        description: '每日系统备份',
        type: 'full',
        status: 'success',
        size: '2.4 GB',
        createdAt: '2026-04-08 01:00'
      },
      {
        id: 4,
        name: '数据备份 2026-04-08',
        description: '增量数据备份',
        type: 'incremental',
        status: 'failed',
        size: '-',
        createdAt: '2026-04-08 02:00'
      },
      {
        id: 5,
        name: '系统备份 2026-04-07',
        description: '每日系统备份',
        type: 'full',
        status: 'success',
        size: '2.3 GB',
        createdAt: '2026-04-07 01:00'
      }
    ]

    // 计算统计数据
    stats.value = {
      total: backups.value.length,
      success: backups.value.filter(b => b.status === 'success').length,
      processing: backups.value.filter(b => b.status === 'processing').length,
      failed: backups.value.filter(b => b.status === 'failed').length
    }
  } catch (error) {
    toast.error('加载备份列表失败')
  } finally {
    loading.value = false
  }
}

const handleCreate = () => {
  toast.info('创建备份功能待实现')
}

const handleRestore = (backup) => {
  toast.info(`恢复 "${backup.name}" 功能待实现`)
}

const handleDelete = (backup) => {
  backupToDelete.value = backup
  showDeleteConfirm.value = true
}

const confirmDelete = async () => {
  try {
    // TODO: 调用 API 删除备份
    // await apiClient.delete(`/api/v1/backups/${backupToDelete.value.id}`)
    
    backups.value = backups.value.filter(b => b.id !== backupToDelete.value.id)
    toast.success('备份已删除')
  } catch (error) {
    toast.error('删除失败')
  }
}

onMounted(() => {
  loadBackups()
})
</script>
