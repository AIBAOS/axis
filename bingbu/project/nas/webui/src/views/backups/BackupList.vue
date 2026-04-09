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

        <!-- 排序 -->
        <div>
          <select
            v-model="sortOrder"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          >
            <option value="newest">最新优先</option>
            <option value="oldest">最早优先</option>
            <option value="size_desc">大小降序</option>
            <option value="size_asc">大小升序</option>
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
            <tr v-for="backup in paginatedBackups" :key="backup.id" class="hover:bg-gray-50 dark:hover:bg-gray-700">
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

    <!-- 分页 -->
    <div v-if="totalPages > 1" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
      <div class="flex items-center justify-between">
        <div class="text-sm text-gray-500 dark:text-gray-400">
          显示 {{ (currentPage - 1) * pageSize + 1 }} - {{ Math.min(currentPage * pageSize, filteredBackups.length) }} 条，共 {{ filteredBackups.length }} 条
        </div>
        <div class="flex space-x-2">
          <button
            @click="currentPage = 1"
            :disabled="currentPage === 1"
            class="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed"
          >
            首页
          </button>
          <button
            @click="currentPage--"
            :disabled="currentPage === 1"
            class="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed"
          >
            上一页
          </button>
          <span class="px-3 py-1 text-sm">
            {{ currentPage }} / {{ totalPages }}
          </span>
          <button
            @click="currentPage++"
            :disabled="currentPage === totalPages"
            class="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed"
          >
            下一页
          </button>
          <button
            @click="currentPage = totalPages"
            :disabled="currentPage === totalPages"
            class="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed"
          >
            末页
          </button>
        </div>
      </div>
    </div>

    <!-- 恢复备份模态框 -->
    <div v-if="showRestoreModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">恢复备份</h3>
        </div>
        <div class="px-6 py-4">
          <div class="mb-4">
            <p class="text-sm text-gray-700 dark:text-gray-300 mb-2">
              确定要恢复备份 <span class="font-medium">{{ restoreBackup?.name }}</span> 吗？
            </p>
            <div class="mt-3 p-3 bg-yellow-50 dark:bg-yellow-900/20 rounded-md">
              <p class="text-xs text-yellow-800 dark:text-yellow-400">
                ⚠️ 警告：恢复备份将覆盖当前系统数据，此操作不可逆。
              </p>
            </div>
          </div>
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-500 dark:text-gray-400">备份类型：</span>
              <span class="text-gray-900 dark:text-white">{{ typeLabels[restoreBackup?.type || ''] }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500 dark:text-gray-400">创建时间：</span>
              <span class="text-gray-900 dark:text-white">{{ restoreBackup?.createdAt }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500 dark:text-gray-400">备份大小：</span>
              <span class="text-gray-900 dark:text-white">{{ restoreBackup?.size }}</span>
            </div>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="closeRestoreModal"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="submitRestore"
            :disabled="restoring"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ restoring ? '恢复中...' : '确认恢复' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 创建备份模态框 -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">创建备份</h3>
        </div>
        <div class="px-6 py-4 space-y-4">
          <!-- 备份名称 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              备份名称 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="createFormData.name"
              type="text"
              placeholder="例如：系统备份 2026-04-09"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              :class="{ 'border-red-500': createFormErrors.name }"
            />
            <p v-if="createFormErrors.name" class="mt-1 text-sm text-red-600 dark:text-red-400">{{ createFormErrors.name }}</p>
          </div>

          <!-- 备份类型 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              备份类型 <span class="text-red-500">*</span>
            </label>
            <select
              v-model="createFormData.type"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              :class="{ 'border-red-500': createFormErrors.type }"
            >
              <option value="">请选择备份类型</option>
              <option value="full">全量备份</option>
              <option value="incremental">增量备份</option>
            </select>
            <p v-if="createFormErrors.type" class="mt-1 text-sm text-red-600 dark:text-red-400">{{ createFormErrors.type }}</p>
          </div>

          <!-- 描述 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              描述
            </label>
            <textarea
              v-model="createFormData.description"
              rows="3"
              placeholder="例如：每日系统备份"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            ></textarea>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="closeCreateModal"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="submitCreate"
            :disabled="creating"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ creating ? '创建中...' : '创建' }}
          </button>
        </div>
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
const sortOrder = ref('newest')

