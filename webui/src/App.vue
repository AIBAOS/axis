<template>
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900 transition-colors duration-200">
    <!-- OPT-3: 离线状态提示横幅 -->
    <div
      v-if="isOffline || isReconnecting"
      class="fixed top-0 left-0 right-0 z-50 px-4 py-2 text-center text-white text-sm font-medium"
      :class="{
        'bg-red-600': isOffline && !isReconnecting,
        'bg-yellow-500 animate-pulse': isReconnecting
      }"
    >
      <span v-if="isReconnecting" class="flex items-center justify-center gap-2">
        <svg class="animate-spin h-4 w-4" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
        </svg>
        正在重新连接... ({{ reconnectAttempts }}/{{ maxReconnectAttempts }})
      </span>
      <span v-else class="flex items-center justify-center gap-2">
        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
        </svg>
        网络连接已断开
        <button
          @click="handleManualReconnect"
          class="ml-2 px-2 py-0.5 bg-white text-red-600 rounded text-xs hover:bg-gray-100"
        >
          重试连接
        </button>
      </span>
    </div>

    <!-- 网络恢复成功提示（短暂显示） -->
    <Transition name="slide-down">
      <div
        v-if="showReconnectedBanner"
        class="fixed top-0 left-0 right-0 z-50 bg-green-500 text-white text-center py-2 text-sm font-medium"
      >
        ✓ 网络已恢复连接
      </div>
    </Transition>

    <nav class="bg-primary-700 dark:bg-gray-800 text-white shadow-lg">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <h1 class="text-xl font-bold">Axis NAS 管理面板</h1>
            </div>
          </div>
          <div class="flex items-center space-x-4">
            <!-- 网络状态指示器 -->
            <div class="flex items-center gap-1.5" :title="networkStatusText">
              <span
                class="w-2 h-2 rounded-full"
                :class="{
                  'bg-green-400': isOnline && !isReconnecting,
                  'bg-yellow-400 animate-pulse': isReconnecting,
                  'bg-red-400': isOffline && !isReconnecting
                }"
              ></span>
              <span class="text-xs text-primary-200 dark:text-gray-400">
                {{ networkStatusText }}
              </span>
            </div>
            
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
import { onMounted, onUnmounted, ref, computed } from 'vue'
import { useTheme } from '@/composables/useTheme'
import { useNetwork } from '@/composables/useNetwork'

const { isDark, toggleTheme, initTheme } = useTheme()
const {
  isOnline,
  isOffline,
  isReconnecting,
  reconnectAttempts,
  maxReconnectAttempts,
  manualReconnect,
} = useNetwork()

// 网络恢复提示
const showReconnectedBanner = ref(false)
let reconnectedBannerTimer: ReturnType<typeof setTimeout> | null = null

// 网络状态文本
const networkStatusText = computed(() => {
  if (isReconnecting.value) return '重连中...'
  if (isOffline.value) return '离线'
  return '在线'
})

// 手动重连
async function handleManualReconnect() {
  await manualReconnect()
}

// 监听网络重连事件
function handleNetworkReconnected() {
  showReconnectedBanner.value = true
  if (reconnectedBannerTimer) clearTimeout(reconnectedBannerTimer)
  reconnectedBannerTimer = setTimeout(() => {
    showReconnectedBanner.value = false
  }, 3000)
}

onMounted(() => {
  initTheme()
  window.addEventListener('network-reconnected', handleNetworkReconnected)
})

onUnmounted(() => {
  window.removeEventListener('network-reconnected', handleNetworkReconnected)
  if (reconnectedBannerTimer) clearTimeout(reconnectedBannerTimer)
})
</script>

<style scoped>
/* 网络状态横幅动画 */
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.3s ease;
}

.slide-down-enter-from,
.slide-down-leave-to {
  transform: translateY(-100%);
  opacity: 0;
}
</style>