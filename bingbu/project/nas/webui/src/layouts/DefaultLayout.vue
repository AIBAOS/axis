<template>
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <!-- 顶部导航栏 -->
    <nav class="bg-white dark:bg-gray-800 shadow-sm">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <router-link to="/" class="text-xl font-bold text-indigo-600 dark:text-indigo-400">
              Axis NAS
            </router-link>
          </div>
          <div class="hidden md:flex items-center space-x-6">
            <router-link to="/dashboard" class="text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400">
              控制面板
            </router-link>
            <router-link to="/files" class="text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400">
              文件管理
            </router-link>
            <router-link to="/storage" class="text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400">
              存储管理
            </router-link>
            <button @click="logout" class="text-gray-600 dark:text-gray-300 hover:text-red-600 dark:hover:text-red-400">
              退出
            </button>
          </div>
          <!-- 移动端菜单按钮 -->
          <div class="md:hidden flex items-center">
            <button @click="showMobileMenu = !showMobileMenu" class="text-gray-600 dark:text-gray-300">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </nav>

    <!-- 移动端菜单 -->
    <div v-if="showMobileMenu" class="md:hidden bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700">
      <div class="px-4 py-3 space-y-3">
        <router-link to="/dashboard" class="block text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400" @click="showMobileMenu = false">
          控制面板
        </router-link>
        <router-link to="/files" class="block text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400" @click="showMobileMenu = false">
          文件管理
        </router-link>
        <router-link to="/storage" class="block text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400" @click="showMobileMenu = false">
          存储管理
        </router-link>
        <button @click="logout; showMobileMenu = false" class="block text-left text-gray-600 dark:text-gray-300 hover:text-red-600 dark:hover:text-red-400">
          退出
        </button>
      </div>
    </div>

    <!-- 主内容区 -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <slot />
    </main>

    <!-- 页脚 -->
    <footer class="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 mt-auto">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <p class="text-center text-gray-500 dark:text-gray-400 text-sm">
          © 2026 Axis NAS Management Panel. All rights reserved.
        </p>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const showMobileMenu = ref(false)

const logout = () => {
  localStorage.removeItem('jwt_token')
  router.push('/login')
}
</script>
