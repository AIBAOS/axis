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

      <!-- 搜索和筛选栏 -->
      <div class="flex space-x-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索用户名或邮箱..."
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            @input="handleSearch"
          />
        </div>
        <select
          v-model="roleFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          @change="handleFilter"
        >
          <option value="all">全部角色</option>
          <option value="admin">管理员</option>
          <option value="user">普通用户</option>
          <option value="guest">访客</option>
        </select>
        <select
          v-model="statusFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          @change="handleFilter"
        >
          <option value="all">全部状态</option>
          <option value="active">活跃</option>
          <option value="inactive">离线</option>
          <option value="disabled">禁用</option>
        </select>
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
      <div v-else-if="users.length === 0" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
        <p class="mt-4 text-gray-600">暂无用户</p>
        <p class="mt-2 text-sm text-gray-500">创建第一个用户开始使用</p>
      </div>

      <div v-else class="bg-white shadow rounded-lg overflow-hidden">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">用户</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">角色</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">状态</th>
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
              @delete="handleDeleteUser"
            />
          </tbody>
        </table>

        <!-- 分页 -->
        <div v-if="totalPages > 1" class="flex justify-center items-center space-x-2 py-4">
          <button
            @click="currentPage--"
            :disabled="currentPage === 1"
            class="px-4 py-2 border rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
          >
            上一页
          </button>
          <span class="text-gray-600">
            第 {{ currentPage }} / {{ totalPages }} 页
          </span>
          <button
            @click="currentPage++"
            :disabled="currentPage === totalPages"
            class="px-4 py-2 border rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
          >
            下一页
          </button>
        </div>
      </div>
    </div>

    <!-- 新建/编辑用户模态框 -->
    <UserModal
      v-if="showCreateModal || showEditModal"
      :user="editingUser"
      :mode="editingUser ? 'edit' : 'create'"
      @close="closeModal"
      @save="handleSaveUser"
    />
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
const currentPage = ref(1)
const totalPages = ref(1)

// 模态框
const showCreateModal = ref(false)
const showEditModal = ref(false)
const editingUser = ref<any>(null)

// 计算筛选后的用户
const filteredUsers = computed(() => {
  let result = users.value

  // 角色筛选
  if (roleFilter.value !== 'all') {
    result = result.filter(u => u.role === roleFilter.value)
  }

  // 状态筛选
  if (statusFilter.value !== 'all') {
    result = result.filter(u => u.status === statusFilter.value)
  }

  // 搜索筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(u =>
      u.username.toLowerCase().includes(query) ||
      u.email.toLowerCase().includes(query)
    )
  }

  return result
})

// 加载用户列表
const loadUsers = async () => {
  loading.value = true
  try {
    const response = await api.users.list({
      page: currentPage.value,
      page_size: 20
    })
    users.value = response.data.users || []
    totalPages.value = response.data.total_pages || 1
  } catch (error) {
    console.error('Failed to load users:', error)
    alert('加载用户列表失败')
  } finally {
    loading.value = false
  }
}

// 搜索
const handleSearch = () => {
  currentPage.value = 1
}

// 筛选
const handleFilter = () => {
  currentPage.value = 1
}

// 编辑用户
const handleEditUser = (user: any) => {
  editingUser.value = user
  showEditModal.value = true
}

// 删除用户
const handleDeleteUser = async (user: any) => {
  if (!confirm(`确定要删除用户 "${user.username}" 吗？`)) return

  try {
    await api.users.delete(user.id)
    await loadUsers()
  } catch (error) {
    console.error('Failed to delete user:', error)
    alert('删除失败')
  }
}

// 保存用户（新建/编辑）
const handleSaveUser = async (userData: any) => {
  try {
    if (editingUser.value) {
      await api.users.update(editingUser.value.id, userData)
    } else {
      await api.users.create(userData)
    }
    closeModal()
    await loadUsers()
  } catch (error) {
    console.error('Failed to save user:', error)
    alert('保存失败')
  }
}

// 关闭模态框
const closeModal = () => {
  showCreateModal.value = false
  showEditModal.value = false
  editingUser.value = null
}

// 生命周期
onMounted(() => {
  loadUsers()
})
</script>
