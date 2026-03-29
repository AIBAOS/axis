<template>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 max-h-[90vh] overflow-y-auto">
      <!-- 标题栏 -->
      <div class="flex justify-between items-center px-6 py-4 border-b sticky top-0 bg-white">
        <h3 class="text-lg font-semibold text-gray-900">
          {{ mode === 'create' ? '新建用户' : '编辑用户' }}
        </h3>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 表单 -->
      <form @submit.prevent="handleSubmit" class="px-6 py-4 space-y-4">
        <!-- 用户名 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">用户名 *</label>
          <input
            v-model="formData.username"
            type="text"
            required
            :disabled="mode === 'edit'"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
            placeholder="请输入用户名"
            minlength="3"
            maxlength="32"
          />
          <p v-if="mode === 'edit'" class="text-xs text-gray-500 mt-1">用户名不可修改</p>
        </div>

        <!-- 邮箱 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">邮箱 *</label>
          <input
            v-model="formData.email"
            type="email"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="请输入邮箱"
          />
        </div>

        <!-- 密码（仅新建时显示） -->
        <div v-if="mode === 'create'">
          <label class="block text-sm font-medium text-gray-700 mb-1">密码 *</label>
          <input
            v-model="formData.password"
            type="password"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="请输入密码"
            minlength="6"
          />
          <p class="text-xs text-gray-500 mt-1">密码长度至少 6 位</p>
        </div>

        <!-- 重置密码（编辑时显示） -->
        <div v-if="mode === 'edit'" class="p-3 bg-yellow-50 rounded-lg">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-yellow-800">重置密码</p>
              <p class="text-xs text-yellow-600">生成随机密码并发送到用户邮箱</p>
            </div>
            <button type="button" @click="resetPassword" class="text-sm text-yellow-700 hover:text-yellow-900 font-medium">
              重置
            </button>
          </div>
        </div>

        <!-- 角色 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">角色 *</label>
          <select
            v-model="formData.role"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          >
            <option value="user">普通用户</option>
            <option value="admin">管理员</option>
            <option value="guest">访客</option>
          </select>
          <div class="mt-2 space-y-1 text-xs text-gray-500">
            <p><span class="font-medium text-red-600">管理员</span> - 完全访问权限，可管理用户和系统设置</p>
            <p><span class="font-medium text-blue-600">普通用户</span> - 标准访问权限，可使用所有功能</p>
            <p><span class="font-medium text-gray-600">访客</span> - 只读权限，仅可查看信息</p>
          </div>
        </div>

        <!-- 状态 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">状态</label>
          <select
            v-model="formData.status"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          >
            <option value="active">活跃</option>
            <option value="inactive">离线</option>
            <option value="disabled">禁用</option>
          </select>
        </div>

        <!-- 显示名称 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">显示名称</label>
          <input
            v-model="formData.display_name"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="可选，用于界面显示"
          />
        </div>

        <!-- 文件夹权限 -->
        <div v-if="mode === 'edit'">
          <label class="block text-sm font-medium text-gray-700 mb-2">文件夹权限</label>
          <div class="space-y-2 border rounded-lg p-3 max-h-40 overflow-y-auto">
            <div v-for="folder in folderPermissions" :key="folder.path" class="flex items-center justify-between py-1">
              <span class="text-sm text-gray-700">{{ folder.path }}</span>
              <select v-model="folder.permission" class="text-xs px-2 py-1 border rounded">
                <option value="none">无权限</option>
                <option value="read">只读</option>
                <option value="write">读写</option>
                <option value="admin">完全控制</option>
              </select>
            </div>
            <div v-if="folderPermissions.length === 0" class="text-sm text-gray-500 text-center py-2">
              暂无共享文件夹
            </div>
          </div>
        </div>

        <!-- 应用权限 -->
        <div v-if="mode === 'edit'">
          <label class="block text-sm font-medium text-gray-700 mb-2">应用访问权限</label>
          <div class="space-y-2 border rounded-lg p-3">
            <label v-for="app in appPermissions" :key="app.id" class="flex items-center">
              <input v-model="app.enabled" type="checkbox" class="h-4 w-4 rounded" />
              <span class="ml-2 text-sm text-gray-700">{{ app.name }}</span>
            </label>
          </div>
        </div>
      </form>

      <!-- 错误提示 -->
      <div v-if="error" class="px-6 py-2 bg-red-50 text-sm text-red-600">
        {{ error }}
      </div>

      <!-- 按钮栏 -->
      <div class="flex justify-end space-x-3 px-6 py-4 border-t bg-gray-50 rounded-b-lg sticky bottom-0">
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
          :disabled="saving"
          class="btn-primary disabled:opacity-50"
        >
          {{ saving ? '保存中...' : (mode === 'create' ? '创建' : '保存') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  mode: 'create' | 'edit'
  user?: any
}>()

const emit = defineEmits<{
  close: []
  save: [data: any]
}>()

const saving = ref(false)
const error = ref('')

// 表单数据
const formData = ref({
  username: '',
  email: '',
  password: '',
  role: 'user',
  status: 'active',
  display_name: ''
})

// 文件夹权限
const folderPermissions = ref([
  { path: '/shared/documents', permission: 'read' },
  { path: '/shared/media', permission: 'read' },
  { path: '/shared/backups', permission: 'none' }
])

// 应用权限
const appPermissions = ref([
  { id: 'files', name: '文件管理', enabled: true },
  { id: 'downloads', name: '下载管理', enabled: true },
  { id: 'printers', name: '打印服务', enabled: true },
  { id: 'backups', name: '备份管理', enabled: false },
  { id: 'settings', name: '系统设置', enabled: false }
])

// 监听用户数据变化（编辑模式）
watch(() => props.user, (newUser) => {
  if (newUser && props.mode === 'edit') {
    formData.value = {
      username: newUser.username || '',
      email: newUser.email || '',
      password: '',
      role: newUser.role || 'user',
      status: newUser.status || 'active',
      display_name: newUser.display_name || ''
    }
  }
}, { immediate: true })

// 重置密码
const resetPassword = () => {
  if (confirm('确定要重置该用户的密码吗？新密码将发送到用户邮箱。')) {
    // 标记需要重置密码
    formData.value.password = 'RESET'
    alert('密码将在保存时重置')
  }
}

// 提交表单
const handleSubmit = async () => {
  error.value = ''

  // 验证
  if (!formData.value.username.trim()) {
    error.value = '请输入用户名'
    return
  }
  if (!formData.value.email.trim()) {
    error.value = '请输入邮箱'
    return
  }
  if (props.mode === 'create' && !formData.value.password) {
    error.value = '请输入密码'
    return
  }

  const data: any = {
    username: formData.value.username.trim(),
    email: formData.value.email.trim(),
    role: formData.value.role,
    status: formData.value.status,
    display_name: formData.value.display_name.trim() || undefined
  }

  // 密码处理
  if (props.mode === 'create') {
    data.password = formData.value.password
  } else if (formData.value.password === 'RESET') {
    data.reset_password = true
  }

  // 权限数据（仅编辑模式）
  if (props.mode === 'edit') {
    data.folder_permissions = folderPermissions.value.map(f => ({
      path: f.path,
      permission: f.permission
    }))
    data.app_permissions = appPermissions.value.filter(a => a.enabled).map(a => a.id)
  }

  saving.value = true
  emit('save', data)
  // 不立即重置 saving，让父组件完成异步操作后关闭 modal
}
</script>