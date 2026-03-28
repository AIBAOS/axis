<template>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4">
      <!-- 标题栏 -->
      <div class="flex justify-between items-center px-6 py-4 border-b">
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
          <label class="block text-sm font-medium text-gray-700 mb-1">用户名</label>
          <input
            v-model="formData.username"
            type="text"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="请输入用户名"
          />
        </div>

        <!-- 邮箱 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">邮箱</label>
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
          <label class="block text-sm font-medium text-gray-700 mb-1">密码</label>
          <input
            v-model="formData.password"
            type="password"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="请输入密码"
            minlength="6"
          />
        </div>

        <!-- 角色 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">角色</label>
          <select
            v-model="formData.role"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          >
            <option value="user">普通用户</option>
            <option value="admin">管理员</option>
            <option value="guest">访客</option>
          </select>
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
      </form>

      <!-- 按钮栏 -->
      <div class="flex justify-end space-x-3 px-6 py-4 border-t bg-gray-50 rounded-b-lg">
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
          class="btn-primary"
        >
          {{ mode === 'create' ? '创建' : '保存' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'

const props = defineProps<{
  mode: 'create' | 'edit'
  user?: any
}>()

const emit = defineEmits<{
  close: []
  save: [data: any]
}>()

// 表单数据
const formData = ref({
  username: '',
  email: '',
  password: '',
  role: 'user',
  status: 'active'
})

// 监听用户数据变化（编辑模式）
watch(() => props.user, (newUser) => {
  if (newUser && props.mode === 'edit') {
    formData.value = {
      username: newUser.username || '',
      email: newUser.email || '',
      password: '',
      role: newUser.role || 'user',
      status: newUser.status || 'active'
    }
  }
}, { immediate: true })

// 提交表单
const handleSubmit = () => {
  const data: any = {
    username: formData.value.username,
    email: formData.value.email,
    role: formData.value.role,
    status: formData.value.status
  }

  // 仅新建时包含密码
  if (props.mode === 'create') {
    data.password = formData.value.password
  }

  emit('save', data)
}
</script>
