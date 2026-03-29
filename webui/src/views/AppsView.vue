<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">应用中心</h1>
          <p class="text-gray-600 mt-1">管理已安装应用和发现新应用</p>
        </div>
        <div class="flex items-center space-x-3">
          <button @click="refreshApps" :disabled="loading" class="btn-secondary flex items-center space-x-2">
            <svg :class="{'animate-spin': loading}" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>刷新</span>
          </button>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总应用</p>
              <p class="text-xl font-bold">{{ stats.total }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-green-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">运行中</p>
              <p class="text-xl font-bold text-green-600">{{ stats.running }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-yellow-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">已停止</p>
              <p class="text-xl font-bold text-yellow-600">{{ stats.stopped }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-purple-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">可更新</p>
              <p class="text-xl font-bold text-purple-600">{{ stats.updates }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">错误</p>
              <p class="text-xl font-bold text-red-600">{{ stats.error }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 筛选栏 -->
      <div class="flex space-x-4">
        <select v-model="statusFilter" @change="loadApps" class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
          <option value="all">全部状态</option>
          <option value="running">运行中</option>
          <option value="stopped">已停止</option>
          <option value="error">错误</option>
        </select>
        <select v-model="categoryFilter" @change="loadApps" class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
          <option value="all">全部类别</option>
          <option value="media">媒体服务</option>
          <option value="download">下载工具</option>
          <option value="productivity">生产力</option>
          <option value="network">网络工具</option>
          <option value="system">系统工具</option>
          <option value="other">其他</option>
        </select>
        <input v-model="searchQuery" type="text" placeholder="搜索应用..." class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="flex justify-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
      </div>

      <!-- 空状态 -->
      <div v-else-if="filteredApps.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
        </svg>
        <p class="mt-4 text-gray-600">暂无应用</p>
        <button @click="showInstallModal = true" class="btn-primary mt-4">安装应用</button>
      </div>

      <!-- 应用卡片网格 -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        <div v-for="app in filteredApps" :key="app.id" 
          class="bg-white rounded-lg shadow hover:shadow-lg transition-shadow border border-gray-200 cursor-pointer"
          @click="showAppDetail(app)">
          <!-- 卡片头部 -->
          <div class="p-4 flex items-start space-x-3">
            <div :class="getCategoryBgClass(app.category)" class="w-14 h-14 rounded-lg flex items-center justify-center flex-shrink-0">
              <span class="text-2xl">{{ getCategoryIcon(app.category) }}</span>
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between">
                <h3 class="font-semibold text-gray-900 truncate">{{ app.name }}</h3>
                <span :class="getStatusClass(app.status)" class="px-2 py-0.5 text-xs rounded-full">
                  {{ getStatusLabel(app.status) }}
                </span>
              </div>
              <p class="text-sm text-gray-500 truncate mt-1">{{ app.description || '暂无描述' }}</p>
              <div class="flex items-center space-x-3 mt-2 text-xs text-gray-400">
                <span>v{{ app.version }}</span>
                <span v-if="app.size_bytes">{{ formatSize(app.size_bytes) }}</span>
              </div>
            </div>
          </div>

          <!-- 卡片操作 -->
          <div class="px-4 py-3 bg-gray-50 rounded-b-lg flex justify-between items-center">
            <div class="flex items-center space-x-1">
              <button v-if="app.status === 'running'" @click.stop="stopApp(app)" class="p-1.5 text-yellow-600 hover:bg-yellow-50 rounded" title="停止">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z" />
                </svg>
              </button>
              <button v-else @click.stop="startApp(app)" class="p-1.5 text-green-600 hover:bg-green-50 rounded" title="启动">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </button>
              <button @click.stop="restartApp(app)" class="p-1.5 text-blue-600 hover:bg-blue-50 rounded" title="重启">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
              </button>
            </div>
            <div class="flex items-center space-x-1">
              <button v-if="app.update_available" @click.stop="updateApp(app)" class="px-2 py-1 text-xs bg-purple-100 text-purple-700 rounded hover:bg-purple-200">
                更新
              </button>
              <button @click.stop="confirmUninstall(app)" class="p-1.5 text-red-600 hover:bg-red-50 rounded" title="卸载">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 分页 -->
      <div v-if="pagination.total_pages > 1" class="flex justify-center space-x-2">
        <button @click="changePage(pagination.page - 1)" :disabled="pagination.page <= 1" class="px-3 py-1 border rounded disabled:opacity-50">上一页</button>
        <span class="px-3 py-1 text-gray-600">第 {{ pagination.page }} / {{ pagination.total_pages }} 页</span>
        <button @click="changePage(pagination.page + 1)" :disabled="pagination.page >= pagination.total_pages" class="px-3 py-1 border rounded disabled:opacity-50">下一页</button>
      </div>

      <!-- 安装模态框 -->
      <div v-if="showInstallModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4">
          <div class="flex justify-between items-center px-6 py-4 border-b">
            <h3 class="text-lg font-semibold text-gray-900">安装应用</h3>
            <button @click="showInstallModal = false" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <form @submit.prevent="installApp" class="p-6 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">应用名称</label>
              <input v-model="installForm.name" type="text" required class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="jellyfin" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">版本</label>
              <input v-model="installForm.version" type="text" required class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="latest" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">类别</label>
              <select v-model="installForm.category" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
                <option value="media">媒体服务</option>
                <option value="download">下载工具</option>
                <option value="productivity">生产力</option>
                <option value="network">网络工具</option>
                <option value="system">系统工具</option>
                <option value="other">其他</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">描述</label>
              <textarea v-model="installForm.description" rows="2" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="应用描述"></textarea>
            </div>
          </form>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button @click="showInstallModal = false" class="px-4 py-2 border rounded-lg text-gray-700 hover:bg-gray-50">取消</button>
            <button @click="installApp" :disabled="installing" class="btn-primary disabled:opacity-50">{{ installing ? '安装中...' : '安装' }}</button>
          </div>
        </div>
      </div>

      <!-- 应用详情模态框 -->
      <div v-if="selectedApp" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" @click="selectedApp = null">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4" @click.stop>
          <div class="flex justify-between items-center px-6 py-4 border-b">
            <div class="flex items-center space-x-3">
              <div :class="getCategoryBgClass(selectedApp.category)" class="w-12 h-12 rounded-lg flex items-center justify-center">
                <span class="text-2xl">{{ getCategoryIcon(selectedApp.category) }}</span>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-gray-900">{{ selectedApp.name }}</h3>
                <p class="text-sm text-gray-500">v{{ selectedApp.version }}</p>
              </div>
            </div>
            <button @click="selectedApp = null" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div class="p-6 space-y-4">
            <div>
              <p class="text-sm text-gray-500">描述</p>
              <p class="text-gray-900">{{ selectedApp.description || '暂无描述' }}</p>
            </div>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <p class="text-sm text-gray-500">状态</p>
                <span :class="getStatusClass(selectedApp.status)" class="px-2 py-1 text-xs rounded-full">
                  {{ getStatusLabel(selectedApp.status) }}
                </span>
              </div>
              <div>
                <p class="text-sm text-gray-500">类别</p>
                <span class="px-2 py-1 text-xs rounded-full bg-gray-100 text-gray-700">
                  {{ getCategoryLabel(selectedApp.category) }}
                </span>
              </div>
              <div>
                <p class="text-sm text-gray-500">大小</p>
                <p class="text-gray-900">{{ formatSize(selectedApp.size_bytes) }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500">安装时间</p>
                <p class="text-gray-900">{{ formatDate(selectedApp.installed_at) }}</p>
              </div>
            </div>
          </div>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button v-if="selectedApp.status === 'running'" @click="stopApp(selectedApp); selectedApp = null" class="px-4 py-2 border border-yellow-300 text-yellow-600 rounded-lg hover:bg-yellow-50">停止</button>
            <button v-else @click="startApp(selectedApp); selectedApp = null" class="px-4 py-2 border border-green-300 text-green-600 rounded-lg hover:bg-green-50">启动</button>
            <button @click="restartApp(selectedApp); selectedApp = null" class="px-4 py-2 border border-blue-300 text-blue-600 rounded-lg hover:bg-blue-50">重启</button>
            <button @click="confirmUninstall(selectedApp); selectedApp = null" class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700">卸载</button>
          </div>
        </div>
      </div>

      <!-- 卸载确认 -->
      <div v-if="uninstallTarget" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
          <div class="p-6">
            <h3 class="text-lg font-semibold text-gray-900 mb-2">确认卸载</h3>
            <p class="text-gray-600">确定要卸载 "{{ uninstallTarget.name }}" 吗？</p>
            <p class="text-sm text-gray-500 mt-1">此操作将删除应用及其数据</p>
          </div>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button @click="uninstallTarget = null" class="px-4 py-2 text-gray-600 hover:text-gray-800">取消</button>
            <button @click="executeUninstall" :disabled="uninstalling" class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50">
              {{ uninstalling ? '卸载中...' : '确认卸载' }}
            </button>
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
import { api } from '@/utils/api'

const loading = ref(true)
const apps = ref<any[]>([])
const statusFilter = ref('all')
const categoryFilter = ref('all')
const searchQuery = ref('')
const pagination = ref({ page: 1, per_page: 20, total: 0, total_pages: 0 })

// 模态框
const showInstallModal = ref(false)
const installing = ref(false)
const installForm = ref({ name: '', version: 'latest', category: 'other', description: '' })
const selectedApp = ref<any>(null)
const uninstallTarget = ref<any>(null)
const uninstalling = ref(false)

// Toast
const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

// 统计
const stats = computed(() => ({
  total: apps.value.length,
  running: apps.value.filter(a => a.status === 'running').length,
  stopped: apps.value.filter(a => a.status === 'stopped').length,
  updates: apps.value.filter(a => a.update_available).length,
  error: apps.value.filter(a => a.status === 'error').length
}))

// 筛选
const filteredApps = computed(() => {
  let result = apps.value
  if (statusFilter.value !== 'all') {
    result = result.filter(a => a.status === statusFilter.value)
  }
  if (categoryFilter.value !== 'all') {
    result = result.filter(a => a.category === categoryFilter.value)
  }
  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase()
    result = result.filter(a => a.name?.toLowerCase().includes(q) || a.description?.toLowerCase().includes(q))
  }
  return result
})

// 加载应用
const loadApps = async () => {
  loading.value = true
  try {
    const params: any = { page: pagination.value.page, per_page: pagination.value.per_page }
    if (statusFilter.value !== 'all') params.status = statusFilter.value
    if (categoryFilter.value !== 'all') params.category = categoryFilter.value
    
    const response = await api.apps.list(params)
    apps.value = response.data.data || []
    pagination.value = response.data.pagination || pagination.value
  } catch (error) {
    console.error('Failed to load apps:', error)
    // 模拟数据
    apps.value = [
      { id: 1, name: 'Jellyfin', version: '10.8.13', description: '媒体服务器', category: 'media', status: 'running', size_bytes: 500000000, installed_at: '2026-03-15T10:00:00Z', update_available: false },
      { id: 2, name: 'Transmission', version: '4.0.3', description: 'BitTorrent 下载客户端', category: 'download', status: 'running', size_bytes: 50000000, installed_at: '2026-03-16T10:00:00Z', update_available: true },
      { id: 3, name: 'Nextcloud', version: '28.0.2', description: '私有云存储和协作平台', category: 'productivity', status: 'stopped', size_bytes: 800000000, installed_at: '2026-03-17T10:00:00Z', update_available: false },
      { id: 4, name: 'Pi-hole', version: '5.17.1', description: '网络广告拦截', category: 'network', status: 'running', size_bytes: 100000000, installed_at: '2026-03-18T10:00:00Z', update_available: false },
      { id: 5, name: 'Portainer', version: '2.19.4', description: 'Docker 管理界面', category: 'system', status: 'running', size_bytes: 80000000, installed_at: '2026-03-19T10:00:00Z', update_available: true }
    ]
  } finally {
    loading.value = false
  }
}

const refreshApps = () => loadApps()

// 安装应用
const installApp = async () => {
  if (!installForm.value.name || !installForm.value.version) return
  installing.value = true
  try {
    await api.apps.install(installForm.value)
    showToast('success', `${installForm.value.name} 安装成功`)
    showInstallModal.value = false
    installForm.value = { name: '', version: 'latest', category: 'other', description: '' }
    loadApps()
  } catch (error) {
    showToast('error', '安装失败')
  } finally {
    installing.value = false
  }
}

// 启动/停止/重启
const startApp = async (app: any) => {
  try {
    await api.apps.start?.(app.id)
    app.status = 'running'
    showToast('success', `${app.name} 已启动`)
  } catch (error) {
    showToast('error', '启动失败')
  }
}

const stopApp = async (app: any) => {
  try {
    await api.apps.stop?.(app.id)
    app.status = 'stopped'
    showToast('success', `${app.name} 已停止`)
  } catch (error) {
    showToast('error', '停止失败')
  }
}

const restartApp = async (app: any) => {
  try {
    await api.apps.restart?.(app.id)
    showToast('success', `${app.name} 正在重启`)
  } catch (error) {
    showToast('error', '重启失败')
  }
}

const updateApp = async (app: any) => {
  try {
    await api.apps.update?.(app.id)
    app.update_available = false
    showToast('success', `${app.name} 更新成功`)
  } catch (error) {
    showToast('error', '更新失败')
  }
}

// 卸载
const confirmUninstall = (app: any) => {
  uninstallTarget.value = app
}

const executeUninstall = async () => {
  if (!uninstallTarget.value) return
  uninstalling.value = true
  try {
    await api.apps.uninstall(uninstallTarget.value.id)
    showToast('success', `${uninstallTarget.value.name} 已卸载`)
    uninstallTarget.value = null
    loadApps()
  } catch (error) {
    showToast('error', '卸载失败')
  } finally {
    uninstalling.value = false
  }
}

// 详情
const showAppDetail = (app: any) => {
  selectedApp.value = app
}

// 分页
const changePage = (page: number) => {
  pagination.value.page = page
  loadApps()
}

// 样式
const getStatusClass = (status: string) => {
  switch (status) {
    case 'running': return 'bg-green-100 text-green-700'
    case 'stopped': return 'bg-yellow-100 text-yellow-700'
    case 'error': return 'bg-red-100 text-red-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

const getStatusLabel = (status: string) => {
  switch (status) {
    case 'running': return '运行中'
    case 'stopped': return '已停止'
    case 'error': return '错误'
    default: return status
  }
}

const getCategoryIcon = (category: string) => {
  switch (category) {
    case 'media': return '🎬'
    case 'download': return '📥'
    case 'productivity': return '💼'
    case 'network': return '🌐'
    case 'system': return '⚙️'
    default: return '📦'
  }
}

const getCategoryBgClass = (category: string) => {
  switch (category) {
    case 'media': return 'bg-purple-100'
    case 'download': return 'bg-blue-100'
    case 'productivity': return 'bg-green-100'
    case 'network': return 'bg-orange-100'
    case 'system': return 'bg-gray-100'
    default: return 'bg-gray-100'
  }
}

const getCategoryLabel = (category: string) => {
  switch (category) {
    case 'media': return '媒体服务'
    case 'download': return '下载工具'
    case 'productivity': return '生产力'
    case 'network': return '网络工具'
    case 'system': return '系统工具'
    default: return '其他'
  }
}

// 格式化
const formatSize = (bytes: number) => {
  if (!bytes) return '-'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

const formatDate = (timestamp: string) => {
  if (!timestamp) return '-'
  return new Date(timestamp).toLocaleDateString('zh-CN')
}

// Toast
const showToast = (type: 'success' | 'error', message: string) => {
  toast.value = { show: true, type, message }
  setTimeout(() => toast.value.show = false, 3000)
}

onMounted(() => loadApps())
</script>