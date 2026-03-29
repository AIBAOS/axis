<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">用户管理</h1>
          <p class="text-gray-600 mt-1">管理系统用户和权限</p>
        </div>
        <button @click="showCreateModal = true" class="btn-primary flex items-center space-x-2">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span>新建用户</span>
        </button>
      </div>

      <!-- 用户统计 -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总用户</p>
              <p class="text-xl font-bold text-gray-900">{{ users.length }}</p>
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
              <p class="text-sm text-gray-500">活跃</p>
              <p class="text-xl font-bold text-green-700">{{ statusCounts.active }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">离线</p>
              <p class="text-xl font-bold text-gray-700">{{ statusCounts.inactive }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">禁用</p>
              <p class="text-xl font-bold text-red-700">{{ statusCounts.disabled }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-purple-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">管理员</p>
              <p class="text-xl font-bold text-purple-700">{{ roleCounts.admin }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 搜索和筛选栏 -->
      <div class="flex space-x-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索用户名或邮箱..."
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          />
        </div>
        <select
          v-model="roleFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
        >
          <option value="all">全部角色</option>
          <option value="admin">管理员</option>
          <option value="user">普通用户</option>
          <option value="guest">访客</option>
        </select>
        <select
          v-model="statusFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
        >
          <option value="all">全部状态</option>
          <option value="active">活跃</option>
          <option value="inactive">离线</option>
          <option value="disabled">禁用</option>
        </select>
        <button
          @click="loadUsers"
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

      <!-- 用户列表 -->
      <div v-else-if="filteredUsers.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
        <p class="mt-4 text-gray-600">暂无用户</p>
        <p class="mt-2 text-sm text-gray-500">点击"新建用户"开始</p>
      </div>

      <div v-else class="bg-white shadow rounded-lg overflow-hidden">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">用户</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">角色</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">状态</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">创建时间</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">最后登录</th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <UserRow
              v-for="user in filteredUsers"
              :key="user.id"
              :user="user"
              @edit="handleEditUser"
              @toggle-status="handleToggleStatus"
              @reset-password="handleResetPassword"
              @delete="confirmDelete"
            />
          </tbody>
        </table>
      </div>

      <!-- 新建/编辑用户模态框 -->
      <UserModal
        v-if="showCreateModal || showEditModal"
        :user="editingUser"
        :mode="editingUser ? 'edit' : 'create'"
        @close="closeModal"
        @save="handleSaveUser"
      />

      <!-- 删除确认 -->
      <div v-if="deleteTarget" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
          <div class="px-6 py-4">
            <h3 class="text-lg font-semibold text-gray-900">确认删除</h3>
            <p class="mt-2 text-gray-600">
              确定要删除用户 "{{ deleteTarget.username }}" 吗？此操作不可撤销。
            </p>
          </div>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button @click="deleteTarget = null" class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50">
              取消
            </button>
            <button @click="executeDelete" :disabled="deleting" class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50">
              {{ deleting ? '删除中...' : '删除' }}
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
import UserRow from '@/components/users/UserRow.vue'
import UserModal from '@/components/users/UserModal.vue'
import { api } from '@/utils/api'

// 状态
const loading = ref(true)
const users = ref<any[]>([])
const searchQuery = ref('')
const roleFilter = ref('all')
const statusFilter = ref('all')

// 模态框
const showCreateModal = ref(false)
const showEditModal = ref(false)
const editingUser = ref<any>(null)

// 删除
const deleteTarget = ref<any>(null)
const deleting = ref(false)

// Toast
const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

// 状态统计
const statusCounts = computed(() => {
  const counts = { active: 0, inactive: 0, disabled: 0 }
  users.value.forEach(u => {
    const status = u.status as keyof typeof counts
    if (counts[status] !== undefined) counts[status]++
  })
  return counts
})

// 角色统计
const roleCounts = computed(() => {
  const counts = { admin: 0, user: 0, guest: 0 }
  users.value.forEach(u => {
    const role = u.role as keyof typeof counts
    if (counts[role] !== undefined) counts[role]++
  })
  return counts
})

// 计算筛选后的用户
const filteredUsers = computed(() => {
  let result = users.value

  if (roleFilter.value !== 'all') {
    result = result.filter(u => u.role === roleFilter.value)
  }

  if (statusFilter.value !== 'all') {
    result = result.filter(u => u.status === statusFilter.value)
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(u =>
      u.username?.toLowerCase().includes(query) ||
      u.email?.toLowerCase().includes(query)
    )
  }

  return result
})

// 加载用户列表
const loadUsers = async () => {
  loading.value = true
  try {
    const response = await api.users.list()
    users.value = response.data.users || response.data.data || response.data || []
  } catch (error) {
    console.error('Failed to load users:', error)
    showToast('error', '加载用户列表失败')
  } finally {
    loading.value = false
  }
}

// 编辑用户
const handleEditUser = (user: any) => {
  editingUser.value = user
  showEditModal.value = true
}

// 切换状态
const handleToggleStatus = async (user: any) => {
  const newStatus = user.status === 'disabled' ? 'active' : 'disabled'
  try {
    await api.users.update(user.id, { status: newStatus })
    user.status = newStatus
    showToast('success', `用户已${newStatus === 'disabled' ? '禁用' : '启用'}`)
  } catch (error) {
    showToast('error', '状态切换失败')
  }
}

// 重置密码
const handleResetPassword = async (user: any) => {
  if (!confirm(`确定要重置用户 "${user.username}" 的密码吗？`)) return
  try {
    await api.users.update(user.id, { reset_password: true })
    showToast('success', '密码已重置，新密码已发送到用户邮箱')
  } catch (error) {
    showToast('error', '密码重置失败')
  }
}

// 确认删除
const confirmDelete = (user: any) => {
  deleteTarget.value = user
}

// 执行删除
const executeDelete = async () => {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    await api.users.delete(deleteTarget.value.id)
    showToast('success', '用户已删除')
    deleteTarget.value = null
    await loadUsers()
  } catch (error) {
    showToast('error', '删除失败')
  } finally {
    deleting.value = false
  }
}

// 保存用户
const handleSaveUser = async (userData: any) => {
  try {
    if (editingUser.value) {
      await api.users.update(editingUser.value.id, userData)
      showToast('success', '用户已更新')
    } else {
      await api.users.create(userData)
      showToast('success', '用户已创建')
    }
    closeModal()
    await loadUsers()
  } catch (error) {
    showToast('error', '保存失败')
  }
}

// 关闭模态框
const closeModal = () => {
  showCreateModal.value = false
  showEditModal.value = false
  editingUser.value = null
}

// Toast
const showToast = (type: 'success' | 'error', message: string) => {
  toast.value = { show: true, type, message }
  setTimeout(() => { toast.value.show = false }, 3000)
}

// 生命周期
onMounted(() => loadUsers())
</script>