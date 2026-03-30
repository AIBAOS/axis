<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">下载管理</h1>
          <p class="text-gray-600 mt-1">管理下载任务（HTTP/FTP/BT）</p>
        </div>
        <div class="flex items-center space-x-3">
          <button @click="showAddModal = true" class="btn-primary flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            <span>添加下载</span>
          </button>
          <button @click="loadDownloads" :disabled="loading" class="btn-secondary">
            <svg :class="{'animate-spin': loading}" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
          </button>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总任务</p>
              <p class="text-xl font-bold text-gray-900">{{ stats.total }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">等待中</p>
              <p class="text-xl font-bold text-gray-600">{{ stats.pending }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600 animate-pulse" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">下载中</p>
              <p class="text-xl font-bold text-blue-600">{{ stats.downloading }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-green-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">已完成</p>
              <p class="text-xl font-bold text-green-600">{{ stats.completed }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">失败</p>
              <p class="text-xl font-bold text-red-600">{{ stats.failed }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 筛选栏 -->
      <div class="flex space-x-4">
        <select v-model="statusFilter" @change="loadDownloads" class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
          <option value="all">全部状态</option>
          <option value="pending">等待中</option>
          <option value="downloading">下载中</option>
          <option value="completed">已完成</option>
          <option value="failed">失败</option>
          <option value="cancelled">已取消</option>
        </select>
        <input v-model="searchQuery" type="text" placeholder="搜索任务..." class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
      </div>

      <!-- 任务列表 -->
      <div v-if="loading" class="flex justify-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
      </div>

      <div v-else-if="filteredTasks.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
        </svg>
        <p class="mt-4 text-gray-600">暂无下载任务</p>
        <button @click="showAddModal = true" class="btn-primary mt-4">添加下载任务</button>
      </div>

      <div v-else class="bg-white rounded-lg shadow overflow-hidden">
        <table class="w-full">
          <thead class="bg-gray-50 border-b">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">文件名</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase w-24">进度</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase w-20">速度</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase w-20">大小</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase w-24">状态</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase w-32">操作</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-100">
            <tr v-for="task in filteredTasks" :key="task.id" class="hover:bg-gray-50">
              <td class="px-4 py-3">
                <div class="flex items-center space-x-3">
                  <div class="w-8 h-8 rounded bg-gray-100 flex items-center justify-center flex-shrink-0">
                    <span class="text-lg">{{ getFileIcon(task.filename) }}</span>
                  </div>
                  <div class="min-w-0">
                    <p class="text-sm font-medium text-gray-900 truncate">{{ task.filename }}</p>
                    <p class="text-xs text-gray-500 truncate">{{ task.url }}</p>
                  </div>
                </div>
              </td>
              <td class="px-4 py-3">
                <div class="flex items-center space-x-2">
                  <div class="flex-1 bg-gray-200 rounded-full h-2 min-w-[60px]">
                    <div :class="getProgressClass(task.status)" class="h-2 rounded-full transition-all" :style="{ width: task.progress + '%' }"></div>
                  </div>
                  <span class="text-xs text-gray-600 w-10 text-right">{{ task.progress }}%</span>
                </div>
              </td>
              <td class="px-4 py-3 text-sm text-gray-600">{{ formatSpeed(task.speed_bps) }}</td>
              <td class="px-4 py-3 text-sm text-gray-600">{{ formatSize(task.total_bytes) }}</td>
              <td class="px-4 py-3">
                <span :class="getStatusClass(task.status)" class="px-2 py-1 text-xs rounded-full">{{ getStatusLabel(task.status) }}</span>
              </td>
              <td class="px-4 py-3 text-right">
                <button v-if="task.status === 'downloading'" @click="pauseTask(task)" class="text-yellow-600 hover:text-yellow-700 mr-2">暂停</button>
                <button v-if="task.status === 'pending'" @click="startTask(task)" class="text-blue-600 hover:text-blue-700 mr-2">开始</button>
                <button v-if="task.status === 'paused'" @click="startTask(task)" class="text-blue-600 hover:text-blue-700 mr-2">继续</button>
                <button v-if="['completed', 'failed', 'cancelled'].includes(task.status)" @click="retryTask(task)" class="text-green-600 hover:text-green-700 mr-2">重试</button>
                <select v-if="['pending', 'downloading', 'paused'].includes(task.status)" @change="changePriority(task, $event)" class="text-xs px-2 py-1 border rounded mr-2">
                  <option value="" disabled selected>优先级</option>
                  <option value="high">高</option>
                  <option value="normal">普通</option>
                  <option value="low">低</option>
                </select>
                <button @click="confirmDelete(task)" class="text-red-600 hover:text-red-700">删除</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- 分页 -->
      <div v-if="pagination.total_pages > 1" class="flex justify-center space-x-2">
        <button @click="changePage(pagination.page - 1)" :disabled="pagination.page <= 1" class="px-3 py-1 border rounded disabled:opacity-50">上一页</button>
        <span class="px-3 py-1 text-gray-600">第 {{ pagination.page }} / {{ pagination.total_pages }} 页</span>
        <button @click="changePage(pagination.page + 1)" :disabled="pagination.page >= pagination.total_pages" class="px-3 py-1 border rounded disabled:opacity-50">下一页</button>
      </div>

      <!-- 带宽限制设置 -->
      <div class="bg-white rounded-lg shadow p-4">
        <h3 class="font-semibold text-gray-900 mb-4">带宽限制</h3>
        <form @submit.prevent="saveBandwidthLimit" class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">下载限速</label>
              <div class="flex items-center space-x-2">
                <input v-model.number="bandwidthLimit.download" type="number" min="0" class="w-24 px-3 py-2 border rounded-lg" />
                <span class="text-sm text-gray-500">KB/s (0 = 不限)</span>
              </div>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">上传限速</label>
              <div class="flex items-center space-x-2">
                <input v-model.number="bandwidthLimit.upload" type="number" min="0" class="w-24 px-3 py-2 border rounded-lg" />
                <span class="text-sm text-gray-500">KB/s (0 = 不限)</span>
              </div>
            </div>
          </div>
          <div class="flex items-center justify-between">
            <label class="flex items-center space-x-2">
              <input v-model="bandwidthLimit.enabled" type="checkbox" class="h-4 w-4 rounded" />
              <span class="text-sm text-gray-700">启用带宽限制</span>
            </label>
            <button type="submit" class="btn-primary text-sm">保存设置</button>
          </div>
        </form>
      </div>

      <!-- 下载历史记录 -->
      <div class="bg-white rounded-lg shadow p-4">
        <div class="flex justify-between items-center mb-4">
          <h3 class="font-semibold text-gray-900">下载历史</h3>
          <button @click="clearHistory" class="text-sm text-red-600 hover:text-red-700">清空历史</button>
        </div>
        <div v-if="downloadHistory.length === 0" class="text-center py-4 text-gray-500 text-sm">暂无历史记录</div>
        <div v-else class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead class="bg-gray-50 border-b">
              <tr>
                <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">文件名</th>
                <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">完成时间</th>
                <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">大小</th>
                <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">耗时</th>
                <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">状态</th>
              </tr>
            </thead>
            <tbody class="divide-y">
              <tr v-for="record in downloadHistory" :key="record.id" class="hover:bg-gray-50">
                <td class="px-4 py-2 font-medium text-gray-900 truncate max-w-xs">{{ record.filename }}</td>
                <td class="px-4 py-2 text-gray-600">{{ record.completed_at }}</td>
                <td class="px-4 py-2 text-gray-600">{{ formatSize(record.size) }}</td>
                <td class="px-4 py-2 text-gray-600">{{ record.duration }}</td>
                <td class="px-4 py-2">
                  <span :class="record.status === 'success' ? 'text-green-600' : 'text-red-600'" class="text-xs font-medium">{{ record.status === 'success' ? '成功' : '失败' }}</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- 添加下载模态框 -->
    <div v-if="showAddModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4">
        <div class="flex justify-between items-center px-6 py-4 border-b">
          <h3 class="text-lg font-semibold text-gray-900">添加下载任务</h3>
          <button @click="showAddModal = false" class="text-gray-400 hover:text-gray-600">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <form @submit.prevent="createDownload" class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">下载链接 *</label>
            <textarea v-model="newTask.url" rows="3" required class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="支持 HTTP/HTTPS/FTP/磁力链接，每行一个"></textarea>
            <p class="text-xs text-gray-500 mt-1">支持批量添加，每行一个链接</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">保存路径</label>
            <input v-model="newTask.save_path" type="text" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="/downloads" />
          </div>
          <div v-if="createError" class="p-3 bg-red-50 text-red-600 text-sm rounded">{{ createError }}</div>
        </form>
        <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
          <button @click="showAddModal = false" class="px-4 py-2 text-gray-600 hover:text-gray-800">取消</button>
          <button @click="createDownload" :disabled="creating" class="btn-primary disabled:opacity-50">{{ creating ? '添加中...' : '添加' }}</button>
        </div>
      </div>
    </div>

    <!-- 删除确认 -->
    <div v-if="deleteTarget" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
        <div class="p-4">
          <p class="text-gray-900">确定删除 "{{ deleteTarget.filename }}" 吗？</p>
          <p class="text-sm text-gray-500 mt-1">已下载的文件将保留</p>
        </div>
        <div class="flex justify-end space-x-3 px-4 py-3 bg-gray-50 rounded-b-lg">
          <button @click="deleteTarget = null" class="px-4 py-2 text-gray-600 hover:text-gray-800">取消</button>
          <button @click="executeDelete" class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700">删除</button>
        </div>
      </div>
    </div>

    <!-- Toast -->
    <div v-if="toast.show" class="fixed bottom-4 right-4 z-50">
      <div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">{{ toast.message }}</div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import { api } from '@/utils/api'
import { useToast } from '@/composables/useToast'

const { showToast } = useToast()

const loading = ref(true)
const tasks = ref<any[]>([])
const stats = ref({ total: 0, pending: 0, downloading: 0, completed: 0, failed: 0, cancelled: 0 })
const statusFilter = ref('all')
const searchQuery = ref('')
const pagination = ref({ page: 1, limit: 20, total: 0, total_pages: 0 })

// 模态框
const showAddModal = ref(false)
const newTask = ref({ url: '', save_path: '/downloads' })
const creating = ref(false)
const createError = ref('')
const deleteTarget = ref<any>(null)

// Toast


// 自动刷新
let refreshTimer: ReturnType<typeof setInterval> | null = null

const filteredTasks = computed(() => {
  if (!searchQuery.value) return tasks.value
  const q = searchQuery.value.toLowerCase()
  return tasks.value.filter(t => t.filename?.toLowerCase().includes(q) || t.url?.toLowerCase().includes(q))
})

const loadDownloads = async () => {
  loading.value = true
  try {
    const params: any = { page: pagination.value.page, per_page: pagination.value.limit }
    if (statusFilter.value !== 'all') params.status = statusFilter.value
    
    const response = await api.downloads.list(params)
    tasks.value = response.data.data || []
    pagination.value = response.data.pagination || pagination.value
  } catch (error) {
    showToast('error', '加载失败')
  } finally {
    loading.value = false
  }
}

const loadStats = async () => {
  try {
    const response = await api.downloads.stats()
    stats.value = response.data
  } catch (error) {
    console.error('Failed to load stats:', error)
  }
}

const createDownload = async () => {
  if (!newTask.value.url.trim()) {
    createError.value = '请输入下载链接'
    return
  }
  
  creating.value = true
  createError.value = ''
  
  try {
    const urls = newTask.value.url.trim().split('\n').filter(Boolean)
    let success = 0
    let failed = 0
    
    for (const url of urls) {
      try {
        await api.downloads.create({
          url: url.trim(),
          save_path: newTask.value.save_path || undefined
        })
        success++
      } catch (e) {
        failed++
      }
    }
    
    showAddModal.value = false
    newTask.value = { url: '', save_path: '/downloads' }
    loadDownloads()
    loadStats()
    
    if (failed === 0) {
      showToast('success', `已添加 ${success} 个下载任务`)
    } else {
      showToast('success', `添加完成：成功 ${success}，失败 ${failed}`)
    }
  } catch (error: any) {
    createError.value = error.message || '添加失败'
  } finally {
    creating.value = false
  }
}

const startTask = async (task: any) => {
  try {
    await api.downloads.start(task.id)
    showToast('success', '任务已开始')
    loadDownloads()
  } catch (e) {
    showToast('error', '启动失败')
  }
}

const pauseTask = async (task: any) => {
  try {
    await api.downloads.pause(task.id)
    showToast('success', '任务已暂停')
    loadDownloads()
  } catch (e) {
    showToast('error', '暂停失败')
  }
}

const retryTask = async (task: any) => {
  try {
    await api.downloads.retry(task.id)
    showToast('success', '任务已重新开始')
    loadDownloads()
  } catch (e) {
    showToast('error', '重试失败')
  }
}

const confirmDelete = (task: any) => {
  deleteTarget.value = task
}

const executeDelete = async () => {
  if (!deleteTarget.value) return
  try {
    await api.downloads.delete(deleteTarget.value.id)
    showToast('success', '已删除')
    deleteTarget.value = null
    loadDownloads()
    loadStats()
  } catch (e) {
    showToast('error', '删除失败')
  }
}

const changePage = (page: number) => {
  pagination.value.page = page
  loadDownloads()
}

const getFileIcon = (filename: string) => {
  const ext = filename?.split('.').pop()?.toLowerCase()
  if (['mp4', 'mkv', 'avi', 'mov'].includes(ext || '')) return '🎬'
  if (['mp3', 'flac', 'wav'].includes(ext || '')) return '🎵'
  if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext || '')) return '📦'
  if (['iso', 'img', 'dmg'].includes(ext || '')) return '💿'
  if (['pdf', 'doc', 'docx'].includes(ext || '')) return '📄'
  return '📁'
}

const formatSize = (bytes: number) => {
  if (!bytes) return '-'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return (bytes / Math.pow(k, i)).toFixed(1) + ' ' + sizes[i]
}

const formatSpeed = (bps: number) => {
  if (!bps) return '-'
  return formatSize(bps) + '/s'
}

const getStatusClass = (status: string) => {
  switch (status) {
    case 'pending': return 'bg-gray-100 text-gray-700'
    case 'downloading': return 'bg-blue-100 text-blue-700'
    case 'completed': return 'bg-green-100 text-green-700'
    case 'failed': return 'bg-red-100 text-red-700'
    case 'cancelled': return 'bg-yellow-100 text-yellow-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

const getStatusLabel = (status: string) => {
  switch (status) {
    case 'pending': return '等待中'
    case 'downloading': return '下载中'
    case 'completed': return '已完成'
    case 'failed': return '失败'
    case 'cancelled': return '已取消'
    default: return status
  }
}

const getProgressClass = (status: string) => {
  if (status === 'failed') return 'bg-red-500'
  if (status === 'completed') return 'bg-green-500'
  return 'bg-blue-500'
}

// 带宽限制
const bandwidthLimit = ref({
  enabled: false,
  download: 0,
  upload: 0
})

const saveBandwidthLimit = async () => {
  try {
    await api.settings.update({ bandwidth_limit: bandwidthLimit.value })
    showToast('success', '带宽限制设置已保存')
  } catch (e) {
    showToast('error', '保存失败')
  }
}

// 优先级调整
const changePriority = async (task: any, event: Event) => {
  const priority = (event.target as HTMLSelectElement).value
  if (!priority) return
  try {
    await api.downloads.update?.(task.id, { priority })
    showToast('success', `优先级已设为${priority === 'high' ? '高' : priority === 'low' ? '低' : '普通'}`)
    loadDownloads()
  } catch (e) {
    showToast('error', '设置失败')
  }
}

// 下载历史记录
const downloadHistory = ref([
  { id: 1, filename: 'ubuntu-24.04-desktop-amd64.iso', completed_at: '2026-03-29 10:30', size: 5368709120, duration: '45分', status: 'success' },
  { id: 2, filename: 'movie_2026.mkv', completed_at: '2026-03-28 22:15', size: 2147483648, duration: '25分', status: 'success' },
  { id: 3, filename: 'backup_2026-03-27.tar.gz', completed_at: '2026-03-27 18:00', size: 1073741824, duration: '10分', status: 'failed' }
])

const clearHistory = () => {
  if (!confirm('确定清空下载历史记录吗？')) return
  downloadHistory.value = []
  showToast('success', '历史记录已清空')
}

onMounted(() => {
  loadDownloads()
  loadStats()
  // 每 5 秒自动刷新
  refreshTimer = setInterval(() => {
    loadDownloads()
    loadStats()
  }, 5000)
})

onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer)
})
</script>