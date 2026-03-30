<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">共享管理</h1>
          <p class="text-gray-600 mt-1">管理 SMB、NFS、WebDAV 和 FTP 共享文件夹</p>
        </div>
        <button
          @click="openCreateModal"
          class="btn-primary flex items-center space-x-2"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span>新建共享</span>
        </button>
      </div>

      <!-- 选项卡 -->
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
              'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center space-x-2'
            ]"
          >
            <span>{{ tab.name }}</span>
            <span
              v-if="getSharesByProtocol(tab.id).length > 0"
              class="px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-600"
            >
              {{ getSharesByProtocol(tab.id).length }}
            </span>
          </button>
        </nav>
      </div>

      <!-- 搜索和筛选 -->
      <div class="flex items-center space-x-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索共享名称或路径..."
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          />
        </div>
        <select
          v-model="statusFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
        >
          <option value="all">全部状态</option>
          <option value="active">活跃</option>
          <option value="inactive">非活跃</option>
        </select>
        <button
          @click="loadShares"
          class="btn-secondary flex items-center space-x-2"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span>刷新</span>
        </button>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="flex justify-center items-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
        <span class="ml-3 text-gray-600">加载中...</span>
      </div>

      <!-- 空数据提示 -->
      <div v-else-if="filteredShares.length === 0" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
        <p class="mt-4 text-gray-600">暂无 {{ currentTabLabel }} 共享</p>
        <p class="mt-2 text-sm text-gray-500">点击上方"新建共享"按钮创建</p>
      </div>

      <!-- 共享列表 -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <ShareCard
          v-for="share in filteredShares"
          :key="share.id"
          :share="share"
          :protocol="currentTab as 'smb' | 'nfs' | 'webdav' | 'ftp'"
          @edit="openEditModal"
          @delete="confirmDelete"
        />
      </div>

      <!-- 新建/编辑模态框 -->
      <ShareModal
        v-if="showModal"
        :mode="modalMode"
        :protocol="currentTab as 'smb' | 'nfs' | 'webdav' | 'ftp'"
        :share="editingShare"
        @close="closeModal"
        @save="handleSave"
      />

      <!-- 删除确认对话框 -->
      <div v-if="showDeleteConfirm" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
          <div class="px-6 py-4">
            <h3 class="text-lg font-semibold text-gray-900">确认删除</h3>
            <p class="mt-2 text-gray-600">
              确定要删除共享 "{{ deletingShare?.name }}" 吗？此操作不可撤销。
            </p>
          </div>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button
              @click="showDeleteConfirm = false"
              class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50"
            >
              取消
            </button>
            <button
              @click="executeDelete"
              :disabled="deleting"
              class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50"
            >
              {{ deleting ? '删除中...' : '删除' }}
            </button>
          </div>
        </div>
      </div>

      <!-- Toast 提示 -->
      <div v-if="toast.show" class="fixed bottom-4 right-4 z-50">
        <div :class="toastClass" class="px-4 py-3 rounded-lg shadow-lg flex items-center space-x-2">
          <svg v-if="toast.type === 'success'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
          <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
          <span>{{ toast.message }}</span>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import ShareCard from '@/components/shares/ShareCard.vue'
import ShareModal from '@/components/shares/ShareModal.vue'
import { api } from '@/utils/api'
import { useToast } from '@/composables/useToast'

const { showToast } = useToast()

// 选项卡
const tabs = [
  { id: 'smb', name: 'SMB/CIFS' },
  { id: 'nfs', name: 'NFS' },
  { id: 'webdav', name: 'WebDAV' },
  { id: 'ftp', name: 'FTP' }
]

const currentTab = ref('smb')
const loading = ref(true)
const searchQuery = ref('')
const statusFilter = ref('all')

// 数据
const smbShares = ref<any[]>([])
const nfsShares = ref<any[]>([])
const webdavShares = ref<any[]>([])
const ftpShares = ref<any[]>([])

// 模态框
const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingShare = ref<any>(null)

// 删除确认
const showDeleteConfirm = ref(false)
const deletingShare = ref<any>(null)
const deleting = ref(false)

// 当前选项卡名称
const currentTabLabel = computed(() => {
  const tab = tabs.find(t => t.id === currentTab.value)
  return tab?.name || ''
})

