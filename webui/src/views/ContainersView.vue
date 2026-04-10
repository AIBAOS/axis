<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">容器管理</h1>
          <p class="text-gray-600 mt-1">管理Docker容器实例</p>
        </div>
        <button @click="showCreateModal = true" class="btn-primary flex items-center space-x-2">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span>创建容器</span>
        </button>
      </div>

      <!-- 状态统计 -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总容器</p>
              <p class="text-xl font-bold text-gray-900">{{ containers.length }}</p>
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
              <p class="text-sm text-gray-500">运行中</p>
              <p class="text-xl font-bold text-green-700">{{ statusCounts.running }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-yellow-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m3-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">已停止</p>
              <p class="text-xl font-bold text-yellow-700">{{ statusCounts.stopped }}</p>
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
              <p class="text-sm text-gray-500">已暂停</p>
              <p class="text-xl font-bold text-gray-700">{{ statusCounts.paused }}</p>
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
              <p class="text-sm text-gray-500">错误</p>
              <p class="text-xl font-bold text-red-700">{{ statusCounts.error }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 筛选 -->
      <div class="flex space-x-4">
        <input v-model="searchQuery" type="text" placeholder="搜索容器..." class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
        <select v-model="statusFilter" class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
          <option value="all">全部状态</option>
          <option value="running">运行中</option>
          <option value="stopped">已停止</option>
          <option value="paused">已暂停</option>
          <option value="error">错误</option>
        </select>
        <button @click="loadContainers" :disabled="loading" class="btn-secondary">
          <svg :class="{'animate-spin': loading}" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span class="ml-1">刷新</span>
        </button>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="flex justify-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
      </div>

      <!-- 空状态 -->
      <div v-else-if="filteredContainers.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
        </svg>
        <p class="mt-4 text-gray-600">暂无容器</p>
        <button @click="showCreateModal = true" class="mt-4 btn-primary">创建容器</button>
      </div>

      <!-- 容器列表 -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div v-for="container in filteredContainers" :key="container.id" class="bg-white rounded-lg shadow p-6">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-medium text-gray-900">{{ container.name }}</h3>
            <span :class="getStatusClass(container.status)" class="px-2 py-1 text-xs font-medium rounded-full">
              {{ getStatusLabel(container.status) }}
            </span>
          </div>

          <p class="text-sm text-gray-600 mb-4">{{ container.image }}</p>

          <div class="space-y-2 text-sm text-gray-600 mb-4">
            <div class="flex justify-between">
              <span>ID:</span>
              <span class="font-mono text-gray-500 truncate max-w-xs">{{ container.id }}</span>
            </div>
            <div v-if="container.ports?.length" class="flex justify-between">
              <span>端口:</span>
              <span class="font-medium">{{ container.ports.join(', ') }}</span>
            </div>
            <div v-if="container.networks?.length" class="flex justify-between">
              <span>网络:</span>
              <span class="font-medium">{{ container.networks.join(', ') }}</span>
            </div>
          </div>

          <div class="flex items-center justify-between pt-4 border-t border-gray-200">
            <span class="text-xs text-gray-500">创建于 {{ formatDate(container.created_at) }}</span>
            <div class="flex space-x-2">
              <button @click="viewLogs(container)" class="text-purple-600 hover:text-purple-800 text-sm" title="查看日志">
                📋 日志
              </button>
              <button v-if="container.status === 'running'" @click="stopContainer(container)" class="text-yellow-600 hover:text-yellow-800 text-sm" title="停止">
                ⏸️ 停止
              </button>
              <button v-if="container.status === 'stopped'" @click="startContainer(container)" class="text-green-600 hover:text-green-800 text-sm" title="启动">
                ▶️ 启动
              </button>
              <button @click="restartContainer(container)" class="text-blue-600 hover:text-blue-800 text-sm" title="重启">
                🔄 重启
              </button>
              <button @click="confirmDelete(container)" class="text-red-600 hover:text-red-800 text-sm" title="删除">
                🗑️ 删除
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 创建容器模态框 -->
      <div v-if="showCreateModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4">
          <div class="flex justify-between items-center px-6 py-4 border-b">
            <h3 class="text-lg font-semibold text-gray-900">创建容器</h3>
            <button @click="showCreateModal = false" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <form @submit.prevent="createContainer" class="px-6 py-4 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">容器名称 *</label>
              <input v-model="formData.name" type="text" required class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary-500" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">镜像 *</label>
              <input v-model="formData.image" type="text" required placeholder="nginx:latest" class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary-500" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">端口映射</label>
              <input v-model="formData.ports" type="text" placeholder="80:80, 443:443" class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary-500" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">网络</label>
              <input v-model="formData.networks" type="text" placeholder="bridge" class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary-500" />
            </div>
            <div class="flex justify-end space-x-3 pt-4">
              <button type="button" @click="showCreateModal = false" class="px-4 py-2 border rounded-md text-sm text-gray-700 hover:bg-gray-50">取消</button>
              <button type="submit" :disabled="submitting" class="px-4 py-2 bg-primary-600 text-white rounded-md text-sm hover:bg-primary-700 disabled:opacity-50">
                {{ submitting ? '创建中...' : '创建' }}
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- 日志查看模态框 -->
      <div v-if="logsContainer" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-hidden">
          <div class="flex justify-between items-center px-6 py-4 border-b">
            <h3 class="text-lg font-semibold text-gray-900">{{ logsContainer.name }} 日志</h3>
            <button @click="logsContainer = null; containerLogs = ''" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div class="px-6 py-4 overflow-y-auto max-h-[60vh]">
            <pre class="text-sm text-gray-800 bg-gray-50 p-4 rounded-lg font-mono whitespace-pre-wrap">{{ containerLogs || '暂无日志' }}</pre>
          </div>
          <div class="px-6 py-4 bg-gray-50 flex justify-between">
            <button @click="refreshLogs" class="btn-secondary text-sm">刷新日志</button>
            <button @click="logsContainer = null; containerLogs = ''" class="btn-primary text-sm">关闭</button>
          </div>
        </div>
      </div>

      <!-- 删除确认对话框 -->
      <div v-if="deleteTarget" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">确认删除</h3>
          <p class="text-sm text-gray-600 mb-6">确定要删除容器 "{{ deleteTarget.name }}" 吗？此操作不可恢复！</p>
          <div class="flex justify-end space-x-3">
            <button @click="deleteTarget = null" class="px-4 py-2 border rounded-md text-sm text-gray-700 hover:bg-gray-50">取消</button>
            <button @click="executeDelete" class="px-4 py-2 bg-red-600 text-white rounded-md text-sm hover:bg-red-700">删除</button>
          </div>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import { api } from '@/utils/api'
import { useToast } from '@/composables/useToast'

const { showToast } = useToast()

const loading = ref(true)
const containers = ref<any[]>([])
const searchQuery = ref('')
const statusFilter = ref('all')

const showCreateModal = ref(false)
const submitting = ref(false)
const formData = ref({
  name: '',
  image: '',
  ports: '',
  networks: ''
})

const logsContainer = ref<any>(null)
const containerLogs = ref('')
const deleteTarget = ref<any>(null)

const statusCounts = computed(() => ({
  running: containers.value.filter(c => c.status === 'running').length,
  stopped: containers.value.filter(c => c.status === 'stopped' || c.status === 'created').length,
  paused: containers.value.filter(c => c.status === 'paused').length,
  error: containers.value.filter(c => c.status === 'error' || c.status === 'dead').length
}))

const filteredContainers = computed(() => {
  let result = containers.value
  if (searchQuery.value) {
    result = result.filter(c => 
      c.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      c.image.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  }
  if (statusFilter.value !== 'all') {
    result = result.filter(c => c.status === statusFilter.value)
  }
  return result
})

const loadContainers = async () => {
  loading.value = true
  try {
    const res = await api.containers.list()
    containers.value = res.data.data || []
  } catch (e) {
    showToast('加载容器列表失败', 'error')
  } finally {
    loading.value = false
  }
}

const createContainer = async () => {
  submitting.value = true
  try {
    const data = {
      name: formData.value.name,
      image: formData.value.image,
      ports: formData.value.ports ? formData.value.ports.split(',').map(p => p.trim()) : [],
      networks: formData.value.networks ? formData.value.networks.split(',').map(n => n.trim()) : ['bridge']
    }
    await api.containers.create(data)
    showToast('容器创建成功', 'success')
    showCreateModal.value = false
    formData.value = { name: '', image: '', ports: '', networks: '' }
    await loadContainers()
  } catch (e) {
    showToast('创建容器失败', 'error')
  } finally {
    submitting.value = false
  }
}

const startContainer = async (container: any) => {
  try {
    await api.containers.start(container.id)
    showToast(`容器 ${container.name} 已启动`, 'success')
    await loadContainers()
  } catch (e) {
    showToast('启动失败', 'error')
  }
}

const stopContainer = async (container: any) => {
  try {
    await api.containers.stop(container.id)
    showToast(`容器 ${container.name} 已停止`, 'success')
    await loadContainers()
  } catch (e) {
    showToast('停止失败', 'error')
  }
}

const restartContainer = async (container: any) => {
  try {
    await api.containers.restart(container.id)
    showToast(`容器 ${container.name} 已重启`, 'success')
    await loadContainers()
  } catch (e) {
    showToast('重启失败', 'error')
  }
}

const viewLogs = async (container: any) => {
  logsContainer.value = container
  try {
    const res = await api.containers.logs(container.id)
    containerLogs.value = res.data.data || '暂无日志'
  } catch (e) {
    containerLogs.value = '获取日志失败'
  }
}

const refreshLogs = async () => {
  if (logsContainer.value) {
    await viewLogs(logsContainer.value)
    showToast('日志已刷新', 'info')
  }
}

const confirmDelete = (container: any) => {
  deleteTarget.value = container
}

const executeDelete = async () => {
  if (!deleteTarget.value) return
  try {
    await api.containers.delete(deleteTarget.value.id)
    showToast('容器已删除', 'success')
    deleteTarget.value = null
    await loadContainers()
  } catch (e) {
    showToast('删除失败', 'error')
  }
}

const getStatusClass = (status: string) => ({
  'bg-green-100 text-green-800': status === 'running',
  'bg-yellow-100 text-yellow-800': status === 'stopped' || status === 'created',
  'bg-gray-100 text-gray-800': status === 'paused',
  'bg-red-100 text-red-800': status === 'error' || status === 'dead'
}[status] || 'bg-gray-100 text-gray-800')

const getStatusLabel = (status: string) => ({
  running: '运行中',
  stopped: '已停止',
  created: '已创建',
  paused: '已暂停',
  error: '错误',
  dead: '已终止'
}[status] || status)

const formatDate = (date: string | number) => {
  if (!date) return '-'
  const d = new Date(date)
  return d.toLocaleDateString('zh-CN')
}

onMounted(() => {
  loadContainers()
})
</script>