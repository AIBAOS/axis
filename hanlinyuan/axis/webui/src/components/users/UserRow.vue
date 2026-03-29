<template>
  <tr class="hover:bg-gray-50 transition-colors">
    <!-- 用户信息 -->
    <td class="px-6 py-4 whitespace-nowrap">
      <div class="flex items-center">
        <div class="flex-shrink-0 h-10 w-10">
          <div class="h-10 w-10 rounded-full bg-primary-100 flex items-center justify-center">
            <span class="text-primary-600 font-medium">{{ userInitial }}</span>
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
      <span
        :class="[
          'px-2 py-1 text-xs font-medium rounded-full',
          roleClass
        ]"
      >
        {{ roleText }}
      </span>
    </td>

    <!-- 状态 -->
    <td class="px-6 py-4 whitespace-nowrap">
      <span
        :class="[
          'px-2 py-1 text-xs font-medium rounded-full',
          statusClass
        ]"
      >
        {{ statusText }}
      </span>
    </td>

    <!-- 最后登录 -->
    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
      {{ formatLastLogin(user.last_login_at) }}
    </td>

    <!-- 操作 -->
    <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
      <button @click="$emit('edit', user)" class="text-primary-600 hover:text-primary-900 mr-4">
        编辑
      </button>
      <button @click="$emit('delete', user)" class="text-red-600 hover:text-red-900">
        删除
      </button>
    </td>
  </tr>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  user: {
    id: string
    username: string
    email: string
    role: string
    status: string
    last_login_at?: string
  }
}>()

defineEmits<{
  edit: [user: any]
  delete: [user: any]
}>()

// 用户首字母
const userInitial = computed(() => {
  return props.user.username.charAt(0).toUpperCase()
})

// 角色样式
const roleClass = computed(() => {
  switch (props.user.role) {
    case 'admin':
      return 'bg-red-100 text-red-800'
    case 'user':
      return 'bg-blue-100 text-blue-800'
    case 'guest':
      return 'bg-gray-100 text-gray-800'
    default:
      return 'bg-gray-100 text-gray-800'
  }
})

// 角色文本
const roleText = computed(() => {
  switch (props.user.role) {
    case 'admin':
      return '管理员'
    case 'user':
      return '普通用户'
    case 'guest':
      return '访客'
    default:
      return props.user.role
  }
})

// 状态样式
const statusClass = computed(() => {
  switch (props.user.status) {
    case 'active':
      return 'bg-green-100 text-green-800'
    case 'inactive':
      return 'bg-gray-100 text-gray-800'
    case 'disabled':
      return 'bg-red-100 text-red-800'
    default:
      return 'bg-gray-100 text-gray-800'
  }
})

// 状态文本
const statusText = computed(() => {
  switch (props.user.status) {
    case 'active':
      return '活跃'
    case 'inactive':
      return '离线'
    case 'disabled':
      return '禁用'
    default:
      return props.user.status
  }
})

// 格式化最后登录时间
const formatLastLogin = (dateString?: string): string => {
  if (!dateString) return '从未登录'
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>
