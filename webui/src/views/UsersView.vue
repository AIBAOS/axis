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

      <!-- 角色管理 -->
      <template v-else-if="currentTab === 'roles'">
        <div class="space-y-6">
          <div class="flex justify-between items-center">
            <h2 class="text-lg font-semibold">角色列表</h2>
            <button @click="showRoleModal = true" class="btn-primary text-sm">新建角色</button>
          </div>
          
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <!-- 管理员角色 -->
            <div class="bg-white rounded-lg shadow p-4 border-l-4 border-red-500">
              <div class="flex justify-between items-start mb-3">
                <div>
                  <h3 class="font-semibold text-gray-900">管理员</h3>
                  <p class="text-sm text-gray-500">Administrator</p>
                </div>
                <span class="px-2 py-0.5 text-xs rounded-full bg-red-100 text-red-700">系统角色</span>
              </div>
              <p class="text-sm text-gray-600 mb-4">拥有系统完全控制权限，可管理所有用户和系统设置</p>
              <div class="space-y-2">
                <p class="text-xs text-gray-500 font-medium">权限:</p>
                <div class="flex flex-wrap gap-1">
                  <span class="px-2 py-0.5 text-xs bg-green-100 text-green-700 rounded">读</span>
                  <span class="px-2 py-0.5 text-xs bg-blue-100 text-blue-700 rounded">写</span>
                  <span class="px-2 py-0.5 text-xs bg-red-100 text-red-700 rounded">删除</span>
                  <span class="px-2 py-0.5 text-xs bg-purple-100 text-purple-700 rounded">分享</span>
                  <span class="px-2 py-0.5 text-xs bg-orange-100 text-orange-700 rounded">管理</span>
                </div>
              </div>
              <div class="mt-4 pt-3 border-t">
                <p class="text-xs text-gray-500">{{ roleCounts.admin }} 个用户</p>
              </div>
            </div>

            <!-- 普通用户角色 -->
            <div class="bg-white rounded-lg shadow p-4 border-l-4 border-blue-500">
              <div class="flex justify-between items-start mb-3">
                <div>
                  <h3 class="font-semibold text-gray-900">普通用户</h3>
                  <p class="text-sm text-gray-500">User</p>
                </div>
                <span class="px-2 py-0.5 text-xs rounded-full bg-blue-100 text-blue-700">系统角色</span>
              </div>
              <p class="text-sm text-gray-600 mb-4">标准访问权限，可使用大部分功能，无管理权限</p>
              <div class="space-y-2">
                <p class="text-xs text-gray-500 font-medium">权限:</p>
                <div class="flex flex-wrap gap-1">
                  <span class="px-2 py-0.5 text-xs bg-green-100 text-green-700 rounded">读</span>
                  <span class="px-2 py-0.5 text-xs bg-blue-100 text-blue-700 rounded">写</span>
                  <span class="px-2 py-0.5 text-xs bg-gray-100 text-gray-400 rounded line-through">删除</span>
                  <span class="px-2 py-0.5 text-xs bg-gray-100 text-gray-400 rounded line-through">管理</span>
                </div>
              </div>
              <div class="mt-4 pt-3 border-t">
                <p class="text-xs text-gray-500">{{ roleCounts.user }} 个用户</p>
              </div>
            </div>

            <!-- 访客角色 -->
            <div class="bg-white rounded-lg shadow p-4 border-l-4 border-gray-400">
              <div class="flex justify-between items-start mb-3">
                <div>
                  <h3 class="font-semibold text-gray-900">访客</h3>
                  <p class="text-sm text-gray-500">Guest</p>
                </div>
                <span class="px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-700">系统角色</span>
              </div>
              <p class="text-sm text-gray-600 mb-4">只读访问权限，仅可查看公开信息</p>
              <div class="space-y-2">
                <p class="text-xs text-gray-500 font-medium">权限:</p>
                <div class="flex flex-wrap gap-1">
                  <span class="px-2 py-0.5 text-xs bg-green-100 text-green-700 rounded">读</span>
                  <span class="px-2 py-0.5 text-xs bg-gray-100 text-gray-400 rounded line-through">写</span>
                  <span class="px-2 py-0.5 text-xs bg-gray-100 text-gray-400 rounded line-through">删除</span>
                  <span class="px-2 py-0.5 text-xs bg-gray-100 text-gray-400 rounded line-through">管理</span>
                </div>
              </div>
              <div class="mt-4 pt-3 border-t">
                <p class="text-xs text-gray-500">{{ roleCounts.guest || 0 }} 个用户</p>
              </div>
            </div>
          </div>

          <!-- 自定义角色 -->
          <div class="bg-white rounded-lg shadow p-4">
            <h3 class="font-semibold text-gray-900 mb-4">自定义角色</h3>
            <div v-if="customRoles.length === 0" class="text-center py-8 text-gray-500">
              <p class="text-sm">暂无自定义角色</p>
              <button @click="showRoleModal = true" class="btn-secondary text-sm mt-2">创建自定义角色</button>
            </div>
            <div v-else class="space-y-3">
              <div v-for="role in customRoles" :key="role.id" class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                <div>
                  <h4 class="font-medium text-gray-900">{{ role.name }}</h4>
                  <p class="text-sm text-gray-500">{{ role.description }}</p>
                </div>
                <div class="flex items-center space-x-3">
                  <div class="flex flex-wrap gap-1">
                    <span v-for="perm in role.permissions" :key="perm" class="px-2 py-0.5 text-xs bg-primary-100 text-primary-700 rounded">{{ perm }}</span>
                  </div>
                  <button @click="editRole(role)" class="text-sm text-primary-600 hover:text-primary-700">编辑</button>
                  <button @click="deleteRole(role)" class="text-sm text-red-600 hover:text-red-700">删除</button>
                </div>
              </div>
            </div>
          </div>

          <!-- 权限矩阵 -->
          <div class="bg-white rounded-lg shadow p-4">
            <h3 class="font-semibold text-gray-900 mb-4">权限矩阵</h3>
            <div class="overflow-x-auto">
              <table class="w-full text-sm">
                <thead class="bg-gray-50">
                  <tr>
                    <th class="px-4 py-2 text-left font-medium text-gray-700">功能模块</th>
                    <th class="px-4 py-2 text-center font-medium text-gray-700">管理员</th>
                    <th class="px-4 py-2 text-center font-medium text-gray-700">普通用户</th>
                    <th class="px-4 py-2 text-center font-medium text-gray-700">访客</th>
                  </tr>
                </thead>
                <tbody class="divide-y">
                  <tr v-for="module in permissionModules" :key="module.id">
                    <td class="px-4 py-2 font-medium text-gray-900">{{ module.name }}</td>
                    <td class="px-4 py-2 text-center"><span class="text-green-600">✓</span></td>
                    <td class="px-4 py-2 text-center"><span :class="module.user ? 'text-green-600' : 'text-red-600'">{{ module.user ? '✓' : '✗' }}</span></td>
                    <td class="px-4 py-2 text-center"><span :class="module.guest ? 'text-green-600' : 'text-red-600'">{{ module.guest ? '✓' : '✗' }}</span></td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </template>

      <!-- 配额管理 -->
      <template v-else-if="currentTab === 'quotas'">
        <div class="space-y-6">
          <!-- 用户配额 -->
          <div class="bg-white rounded-lg shadow p-4">
            <div class="flex justify-between items-center mb-4">
              <h3 class="font-semibold text-gray-900">用户存储配额</h3>
              <button @click="showQuotaModal = true" class="btn-primary text-sm">设置配额</button>
            </div>
            <div class="overflow-x-auto">
              <table class="w-full text-sm">
                <thead class="bg-gray-50 border-b">
                  <tr>
                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">用户</th>
                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">配额限制</th>
                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">已使用</th>
                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">使用率</th>
                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">状态</th>
                    <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">操作</th>
                  </tr>
                </thead>
                <tbody class="divide-y">
                  <tr v-for="quota in userQuotas" :key="quota.id" class="hover:bg-gray-50">
                    <td class="px-4 py-3">
                      <div class="flex items-center space-x-2">
                        <div :class="getAvatarClass(quota.role)" class="w-6 h-6 rounded-full flex items-center justify-center">
                          <span class="text-xs">{{ quota.username?.charAt(0).toUpperCase() }}</span>
                        </div>
                        <span class="font-medium text-gray-900">{{ quota.username }}</span>
                      </div>
                    </td>
                    <td class="px-4 py-3 text-gray-600">{{ quota.limit === 0 ? '无限制' : formatBytes(quota.limit) }}</td>
                    <td class="px-4 py-3 text-gray-600">{{ formatBytes(quota.used) }}</td>
                    <td class="px-4 py-3">
                      <div class="flex items-center space-x-2">
                        <div class="w-20 bg-gray-200 rounded-full h-2">
                          <div :class="getQuotaClass(quota.percent)" class="h-2 rounded-full" :style="{ width: Math.min(quota.percent, 100) + '%' }"></div>
                        </div>
                        <span class="text-xs text-gray-600">{{ quota.percent.toFixed(1) }}%</span>
                      </div>
                    </td>
                    <td class="px-4 py-3">
                      <span :class="getQuotaStatusClass(quota.percent)" class="px-2 py-1 text-xs rounded-full">{{ getQuotaStatusLabel(quota.percent) }}</span>
                    </td>
                    <td class="px-4 py-3 text-right">
                      <button @click="editQuota(quota)" class="text-sm text-primary-600 hover:text-primary-700 mr-2">编辑</button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- 用户组配额 -->
          <div class="bg-white rounded-lg shadow p-4">
            <div class="flex justify-between items-center mb-4">
              <h3 class="font-semibold text-gray-900">用户组存储配额</h3>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <div v-for="groupQuota in groupQuotas" :key="groupQuota.id" class="p-4 border rounded-lg">
                <div class="flex justify-between items-start mb-2">
                  <div>
                    <h4 class="font-medium text-gray-900">{{ groupQuota.name }}</h4>
                    <p class="text-sm text-gray-500">{{ groupQuota.members }} 个成员</p>
                  </div>
                  <span :class="groupQuota.limit === 0 ? 'text-gray-500' : 'text-primary-600'" class="text-sm font-medium">
                    {{ groupQuota.limit === 0 ? '无限制' : formatBytes(groupQuota.limit) }}
                  </span>
                </div>
                <div class="mt-2">
                  <div class="flex justify-between text-xs text-gray-500 mb-1">
                    <span>已使用</span>
                    <span>{{ formatBytes(groupQuota.used) }}</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-2">
                    <div :class="getQuotaClass(groupQuota.percent)" class="h-2 rounded-full" :style="{ width: Math.min(groupQuota.percent, 100) + '%' }"></div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 共享文件夹权限 -->
          <div class="bg-white rounded-lg shadow p-4">
            <div class="flex justify-between items-center mb-4">
              <h3 class="font-semibold text-gray-900">共享文件夹权限</h3>
              <button @click="showFolderPermModal = true" class="btn-primary text-sm">添加权限</button>
            </div>
            <div class="overflow-x-auto">
              <table class="w-full text-sm">
                <thead class="bg-gray-50 border-b">
                  <tr>
                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">文件夹</th>
                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">用户/组</th>
                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">权限</th>
                    <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">操作</th>
                  </tr>
                </thead>
                <tbody class="divide-y">
                  <tr v-for="perm in folderPermissions" :key="perm.id" class="hover:bg-gray-50">
                    <td class="px-4 py-3 font-mono text-gray-900">{{ perm.folder }}</td>
                    <td class="px-4 py-3 text-gray-600">{{ perm.target }}</td>
                    <td class="px-4 py-3">
                      <span :class="getPermClass(perm.permission)" class="px-2 py-1 text-xs rounded-full font-medium">{{ getPermLabel(perm.permission) }}</span>
                    </td>
                    <td class="px-4 py-3 text-right">
                      <button @click="editFolderPerm(perm)" class="text-sm text-primary-600 hover:text-primary-700 mr-2">编辑</button>
                      <button @click="deleteFolderPerm(perm)" class="text-sm text-red-600 hover:text-red-700">删除</button>
                    </td>
                  </tr>
                </tbody>
              </table>
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

