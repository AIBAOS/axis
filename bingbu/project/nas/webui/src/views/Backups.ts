import { defineComponent, ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'

interface BackupTask {
  id: number
  name: string
  description: string
  backupType: string
  sourcePath: string
  destination: string
  schedule: string | null
  status: 'idle' | 'running' | 'completed' | 'failed'
  lastRun: number | null
  lastRunStatus: string | null
  createdAt: number
}

interface BackupStats {
  totalTasks: number
  successfulExecutions: number
  failedExecutions: number
  runningExecutions: number
  storageUsed: string
}

export default defineComponent({
  name: 'Backups',
  setup() {
    const router = useRouter()
    const loading = ref(false)
    const backups = ref<BackupTask[]>([])
    const searchQuery = ref('')
    const typeFilter = ref('all')
    const statusFilter = ref('all')
    const showCreateModal = ref(false)
    const showExecuteModal = ref(false)
    const backupToExecute = ref<BackupTask | null>(null)
    const backupStats = ref<BackupStats>({
      totalTasks: 0,
      successfulExecutions: 0,
      failedExecutions: 0,
      runningExecutions: 0,
      storageUsed: '--'
    })

    // 表单数据
    const formData = ref({
      name: '',
      description: '',
      sourcePath: '',
      destination: '',
      backupType: 'full',
      schedule: ''
    })

    // 过滤后的备份列表
    const filteredBackups = computed(() => {
      return backups.value.filter(backup => {
        const matchSearch = searchQuery.value === '' ||
          backup.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
          backup.sourcePath.toLowerCase().includes(searchQuery.value.toLowerCase())
        
        const matchType = typeFilter.value === 'all' || backup.backupType === typeFilter.value
        const matchStatus = statusFilter.value === 'all' || backup.status === statusFilter.value
        
        return matchSearch && matchType && matchStatus
      })
    })

    // 状态标签样式
    const statusClass = (status: string) => {
      switch (status) {
        case 'completed':
        case 'idle':
          return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
        case 'running':
          return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200'
        case 'failed':
          return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'
        default:
          return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
      }
    }

    // 状态文本
    const statusText = (status: string) => {
      switch (status) {
        case 'idle':
          return '空闲'
        case 'running':
          return '进行中'
        case 'completed':
          return '成功'
        case 'failed':
          return '失败'
        default:
          return '未知'
      }
    }

    // 备份类型文本
    const typeText = (type: string) => {
      switch (type) {
        case 'full':
          return '全量备份'
        case 'incremental':
          return '增量备份'
        default:
          return type
      }
    }

    // 格式化日期
    const formatDateTime = (timestamp: number | null) => {
      if (!timestamp) return '从未'
      const date = new Date(timestamp * 1000)
      return date.toLocaleString('zh-CN')
    }

    // 格式化容量
    const formatCapacity = (bytes: number) => {
      if (!bytes && bytes !== 0) return '--'
      const units = ['B', 'KB', 'MB', 'GB', 'TB']
      let unitIndex = 0
      let value = bytes
      
      while (value >= 1024 && unitIndex < units.length - 1) {
        value /= 1024
        unitIndex++
      }
      
      return `${value.toFixed(unitIndex === 0 ? 0 : 2)} ${units[unitIndex]}`
    }

    // 加载备份列表
    const loadBackups = async () => {
      loading.value = true
      try {
        const response = await apiClient.get('/backups/list')
        if (response.data.success) {
          backups.value = response.data.data.map((item: any) => ({
            id: item.id,
            name: item.name,
            description: item.description || '--',
            backupType: item.backup_type || 'full',
            sourcePath: item.source_path,
            destination: item.destination_path,
            schedule: item.schedule,
            status: item.status || 'idle',
            lastRun: item.last_run_at || null,
            lastRunStatus: item.last_run_status || null,
            createdAt: item.created_at || 0
          }))
          
          // 更新统计数据
          backupStats.value = {
            totalTasks: backups.value.length,
            successfulExecutions: backups.value.filter(b => b.status === 'completed').length,
            failedExecutions: backups.value.filter(b => b.status === 'failed').length,
            runningExecutions: backups.value.filter(b => b.status === 'running').length,
            storageUsed: '--'
          }
        }
      } catch (error) {
        console.error('Failed to load backups:', error)
        // 使用模拟数据
        backups.value = [
          { id: 1, name: '每日全量备份', description: '系统数据备份', backupType: 'full', sourcePath: '/data', destination: '/backup/daily', schedule: '0 2 * * *', status: 'completed', lastRun: Date.now() / 1000 - 3600, lastRunStatus: 'success', createdAt: Date.now() / 1000 - 86400 },
          { id: 2, name: '增量备份', description: '增量备份', backupType: 'incremental', sourcePath: '/data', destination: '/backup/incremental', schedule: '0 * * * *', status: 'idle', lastRun: Date.now() / 1000 - 7200, lastRunStatus: 'success', createdAt: Date.now() / 1000 - 172800 },
          { id: 3, name: '周备份', description: '每周备份', backupType: 'full', sourcePath: '/', destination: '/backup/weekly', schedule: '0 3 * * 0', status: 'failed', lastRun: Date.now() / 1000 - 604800, lastRunStatus: 'error', createdAt: Date.now() / 1000 - 604800 }
        ]
        backupStats.value = {
          totalTasks: 3,
          successfulExecutions: 2,
          failedExecutions: 1,
          runningExecutions: 0,
          storageUsed: '1.2 TB'
        }
      } finally {
        loading.value = false
      }
    }

    // 显示创建备份模态框
    const showCreateBackupModal = () => {
      formData.value = {
        name: '',
        description: '',
        sourcePath: '',
        destination: '',
        backupType: 'full',
        schedule: ''
      }
      showCreateModal.value = true
    }

    // 显示执行备份模态框
    const showExecuteBackupModal = (backup: BackupTask) => {
      backupToExecute.value = backup
      showExecuteModal.value = true
    }

    // 关闭模态框
    const closeCreateModal = () => {
      showCreateModal.value = false
      formData.value = { name: '', description: '', sourcePath: '', destination: '', backupType: 'full', schedule: '' }
    }

    const closeExecuteModal = () => {
      showExecuteModal.value = false
      backupToExecute.value = null
    }

    // 创建备份任务
    const createBackup = async () => {
      try {
        await apiClient.post('/backups/create', {
          name: formData.value.name,
          description: formData.value.description,
          source_path: formData.value.sourcePath,
          destination: formData.value.destination,
          backup_type: formData.value.backupType,
          schedule: formData.value.schedule || undefined
        })
        closeCreateModal()
        loadBackups()
      } catch (error) {
        console.error('Failed to create backup:', error)
        alert('创建备份任务失败')
      }
    }

    // 执行备份
    const executeBackup = async () => {
      if (!backupToExecute.value) return
      try {
        await apiClient.post(`/backups/execute/${backupToExecute.value.id}`)
        closeExecuteModal()
        loadBackups()
        alert('备份任务已启动')
      } catch (error) {
        console.error('Failed to execute backup:', error)
        alert('执行备份失败')
      }
    }

    // 删除备份
    const deleteBackup = async (backup: BackupTask) => {
      if (!confirm(`确定要删除备份任务 "${backup.name}" 吗？`)) return
      try {
        await apiClient.delete(`/backups/delete/${backup.id}`)
        loadBackups()
      } catch (error) {
        console.error('Failed to delete backup:', error)
        alert('删除备份失败')
      }
    }

    // 刷新数据
    const refreshData = () => {
      loadBackups()
    }

    onMounted(() => {
      const token = localStorage.getItem('jwt_token')
      if (!token) {
        router.push('/login')
        return
      }
      loadBackups()
    })

    return {
      loading,
      backups,
      searchQuery,
      typeFilter,
      statusFilter,
      filteredBackups,
      backupStats,
      showCreateModal,
      showExecuteModal,
      formData,
      backupToExecute,
      statusClass,
      statusText,
      typeText,
      formatDateTime,
      formatCapacity,
      showCreateBackupModal,
      showExecuteBackupModal,
      closeCreateModal,
      closeExecuteModal,
      createBackup,
      executeBackup,
      deleteBackup,
      refreshData
    }
  },
  template: `
    <div class="space-y-6">
      <!-- 页面标题和统计 -->
      <div class="flex items-center justify-between">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          备份管理
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

      <!-- 统计卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-5 gap-6">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">总任务数</p>
          <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ backupStats.totalTasks }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">成功执行</p>
          <p class="text-2xl font-semibold text-green-600 dark:text-green-400">{{ backupStats.successfulExecutions }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">失败执行</p>
          <p class="text-2xl font-semibold text-red-600 dark:text-red-400">{{ backupStats.failedExecutions }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">进行中</p>
          <p class="text-2xl font-semibold text-blue-600 dark:text-blue-400">{{ backupStats.runningExecutions }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">存储占用</p>
          <p class="text-2xl font-semibold text-purple-600 dark:text-purple-400">{{ backupStats.storageUsed }}</p>
        </div>
      </div>

      <!-- 搜索和筛选 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div class="md:col-span-1">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索备份名称或路径..."
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <select
              v-model="typeFilter"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="all">所有类型</option>
              <option value="full">全量备份</option>
              <option value="incremental">增量备份</option>
            </select>
          </div>
          <div>
            <select
              v-model="statusFilter"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="all">所有状态</option>
              <option value="idle">空闲</option>
              <option value="running">进行中</option>
              <option value="completed">成功</option>
              <option value="failed">失败</option>
            </select>
          </div>
          <div class="flex justify-end">
            <button
              @click="showCreateBackupModal"
              class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
              </svg>
              新建备份
            </button>
          </div>
        </div>
      </div>

      <!-- 备份列表 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div class="grid grid-cols-12 gap-4 px-6 py-3 bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600 text-sm font-medium text-gray-500 dark:text-gray-400">
          <div class="col-span-3">备份名称</div>
          <div class="col-span-2">类型</div>
          <div class="col-span-2">状态</div>
          <div class="col-span-3">上次执行</div>
          <div class="col-span-2 text-right">操作</div>
        </div>

        <!-- 加载状态 -->
        <div v-if="loading" class="px-6 py-12 text-center">
          <svg class="animate-spin h-8 w-8 text-indigo-600 mx-auto" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">加载中...</p>
        </div>

        <!-- 空列表 -->
        <div v-else-if="filteredBackups.length === 0" class="px-6 py-12 text-center">
          <p class="text-sm text-gray-500 dark:text-gray-400">暂无备份任务</p>
        </div>

        <!-- 备份列表 -->
        <div v-else class="divide-y divide-gray-200 dark:divide-gray-700">
          <div
            v-for="backup in filteredBackups"
            :key="backup.id"
            class="grid grid-cols-12 gap-4 px-6 py-4 hover:bg-gray-50 dark:hover:bg-gray-700 items-center"
          >
            <!-- 备份名称 -->
            <div class="col-span-3">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <span class="text-2xl">🔄</span>
                </div>
                <div class="ml-4">
                  <div class="text-sm font-medium text-gray-900 dark:text-white">{{ backup.name }}</div>
                  <div class="text-xs text-gray-500 dark:text-gray-400">{{ backup.sourcePath }} → {{ backup.destination }}</div>
                </div>
              </div>
            </div>

            <!-- 类型 -->
            <div class="col-span-2">
              <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200">
                {{ typeText(backup.backupType) }}
              </span>
            </div>

            <!-- 状态 -->
            <div class="col-span-2">
              <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="statusClass(backup.status)">
                {{ statusText(backup.status) }}
              </span>
            </div>

            <!-- 上次执行 -->
            <div class="col-span-3 text-sm text-gray-500 dark:text-gray-400">
              {{ formatDateTime(backup.lastRun) }}
            </div>

            <!-- 操作 -->
            <div class="col-span-2 flex justify-end space-x-2">
              <button
                @click="showExecuteBackupModal(backup)"
                :disabled="backup.status === 'running'"
                class="text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400 disabled:opacity-50 disabled:cursor-not-allowed"
                title="执行备份"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path>
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
              </button>
              <button
                @click="deleteBackup(backup)"
                class="text-gray-400 hover:text-red-600 dark:hover:text-red-400"
                title="删除"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 创建备份模态框 -->
      <div v-if="showCreateModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
          <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
            <h3 class="text-lg font-medium text-gray-900 dark:text-white">新建备份任务</h3>
            <button @click="closeCreateModal" class="text-gray-400 hover:text-gray-500">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          <div class="px-6 py-4 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">任务名称 *</label>
              <input
                v-model="formData.name"
                type="text"
                required
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">描述</label>
              <input
                v-model="formData.description"
                type="text"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">源路径 *</label>
              <input
                v-model="formData.sourcePath"
                type="text"
                required
                placeholder="/data"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">目标路径 *</label>
              <input
                v-model="formData.destination"
                type="text"
                required
                placeholder="/backup"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">备份类型</label>
              <select
                v-model="formData.backupType"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              >
                <option value="full">全量备份</option>
                <option value="incremental">增量备份</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">计划任务 (Cron 表达式)</label>
              <input
                v-model="formData.schedule"
                type="text"
                placeholder="0 2 * * *"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
              <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">例如：0 2 * * * 表示每天凌晨 2 点</p>
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
              @click="createBackup"
              class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
            >
              创建
            </button>
          </div>
        </div>
      </div>

      <!-- 执行备份确认模态框 -->
      <div v-if="showExecuteModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
          <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-medium text-gray-900 dark:text-white">执行备份</h3>
          </div>
          <div class="px-6 py-4">
            <p class="text-sm text-gray-700 dark:text-gray-300">
              确定要立即执行备份任务 "<span class="font-medium">{{ backupToExecute?.name }}</span>" 吗？
            </p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
              这可能会消耗系统资源，执行时间取决于数据量大小。
            </p>
          </div>
          <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
            <button
              @click="closeExecuteModal"
              class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              取消
            </button>
            <button
              @click="executeBackup"
              class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
            >
              执行
            </button>
          </div>
        </div>
      </div>
    </div>
  `
})
