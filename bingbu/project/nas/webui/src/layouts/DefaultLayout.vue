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
          <div class="flex items-center space-x-4">
            <router-link to="/dashboard" class="text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400">
              控制面板
            </router-link>
            <router-link to="/login" class="text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400">
              登录
            </router-link>
          </div>
        </div>
      </div>
    </nav>

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
// 默认布局组件
</script>
