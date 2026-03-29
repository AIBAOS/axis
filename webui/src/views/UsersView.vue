<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">用户管理</h1>
          <p class="text-gray-600 mt-1">管理系统用户、用户组和权限</p>
        </div>
        <div class="flex space-x-2">
          <button @click="showGroupModal = true" class="btn-secondary flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
            <span>新建组</span>
          </button>
          <button @click="showCreateModal = true" class="btn-primary flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" /></svg>
            <span>新建用户</span>
          </button>
        </div>
      </div>

      <!-- 选项卡 -->
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8">
          <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id" :class="[currentTab === tab.id ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700', 'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm']">{{ tab.name }}</button>
        </nav>
      </div>

      <!-- 用户列表 -->
      <template v-if="currentTab === 'users'">
        <!-- 统计卡片 -->
        <div class="grid grid-cols-2 md:grid-cols-6 gap-3">
          <div class="bg-white rounded-lg shadow p-3"><p class="text-xs text-gray-500">总用户</p><p class="text-xl font-bold">{{ users.length }}</p></div>
          <div class="bg-white rounded-lg shadow p-3"><p class="text-xs text-gray-500">活跃</p><p class="text-xl font-bold text-green-600">{{ statusCounts.active }}</p></div>
          <div class="bg-white rounded-lg shadow p-3"><p class="text-xs text-gray-500">离线</p><p class="text-xl font-bold text-gray-500">{{ statusCounts.inactive }}</p></div>
          <div class="bg-white rounded-lg shadow p-3"><p class="text-xs text-gray-500">锁定</p><p class="text-xl font-bold text-yellow-600">{{ statusCounts.locked }}</p></div>
          <div class="bg-white rounded-lg shadow p-3"><p class="text-xs text-gray-500">禁用</p><p class="text-xl font-bold text-red-600">{{ statusCounts.disabled }}</p></div>
          <div class="bg-white rounded-lg shadow p-3"><p class="text-xs text-gray-500">管理员</p><p class="text-xl font-bold text-purple-600">{{ roleCounts.admin }}</p></div>
        </div>

        <!-- 筛选 -->
        <div class="flex space-x-4">
          <input v-model="searchQuery" type="text" placeholder="搜索用户名或邮箱..." class="flex-1 px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 text-sm" />
          <select v-model="statusFilter" class="px-3 py-2 border rounded-lg text-sm"><option value="all">全部状态</option><option value="active">活跃</option><option value="inactive">离线</option><option value="locked">锁定</option><option value="disabled">禁用</option></select>
          <select v-model="roleFilter" class="px-3 py-2 border rounded-lg text-sm"><option value="all">全部角色</option><option value="admin">管理员</option><option value="user">用户</option><option value="guest">访客</option></select>
          <button @click="loadUsers" :disabled="loading" class="btn-secondary text-sm">刷新</button>
        </div>

        <!-- 批量操作 -->
        <div v-if="selectedUsers.length > 0" class="flex items-center space-x-4 bg-blue-50 rounded-lg p-3">
          <span class="text-sm text-blue-700">已选 {{ selectedUsers.length }} 个用户</span>
          <button @click="batchEnable" class="text-sm text-green-600 hover:text-green-700 font-medium">批量启用</button>
          <button @click="batchDisable" class="text-sm text-yellow-600 hover:text-yellow-700 font-medium">批量禁用</button>
          <button @click="batchDelete" class="text-sm text-red-600 hover:text-red-700 font-medium">批量删除</button>
          <button @click="selectedUsers = []" class="text-sm text-gray-500 hover:text-gray-700">取消选择</button>
        </div>

        <!-- 用户表格 -->
        <div v-if="loading" class="flex justify-center py-12"><svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" /></svg></div>
        <div v-else-if="filteredUsers.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" /></svg><p class="mt-4 text-gray-600">暂无用户</p></div>
        <div v-else class="bg-white rounded-lg shadow overflow-hidden">
          <table class="w-full"><thead class="bg-gray-50 border-b"><tr>
            <th class="w-8 px-2"><input type="checkbox" @change="toggleSelectAll" :checked="allSelected" class="w-4 h-4 rounded" /></th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">用户</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">UID</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">主组</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">角色</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">状态</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">家目录</th>
            <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">操作</th>
          </tr></thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-for="user in filteredUsers" :key="user.id" @dblclick="openEditModal(user)" :class="{'bg-primary-50': isSelected(user.id)}" class="hover:bg-gray-50 cursor-pointer">
                <td class="px-2"><input type="checkbox" :checked="isSelected(user.id)" @click.stop="toggleSelect(user.id)" class="w-4 h-4 rounded" /></td>
                <td class="px-4 py-3"><div class="flex items-center space-x-3"><div :class="getAvatarClass(user.role)" class="w-8 h-8 rounded-full flex items-center justify-center"><span class="text-xs font-medium">{{ user.username?.charAt(0).toUpperCase() }}</span></div><div><p class="text-sm font-medium text-gray-900">{{ user.username }}</p><p class="text-xs text-gray-500">{{ user.email }}</p></div></div></td>
                <td class="px-4 py-3 text-sm text-gray-600 font-mono">{{ user.uid || user.id }}</td>
                <td class="px-4 py-3 text-sm text-gray-600">{{ user.primary_group || 'users' }}</td>
                <td class="px-4 py-3"><span :class="getRoleClass(user.role)" class="px-2 py-0.5 text-xs rounded-full">{{ getRoleLabel(user.role) }}</span></td>
                <td class="px-4 py-3"><div class="flex items-center space-x-1"><span class="w-2 h-2 rounded-full" :class="getStatusDotClass(user.status)"></span><span class="text-sm" :class="getStatusTextClass(user.status)">{{ getStatusLabel(user.status) }}</span></div></td>
                <td class="px-4 py-3 text-sm text-gray-500 font-mono text-xs truncate max-w-32">{{ user.home_dir || `/home/${user.username}` }}</td>
                <td class="px-4 py-3 text-right">
                  <button @click.stop="openEditModal(user)" class="text-sm text-primary-600 hover:text-primary-700 mr-2">编辑</button>
                  <button @click.stop="toggleUserStatus(user)" :class="user.status === 'disabled' ? 'text-green-600 hover:text-green-700' : 'text-yellow-600 hover:text-yellow-700'" class="text-sm mr-2">{{ user.status === 'disabled' ? '启用' : '禁用' }}</button>
                  <button @click.stop="resetPassword(user)" class="text-sm text-blue-600 hover:text-blue-700 mr-2">重置密码</button>
                  <button @click.stop="confirmDelete(user)" class="text-sm text-red-600 hover:text-red-700">删除</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>

      <!-- 用户组 -->
      <template v-else-if="currentTab === 'groups'">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-semibold">用户组列表</h2>
          <button @click="showGroupModal = true" class="btn-primary text-sm">新建组</button>
        </div>
        <div v-if="groups.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" /></svg><p class="mt-4 text-gray-600">暂无用户组</p></div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div v-for="group in groups" :key="group.id" class="bg-white rounded-lg shadow p-4">
            <div class="flex justify-between items-start">
              <div><h3 class="font-semibold text-gray-900">{{ group.name }}</h3><p class="text-sm text-gray-500">GID: {{ group.gid }}</p></div>
              <span :class="group.admin ? 'bg-purple-100 text-purple-700' : 'bg-gray-100 text-gray-700'" class="px-2 py-0.5 text-xs rounded-full">{{ group.admin ? '管理组' : '普通组' }}</span>
            </div>
            <p class="text-sm text-gray-600 mt-2">{{ group.description || '暂无描述' }}</p>
            <div class="mt-3 flex justify-between items-center">
              <span class="text-xs text-gray-500">{{ group.members?.length || 0 }} 个成员</span>
              <div class="flex space-x-2">
                <button @click="editGroup(group)" class="text-sm text-primary-600 hover:text-primary-700">编辑</button>
                <button @click="deleteGroup(group)" class="text-sm text-red-600 hover:text-red-700">删除</button>
              </div>
            </div>
          </div>
        </div>
      </template>

      <!-- 权限概览 -->
      <template v-else-if="currentTab === 'permissions'">
        <div class="bg-white rounded-lg shadow p-6">
          <h2 class="text-lg font-semibold mb-4">权限概览</h2>
          <div class="space-y-4">
            <div v-for="user in users" :key="user.id" class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <div class="flex items-center space-x-3"><div :class="getAvatarClass(user.role)" class="w-8 h-8 rounded-full flex items-center justify-center"><span class="text-xs font-medium">{{ user.username?.charAt(0).toUpperCase() }}</span></div><div><p class="font-medium text-gray-900">{{ user.username }}</p><p class="text-sm text-gray-500">{{ user.email }}</p></div></div>
              <div class="flex items-center space-x-2">
                <span v-for="group in getUserGroups(user.id)" :key="group" class="px-2 py-0.5 text-xs bg-gray-200 text-gray-700 rounded">{{ group }}</span>
                <span :class="getRoleClass(user.role)" class="px-2 py-0.5 text-xs rounded-full">{{ getRoleLabel(user.role) }}</span>
              </div>
            </div>
          </div>
        </div>
      </template>

      <!-- 密码策略 -->
      <template v-else-if="currentTab === 'policy'">
        <div class="max-w-2xl space-y-6">
          <div class="bg-white rounded-lg shadow p-6">
            <h3 class="font-semibold text-gray-900 mb-4">密码策略设置</h3>
            <form @submit.prevent="savePasswordPolicy" class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">最小密码长度</label>
                <input v-model.number="passwordPolicy.minLength" type="number" min="6" max="32" class="w-32 px-3 py-2 border rounded-lg" />
                <p class="text-xs text-gray-500 mt-1">建议至少 8 位</p>
              </div>
              
              <div class="space-y-2">
                <label class="block text-sm font-medium text-gray-700">密码复杂度要求</label>
                <div class="space-y-2">
                  <label class="flex items-center">
                    <input v-model="passwordPolicy.requireUppercase" type="checkbox" class="h-4 w-4 rounded" />
                    <span class="ml-2 text-sm text-gray-700">必须包含大写字母 (A-Z)</span>
                  </label>
                  <label class="flex items-center">
                    <input v-model="passwordPolicy.requireLowercase" type="checkbox" class="h-4 w-4 rounded" />
                    <span class="ml-2 text-sm text-gray-700">必须包含小写字母 (a-z)</span>
                  </label>
                  <label class="flex items-center">
                    <input v-model="passwordPolicy.requireNumbers" type="checkbox" class="h-4 w-4 rounded" />
                    <span class="ml-2 text-sm text-gray-700">必须包含数字 (0-9)</span>
                  </label>
                  <label class="flex items-center">
                    <input v-model="passwordPolicy.requireSpecial" type="checkbox" class="h-4 w-4 rounded" />
                    <span class="ml-2 text-sm text-gray-700">必须包含特殊字符 (!@#$%^&*)</span>
                  </label>
                </div>
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">密码有效期（天）</label>
                  <input v-model.number="passwordPolicy.maxAge" type="number" min="0" class="w-full px-3 py-2 border rounded-lg" />
                  <p class="text-xs text-gray-500 mt-1">0 表示永不过期</p>
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">密码历史记录</label>
                  <input v-model.number="passwordPolicy.historyCount" type="number" min="0" max="24" class="w-full px-3 py-2 border rounded-lg" />
                  <p class="text-xs text-gray-500 mt-1">不能重复使用最近 N 个密码</p>
                </div>
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">最大登录失败次数</label>
                  <input v-model.number="passwordPolicy.maxFailedAttempts" type="number" min="0" max="10" class="w-full px-3 py-2 border rounded-lg" />
                  <p class="text-xs text-gray-500 mt-1">超过后锁定账户</p>
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">锁定时间（分钟）</label>
                  <input v-model.number="passwordPolicy.lockoutDuration" type="number" min="5" class="w-full px-3 py-2 border rounded-lg" />
                </div>
              </div>

              <div class="flex justify-end">
                <button type="submit" :disabled="savingPolicy" class="btn-primary">
                  {{ savingPolicy ? '保存中...' : '保存策略' }}
                </button>
              </div>
            </form>
          </div>

          <div class="bg-white rounded-lg shadow p-6">
            <h3 class="font-semibold text-gray-900 mb-4">会话设置</h3>
            <form @submit.prevent="saveSessionPolicy" class="space-y-4">
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">会话超时（分钟）</label>
                  <input v-model.number="sessionPolicy.timeout" type="number" min="5" class="w-full px-3 py-2 border rounded-lg" />
                  <p class="text-xs text-gray-500 mt-1">无操作自动登出时间</p>
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">最大并发会话</label>
                  <input v-model.number="sessionPolicy.maxConcurrent" type="number" min="1" max="10" class="w-full px-3 py-2 border rounded-lg" />
                </div>
              </div>

              <div class="flex items-center justify-between">
                <div>
                  <label class="block text-sm font-medium text-gray-700">双因素认证</label>
                  <p class="text-sm text-gray-500">要求用户启用 2FA</p>
                </div>
                <input v-model="sessionPolicy.require2FA" type="checkbox" class="h-5 w-5 rounded" />
              </div>

              <div class="flex justify-end">
                <button type="submit" class="btn-primary">保存设置</button>
              </div>
            </form>
          </div>
        </div>
      </template>

      <!-- 模态框 -->
      <UserModal v-if="showCreateModal || showEditModal" :mode="showEditModal ? 'edit' : 'create'" :user="editingUser" @close="closeModal" @save="handleSaveUser" />

      <!-- 用户组模态框 -->
      <div v-if="showGroupModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4">
          <div class="flex justify-between items-center px-6 py-4 border-b"><h3 class="text-lg font-semibold">{{ editingGroup ? '编辑用户组' : '新建用户组' }}</h3><button @click="showGroupModal = false; editingGroup = null" class="text-gray-400 hover:text-gray-600"><svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button></div>
          <form @submit.prevent="saveGroup" class="p-6 space-y-4">
            <div><label class="block text-sm font-medium text-gray-700 mb-1">组名</label><input v-model="groupForm.name" type="text" required class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="developers" /></div>
            <div><label class="block text-sm font-medium text-gray-700 mb-1">GID</label><input v-model.number="groupForm.gid" type="number" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="1000" /></div>
            <div><label class="block text-sm font-medium text-gray-700 mb-1">描述</label><textarea v-model="groupForm.description" rows="2" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="开发团队"></textarea></div>
            <div class="flex items-center"><input v-model="groupForm.admin" type="checkbox" id="adminGroup" class="h-4 w-4 text-primary-600 rounded" /><label for="adminGroup" class="ml-2 text-sm text-gray-700">管理组（拥有 sudo 权限）</label></div>
          </form>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg"><button @click="showGroupModal = false; editingGroup = null" class="px-4 py-2 border rounded-lg text-gray-700 hover:bg-gray-50">取消</button><button @click="saveGroup" class="btn-primary">保存</button></div>
        </div>
      </div>

      <!-- 删除确认 -->
      <div v-if="deleteTarget" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
          <div class="px-6 py-4"><h3 class="text-lg font-semibold text-gray-900">确认删除</h3><p class="mt-2 text-gray-600">确定要删除用户 "{{ deleteTarget.username }}" 吗？此操作不可撤销。</p></div>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg"><button @click="deleteTarget = null" class="px-4 py-2 border rounded-lg text-gray-700 hover:bg-gray-50">取消</button><button @click="executeDelete" :disabled="deleting" class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50">{{ deleting ? '删除中...' : '删除' }}</button></div>
        </div>
      </div>

      <!-- Toast -->
      <div v-if="toast.show" class="fixed bottom-4 right-4 z-50"><div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">{{ toast.message }}</div></div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import UserModal from '@/components/users/UserModal.vue'
import { api } from '@/utils/api'

const tabs = [{ id: 'users', name: '用户列表' }, { id: 'groups', name: '用户组' }, { id: 'permissions', name: '权限概览' }, { id: 'policy', name: '密码策略' }]
const currentTab = ref('users')
const loading = ref(true)
const users = ref<any[]>([])
const groups = ref<any[]>([])
const searchQuery = ref('')
const statusFilter = ref('all')
const roleFilter = ref('all')

// 选择
const selectedUsers = ref<number[]>([])

// 模态框
const showCreateModal = ref(false)
const showEditModal = ref(false)
const editingUser = ref<any>(null)
const deleteTarget = ref<any>(null)
const deleting = ref(false)

// 用户组
const showGroupModal = ref(false)
const editingGroup = ref<any>(null)
const groupForm = ref({ name: '', gid: 1000, description: '', admin: false })

// 密码策略
const savingPolicy = ref(false)
const passwordPolicy = ref({
  minLength: 8,
  requireUppercase: true,
  requireLowercase: true,
  requireNumbers: true,
  requireSpecial: false,
  maxAge: 90,
  historyCount: 5,
  maxFailedAttempts: 5,
  lockoutDuration: 30
})

const sessionPolicy = ref({
  timeout: 30,
  maxConcurrent: 3,
  require2FA: false
})

const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

const statusCounts = computed(() => { const c: Record<string, number> = { active: 0, inactive: 0, locked: 0, disabled: 0 }; users.value.forEach(u => { if (c[u.status] !== undefined) c[u.status]++ }); return c })
const roleCounts = computed(() => { const c: Record<string, number> = { admin: 0, user: 0, guest: 0 }; users.value.forEach(u => { if (c[u.role] !== undefined) c[u.role]++ }); return c })
const filteredUsers = computed(() => { let r = users.value; if (statusFilter.value !== 'all') r = r.filter(u => u.status === statusFilter.value); if (roleFilter.value !== 'all') r = r.filter(u => u.role === roleFilter.value); if (searchQuery.value) { const q = searchQuery.value.toLowerCase(); r = r.filter(u => u.username?.toLowerCase().includes(q) || u.email?.toLowerCase().includes(q)) } return r })
const allSelected = computed(() => filteredUsers.value.length > 0 && selectedUsers.value.length === filteredUsers.value.length)
const isSelected = (id: number) => selectedUsers.value.includes(id)
const toggleSelect = (id: number) => { const i = selectedUsers.value.indexOf(id); if (i >= 0) selectedUsers.value.splice(i, 1); else selectedUsers.value.push(id) }
const toggleSelectAll = () => { if (allSelected.value) selectedUsers.value = []; else selectedUsers.value = filteredUsers.value.map(u => u.id) }

const loadUsers = async () => { loading.value = true; try { const r = await api.users.list(); users.value = r.data.data || r.data || [] } catch (e) {} finally { loading.value = false } }
const loadGroups = async () => { groups.value = [{ id: 1, name: 'admin', gid: 1000, description: '系统管理员', admin: true, members: [1] }, { id: 2, name: 'users', gid: 100, description: '普通用户', admin: false, members: [2, 3] }, { id: 3, name: 'developers', gid: 1001, description: '开发团队', admin: false, members: [] }] }

const openEditModal = (u: any) => { editingUser.value = u; showEditModal.value = true }
const closeModal = () => { showCreateModal.value = false; showEditModal.value = false; editingUser.value = null }
const handleSaveUser = async (data: any) => { try { if (showEditModal.value && editingUser.value) await api.users.update(editingUser.value.id, data); else await api.users.create(data); showToast('success', '保存成功'); closeModal(); loadUsers() } catch (e) { showToast('error', '保存失败') } }

const toggleUserStatus = async (u: any) => { const newStatus = u.status === 'disabled' ? 'active' : 'disabled'; try { await api.users.update(u.id, { status: newStatus }); u.status = newStatus; showToast('success', `用户已${newStatus === 'active' ? '启用' : '禁用'}`) } catch (e) { showToast('error', '操作失败') } }
const resetPassword = async (u: any) => { if (!confirm(`确定重置用户 "${u.username}" 的密码吗？`)) return; try { await api.users.update(u.id, { reset_password: true }); showToast('success', '密码已重置') } catch (e) { showToast('error', '重置失败') } }
const confirmDelete = (u: any) => { deleteTarget.value = u }
const executeDelete = async () => { if (!deleteTarget.value) return; deleting.value = true; try { await api.users.delete(deleteTarget.value.id); showToast('success', '用户已删除'); deleteTarget.value = null; selectedUsers.value = selectedUsers.value.filter(id => !deleteTarget.value || id !== deleteTarget.value.id); loadUsers() } catch (e) { showToast('error', '删除失败') } finally { deleting.value = false } }

const batchEnable = async () => { for (const id of selectedUsers.value) { const u = users.value.find(x => x.id === id); if (u && u.status !== 'active') await toggleUserStatus(u) } selectedUsers.value = [] }
const batchDisable = async () => { for (const id of selectedUsers.value) { const u = users.value.find(x => x.id === id); if (u && u.status !== 'disabled') await toggleUserStatus(u) } selectedUsers.value = [] }
const batchDelete = async () => { if (!confirm(`确定删除选中的 ${selectedUsers.value.length} 个用户吗？`)) return; for (const id of selectedUsers.value) { try { await api.users.delete(id) } catch (e) {} } showToast('success', '批量删除完成'); selectedUsers.value = []; loadUsers() }

// 用户组
const saveGroup = async () => { if (!groupForm.value.name) return; if (editingGroup.value) { const i = groups.value.findIndex(g => g.id === editingGroup.value.id); if (i >= 0) groups.value[i] = { ...editingGroup.value, ...groupForm.value }; showToast('success', '用户组已更新') } else { groups.value.push({ id: Date.now(), ...groupForm.value, members: [] }); showToast('success', '用户组已创建') } showGroupModal.value = false; editingGroup.value = null; groupForm.value = { name: '', gid: 1000, description: '', admin: false } }
const editGroup = (g: any) => { editingGroup.value = g; groupForm.value = { name: g.name, gid: g.gid, description: g.description || '', admin: g.admin || false }; showGroupModal.value = true }
const deleteGroup = async (g: any) => { if (!confirm(`确定删除用户组 "${g.name}" 吗？`)) return; groups.value = groups.value.filter(x => x.id !== g.id); showToast('success', '用户组已删除') }
const getUserGroups = (userId: number) => groups.value.filter(g => g.members?.includes(userId)).map(g => g.name)

// 样式
const getAvatarClass = (role: string) => ({ admin: 'bg-red-100 text-red-600', user: 'bg-blue-100 text-blue-600', guest: 'bg-gray-100 text-gray-600' }[role] || 'bg-gray-100 text-gray-600')
const getRoleClass = (role: string) => ({ admin: 'bg-red-100 text-red-700', user: 'bg-blue-100 text-blue-700', guest: 'bg-gray-100 text-gray-700' }[role] || 'bg-gray-100 text-gray-700')
const getRoleLabel = (role: string) => ({ admin: '管理员', user: '用户', guest: '访客' }[role] || role)
const getStatusDotClass = (status: string) => ({ active: 'bg-green-500', inactive: 'bg-gray-400', locked: 'bg-yellow-500', disabled: 'bg-red-500' }[status] || 'bg-gray-400')
const getStatusTextClass = (status: string) => ({ active: 'text-green-700', inactive: 'text-gray-600', locked: 'text-yellow-700', disabled: 'text-red-700' }[status] || 'text-gray-600')
const getStatusLabel = (status: string) => ({ active: '正常', inactive: '离线', locked: '锁定', disabled: '禁用' }[status] || status)

const showToast = (type: 'success' | 'error', msg: string) => { toast.value = { show: true, type, message: msg }; setTimeout(() => toast.value.show = false, 3000) }

// 密码策略保存
const savePasswordPolicy = async () => {
  savingPolicy.value = true
  try {
    await api.settings.update({ password_policy: passwordPolicy.value })
    showToast('success', '密码策略已保存')
  } catch (e) {
    showToast('error', '保存失败')
  } finally {
    savingPolicy.value = false
  }
}

const saveSessionPolicy = async () => {
  try {
    await api.settings.update({ session_policy: sessionPolicy.value })
    showToast('success', '会话设置已保存')
  } catch (e) {
    showToast('error', '保存失败')
  }
}

onMounted(() => { loadUsers(); loadGroups() })
</script>