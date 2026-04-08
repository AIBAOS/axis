<template>
  <div class="text-center py-8 sm:py-12 px-4">
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 dark:text-white mb-3 sm:mb-4">
      Axis NAS 管理面板
    </h1>
    <p class="text-base sm:text-lg text-gray-600 dark:text-gray-300 mb-6 sm:mb-8">
      企业级网络存储管理系统
    </p>
    
    <!-- 版本信息 -->
    <div class="inline-block bg-indigo-100 dark:bg-indigo-900 rounded-lg px-4 sm:px-6 py-2 sm:py-3">
      <p class="text-xs sm:text-sm text-indigo-800 dark:text-indigo-200">
        <span class="font-semibold">版本:</span> v1.0.0
      </p>
      <p class="text-xs sm:text-sm text-indigo-800 dark:text-indigo-200 mt-1">
        <span class="font-semibold">API 状态:</span> <span class="text-green-600 dark:text-green-400">● 正常运行</span>
      </p>
      <p class="text-xs sm:text-sm text-indigo-800 dark:text-indigo-200 mt-1">
        <span class="font-semibold">Build:</span> 2026.03.28
      </p>
    </div>

    <!-- 快速操作按钮 -->
    <div class="mt-12 grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl mx-auto">
      <router-link 
        to="/dashboard" 
        class="block p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm hover:shadow-md transition-shadow border border-gray-200 dark:border-gray-700"
      >
        <div class="text-indigo-600 dark:text-indigo-400 text-2xl mb-2">📊</div>
        <h3 class="font-semibold text-gray-900 dark:text-white">控制面板</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">查看系统状态和统计信息</p>
      </router-link>

      <router-link 
        to="/login" 
        class="block p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm hover:shadow-md transition-shadow border border-gray-200 dark:border-gray-700"
      >
        <div class="text-indigo-600 dark:text-indigo-400 text-2xl mb-2">🔐</div>
        <h3 class="font-semibold text-gray-900 dark:text-white">用户登录</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">访问完整管理功能</p>
      </router-link>

      <a 
        href="https://github.com/AIBAOS/axis" 
        target="_blank"
        class="block p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm hover:shadow-md transition-shadow border border-gray-200 dark:border-gray-700"
      >
        <div class="text-indigo-600 dark:text-indigo-400 text-2xl mb-2">📚</div>
        <h3 class="font-semibold text-gray-900 dark:text-white">API 文档</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">查看 REST API 文档</p>
      </a>
    </div>

    <!-- 功能特性 -->
    <div class="mt-12 sm:mt-16 text-left max-w-4xl mx-auto px-4 sm:px-0">
      <h2 class="text-xl sm:text-2xl font-bold text-gray-900 dark:text-white mb-4 sm:mb-6 text-center">核心功能</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 sm:gap-4">
        <div class="flex items-start space-x-2 sm:space-x-3">
          <span class="text-green-500 text-sm sm:text-base">✓</span>
          <div>
            <h4 class="font-medium text-sm sm:text-base text-gray-900 dark:text-white">文件管理</h4>
            <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">上传、下载、分享、权限管理</p>
          </div>
        </div>
        <div class="flex items-start space-x-2 sm:space-x-3">
          <span class="text-green-500 text-sm sm:text-base">✓</span>
          <div>
            <h4 class="font-medium text-sm sm:text-base text-gray-900 dark:text-white">存储管理</h4>
            <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">磁盘、存储池、卷管理</p>
          </div>
        </div>
        <div class="flex items-start space-x-2 sm:space-x-3">
          <span class="text-green-500 text-sm sm:text-base">✓</span>
          <div>
            <h4 class="font-medium text-sm sm:text-base text-gray-900 dark:text-white">用户管理</h4>
            <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">用户、角色、权限配置</p>
          </div>
        </div>
        <div class="flex items-start space-x-2 sm:space-x-3">
          <span class="text-green-500 text-sm sm:text-base">✓</span>
          <div>
            <h4 class="font-medium text-sm sm:text-base text-gray-900 dark:text-white">备份管理</h4>
            <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">全量/增量备份、计划任务</p>
          </div>
        </div>
        <div class="flex items-start space-x-2 sm:space-x-3">
          <span class="text-green-500 text-sm sm:text-base">✓</span>
          <div>
            <h4 class="font-medium text-sm sm:text-base text-gray-900 dark:text-white">网络共享</h4>
            <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">SMB/NFS/WebDAV/FTP 支持</p>
          </div>
        </div>
        <div class="flex items-start space-x-2 sm:space-x-3">
          <span class="text-green-500 text-sm sm:text-base">✓</span>
          <div>
            <h4 class="font-medium text-sm sm:text-base text-gray-900 dark:text-white">系统监控</h4>
            <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">资源使用、日志、告警</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
// 首页组件
</script>