// 分页
const currentPage = ref(1)
const pageSize = ref(10)

const showDeleteConfirm = ref(false)
const backupToDelete = ref(null)

// 恢复备份
const showRestoreModal = ref(false)
const restoring = ref(false)
const restoreBackup = ref(null)

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

const filteredAndSortedBackups = computed(() => {
  let result = [...backups.value]
  
  // 筛选
  result = result.filter(backup => {
    const matchSearch = backup.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                       backup.description.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchType = !typeFilter.value || backup.type === typeFilter.value
    const matchStatus = !statusFilter.value || backup.status === statusFilter.value
    return matchSearch && matchType && matchStatus
  })
  
  // 排序
  result.sort((a, b) => {
    switch (sortOrder.value) {
      case 'oldest':
        return new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime()
      case 'size_desc':
        return (parseFloat(b.size) || 0) - (parseFloat(a.size) || 0)
      case 'size_asc':
        return (parseFloat(a.size) || 0) - (parseFloat(b.size) || 0)
      case 'newest':
      default:
        return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
    }
  })
  
  return result
})

const totalPages = computed(() => {
  return Math.ceil(filteredAndSortedBackups.value.length / pageSize.value)
})

const paginatedBackups = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filteredAndSortedBackups.value.slice(start, end)
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

// 创建备份相关
const showCreateModal = ref(false)
const creating = ref(false)
const createFormData = ref({
  name: '',
  type: '',
  description: ''
})
const createFormErrors = ref({})

const showCreateModalFunc = () => {
  showCreateModal.value = true
  createFormData.value = {
    name: '',
    type: '',
    description: ''
  }
  createFormErrors.value = {}
}

const closeCreateModal = () => {
  showCreateModal.value = false
  createFormData.value = {
    name: '',
    type: '',
    description: ''
  }
  createFormErrors.value = {}
}

const validateCreateForm = () => {
  createFormErrors.value = {}
  let isValid = true

  if (!createFormData.value.name.trim()) {
    createFormErrors.value.name = '请输入备份名称'
    isValid = false
  }

  if (!createFormData.value.type) {
    createFormErrors.value.type = '请选择备份类型'
    isValid = false
  }

  return isValid
}

const submitCreate = async () => {
  if (!validateCreateForm()) return

  creating.value = true
  try {
    // TODO: 调用 API 创建备份
    // const response = await apiClient.post('/api/v1/backups', createFormData.value)
    
    // 模拟 API 调用
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    // 添加新备份到列表
    const newBackup = {
      id: Date.now(),
      name: createFormData.value.name,
      description: createFormData.value.description || '手动创建备份',
      type: createFormData.value.type,
      status: 'processing',
      size: '-',
      createdAt: new Date().toLocaleString('zh-CN', { 
        year: 'numeric', 
        month: '2-digit', 
        day: '2-digit', 
        hour: '2-digit', 
        minute: '2-digit' 
      })
    }
    
    backups.value.unshift(newBackup)
    
    // 更新统计数据
    stats.value.total = backups.value.length
    stats.value.processing = backups.value.filter(b => b.status === 'processing').length
    
    toast.success('备份任务已创建')
    closeCreateModal()
  } catch (error) {
    toast.error('创建备份失败')
  } finally {
    creating.value = false
  }
}

const handleCreate = showCreateModalFunc

const handleRestore = (backup) => {
  restoreBackup.value = backup
  showRestoreModal.value = true
}

const closeRestoreModal = () => {
  showRestoreModal.value = false
  restoreBackup.value = null
}

const submitRestore = async () => {
  if (!restoreBackup.value) return
  
  restoring.value = true
  try {
    // TODO: 调用 API 恢复备份
    // await apiClient.post(`/api/v1/backups/${restoreBackup.value.id}/restore`)
    
    // 模拟 API 调用
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    // 更新备份状态
    const backup = backups.value.find(b => b.id === restoreBackup.value?.id)
    if (backup) {
      backup.status = 'processing'
    }
    
    toast.success('备份恢复任务已启动')
    closeRestoreModal()
    
    // 更新统计数据
    stats.value.processing = backups.value.filter(b => b.status === 'processing').length
  } catch (error) {
    toast.error('恢复备份失败')
  } finally {
    restoring.value = false
  }
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
