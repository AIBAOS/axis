<template>
  <div class="min-h-screen bg-gradient-to-br from-primary-50 to-primary-100">
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
            <h1 class="text-xl font-bold text-gray-900">Axis NAS 管理面板</h1>
          </div>
          <nav class="flex space-x-4">
            <router-link to="/" class="text-primary-600 hover:text-primary-700 font-medium">首页</router-link>
            <router-link to="/files" class="text-gray-600 hover:text-gray-900 font-medium">文件</router-link>
            <router-link to="/storage" class="text-gray-600 hover:text-gray-900 font-medium">存储</router-link>
            <router-link to="/users" class="text-gray-600 hover:text-gray-900 font-medium">用户</router-link>
            <router-link to="/about" class="text-gray-600 hover:text-gray-900 font-medium">关于</router-link>
          </nav>
        </div>
      </div>
    </header>

    <!-- 主要内容 -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <div class="card">
        <div class="text-center">
          <h2 class="text-3xl font-bold text-gray-900 mb-4">欢迎使用 Axis NAS</h2>
          <p class="text-lg text-gray-600 mb-8">强大的网络存储解决方案</p>
          
          <div class="bg-primary-50 rounded-lg p-6 inline-block">
            <div class="text-sm text-primary-600 font-medium mb-2">当前版本</div>
            <div class="text-2xl font-bold text-primary-700">{{ version }}</div>
          </div>
        </div>

        <!-- 快速入口 -->
        <div class="mt-12 grid grid-cols-1 md:grid-cols-3 gap-6">
          <router-link to="/files" class="card hover:shadow-lg transition-shadow cursor-pointer block">
            <div class="text-center">
              <div class="w-12 h-12 bg-primary-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                </svg>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">文件管理</h3>
              <p class="text-gray-600 text-sm">浏览和管理您的文件</p>
            </div>
          </router-link>

          <router-link to="/storage" class="card hover:shadow-lg transition-shadow cursor-pointer block">
            <div class="text-center">
              <div class="w-12 h-12 bg-primary-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
                </svg>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">存储管理</h3>
              <p class="text-gray-600 text-sm">管理存储卷、池和磁盘</p>
            </div>
          </router-link>

          <router-link to="/users" class="card hover:shadow-lg transition-shadow cursor-pointer block">
            <div class="text-center">
              <div class="w-12 h-12 bg-primary-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                </svg>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">用户管理</h3>
              <p class="text-gray-600 text-sm">管理用户和权限</p>
            </div>
          </router-link>
        </div>
      </div>
    </main>

    <!-- 页脚 -->
    <footer class="bg-white border-t mt-12">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        <div class="text-center text-gray-600 text-sm">
          <p>© 2026 Axis NAS. All rights reserved.</p>
          <p class="mt-2">Powered by Vue 3 + Vite + TailwindCSS</p>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
const version = import.meta.env.VITE_APP_VERSION || '0.1.0'
</script>
