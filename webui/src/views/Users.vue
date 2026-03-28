<template>
  <div class="px-4 py-6 sm:px-0">
    <!-- 页面标题 -->
    <div class="mb-6">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-2xl font-bold text-gray-900">👥 用户管理</h2>
          <p class="text-gray-600 mt-1">管理系统用户和权限</p>
        </div>
        <button
          @click="showCreateModal = true"
          class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
        >
          ➕ 新建用户
        </button>
      </div>
    </div>

    <!-- 搜索和筛选栏 -->
    <div class="bg-white shadow rounded-lg mb-6">
      <div class="px-4 py-5 sm:p-6">
        <div class="flex flex-col md:flex-row md:items-center md:space-x-4 space-y-4 md:space-y-0">
          <!-- 搜索框 -->
          <div class="flex-1">
            <label for="search" class="sr-only">搜索用户</label>
            <div class="relative rounded-md shadow-sm">
              <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <span class="text-gray-500 sm:text-sm">🔍</span>
              </div>
              <input
                v-model="searchQuery"
                @input="debouncedSearch"
                type="text"
                id="search"
                class="focus:ring-primary-500 focus:border-primary-500 block w-full pl-10 sm:text-sm border-gray-300 rounded-md"
                placeholder="搜索用户名或邮箱..."
              />
            </div>
          </div>

          <!-- 角色筛选 -->
          <div class="w-full md:w-48">
            <label for="role" class="block text-sm font-medium text-gray-700 mb-1">角色</label>
            <select
              v-model="selectedRole"
              @change="fetchUsers"
              id="role"
              class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm rounded-md"
            >
              <option value="">全部角色</option>
              <option value="admin">管理员</option>
              <option value="user">普通用户</option>
              <option value="guest">访客</option>
            </select>
          </div>

          <!-- 状态筛选 -->
          <div class="w-full md:w-48">
            <label for="status" class="block text-sm font-medium text-gray-700 mb-1">状态</label>
            <select
              v-model="selectedStatus"
              @change="fetchUsers"
              id="status"
              class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm rounded-md"
            >
              <option value="">全部状态</option>
              <option value="active">活跃</option>
              <option value="inactive">非活跃</option>
              <option value="disabled">禁用</option>
            </select>
          </div>

          <!-- 刷新按钮 -->
          <div class="w-full md:w-auto">
            <button
              @click="refreshUsers"
              class="w-full md:w-auto inline-flex items-center justify-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
            >
              🔄 刷新
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 用户统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-6">
      <div class="bg-white overflow-hidden shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <dt class="text-sm font-medium text-gray-500 truncate">总用户数</dt>
          <dd class="mt-1 text-3xl font-semibold text-gray-900">{{ totalUsers }}</dd>
        </div>
      </div>
      <div class="bg-white overflow-hidden shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <dt class="text-sm font-medium text-gray-500 truncate">活跃用户</dt>
          <dd class="mt-1 text-3xl font-semibold text-green-600">{{ activeUsers }}</dd>
        </div>
      </div>
      <div class="bg-white overflow-hidden shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <dt class="text-sm font-medium text-gray-500 truncate">管理员</dt>
          <dd class="mt-1 text-3xl font-semibold text-primary-600">{{ adminUsers }}</dd>
        </div>
      </div>
      <div class="bg-white overflow-hidden shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <dt class="text-sm font-medium text-gray-500 truncate">在线用户</dt>
          <dd class="mt-1 text-3xl font-semibold text-blue-600">{{ onlineUsers }}</dd>
        </div>
      </div>
    </div>

    <!-- 用户列表 -->
    <div class="bg-white shadow rounded-lg">
      <div class="px-4 py-5 sm:p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">用户列表</h3>

        <!-- 加载状态 -->
        <div v-if="loading" class="text-center py-12">
          <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
          <p class="mt-2 text-gray-600">加载中...</p>
        </div>

        <!-- 错误提示 -->
        <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
          {{ error }}
        </div>

        <!-- 用户表格 -->
        <div v-else class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200">
            <thead class="bg-gray-50">
              <tr>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  用户
                </th>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  角色
                </th>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  状态
                </th>
                <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  最后登录
                </th>
                <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                  操作
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              <tr v-for="user in users" :key="user.id" class="hover:bg-gray-50">
                <td class="px-6 py-4 whitespace-nowrap">
                  <div class="flex items-center">
                    <div class="flex-shrink-0 h-10 w-10">
                      <div class="h-10 w-10 rounded-full bg-primary-100 flex items-center justify-center">
                        <span class="text-primary-600 font-medium">{{ getInitials(user.username) }}</span>
                      </div>
                    </div>
                    <div class="ml-4">
                      <div class="text-sm font-medium text-gray-900">{{ user.username }}</div>
                      <div class="text-sm text-gray-500">{{ user.email }}</div>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <div class="flex flex-wrap gap-1">
                    <span
                      v-for="role in user.roles"
                      :key="role"
                      :class="{
                        'bg-primary-100 text-primary-800': role === 'admin',
                        'bg-gray-100 text-gray-800': role === 'user',
                        'bg-yellow-100 text-yellow-800': role === 'guest',
                      }"
                      class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
                    >
                      {{ getRoleLabel(role) }}
                    </span>
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span
                    :class="{
                      'bg-green-100 text-green-800': user.is_active && user.is_online,
                      'bg-blue-100 text-blue-800': user.is_active && !user.is_online,
                      'bg-red-100 text-red-800': !user.is_active,
                    }"
                    class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
                  >
                    <span
                      :class="{
                        'bg-green-400': user.is_active && user.is_online,
                        'bg-blue-400': user.is_active && !user.is_online,
                        'bg-red-400': !user.is_active,
                      }"
                      class="flex-shrink-0 w-2 h-2 mr-1.5 rounded-full"
                    ></span>
                    {{ getStatusLabel(user) }}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {{ formatLastLogin(user.last_login) }}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                  <button
                    @click="editUser(user)"
                    class="text-primary-600 hover:text-primary-900 mr-3"
                  >
                    ✏️ 编辑
                  </button>
                  <button
                    @click="toggleUserStatus(user)"
                    class="text-blue-600 hover:text-blue-900 mr-3"
                  >
                    {{ user.is_active ? '🔒 禁用' : '🔓 启用' }}
                  </button>
                  <button
                    @click="deleteUser(user)"
                    class="text-red-600 hover:text-red-900"
                  >
                    🗑️ 删除
                  </button>
                </td>
              </tr>
            </tbody>
          </table>

          <!-- 空状态 -->
          <div v-if="users.length === 0" class="text-center py-12 text-gray-500">
            <p class="text-4xl mb-4">👥</p>
            <p>暂无用户数据</p>
          </div>
        </div>

        <!-- 分页 -->
        <div v-if="!loading && pagination.total_pages > 1" class="mt-4 flex items-center justify-between">
          <span class="text-sm text-gray-600">
            第 {{ pagination.page }} / {{ pagination.total_pages }} 页，共 {{ pagination.total }} 项
          </span>
          <div class="flex space-x-2">
            <button
              @click="changePage(pagination.page - 1)"
              :disabled="pagination.page <= 1"
              class="px-3 py-1 border border-gray-300 rounded text-sm disabled:opacity-50"
            >
              上一页
            </button>
            <button
              @click="changePage(pagination.page + 1)"
              :disabled="pagination.page >= pagination.total_pages"
              class="px-3 py-1 border border-gray-300 rounded text-sm disabled:opacity-50"
            >
              下一页
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 新建/编辑用户模态框 -->
    <div v-if="showCreateModal || showEditModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center">
      <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h3 class="text-lg font-medium text-gray-900 mb-4">
          {{ showEditModal ? '编辑用户' : '新建用户' }}
        </h3>
        <form @submit.prevent="submitUserForm" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">用户名</label>
            <input
              v-model="formData.username"
              type="text"
              required
              :disabled="showEditModal"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
            />
          </div>

          <div v-if="!showEditModal">
            <label class="block text-sm font-medium text-gray-700 mb-2">密码</label>
            <input
              v-model="formData.password"
              type="password"
              required
              minlength="6"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">邮箱</label>
            <input
              v-model="formData.email"
              type="email"
              required
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">角色</label>
            <select
              v-model="formData.role"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="user">普通用户</option>
              <option value="admin">管理员</option>
              <option value="guest">访客</option>
            </select>
          </div>

          <div v-if="showEditModal">
            <label class="flex items-center">
              <input
                v-model="formData.is_active"
                type="checkbox"
                class="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
              />
              <span class="ml-2 text-sm text-gray-700">启用账户</span>
            </label>
          </div>

          <div class="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              @click="closeModal"
              class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
            >
              取消
            </button>
            <button
              type="submit"
              :disabled="submitting"
              class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
            >
              {{ submitting ? '保存中...' : '保存' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import apiClient from '../api/client';

interface User {
  id: number;
  username: string;
  email: string;
  roles: string[];
  is_active: boolean;
  is_online?: boolean;
  created_at: number;
  updated_at: number;
  last_login?: number;
}

interface Pagination {
  page: number;
  per_page: number;
  total: number;
  total_pages: number;
}

const users = ref<User[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const searchQuery = ref('');
const selectedRole = ref('');
const selectedStatus = ref('');
const pagination = ref<Pagination>({
  page: 1,
  per_page: 20,
  total: 0,
  total_pages: 1,
});

// 模态框状态
const showCreateModal = ref(false);
const showEditModal = ref(false);
const submitting = ref(false);
const editingUser = ref<User | null>(null);

const formData = ref({
  username: '',
  password: '',
  email: '',
  role: 'user',
  is_active: true,
});

// 计算属性
const totalUsers = computed(() => pagination.value.total);
const activeUsers = computed(() => users.value.filter(u => u.is_active).length);
const adminUsers = computed(() => users.value.filter(u => u.roles.includes('admin')).length);
const onlineUsers = computed(() => users.value.filter(u => u.is_online).length);

// 获取用户列表
const fetchUsers = async () => {
  loading.value = true;
  error.value = null;
  try {
    const response = await apiClient.getUsers({
      page: pagination.value.page,
      per_page: pagination.value.per_page,
      role: selectedRole.value || undefined,
      status: selectedStatus.value || undefined,
    });
    if (response.success && response.data) {
      users.value = response.data.items || response.data || [];
      pagination.value = response.pagination || pagination.value;
    }
  } catch (err) {
    error.value = '加载用户列表失败';
    console.error('Failed to fetch users:', err);
  } finally {
    loading.value = false;
  }
};

// 刷新用户列表
const refreshUsers = () => {
  fetchUsers();
};

// 搜索（防抖）
let searchTimeout: number | null = null;
const debouncedSearch = () => {
  if (searchTimeout) clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => {
    pagination.value.page = 1;
    fetchUsers();
  }, 300);
};

// 分页
const changePage = (page: number) => {
  if (page < 1 || page > pagination.value.total_pages) return;
  pagination.value.page = page;
  fetchUsers();
};

// 编辑用户
const editUser = (user: User) => {
  editingUser.value = user;
  formData.value = {
    username: user.username,
    password: '',
    email: user.email,
    role: user.roles[0] || 'user',
    is_active: user.is_active,
  };
  showEditModal.value = true;
};

// 提交表单
const submitUserForm = async () => {
  submitting.value = true;
  try {
    if (showEditModal.value && editingUser.value) {
      // 更新用户
      await apiClient.updateUser(editingUser.value.id, {
        email: formData.value.email,
        role: formData.value.role,
        is_active: formData.value.is_active,
      });
    } else {
      // 创建用户
      await apiClient.createUser({
        username: formData.value.username,
        password: formData.value.password,
        email: formData.value.email,
        role: formData.value.role,
      });
    }
    closeModal();
    await fetchUsers();
  } catch (err) {
    alert(showEditModal.value ? '更新用户失败' : '创建用户失败');
    console.error('Submit user form failed:', err);
  } finally {
    submitting.value = false;
  }
};

// 关闭模态框
const closeModal = () => {
  showCreateModal.value = false;
  showEditModal.value = false;
  editingUser.value = null;
  formData.value = {
    username: '',
    password: '',
    email: '',
    role: 'user',
    is_active: true,
  };
};

// 切换用户状态
const toggleUserStatus = async (user: User) => {
  if (!confirm(`确定要${user.is_active ? '禁用' : '启用'}用户 "${user.username}" 吗？`)) return;
  try {
    await apiClient.updateUser(user.id, {
      is_active: !user.is_active,
    });
    await fetchUsers();
  } catch (err) {
    alert('操作失败');
    console.error('Toggle user status failed:', err);
  }
};

// 删除用户
const deleteUser = async (user: User) => {
  if (!confirm(`确定要删除用户 "${user.username}" 吗？此操作不可恢复！`)) return;
  try {
    await apiClient.deleteUser(user.id);
    await fetchUsers();
  } catch (err) {
    alert('删除用户失败');
    console.error('Delete user failed:', err);
  }
};

// 工具函数
const getInitials = (username: string): string => {
  return username.substring(0, 2).toUpperCase();
};

const getRoleLabel = (role: string): string => {
  const labels: Record<string, string> = {
    admin: '管理员',
    user: '用户',
    guest: '访客',
  };
  return labels[role] || role;
};

const getStatusLabel = (user: User): string => {
  if (!user.is_active) return '禁用';
  if (user.is_online) return '在线';
  return '离线';
};

const formatLastLogin = (timestamp?: number): string => {
  if (!timestamp) return '从未登录';
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const hours = Math.floor(diff / (1000 * 60 * 60));
  
  if (hours < 1) return '刚刚';
  if (hours < 24) return `${hours}小时前`;
  if (hours < 168) return `${Math.floor(hours / 24)}天前`;
  return date.toLocaleDateString('zh-CN');
};

onMounted(() => {
  fetchUsers();
});
</script>

<style scoped>
/* Users view specific styles */
</style>
