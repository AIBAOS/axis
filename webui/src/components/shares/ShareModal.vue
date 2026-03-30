<template>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto">
      <!-- 标题栏 -->
      <div class="flex justify-between items-center px-6 py-4 border-b sticky top-0 bg-white">
        <h3 class="text-lg font-semibold text-gray-900">
          {{ mode === 'create' ? `新建 ${protocolLabel} 共享` : `编辑 ${protocolLabel} 共享` }}
        </h3>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 表单 -->
      <form @submit.prevent="handleSubmit" class="px-6 py-4 space-y-4">
        <!-- 基本信息 -->
        <div class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">基本信息</h4>

          <!-- 名称 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">共享名称 *</label>
            <input
              v-model="formData.name"
              type="text"
              required
              :class="['w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500', nameError ? 'border-red-500' : 'border-gray-300']"
              placeholder="请输入共享名称（如 Public, Home）"
              maxlength="64"
            />
            <p v-if="nameError" class="text-xs text-red-500 mt-1">{{ nameError }}</p>
            <p v-else class="text-xs text-gray-500 mt-1">1-64 字符，允许字母、数字、-、_、.</p>
          </div>

          <!-- 路径 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">共享路径 *</label>
            <input
              v-model="formData.path"
              type="text"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="/srv/shares/..."
              maxlength="256"
            />
            <p class="text-xs text-gray-500 mt-1">必须以 / 开头，服务器上的实际路径</p>
          </div>

          <!-- 描述 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">描述</label>
            <textarea
              v-model="formData.description"
              rows="2"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="共享文件夹用途说明..."
            ></textarea>
          </div>
        </div>

        <!-- 访问控制 -->
        <div class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">访问控制</h4>

          <!-- 公开访问 -->
          <div class="flex items-center">
            <input
              v-model="formData.public"
              type="checkbox"
              id="public"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
            <label for="public" class="ml-2 block text-sm text-gray-700">
              公开访问（允许访客访问）
            </label>
          </div>

          <!-- 只读 -->
          <div class="flex items-center">
            <input
              v-model="formData.read_only"
              type="checkbox"
              id="read_only"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
            <label for="read_only" class="ml-2 block text-sm text-gray-700">
              只读模式
            </label>
          </div>
        </div>

        <!-- SMB 特有配置 -->
        <div v-if="protocol === 'smb'" class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">SMB 配置</h4>

          <!-- 访客访问 -->
          <div class="flex items-center">
            <input
              v-model="formData.guest_access"
              type="checkbox"
              id="guest_access"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
            <label for="guest_access" class="ml-2 block text-sm text-gray-700">
              允许访客访问（无需认证）
            </label>
          </div>

          <!-- 可浏览 -->
          <div class="flex items-center">
            <input
              v-model="formData.browseable"
              type="checkbox"
              id="browseable"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
            <label for="browseable" class="ml-2 block text-sm text-gray-700">
              在网络中可见
            </label>
          </div>

          <!-- 有效用户 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">有效用户</label>
            <input
              v-model="formData.valid_users"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="user1, user2, @group1"
            />
            <p class="text-xs text-gray-500 mt-1">逗号分隔，@group 表示组</p>
          </div>
        </div>

        <!-- NFS 特有配置 -->
        <div v-if="protocol === 'nfs'" class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">NFS 配置</h4>

          <!-- 客户端配置 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">允许的客户端 *</label>
            <div class="space-y-2">
              <div v-for="(client, index) in formData.clients" :key="index" class="flex items-center space-x-2">
                <input
                  v-model="client.network"
                  type="text"
                  class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                  placeholder="192.168.1.0/24"
                />
                <select
                  v-model="client.access"
                  class="px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                >
                  <option value="rw">读写</option>
                  <option value="ro">只读</option>
                </select>
                <button
                  @click="removeClient(index)"
                  type="button"
                  class="p-2 text-red-500 hover:bg-red-50 rounded"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
              <button
                @click="addClient"
                type="button"
                class="text-sm text-primary-600 hover:text-primary-700 flex items-center"
              >
                <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                添加客户端
              </button>
            </div>
          </div>

          <!-- 其他 NFS 选项 -->
          <div class="flex items-center space-x-6">
            <div class="flex items-center">
              <input
                v-model="formData.no_subtree_check"
                type="checkbox"
                id="no_subtree_check"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="no_subtree_check" class="ml-2 block text-sm text-gray-700">
                禁用子树检查
              </label>
            </div>
            <div class="flex items-center">
              <input
                v-model="formData.sync"
                type="checkbox"
                id="sync"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="sync" class="ml-2 block text-sm text-gray-700">
                同步写入
              </label>
            </div>
          </div>
        </div>

        <!-- WebDAV 特有配置 -->
        <div v-if="protocol === 'webdav'" class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">WebDAV 配置</h4>
          <p class="text-sm text-gray-500">WebDAV 共享使用标准 HTTP 认证，配置与 SMB 类似。</p>
        </div>

        <!-- FTP 特有配置 -->
        <div v-if="protocol === 'ftp'" class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">FTP 配置</h4>
          <p class="text-sm text-gray-500">FTP 共享将使用服务器全局 FTP 配置。</p>
        </div>
      </form>

      <!-- 按钮栏 -->
      <div class="flex justify-end space-x-3 px-6 py-4 border-t bg-gray-50 sticky bottom-0">
        <button
          type="button"
          @click="$emit('close')"
          class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50"
        >
          取消
        </button>
        <button
          type="submit"
          @click="handleSubmit"
          :disabled="loading"
          class="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ loading ? '处理中...' : (mode === 'create' ? '创建' : '保存') }}
        </button>
      </div>

      <!-- 错误提示 -->
      <div v-if="error" class="px-6 py-3 bg-red-50 border-t border-red-100">
        <p class="text-sm text-red-600">{{ error }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { validateShareName } from '@/utils/validators'

const props = defineProps<{
  mode: 'create' | 'edit'
  protocol: 'smb' | 'nfs' | 'webdav' | 'ftp'
  share?: any
}>()

const emit = defineEmits<{
  close: []
  save: [data: any]
}>()

const loading = ref(false)
const error = ref('')

// 实时验证错误
const nameError = computed(() => {
  if (!formData.value.name) return ''
  return validateShareName(formData.value.name).error || ''
})

// 协议名称
const protocolLabel = computed(() => {
  switch (props.protocol) {
    case 'smb': return 'SMB/CIFS'
    case 'nfs': return 'NFS'
    case 'webdav': return 'WebDAV'
    case 'ftp': return 'FTP'
    default: return props.protocol.toUpperCase()
  }
})

// 表单数据
const formData = ref({
  name: '',
  path: '',
  description: '',
  public: false,
  read_only: false,
  // SMB 专用
  guest_access: false,
  browseable: true,
  valid_users: '',
  // NFS 专用
  clients: [{ network: '', access: 'rw' }],
  no_subtree_check: true,
  sync: false
})

// 监听共享数据变化（编辑模式）
watch(() => props.share, (newShare) => {
  if (newShare && props.mode === 'edit') {
    formData.value = {
      name: newShare.name || '',
      path: newShare.path || '',
      description: newShare.description || '',
      public: newShare.public || false,
      read_only: newShare.read_only || false,
      guest_access: newShare.guest_access || false,
      browseable: newShare.browseable ?? true,
      valid_users: newShare.valid_users || '',
      clients: newShare.clients || [{ network: '', access: 'rw' }],
      no_subtree_check: newShare.no_subtree_check ?? true,
      sync: newShare.sync || false
    }
  }
}, { immediate: true })

// NFS 客户端操作
const addClient = () => {
  formData.value.clients.push({ network: '', access: 'rw' })
}

const removeClient = (index: number) => {
  if (formData.value.clients.length > 1) {
    formData.value.clients.splice(index, 1)
  }
}

// 提交表单
const handleSubmit = async () => {
  error.value = ''
  loading.value = true

  // 验证名称
  if (!formData.value.name) {
    error.value = '请输入共享名称'
    loading.value = false
    return
  }
  
  const nameValidation = validateShareName(formData.value.name)
  if (!nameValidation.valid) {
    error.value = nameValidation.error || '名称格式错误'
    loading.value = false
    return
  }

  // 验证路径
  if (!formData.value.path || !formData.value.path.startsWith('/')) {
    error.value = '共享路径必须以 / 开头'
    loading.value = false
    return
  }

  // NFS 客户端验证
  if (props.protocol === 'nfs') {
    const validClients = formData.value.clients.filter(c => c.network.trim())
    if (validClients.length === 0) {
      error.value = '必须至少配置一个 NFS 客户端'
      loading.value = false
      return
    }
  }

  // 构建提交数据
  const data: any = {
    name: formData.value.name,
    path: formData.value.path,
    description: formData.value.description,
    public: formData.value.public,
    read_only: formData.value.read_only
  }

  // SMB 专用字段
  if (props.protocol === 'smb') {
    data.guest_access = formData.value.guest_access
    data.browseable = formData.value.browseable
    if (formData.value.valid_users) {
      data.valid_users = formData.value.valid_users
    }
  }

  // NFS 专用字段
  if (props.protocol === 'nfs') {
    data.clients = formData.value.clients.filter(c => c.network.trim())
    data.no_subtree_check = formData.value.no_subtree_check
    data.sync = formData.value.sync
  }

  try {
    emit('save', data)
  } finally {
    loading.value = false
  }
}
</script>