const tabs = [{ id: 'users', name: '用户列表' }, { id: 'groups', name: '用户组' }, { id: 'roles', name: '角色管理' }, { id: 'permissions', name: '权限概览' }, { id: 'quotas', name: '配额管理' }, { id: 'policy', name: '密码策略' }]
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

// 角色管理
const showRoleModal = ref(false)
const customRoles = ref<any[]>([])
const permissionModules = ref([
  { id: 'dashboard', name: '仪表板', user: true, guest: true },
  { id: 'files', name: '文件管理', user: true, guest: true },
  { id: 'storage', name: '存储管理', user: true, guest: false },
  { id: 'downloads', name: '下载管理', user: true, guest: false },
  { id: 'printers', name: '打印服务', user: true, guest: true },
  { id: 'users', name: '用户管理', user: false, guest: false },
  { id: 'settings', name: '系统设置', user: false, guest: false },
  { id: 'logs', name: '系统日志', user: true, guest: false },
  { id: 'apps', name: '应用管理', user: false, guest: false }
])

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
const executeDelete = async () => { if (!deleteTarget.value) return; const deletedId = deleteTarget.value.id; deleting.value = true; try { await api.users.delete(deletedId); showToast('success', '用户已删除'); deleteTarget.value = null; selectedUsers.value = selectedUsers.value.filter(id => id !== deletedId); loadUsers() } catch (e) { showToast('error', '删除失败') } finally { deleting.value = false } }

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

