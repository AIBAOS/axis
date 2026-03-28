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
            <h1 class="text-xl font-bold text-gray-900">Axis NAS 管理面板</h1>
          </div>
          <nav class="flex space-x-4">
            <router-link to="/" class="text-gray-600 hover:text-gray-900 font-medium">首页</router-link>
            <router-link to="/about" class="text-primary-600 hover:text-primary-700 font-medium">关于</router-link>
          </nav>
        </div>
      </div>
    </header>

    <!-- 主要内容 -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <div class="card">
        <h2 class="text-2xl font-bold text-gray-900 mb-6">关于 Axis NAS</h2>
        
        <div class="prose max-w-none">
          <p class="text-gray-600 mb-4">
            Axis NAS 是一个功能强大的网络存储解决方案，提供文件管理、用户管理、共享管理、容器管理、备份管理等核心功能。
          </p>

          <h3 class="text-lg font-semibold text-gray-900 mt-6 mb-3">技术栈</h3>
          <ul class="list-disc list-inside text-gray-600 space-y-2">
            <li><strong>后端:</strong> Rust + Axum + Tokio + SQLite</li>
            <li><strong>前端:</strong> Vue 3 + TypeScript + Vite + TailwindCSS</li>
            <li><strong>状态管理:</strong> Pinia</li>
            <li><strong>路由:</strong> Vue Router</li>
            <li><strong>HTTP 客户端:</strong> Axios</li>
          </ul>

          <h3 class="text-lg font-semibold text-gray-900 mt-6 mb-3">核心功能</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="bg-gray-50 rounded-lg p-4">
              <h4 class="font-semibold text-gray-900 mb-2">📁 文件管理</h4>
              <p class="text-gray-600 text-sm">浏览、上传、下载、删除文件，支持文件搜索和批量操作</p>
            </div>
            <div class="bg-gray-50 rounded-lg p-4">
              <h4 class="font-semibold text-gray-900 mb-2">👥 用户管理</h4>
              <p class="text-gray-600 text-sm">用户 CRUD、角色权限管理、JWT 认证</p>
            </div>
            <div class="bg-gray-50 rounded-lg p-4">
              <h4 class="font-semibold text-gray-900 mb-2">📂 共享管理</h4>
              <p class="text-gray-600 text-sm">SMB/NFS/WebDAV/FTP 共享配置和管理</p>
            </div>
            <div class="bg-gray-50 rounded-lg p-4">
              <h4 class="font-semibold text-gray-900 mb-2">🐳 容器管理</h4>
              <p class="text-gray-600 text-sm">Docker 容器生命周期管理、日志查看</p>
            </div>
            <div class="bg-gray-50 rounded-lg p-4">
              <h4 class="font-semibold text-gray-900 mb-2">💾 备份管理</h4>
              <p class="text-gray-600 text-sm">备份任务创建、执行、恢复、归档</p>
            </div>
            <div class="bg-gray-50 rounded-lg p-4">
              <h4 class="font-semibold text-gray-900 mb-2">⚙️ 系统管理</h4>
              <p class="text-gray-600 text-sm">系统监控、日志查询、定时任务、电源管理</p>
            </div>
          </div>

          <h3 class="text-lg font-semibold text-gray-900 mt-6 mb-3">版本信息</h3>
          <div class="bg-primary-50 rounded-lg p-4">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <div class="text-sm text-primary-600 font-medium">前端版本</div>
                <div class="text-lg font-bold text-primary-700">{{ version }}</div>
              </div>
              <div>
                <div class="text-sm text-primary-600 font-medium">构建时间</div>
                <div class="text-lg font-bold text-primary-700">{{ buildTime }}</div>
              </div>
            </div>
          </div>
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
const buildTime = new Date().toLocaleString('zh-CN')
</script>