// 获取指定协议的共享列表
const getSharesByProtocol = (protocol: string) => {
  switch (protocol) {
    case 'smb': return smbShares.value
    case 'nfs': return nfsShares.value
    case 'webdav': return webdavShares.value
    case 'ftp': return ftpShares.value
    default: return []
  }
}

// 筛选后的共享列表
const filteredShares = computed(() => {
  let shares = getSharesByProtocol(currentTab.value)

  // 状态筛选
  if (statusFilter.value !== 'all') {
    shares = shares.filter(s => s.status === statusFilter.value)
  }

  // 搜索筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    shares = shares.filter(s =>
      s.name.toLowerCase().includes(query) ||
      s.path.toLowerCase().includes(query) ||
      (s.description && s.description.toLowerCase().includes(query))
    )
  }

  return shares
})

// Toast 样式
const toastClass = computed(() => {
  return toast.value.type === 'success'
    ? 'bg-green-500 text-white'
    : 'bg-red-500 text-white'
})



// 加载各类型共享
const loadShares = async () => {
  loading.value = true
  try {
    // 并行加载所有共享类型
    const [smb, nfs, webdav, ftp] = await Promise.all([
      api.shares.listSmb(),
      api.shares.listNfs(),
      api.shares.listWebdav(),
      api.shares.listFtp()
    ])

    smbShares.value = smb.data?.data || smb.data || []
    nfsShares.value = nfs.data?.data || nfs.data || []
    webdavShares.value = webdav.data?.data || webdav.data || []
    ftpShares.value = ftp.data?.data || ftp.data || []
  } catch (error) {
    console.error('Failed to load shares:', error)
    showToast('error', '加载共享列表失败')
  } finally {
    loading.value = false
  }
}

// 打开新建模态框
const openCreateModal = () => {
  modalMode.value = 'create'
  editingShare.value = null
  showModal.value = true
}

// 打开编辑模态框
const openEditModal = (share: any) => {
  modalMode.value = 'edit'
  editingShare.value = share
  showModal.value = true
}

// 关闭模态框
const closeModal = () => {
  showModal.value = false
  editingShare.value = null
}

// 处理保存
const handleSave = async (data: any) => {
  try {
    const protocol = currentTab.value

    if (modalMode.value === 'create') {
      // 创建共享
      switch (protocol) {
        case 'smb':
          await api.shares.createSmb(data)
          break
        case 'nfs':
          await api.shares.createNfs(data)
          break
        case 'webdav':
          await api.shares.createWebdav(data)
          break
        case 'ftp':
          await api.shares.createFtp(data)
          break
      }
      showToast('success', '共享创建成功')
    } else {
      // 更新共享
      const id = editingShare.value.id
      switch (protocol) {
        case 'smb':
          await api.shares.updateSmb(id, data)
          break
        case 'nfs':
          await api.shares.updateNfs(id, data)
          break
        case 'webdav':
          await api.shares.updateWebdav(id, data)
          break
        case 'ftp':
          await api.shares.updateFtp(id, data)
          break
      }
      showToast('success', '共享更新成功')
    }

    closeModal()
    await loadShares()
  } catch (error: any) {
    console.error('Failed to save share:', error)
    const message = error.response?.data?.error || '操作失败'
    showToast('error', message)
  }
}

// 确认删除
const confirmDelete = (share: any) => {
  deletingShare.value = share
  showDeleteConfirm.value = true
}

// 执行删除
const executeDelete = async () => {
  if (!deletingShare.value) return

  deleting.value = true
  try {
    const id = deletingShare.value.id
    const protocol = currentTab.value

    switch (protocol) {
      case 'smb':
        await api.shares.deleteSmb(id)
        break
      case 'nfs':
        await api.shares.deleteNfs(id)
        break
      case 'webdav':
        await api.shares.deleteWebdav(id)
        break
      case 'ftp':
        await api.shares.deleteFtp(id)
        break
    }

    showToast('success', '共享删除成功')
    showDeleteConfirm.value = false
    deletingShare.value = null
    await loadShares()
  } catch (error: any) {
    console.error('Failed to delete share:', error)
    const message = error.response?.data?.error || '删除失败'
    showToast('error', message)
  } finally {
    deleting.value = false
  }
}

// 生命周期
onMounted(() => {
  loadShares()
})
</script>