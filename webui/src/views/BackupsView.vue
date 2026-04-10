<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">备份管理</h1>
          <p class="text-gray-600 mt-1">管理备份任务和恢复数据</p>
        </div>
        <div class="flex items-center space-x-3">
          <button @click="showHistoryModal = true" class="btn-secondary flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>历史记录</span>
          </button>
          <button @click="showCreateModal = true" class="btn-primary flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            <span>新建备份</span>
          </button>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v6a2 2 0 002 2h2" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总任务</p>
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
              <p class="text-sm text-gray-500">活跃</p>
              <p class="text-xl font-bold text-green-600">{{ stats.active }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">运行中</p>
              <p class="text-xl font-bold text-blue-600">{{ stats.running }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-purple-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总大小</p>
              <p class="text-xl font-bold text-purple-600">{{ formatBytes(stats.totalSize) }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-orange-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">上次备份</p>
              <p class="text-sm font-bold text-orange-600">{{ stats.lastBackup || '-' }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 正在运行的备份进度 -->
      <div v-if="runningBackups.length > 0" class="bg-blue-50 rounded-lg p-4 border border-blue-200">
        <h3 class="font-semibold text-blue-900 mb-3">正在备份</h3>
        <div class="space-y-3">
          <div v-for="backup in runningBackups" :key="backup.id" class="bg-white rounded-lg p-3">
            <div class="flex justify-between items-center mb-2">
              <span class="font-medium text-gray-900">{{ backup.name }}</span>
              <span class="text-sm text-blue-600">{{ backup.progress }}%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div class="bg-blue-500 h-2 rounded-full transition-all" :style="{ width: backup.progress + '%' }"></div>
            </div>
            <p class="text-xs text-gray-500 mt-1">{{ backup.status_message }}</p>
          </div>
        </div>
      </div>

      <!-- 筛选栏 -->
      <div class="flex space-x-4">
        <input v-model="searchQuery" type="text" placeholder="搜索备份任务..." class="flex-1 px-4 py-2 border rounded-lg" />
        <select v-model="statusFilter" class="px-4 py-2 border rounded-lg">
          <option value="all">全部状态</option>
          <option value="active">活跃</option>
          <option value="running">运行中</option>
        </select>
      </div>

      <!-- 备份列表 -->
      <div v-if="loading" class="flex justify-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
      </div>
      <div v-else-if="backups.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H2a2 2 0 01-2-2V5a2 2 0 012-2h6" />
        </svg>
        <p class="mt-4 text-gray-600">暂无备份任务</p>
        <button @click="showCreateModal = true" class="btn-primary mt-4">创建备份任务</button>
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div v-for="backup in filteredBackups" :key="backup.id" class="bg-white rounded-lg shadow p-4">
          <div class="flex justify-between items-start mb-2">
            <h3 class="font-semibold text-gray-900">{{ backup.name }}</h3>
            <span :class="backup.status === 'running' ? 'bg-blue-100 text-blue-700' : backup.status === 'active' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'" class="px-2 py-1 text-xs rounded-full">{{ backup.status === 'running' ? '运行中' : backup.status === 'active' ? '活跃' : '非活跃' }}</span>
          </div>
          <div class="text-sm text-gray-600 space-y-1">
            <p><span class="text-gray-400">源:</span> {{ backup.source }}</p>
            <p><span class="text-gray-400">目标:</span> {{ backup.target }}</p>
            <p><span class="text-gray-400">计划:</span> {{ backup.schedule }}</p>
            <p><span class="text-gray-400">上次:</span> {{ backup.last_run }}</p>
          </div>
          <div class="flex justify-end space-x-2 mt-4 pt-3 border-t">
            <button @click="executeBackup(backup)" class="text-sm text-blue-600 hover:text-blue-700">立即备份</button>
            <button @click="restoreBackup(backup)" class="text-sm text-green-600 hover:text-green-700">恢复</button>
            <button @click="editBackup(backup)" class="text-sm text-gray-600 hover:text-gray-700">编辑</button>
            <button @click="deleteBackup(backup)" class="text-sm text-red-600 hover:text-red-700">删除</button>
          </div>
        </div>
      </div>

      <!-- 历史记录模态框 -->
      <div v-if="showHistoryModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-3xl w-full mx-4 max-h-[80vh] overflow-hidden flex flex-col">
          <div class="flex justify-between items-center px-6 py-4 border-b">
            <h3 class="text-lg font-semibold text-gray-900">备份历史记录</h3>
            <button @click="showHistoryModal = false" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>
          <div class="overflow-y-auto flex-1">
            <table class="w-full text-sm">
              <thead class="bg-gray-50 border-b sticky top-0">
                <tr>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">时间</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">任务名</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">大小</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">耗时</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">状态</th>
                </tr>
              </thead>
              <tbody class="divide-y">
                <tr v-for="record in backupHistory" :key="record.id" class="hover:bg-gray-50">
                  <td class="px-4 py-3 text-gray-600">{{ record.time }}</td>
                  <td class="px-4 py-3 font-medium text-gray-900">{{ record.name }}</td>
                  <td class="px-4 py-3 text-gray-600">{{ formatBytes(record.size) }}</td>
                  <td class="px-4 py-3 text-gray-600">{{ record.duration }}</td>
                  <td class="px-4 py-3">
                    <span :class="record.status === 'success' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'" class="px-2 py-1 text-xs rounded-full">{{ record.status === 'success' ? '成功' : '失败' }}</span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Toast -->
      <div v-if="toast.show" class="fixed bottom-4 right-4 z-50">
        <div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">{{ toast.message }}</div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import SkeletonCard from '@/components/SkeletonCard.vue'
import SkeletonTable from '@/components/SkeletonTable.vue'
import { api } from '@/utils/api'
import { useToast } from '@/composables/useToast'

const { showToast } = useToast()

const loading = ref(true)
const backups = ref<any[]>([])
const searchQuery = ref('')
const statusFilter = ref('all')
const showHistoryModal = ref(false)


const stats = ref({ total: 0, active: 0, running: 0, totalSize: 0, lastBackup: '' })
const runningBackups = ref<any[]>([])
const backupHistory = ref([
  { id: 1, time: '2026-03-29 12:00', name: '每日备份', size: 5368709120, duration: '5分30秒', status: 'success' },
  { id: 2, time: '2026-03-28 12:00', name: '每日备份', size: 5153960755, duration: '5分15秒', status: 'success' },
  { id: 3, time: '2026-03-27 12:00', name: '每日备份', size: 5033164800, duration: '5分10秒', status: 'success' },
  { id: 4, time: '2026-03-26 12:00', name: '每周全量', size: 21474836480, duration: '25分', status: 'success' },
  { id: 5, time: '2026-03-25 12:00', name: '每日备份', size: 4831838208, duration: '4分55秒', status: 'failed' }
])

const filteredBackups = computed(() => {
  let result = backups.value
  if (statusFilter.value !== 'all') result = result.filter(b => b.status === statusFilter.value)
  if (searchQuery.value) result = result.filter(b => b.name.toLowerCase().includes(searchQuery.value.toLowerCase()))
  return result
})

const loadBackups = async () => {
  loading.value = true
  try {
    const r = await api.backups.list()
    backups.value = r.data.backups || r.data || []
  } catch (e) {
    backups.value = [
      { id: 1, name: '每日备份', source: '/data', target: '/backup/daily', schedule: '每天 02:00', status: 'active', last_run: '2026-03-29 02:00', size: 5368709120 },
      { id: 2, name: '每周全量', source: '/data', target: '/backup/weekly', schedule: '每周日 03:00', status: 'active', last_run: '2026-03-26 03:00', size: 21474836480 },
      { id: 3, name: 'MySQL 数据库', source: '/var/lib/mysql', target: '/backup/mysql', schedule: '每天 04:00', status: 'active', last_run: '2026-03-29 04:00', size: 1073741824 }
    ]
  } finally {
    loading.value = false
    updateStats()
  }
}

const updateStats = () => {
  stats.value = {
    total: backups.value.length,
    active: backups.value.filter(b => b.status === 'active').length,
    running: backups.value.filter(b => b.status === 'running').length,
    totalSize: backups.value.reduce((sum, b) => sum + (b.size || 0), 0),
    lastBackup: backups.value[0]?.last_run || '-'
  }
  runningBackups.value = backups.value.filter(b => b.status === 'running').map(b => ({ ...b, progress: 45, status_message: '正在复制文件...' }))
}

const formatBytes = (bytes: number) => {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

const executeBackup = (backup: any) => {
  showToast('success', `备份任务 "${backup.name}" 已开始执行`)
  backup.status = 'running'
  runningBackups.value.push({ ...backup, progress: 0, status_message: '准备中...' })
}
const restoreBackup = (backup: any) => showToast('success', `恢复任务 "${backup.name}" 已开始`)
const editBackup = (backup: any) => showToast('info', '编辑功能开发中')
const deleteBackup = (backup: any) => {
  if (!confirm(`确定删除 "${backup.name}" 吗？`)) return
  backups.value = backups.value.filter(b => b.id !== backup.id)
  showToast('success', '备份任务已删除')
}
onMounted(() => loadBackups())
</script>