<template>
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900 transition-colors duration-200">
    <nav class="bg-primary-700 dark:bg-gray-800 text-white shadow-lg">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <h1 class="text-xl font-bold">Axis NAS 管理面板</h1>
            </div>
          </div>
          <div class="flex items-center space-x-4">
            <!-- 深色模式切换按钮 -->
            <button
              @click="toggleTheme"
              class="p-2 rounded-lg hover:bg-primary-600 dark:hover:bg-gray-700 transition-colors"
              :title="isDark ? '切换到浅色模式' : '切换到深色模式'"
            >
              <svg v-if="isDark" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd" />
              </svg>
              <svg v-else class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
              </svg>
            </button>
            <span class="text-sm text-primary-200 dark:text-gray-400">v0.1.0</span>
          </div>
        </div>
      </div>

      <!-- Navigation Links -->
      <div class="bg-primary-800 dark:bg-gray-700">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div class="flex space-x-4">
            <router-link
              to="/"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600 dark:hover:bg-gray-600"
              active-class="bg-primary-600 dark:bg-gray-600"
            >
              🏠 首页
            </router-link>
            <router-link
              to="/files"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600 dark:hover:bg-gray-600"
              active-class="bg-primary-600 dark:bg-gray-600"
            >
              📁 文件管理
            </router-link>
            <router-link
              to="/storage"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600 dark:hover:bg-gray-600"
              active-class="bg-primary-600 dark:bg-gray-600"
            >
              💾 存储管理
            </router-link>
            <router-link
              to="/backups"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600 dark:hover:bg-gray-600"
              active-class="bg-primary-600 dark:bg-gray-600"
            >
              📦 备份管理
            </router-link>
            <router-link
              to="/users"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600 dark:hover:bg-gray-600"
              active-class="bg-primary-600 dark:bg-gray-600"
            >
              👥 用户管理
            </router-link>
            <router-link
              to="/system"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600 dark:hover:bg-gray-600"
              active-class="bg-primary-600 dark:bg-gray-600"
            >
              ⚙️ 系统设置
            </router-link>
          </div>
        </div>
      </div>
    </nav>

    <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
      <router-view />
    </main>

    <footer class="bg-gray-800 dark:bg-gray-950 text-gray-400 py-4 mt-8">
      <div class="max-w-7xl mx-auto px-4 text-center text-sm">
        <p>Axis NAS Management Panel &copy; 2026</p>
        <p class="mt-1">Powered by Axis Backend API</p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useTheme } from '@/composables/useTheme'

const { isDark, toggleTheme, initTheme } = useTheme()

onMounted(() => {
  initTheme()
})
</script>

<style scoped>
/* App specific styles */
</style>