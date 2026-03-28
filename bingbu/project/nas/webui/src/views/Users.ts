import { defineComponent, ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'

interface User {
  id: number
  username: string
  email: string
  role: string
  status: 'active' | 'inactive' | 'disabled'
  lastLogin: number | null
  createdAt: number
}

interface UserStats {
  total: number
  active: number
  online: number
  disabled: number
}

export default defineComponent({
  name: 'Users',
  setup() {
    const router = useRouter()
    const loading = ref(false)
    const users = ref<User[]>([])
    const searchQuery = ref('')
    const roleFilter = ref('all')
    const statusFilter = ref('all')
    const showCreateModal = ref(false)
    const showEditModal = ref(false)
    const userToEdit = ref<User | null>(null)
    const userStats = ref<UserStats>({
      total: 0,
      active: 0,
      online: 0,
      disabled: 0
    })

    // 表单数据
    const formData = ref({
      username: '',
      email: '',
      password: '',
      role: 'user',
      status: 'active'
    })

    // 过滤后的用户列表
    const filteredUsers = computed(() => {
      return users.value.filter(user => {
        const matchSearch = searchQuery.value === '' ||
          user.username.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
          user.email.toLowerCase().includes(searchQuery.value.toLowerCase())
        
        const matchRole = roleFilter.value === 'all' || user.role === roleFilter.value
        const matchStatus = statusFilter.value === 'all' || user.status === statusFilter.value
        
        return matchSearch && matchRole && matchStatus
      })
    })

    // 状态标签样式
    const statusClass = (status: string) => {
      switch (status) {
        case 'active':
          return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
        case 'inactive':
          return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
        case 'disabled':
          return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'
        default:
          return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
      }
    }

    // 状态文本
    const statusText = (status: string) => {
      switch (status) {
        case 'active':
          return '在线'
        case 'inactive':
          return '离线'
        case 'disabled':
          return '禁用'
        default:
          return '未知'
      }
    }

    // 角色文本
    const roleText = (role: string) => {
      switch (role) {
        case 'admin':
          return '管理员'
        case 'user':
          return '普通用户'
        case 'guest':
          return '访客'
        default:
          return role
      }
    }

    // 格式化日期
    const formatDateTime = (timestamp: number | null) => {
      if (!timestamp) return '从未'
      const date = new Date(timestamp * 1000)
      return date.toLocaleString('zh-CN')
    }

    // 加载用户列表
    const loadUsers = async () => {
      loading.value = true
      try {
        const response = await apiClient.get('/users/list')
        if (response.data.success) {
          users.value = response.data.data.map((user: any) => ({
            id: user.id,
            username: user.username,
            email: user.email || '--',
            role: user.role || 'user',
            status: user.status || 'active',
            lastLogin: user.last_login || null,
            createdAt: user.created_at || 0
          }))
          
          // 更新统计数据
          userStats.value = {
            total: users.value.length,
            active: users.value.filter(u => u.status === 'active').length,
            online: users.value.filter(u => u.status === 'active').length,
            disabled: users.value.filter(u => u.status === 'disabled').length
          }
        }
      } catch (error) {
        console.error('Failed to load users:', error)
        // 使用模拟数据
        users.value = [
          { id: 1, username: 'admin', email: 'admin@axis.com', role: 'admin', status: 'active', lastLogin: Date.now() / 1000, createdAt: Date.now() / 1000 },
          { id: 2, username: 'user1', email: 'user1@axis.com', role: 'user', status: 'active', lastLogin: Date.now() / 1000 - 3600, createdAt: Date.now() / 1000 - 86400 },
          { id: 3, username: 'user2', email: 'user2@axis.com', role: 'user', status: 'inactive', lastLogin: Date.now() / 1000 - 86400, createdAt: Date.now() / 1000 - 172800 },
          { id: 4, username: 'guest', email: '--', role: 'guest', status: 'disabled', lastLogin: null, createdAt: Date.now() / 1000 - 259200 }
        ]
        userStats.value = {
          total: users.value.length,
          active: 2,
          online: 2,
          disabled: 1
        }
      } finally {
        loading.value = false
      }
    }

    // 显示创建用户模态框
    const showCreateUserModal = () => {
      formData.value = {
        username: '',
        email: '',
        password: '',
        role: 'user',
        status: 'active'
      }
      showCreateModal.value = true
    }

    // 显示编辑用户模态框
    const showEditUserModal = (user: User) => {
      userToEdit.value = user
      formData.value = {
        username: user.username,
        email: user.email !== '--' ? user.email : '',
        password: '',
        role: user.role,
        status: user.status
      }
      showEditModal.value = true
    }

    // 关闭模态框
    const closeCreateModal = () => {
      showCreateModal.value = false
      formData.value = { username: '', email: '', password: '', role: 'user', status: 'active' }
    }

    const closeEditModal = () => {
      showEditModal.value = false
      userToEdit.value = null
      formData.value = { username: '', email: '', password: '', role: 'user', status: 'active' }
    }

    // 创建用户
    const createUser = async () => {
      try {
        await apiClient.post('/users/create', {
          username: formData.value.username,
          email: formData.value.email,
          password: formData.value.password,
          role: formData.value.role
        })
        closeCreateModal()
        loadUsers()
      } catch (error) {
        console.error('Failed to create user:', error)
        alert('创建用户失败')
      }
    }

    // 更新用户
    const updateUser = async () => {
      if (!userToEdit.value) return
      try {
        await apiClient.put(`/users/update/${userToEdit.value.id}`, {
          email: formData.value.email,
          role: formData.value.role,
          status: formData.value.status
        })
        closeEditModal()
        loadUsers()
      } catch (error) {
        console.error('Failed to update user:', error)
        alert('更新用户失败')
      }
    }

    // 删除用户
    const deleteUser = async (user: User) => {
      if (!confirm(`确定要删除用户 "${user.username}" 吗？`)) return
      try {
        await apiClient.delete(`/users/delete/${user.id}`)
        loadUsers()
      } catch (error) {
        console.error('Failed to delete user:', error)
        alert('删除用户失败')
      }
    }

    // 刷新数据
    const refreshData = () => {
      loadUsers()
    }

    onMounted(() => {
      const token = localStorage.getItem('jwt_token')
      if (!token) {
        router.push('/login')
        return
      }
      loadUsers()
    })

    return {
      loading,
      users,
      searchQuery,
      roleFilter,
      statusFilter,
      filteredUsers,
      userStats,
      showCreateModal,
      showEditModal,
      formData,
      userToEdit,
      statusClass,
      statusText,
      roleText,
      formatDateTime,
      showCreateUserModal,
      showEditUserModal,
      closeCreateModal,
      closeEditModal,
      createUser,
      updateUser,
      deleteUser,
      refreshData
    }
  },
  template: `
    <div class="space-y-6">
      <!-- 页面标题和统计 -->
      <div class="flex items-center justify-between">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          用户管理
        </h1>
        <button
          @click="refreshData"
          class="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
        >
          <svg class="w-5 h-5 mr-2" :class="{ 'animate-spin': loading }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
          </svg>
          刷新
        </button>
      </div>

      <!-- 统计卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">总用户数</p>
          <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ userStats.total }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">活跃用户</p>
          <p class="text-2xl font-semibold text-green-600 dark:text-green-400">{{ userStats.active }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">在线用户</p>
          <p class="text-2xl font-semibold text-blue-600 dark:text-blue-400">{{ userStats.online }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">禁用用户</p>
          <p class="text-2xl font-semibold text-red-600 dark:text-red-400">{{ userStats.disabled }}</p>
        </div>
      </div>

      <!-- 搜索和筛选 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div class="md:col-span-1">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索用户名或邮箱..."
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <select
              v-model="roleFilter"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="all">所有角色</option>
              <option value="admin">管理员</option>
              <option value="user">普通用户</option>
              <option value="guest">访客</option>
            </select>
          </div>
          <div>
            <select
              v-model="statusFilter"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="all">所有状态</option>
              <option value="active">在线</option>
              <option value="inactive">离线</option>
              <option value="disabled">禁用</option>
            </select>
          </div>
          <div class="flex justify-end">
            <button
              @click="showCreateUserModal"
              class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
              </svg>
              新建用户
            </button>
          </div>
        </div>
      </div>

      <!-- 用户列表 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div class="grid grid-cols-12 gap-4 px-6 py-3 bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600 text-sm font-medium text-gray-500 dark:text-gray-400">
          <div class="col-span-3">用户</div>
          <div class="col-span-3">角色</div>
          <div class="col-span-2">状态</div>
          <div class="col-span-3">最后登录</div>
          <div class="col-span-1 text-right">操作</div>
        </div>

        <!-- 加载状态 -->
        <div v-if="loading" class="px-6 py-12 text-center">
          <svg class="animate-spin h-8 w-8 text-indigo-600 mx-auto" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">加载中...</p>
        </div>

        <!-- 空列表 -->
        <div v-else-if="filteredUsers.length === 0" class="px-6 py-12 text-center">
          <p class="text-sm text-gray-500 dark:text-gray-400">暂无用户</p>
        </div>

        <!-- 用户列表 -->
        <div v-else class="divide-y divide-gray-200 dark:divide-gray-700">
          <div
            v-for="user in filteredUsers"
            :key="user.id"
            class="grid grid-cols-12 gap-4 px-6 py-4 hover:bg-gray-50 dark:hover:bg-gray-700 items-center"
          >
            <!-- 用户信息 -->
            <div class="col-span-3">
              <div class="flex items-center">
                <div class="h-10 w-10 flex-shrink-0">
                  <div class="h-10 w-10 rounded-full bg-indigo-500 flex items-center justify-center text-white font-medium">
                    {{ user.username.charAt(0).toUpperCase() }}
                  </div>
                </div>
                <div class="ml-4">
                  <div class="text-sm font-medium text-gray-900 dark:text-white">{{ user.username }}</div>
                  <div class="text-sm text-gray-500 dark:text-gray-400">{{ user.email }}</div>
                </div>
              </div>
            </div>

            <!-- 角色 -->
            <div class="col-span-3">
              <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-indigo-100 text-indigo-800 dark:bg-indigo-900 dark:text-indigo-200">
                {{ roleText(user.role) }}
              </span>
            </div>

            <!-- 状态 -->
            <div class="col-span-2">
              <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="statusClass(user.status)">
                {{ statusText(user.status) }}
              </span>
            </div>

            <!-- 最后登录 -->
            <div class="col-span-3 text-sm text-gray-500 dark:text-gray-400">
              {{ formatDateTime(user.lastLogin) }}
            </div>

            <!-- 操作 -->
            <div class="col-span-1 flex justify-end space-x-2">
              <button
                @click="showEditUserModal(user)"
                class="text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400"
                title="编辑"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                </svg>
              </button>
              <button
                @click="deleteUser(user)"
                class="text-gray-400 hover:text-red-600 dark:hover:text-red-400"
                title="删除"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 创建用户模态框 -->
      <div v-if="showCreateModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
          <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
            <h3 class="text-lg font-medium text-gray-900 dark:text-white">新建用户</h3>
            <button @click="closeCreateModal" class="text-gray-400 hover:text-gray-500">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          <div class="px-6 py-4 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">用户名 *</label>
              <input
                v-model="formData.username"
                type="text"
                required
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">邮箱</label>
              <input
                v-model="formData.email"
                type="email"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">密码 *</label>
              <input
                v-model="formData.password"
                type="password"
                required
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">角色</label>
              <select
                v-model="formData.role"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              >
                <option value="user">普通用户</option>
                <option value="admin">管理员</option>
                <option value="guest">访客</option>
              </select>
            </div>
          </div>
          <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
            <button
              @click="closeCreateModal"
              class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              取消
            </button>
            <button
              @click="createUser"
              class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
            >
              创建
            </button>
          </div>
        </div>
      </div>

      <!-- 编辑用户模态框 -->
      <div v-if="showEditModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
          <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
            <h3 class="text-lg font-medium text-gray-900 dark:text-white">编辑用户</h3>
            <button @click="closeEditModal" class="text-gray-400 hover:text-gray-500">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          <div class="px-6 py-4 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">用户名</label>
              <input
                v-model="formData.username"
                type="text"
                disabled
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-500 dark:text-gray-400 dark:bg-gray-600 cursor-not-allowed"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">邮箱</label>
              <input
                v-model="formData.email"
                type="email"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">角色</label>
              <select
                v-model="formData.role"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              >
                <option value="user">普通用户</option>
                <option value="admin">管理员</option>
                <option value="guest">访客</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">状态</label>
              <select
                v-model="formData.status"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              >
                <option value="active">在线</option>
                <option value="inactive">离线</option>
                <option value="disabled">禁用</option>
              </select>
            </div>
          </div>
          <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
            <button
              @click="closeEditModal"
              class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              取消
            </button>
            <button
              @click="updateUser"
              class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
            >
              保存
            </button>
          </div>
        </div>
      </div>
    </div>
  `
})
