<template>
  <tr class="hover:bg-gray-50 transition-colors">
    <!-- 用户信息 -->
    <td class="px-6 py-4 whitespace-nowrap">
      <div class="flex items-center">
        <div class="flex-shrink-0 h-10 w-10">
          <div :class="avatarBgClass" class="h-10 w-10 rounded-full flex items-center justify-center">
            <span :class="avatarTextClass" class="font-medium">{{ userInitial }}</span>
          </div>
        </div>
        <div class="ml-4">
          <div class="text-sm font-medium text-gray-900">{{ user.username }}</div>
          <div class="text-sm text-gray-500">{{ user.email }}</div>
        </div>
      </div>
    </td>

    <!-- 角色 -->
    <td class="px-6 py-4 whitespace-nowrap">
      <span :class="roleClass" class="px-2.5 py-1 text-xs font-medium rounded-full">
        {{ roleText }}
      </span>
    </td>

    <!-- 状态 -->
    <td class="px-6 py-4 whitespace-nowrap">
      <div class="flex items-center space-x-2">
        <span class="w-2 h-2 rounded-full" :class="statusDotClass"></span>
        <span :class="statusClass" class="text-sm">
          {{ statusText }}
        </span>
      </div>
    </td>

    <!-- 创建时间 -->
    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
      {{ formatDate(user.created_at) }}
    </td>

    <!-- 最后登录 -->
    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
      {{ formatLastLogin(user.last_login_at) }}
    </td>

    <!-- 操作 -->
    <td class="px-6 py-4 whitespace-nowrap text-right text-sm">
      <div class="flex justify-end items-center space-x-3">
        <!-- 编辑 -->
        <button @click="$emit('edit', user)" class="text-primary-600 hover:text-primary-800 font-medium">
          编辑
        </button>
        <!-- 状态切换 -->
        <button 
          @click="$emit('toggle-status', user)" 
          :class="user.status === 'disabled' ? 'text-green-600 hover:text-green-800' : 'text-yellow-600 hover:text-yellow-800'"
          class="font-medium"
        >
          {{ user.status === 'disabled' ? '启用' : '禁用' }}
        </button>
        <!-- 重置密码 -->
        <button @click="$emit('reset-password', user)" class="text-blue-600 hover:text-blue-800 font-medium">
          重置密码
        </button>
        <!-- 删除 -->
        <button @click="$emit('delete', user)" class="text-red-600 hover:text-red-800 font-medium">
          删除
        </button>
      </div>
    </td>
  </tr>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  user: {
    id: string | number
    username: string
    email: string
    role: string
    status: string
    created_at?: string | number
    last_login_at?: string | number
  }
}>()

defineEmits<{
  edit: [user: any]
  'toggle-status': [user: any]
  'reset-password': [user: any]
  delete: [user: any]
}>()

// 用户首字母
const userInitial = computed(() => {
  return props.user.username?.charAt(0).toUpperCase() || '?'
})

// 头像背景色
const avatarBgClass = computed(() => {
  switch (props.user.role) {
    case 'admin': return 'bg-red-100'
    case 'user': return 'bg-blue-100'
    case 'guest': return 'bg-gray-100'
    default: return 'bg-primary-100'
  }
})

// 头像文字颜色
const avatarTextClass = computed(() => {
  switch (props.user.role) {
    case 'admin': return 'text-red-600'
    case 'user': return 'text-blue-600'
    case 'guest': return 'text-gray-600'
    default: return 'text-primary-600'
  }
})

// 角色样式
const roleClass = computed(() => {
  switch (props.user.role) {
    case 'admin': return 'bg-red-100 text-red-700'
    case 'user': return 'bg-blue-100 text-blue-700'
    case 'guest': return 'bg-gray-100 text-gray-700'
    default: return 'bg-gray-100 text-gray-700'
  }
})

// 角色文本
const roleText = computed(() => {
  switch (props.user.role) {
    case 'admin': return '管理员'
    case 'user': return '普通用户'
    case 'guest': return '访客'
    default: return props.user.role
  }
})

// 状态点样式
const statusDotClass = computed(() => {
  switch (props.user.status) {
    case 'active': return 'bg-green-500'
    case 'inactive': return 'bg-gray-400'
    case 'disabled': return 'bg-red-500'
    default: return 'bg-gray-400'
  }
})

// 状态样式
const statusClass = computed(() => {
  switch (props.user.status) {
    case 'active': return 'text-green-700'
    case 'inactive': return 'text-gray-600'
    case 'disabled': return 'text-red-700'
    default: return 'text-gray-600'
  }
})

// 状态文本
const statusText = computed(() => {
  switch (props.user.status) {
    case 'active': return '活跃'
    case 'inactive': return '离线'
    case 'disabled': return '禁用'
    default: return props.user.status
  }
})

// 格式化日期
const formatDate = (timestamp: string | number | undefined): string => {
  if (!timestamp) return '-'
  const date = typeof timestamp === 'number' 
    ? new Date(timestamp * 1000) 
    : new Date(timestamp)
  return date.toLocaleDateString('zh-CN')
}

// 格式化最后登录时间
const formatLastLogin = (timestamp: string | number | undefined): string => {
  if (!timestamp) return '从未登录'
  const date = typeof timestamp === 'number' 
    ? new Date(timestamp * 1000) 
    : new Date(timestamp)
  return date.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>