// 角色管理函数
const editRole = (role: any) => {
  showToast('info', '角色编辑功能开发中')
}

const deleteRole = async (role: any) => {
  if (!confirm(`确定删除角色 "${role.name}" 吗？`)) return
  customRoles.value = customRoles.value.filter(r => r.id !== role.id)
  showToast('success', '角色已删除')
}

// 配额管理
const showQuotaModal = ref(false)
const showFolderPermModal = ref(false)
const userQuotas = ref([
  { id: 1, username: 'admin', role: 'admin', limit: 0, used: 53687091200, percent: 0 },
  { id: 2, username: 'user1', role: 'user', limit: 107374182400, used: 53687091200, percent: 50 },
  { id: 3, username: 'user2', role: 'user', limit: 53687091200, used: 48318382080, percent: 90 },
  { id: 4, username: 'guest1', role: 'guest', limit: 10737418240, used: 2147483648, percent: 20 }
])
const groupQuotas = ref([
  { id: 1, name: 'admin', members: 1, limit: 0, used: 53687091200, percent: 0 },
  { id: 2, name: 'users', members: 5, limit: 536870912000, used: 214748364800, percent: 40 },
  { id: 3, name: 'developers', members: 3, limit: 107374182400, used: 85899345920, percent: 80 }
])
const folderPermissions = ref([
  { id: 1, folder: '/shared/documents', target: 'users (组)', permission: 'rw' },
  { id: 2, folder: '/shared/media', target: 'everyone', permission: 'ro' },
  { id: 3, folder: '/shared/backups', target: 'admin (用户)', permission: 'rw' },
  { id: 4, folder: '/shared/private', target: 'developers (组)', permission: 'deny' }
])

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

