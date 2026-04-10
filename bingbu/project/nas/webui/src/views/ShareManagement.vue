<template>
  <div class="space-y-6">
    <!-- 页面标题和选项卡 -->
    <div class="space-y-4">
      <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          共享管理
        </h1>
        <button
          @click="showAddModal = true"
          class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
          </svg>
          添加共享
        </button>
      </div>

      <!-- 选项卡 -->
      <div class="border-b border-gray-200 dark:border-gray-700">
        <nav class="-mb-px flex space-x-8">
          <button
            @click="activeTab = 'smb'"
            :class="activeTab === 'smb' ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400' : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300'"
            class="whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm"
          >
            SMB 共享
          </button>
          <button
            @click="activeTab = 'nfs'"
            :class="activeTab === 'nfs' ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400' : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300'"
            class="whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm"
          >
            NFS 共享
          </button>
        </nav>
      </div>
    </div>

    <!-- SMB 共享列表 -->
    <div v-if="activeTab === 'smb'" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <!-- 列表头部 -->
      <div class="grid grid-cols-12 gap-4 px-6 py-3 bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600 text-sm font-medium text-gray-500 dark:text-gray-400">
        <div class="col-span-3">共享名</div>
        <div class="col-span-3">路径</div>
        <div class="col-span-2">权限</div>
        <div class="col-span-2">状态</div>
        <div class="col-span-2 text-right">操作</div>
      </div>

      <!-- 空状态 -->
      <div v-if="smbShares.length === 0" class="px-6 py-12 text-center">
        <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"></path>
        </svg>
        <p class="text-lg font-medium text-gray-900 dark:text-white mb-2">暂无 SMB 共享</p>
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">添加共享开始使用</p>
        <button
          @click="showAddModal = true"
          class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
        >
          添加共享
        </button>
      </div>

      <!-- 列表 -->
      <div v-else class="divide-y divide-gray-200 dark:divide-gray-700">
        <div
          v-for="share in smbShares"
          :key="share.id"
          class="grid grid-cols-12 gap-4 px-6 py-4 hover:bg-gray-50 dark:hover:bg-gray-700 items-center"
        >
          <div class="col-span-3">
            <span class="text-sm font-medium text-gray-900 dark:text-white">{{ share.name }}</span>
          </div>
          <div class="col-span-3">
            <span class="text-sm text-gray-500 dark:text-gray-400">{{ share.path }}</span>
          </div>
          <div class="col-span-2">
            <span :class="permissionClasses[share.permission]" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">
              {{ permissionLabels[share.permission] }}
            </span>
          </div>
          <div class="col-span-2">
            <span :class="share.enabled ? 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400' : 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-400'" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">
              {{ share.enabled ? '已启用' : '已禁用' }}
            </span>
          </div>
          <div class="col-span-2 flex justify-end space-x-2">
            <button
              @click="editShare(share)"
              class="text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400"
            >
              编辑
            </button>
            <button
              @click="deleteShare(share)"
              class="text-gray-400 hover:text-red-600 dark:hover:text-red-400"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- NFS 共享列表 -->
    <div v-if="activeTab === 'nfs'" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <!-- 列表头部 -->
      <div class="grid grid-cols-12 gap-4 px-6 py-3 bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600 text-sm font-medium text-gray-500 dark:text-gray-400">
        <div class="col-span-3">共享名</div>
        <div class="col-span-3">路径</div>
        <div class="col-span-3">允许客户端</div>
        <div class="col-span-2">权限</div>
        <div class="col-span-1 text-right">操作</div>
      </div>

      <!-- 空状态 -->
      <div v-if="nfsShares.length === 0" class="px-6 py-12 text-center">
        <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"></path>
        </svg>
        <p class="text-lg font-medium text-gray-900 dark:text-white mb-2">暂无 NFS 共享</p>
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">添加共享开始使用</p>
        <button
          @click="showAddModal = true"
          class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
        >
          添加共享
        </button>
      </div>

      <!-- 列表 -->
      <div v-else class="divide-y divide-gray-200 dark:divide-gray-700">
        <div
          v-for="share in nfsShares"
          :key="share.id"
          class="grid grid-cols-12 gap-4 px-6 py-4 hover:bg-gray-50 dark:hover:bg-gray-700 items-center"
        >
          <div class="col-span-3">
            <span class="text-sm font-medium text-gray-900 dark:text-white">{{ share.name }}</span>
          </div>
          <div class="col-span-3">
            <span class="text-sm text-gray-500 dark:text-gray-400">{{ share.path }}</span>
          </div>
          <div class="col-span-3">
            <span class="text-sm text-gray-500 dark:text-gray-400">{{ share.clients.join(', ') }}</span>
          </div>
          <div class="col-span-2">
            <span :class="permissionClasses[share.permission]" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">
              {{ permissionLabels[share.permission] }}
            </span>
          </div>
          <div class="col-span-1 flex justify-end space-x-2">
            <button
              @click="editShare(share)"
              class="text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400"
            >
              编辑
            </button>
            <button
              @click="deleteShare(share)"
              class="text-gray-400 hover:text-red-600 dark:hover:text-red-400"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 添加/编辑共享模态框 -->
    <div v-if="showAddModal || showEditModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">{{ showEditModal ? '编辑共享' : '添加共享' }}</h3>
        </div>
        <div class="px-6 py-4 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              共享类型
            </label>
            <select
              v-model="formData.type"
              :disabled="showEditModal"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="smb">SMB 共享</option>
              <option value="nfs">NFS 共享</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              共享名 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formData.name"
              @blur="validateShareName"
              type="text"
              required
              :class="formErrors.name ? 'border-red-500' : 'border-gray-300'"
              class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
            <p v-if="formErrors.name" class="mt-1 text-sm text-red-600">{{ formErrors.name }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              路径 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formData.path"
              @blur="validateSharePath"
              type="text"
              required
              placeholder="/srv/samba/share"
              :class="formErrors.path ? 'border-red-500' : 'border-gray-300'"
              class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
            <p v-if="formErrors.path" class="mt-1 text-sm text-red-600">{{ formErrors.path }}</p>
          </div>
          <div v-if="formData.type === 'nfs'">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              允许客户端
            </label>
            <input
              v-model="formData.clients"
              type="text"
              placeholder="192.168.1.0/24"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">多个客户端用逗号分隔，或使用 * 允许所有</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              权限
            </label>
            <select
              v-model="formData.permission"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="read">只读</option>
              <option value="write">读写</option>
              <option value="readonly_guest">只读（访客）</option>
            </select>
          </div>
          <div v-if="!showEditModal">
            <label class="flex items-center">
              <input
                v-model="formData.enabled"
                type="checkbox"
                class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">启用共享</span>
            </label>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="closeModal"
            :disabled="saving"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 disabled:opacity-50"
          >
            取消
          </button>
          <button
            @click="saveShare"
            :disabled="saving || !isFormValid"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <svg v-if="saving" class="animate-spin -ml-1 mr-2 h-4 w-4 inline" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            {{ saving ? '保存中...' : (showEditModal ? '保存' : '添加') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useToast } from '../composables/useToast'

const toast = useToast()

const activeTab = ref('smb')
const showAddModal = ref(false)
const showEditModal = ref(false)
const editingShare = ref(null)
const saving = ref(false)

const smbShares = ref([
  { id: 1, name: '公共共享', path: '/srv/samba/public', permission: 'read', enabled: true },
  { id: 2, name: '用户共享', path: '/srv/samba/users', permission: 'write', enabled: true },
  { id: 3, name: '访客共享', path: '/srv/samba/guest', permission: 'readonly_guest', enabled: false }
])

const nfsShares = ref([
  { id: 1, name: 'NFS 公共', path: '/srv/nfs/public', clients: ['192.168.1.0/24'], permission: 'read' },
  { id: 2, name: 'NFS 私有', path: '/srv/nfs/private', clients: ['192.168.1.100'], permission: 'write' }
])

const formData = ref({
  type: 'smb',
  name: '',
  path: '',
  clients: '',
  permission: 'read',
  enabled: true
})

const formErrors = ref({
  name: '',
  path: '',
  clients: ''
})

// 共享名验证：2-100 字符，禁止 / \ : * ? " < > |
const validateShareName = () => {
  const name = formData.value.name.trim()
  
  if (!name) {
    formErrors.value.name = '共享名不能为空'
    return false
  }
  
  if (name.length < 2) {
    formErrors.value.name = '共享名至少 2 个字符'
    return false
  }
  
  if (name.length > 100) {
    formErrors.value.name = '共享名最多 100 个字符'
    return false
  }
  
  // 禁止特殊字符 / \ : * ? " < > |
  const invalidChars = /[\/\\:*?"<>|]/
  if (invalidChars.test(name)) {
    formErrors.value.name = '共享名不能包含 / \\ : * ? " < > |'
    return false
  }
  
  formErrors.value.name = ''
  return true
}

// 路径验证：合法 Unix 路径格式
const validateSharePath = () => {
  const path = formData.value.path.trim()
  
  if (!path) {
    formErrors.value.path = '路径不能为空'
    return false
  }
  
  // 必须以 / 开头
  if (!path.startsWith('/')) {
    formErrors.value.path = '路径必须以 / 开头'
    return false
  }
  
  // 不能包含非法字符
  const invalidChars = /[<>:"|]/
  if (invalidChars.test(path)) {
    formErrors.value.path = '路径包含非法字符'
    return false
  }
  
  formErrors.value.path = ''
  return true
}

// NFS 客户端验证
const validateClients = () => {
  const clients = formData.value.clients.trim()
  
  // NFS 类型必填
  if (formData.value.type === 'nfs' && !clients) {
    formErrors.value.clients = '请输入允许的客户端'
    return false
  }
  
  // 验证格式：IP/CIDR 或 *
  if (clients && clients !== '*') {
    const clientList = clients.split(',').map(c => c.trim())
    const ipPattern = /^(\d{1,3}\.){3}\d{1,3}(\/\d{1,2})?$/
    const valid = clientList.every(c => c === '*' || ipPattern.test(c))
    
    if (!valid) {
      formErrors.value.clients = '客户端格式无效，使用 IP/CIDR 或 *'
      return false
    }
  }
  
  formErrors.value.clients = ''
  return true
}

// 表单有效性
const isFormValid = computed(() => {
  const nameValid = validateShareName()
  const pathValid = validateSharePath()
  const clientsValid = validateClients()
  
  return nameValid && pathValid && clientsValid
})

const permissionClasses = {
  read: 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400',
  write: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400',
  readonly_guest: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400'
}

const permissionLabels = {
  read: '只读',
  write: '读写',
  readonly_guest: '只读（访客）'
}

const closeModal = () => {
  showAddModal.value = false
  showEditModal.value = false
  editingShare.value = null
  formData.value = {
    type: 'smb',
    name: '',
    path: '',
    clients: '',
    permission: 'read',
    enabled: true
  }
  formErrors.value = {
    name: '',
    path: '',
    clients: ''
  }
}

const editShare = (share) => {
  editingShare.value = share
  showEditModal.value = true
  formData.value = {
    type: 'smb',
    name: share.name,
    path: share.path,
    clients: share.clients ? share.clients.join(', ') : '',
    permission: share.permission,
    enabled: share.enabled
  }
}

const saveShare = async () => {
  if (!isFormValid.value) {
    toast.error('请修正表单错误')
    return
  }
  
  saving.value = true
  
  try {

  if (showEditModal.value && editingShare.value) {
    // 编辑现有共享
    if (editingShare.value.type === 'smb') {
      const index = smbShares.value.findIndex(s => s.id === editingShare.value.id)
      if (index !== -1) {
        smbShares.value[index] = {
          ...editingShare.value,
          name: formData.value.name,
          path: formData.value.path,
          permission: formData.value.permission
        }
      }
    } else {
      const index = nfsShares.value.findIndex(s => s.id === editingShare.value.id)
      if (index !== -1) {
        nfsShares.value[index] = {
          ...editingShare.value,
          name: formData.value.name,
          path: formData.value.path,
          clients: formData.value.clients.split(',').map(c => c.trim()),
          permission: formData.value.permission
        }
      }
    }
    toast.success('共享已更新')
  } else {
    // 添加新共享
    const newShare = {
      id: Date.now(),
      name: formData.value.name,
      path: formData.value.path,
      permission: formData.value.permission,
      enabled: formData.value.enabled
    }

    if (formData.value.type === 'smb') {
      smbShares.value.push(newShare)
    } else {
      newShare.clients = formData.value.clients.split(',').map(c => c.trim())
      nfsShares.value.push(newShare)
    }

    toast.success('共享已添加')
  }
  
  closeModal()
  } catch (error) {
    console.error('Failed to save share:', error)
    toast.error('保存失败：' + (error.response?.data?.error || '未知错误'))
  } finally {
    saving.value = false
  }
}

const deleteShare = (share) => {
  if (!confirm(`确定要删除共享 "${share.name}" 吗？`)) return

  if (share.path.includes('samba')) {
    smbShares.value = smbShares.value.filter(s => s.id !== share.id)
  } else {
    nfsShares.value = nfsShares.value.filter(s => s.id !== share.id)
  }

  toast.success('共享已删除')
}

onMounted(() => {
  // 加载共享列表
})
</script>
