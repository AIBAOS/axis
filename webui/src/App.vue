<template>
  <div class="min-h-screen bg-gray-100">
    <nav class="bg-primary-700 text-white shadow-lg">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <h1 class="text-xl font-bold">Axis NAS 管理面板</h1>
            </div>
          </div>
          <div class="flex items-center space-x-4">
            <span class="text-sm text-primary-200">v0.1.0</span>
          </div>
        </div>
      </div>

      <!-- Navigation Links -->
      <div class="bg-primary-800">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div class="flex space-x-4">
            <router-link
              to="/"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600"
              active-class="bg-primary-600"
            >
              🏠 首页
            </router-link>
            <router-link
              to="/backups"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600"
              active-class="bg-primary-600"
            >
              📦 备份管理
            </router-link>
            <router-link
              to="/files"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600"
              active-class="bg-primary-600"
            >
              📁 文件管理
            </router-link>
            <router-link
              to="/users"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600"
              active-class="bg-primary-600"
            >
              👥 用户管理
            </router-link>
            <router-link
              to="/system"
              class="px-3 py-2 rounded-md text-sm font-medium hover:bg-primary-600"
              active-class="bg-primary-600"
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

    <footer class="bg-gray-800 text-gray-400 py-4 mt-8">
      <div class="max-w-7xl mx-auto px-4 text-center text-sm">
        <p>Axis NAS Management Panel &copy; 2026</p>
        <p class="mt-1">Powered by Axis Backend API</p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
// App root component
</script>

<style scoped>
/* App specific styles */
</style>