const getQuotaClass = (percent: number) => {
  if (percent >= 95) return 'bg-red-500'
  if (percent >= 80) return 'bg-yellow-500'
  return 'bg-green-500'
}

const getQuotaStatusClass = (percent: number) => {
  if (percent >= 95) return 'bg-red-100 text-red-700'
  if (percent >= 80) return 'bg-yellow-100 text-yellow-700'
  return 'bg-green-100 text-green-700'
}

const getQuotaStatusLabel = (percent: number) => {
  if (percent >= 95) return '即将满'
  if (percent >= 80) return '使用率高'
  return '正常'
}

const getPermClass = (perm: string) => {
  switch (perm) {
    case 'rw': return 'bg-green-100 text-green-700'
    case 'ro': return 'bg-blue-100 text-blue-700'
    case 'deny': return 'bg-red-100 text-red-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

const getPermLabel = (perm: string) => {
  switch (perm) {
    case 'rw': return '读写'
    case 'ro': return '只读'
    case 'deny': return '拒绝'
    default: return perm
  }
}

const editQuota = (quota: any) => {
  const newLimit = prompt(`设置 ${quota.username} 的配额限制 (GB, 0 表示无限制):`, quota.limit === 0 ? '0' : Math.round(quota.limit / 1073741824).toString())
  if (newLimit !== null) {
    quota.limit = parseInt(newLimit) * 1073741824
    showToast('success', '配额已更新')
  }
}

const editFolderPerm = (perm: any) => {
  showToast('info', '权限编辑功能开发中')
}

const deleteFolderPerm = (perm: any) => {
  if (!confirm(`确定删除此权限吗？`)) return
  folderPermissions.value = folderPermissions.value.filter(p => p.id !== perm.id)
  showToast('success', '权限已删除')
}

onMounted(() => { loadUsers(); loadGroups() })
</script>