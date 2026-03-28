<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        控制面板
      </h1>
      <div class="flex items-center space-x-2">
        <span class="text-sm text-gray-500 dark:text-gray-400">
          最后更新：{{ lastUpdateTime }}
        </span>
        <button
          @click="refreshData"
          class="p-2 text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          title="刷新数据"
        >
          <svg class="w-5 h-5" :class="{ 'animate-spin': loading }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
          </svg>
        </button>
      </div>
    </div>

    <!-- 系统状态卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <!-- CPU 使用率 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">CPU 使用率</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">
              {{ systemStats.cpu || '--' }}%
            </p>
          </div>
        </div>
      </div>

      <!-- 内存使用率 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-green-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">内存使用率</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">
              {{ systemStats.memory || '--' }}%
            </p>
          </div>
        </div>
      </div>

      <!-- 存储使用率 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-yellow-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">存储使用率</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">
              {{ systemStats.storage || '--' }}%
            </p>
          </div>
        </div>
      </div>

      <!-- 在线用户 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="flex items-center justify-center h-12 w-12 rounded-md bg-purple-500 text-white">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path>
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500 dark:text-gray-400">在线用户</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">
              {{ systemStats.onlineUsers || 0 }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- 系统信息 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 系统概览 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">系统信息</h3>
        <dl class="space-y-3">
          <div class="flex justify-between">
            <dt class="text-sm text-gray-500 dark:text-gray-400">系统名称</dt>
            <dd class="text-sm font-medium text-gray-900 dark:text-white">Axis NAS</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-sm text-gray-500 dark:text-gray-400">版本号</dt>
            <dd class="text-sm font-medium text-gray-900 dark:text-white">v1.0.0</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-sm text-gray-500 dark:text-gray-400">运行时间</dt>
            <dd class="text-sm font-medium text-gray-900 dark:text-white">
              {{ systemInfo.uptime || '--' }}
            </dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-sm text-gray-500 dark:text-gray-400">主机名</dt>
            <dd class="text-sm font-medium text-gray-900 dark:text-white">
              {{ systemInfo.hostname || '--' }}
            </dd>
          </div>
        </dl>
      </div>

      <!-- 快速操作 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">快速操作</h3>
        <div class="grid grid-cols-2 gap-3">
          <router-link
            to="/files"
            class="flex items-center justify-center px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
          >
            📁 文件管理
          </router-link>
          <router-link
            to="/storage"
            class="flex items-center justify-center px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
          >
            💾 存储管理
          </router-link>
          <router-link
            to="/users"
            class="flex items-center justify-center px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
          >
            👥 用户管理
          </router-link>
          <router-link
            to="/backups"
            class="flex items-center justify-center px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
          >
            🔄 备份管理
          </router-link>
        </div>
      </div>
    </div>

    <!-- 最近活动 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">最近活动</h3>
      <div class="text-center py-8 text-gray-500 dark:text-gray-400">
        <p>暂无活动记录</p>
        <p class="text-sm mt-1">功能开发中...</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const loading = ref(false)
const lastUpdateTime = ref(new Date().toLocaleTimeString())

const systemStats = ref({
  cpu: null,
  memory: null,
  storage: null,
  onlineUsers: 0
})

const systemInfo = ref({
  uptime: null,
  hostname: null
})

const refreshData = async () => {
  loading.value = true
  
  // 模拟数据加载（实际应调用 API）
  await new Promise(resolve => setTimeout(resolve, 500))
  
  // 临时模拟数据
  systemStats.value = {
    cpu: 23,
    memory: 45,
    storage: 67,
    onlineUsers: 3
  }
  
  systemInfo.value = {
    uptime: '15 天 7 小时 23 分钟',
    hostname: 'axis-nas-01'
  }
  
  lastUpdateTime.value = new Date().toLocaleTimeString()
  loading.value = false
}

// 检查登录状态
onMounted(() => {
  const token = localStorage.getItem('jwt_token')
  if (!token) {
    router.push('/login')
    return
  }
  
  refreshData()
  
  // 定时刷新数据（每 30 秒）
  setInterval(refreshData, 30000)
})
</script>
