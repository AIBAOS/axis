<template>
  <div class="min-h-screen bg-gray-50">
    <!-- 顶部导航栏 -->
    <header class="bg-white shadow-sm">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center py-4">
          <div class="flex items-center space-x-3">
            <div class="w-10 h-10 bg-primary-600 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
              </svg>
            </div>
            <h1 class="text-xl font-bold text-gray-900">Axis NAS</h1>
          </div>
          <nav class="flex items-center space-x-4">
            <router-link to="/" class="text-gray-600 hover:text-gray-900">首页</router-link>
            <router-link to="/files" class="text-gray-600 hover:text-gray-900">文件</router-link>
            <router-link to="/about" class="text-gray-600 hover:text-gray-900">关于</router-link>
            <span v-if="authStore.isAuthenticated" class="text-gray-600">|</span>
            <button
              v-if="authStore.isAuthenticated"
              @click="handleLogout"
              class="text-gray-600 hover:text-gray-900"
            >
              退出
            </button>
          </nav>
        </div>
      </div>
    </header>

    <!-- 主内容区 -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <slot />
    </main>

    <!-- 页脚 -->
    <footer class="bg-white border-t mt-auto">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <p class="text-center text-gray-600 text-sm">
          © 2026 Axis NAS. Version {{ version }}
        </p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
const version = import.meta.env.VITE_APP_VERSION || '0.1.0'

const handleLogout = () => {
  authStore.logout()
  window.location.reload()
}
</script>
