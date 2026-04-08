<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <!-- Logo 和标题 -->
      <div class="text-center">
        <h1 class="text-3xl sm:text-4xl font-bold text-indigo-600 dark:text-indigo-400">Axis NAS</h1>
        <h2 class="mt-2 text-xl sm:text-2xl font-semibold text-gray-900 dark:text-white">
          用户登录
        </h2>
        <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
          企业级网络存储管理系统
        </p>
      </div>

      <!-- 登录表单 -->
      <form class="mt-6 sm:mt-8 space-y-4 sm:space-y-6" @submit.prevent="handleLogin">
        <!-- 错误提示 -->
        <div v-if="errorMessage" class="rounded-md bg-red-50 dark:bg-red-900/20 p-3 sm:p-4">
          <div class="flex">
            <div class="ml-3">
              <h3 class="text-xs sm:text-sm font-medium text-red-800 dark:text-red-200">
                {{ errorMessage }}
              </h3>
            </div>
          </div>
        </div>

        <div class="rounded-md shadow-sm space-y-3 sm:space-y-4">
          <!-- 用户名 -->
          <div>
            <label for="username" class="block text-xs sm:text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              用户名
            </label>
            <input
              id="username"
              v-model="username"
              name="username"
              type="text"
              autocomplete="username"
              required
              class="appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 text-gray-900 dark:text-white dark:bg-gray-800 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 text-sm"
              placeholder="请输入用户名"
            />
          </div>

          <!-- 密码 -->
          <div>
            <label for="password" class="block text-xs sm:text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              密码
            </label>
            <input
              id="password"
              v-model="password"
              name="password"
              type="password"
              autocomplete="current-password"
              required
              class="appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 text-gray-900 dark:text-white dark:bg-gray-800 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 text-sm"
              placeholder="请输入密码"
            />
          </div>
        </div>

        <!-- 记住我 -->
        <div class="flex items-center">
          <input
            id="remember-me"
            v-model="rememberMe"
            name="remember-me"
            type="checkbox"
            class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
          />
          <label for="remember-me" class="ml-2 block text-xs sm:text-sm text-gray-900 dark:text-gray-300">
            记住我
          </label>
        </div>

        <!-- 登录按钮 -->
        <div>
          <button
            type="submit"
            :disabled="loading"
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="loading" class="absolute left-0 inset-y-0 flex items-center pl-3">
              <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
            </span>
            {{ loading ? '登录中...' : '登录' }}
          </button>
        </div>
      </form>

      <!-- 页脚信息 -->
      <div class="text-center mt-4">
        <p class="text-xs text-gray-500 dark:text-gray-400">
          版本 v1.0.0 | © 2026 Axis NAS
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'

const router = useRouter()

const username = ref('')
const password = ref('')
const rememberMe = ref(false)
const loading = ref(false)
const errorMessage = ref('')

const handleLogin = async () => {
  if (!username.value || !password.value) {
    errorMessage.value = '请输入用户名和密码'
    return
  }

  loading.value = true
  errorMessage.value = ''

  try {
    const response = await apiClient.post('/auth/login', {
      username: username.value,
      password: password.value
    })

    if (response.data.success) {
      // 保存 JWT Token
      const token = response.data.data.token
      localStorage.setItem('jwt_token', token)
      
      if (rememberMe.value) {
        localStorage.setItem('remembered_username', username.value)
      } else {
        localStorage.removeItem('remembered_username')
      }

      // 跳转到仪表盘
      router.push('/dashboard')
    }
  } catch (error) {
    console.error('Login failed:', error)
    
    if (error.response) {
      switch (error.response.status) {
        case 401:
          errorMessage.value = '用户名或密码错误'
          break
        case 403:
          errorMessage.value = '账户已被禁用'
          break
        case 500:
          errorMessage.value = '服务器错误，请稍后重试'
          break
        default:
          errorMessage.value = error.response.data?.message || '登录失败，请重试'
      }
    } else if (error.request) {
      errorMessage.value = '无法连接到服务器，请检查网络连接'
    } else {
      errorMessage.value = '登录失败，请重试'
    }
  } finally {
    loading.value = false
  }
}

// 页面加载时检查是否有记住的用户名
if (localStorage.getItem('remembered_username')) {
  username.value = localStorage.getItem('remembered_username')
  rememberMe.value = true
}
</script>
