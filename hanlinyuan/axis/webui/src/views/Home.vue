<template>
  <div class="px-4 py-6 sm:px-0">
    <div class="bg-white overflow-hidden shadow rounded-lg">
      <div class="px-4 py-5 sm:p-6">
        <h2 class="text-2xl font-bold text-gray-900 mb-4">欢迎使用 Axis NAS 管理面板</h2>
        
        <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
          <div class="bg-primary-50 p-4 rounded-lg">
            <h3 class="text-lg font-medium text-primary-900">系统状态</h3>
            <p class="mt-2 text-sm text-primary-700">运行正常</p>
          </div>
          
          <div class="bg-green-50 p-4 rounded-lg">
            <h3 class="text-lg font-medium text-green-900">API 版本</h3>
            <p class="mt-2 text-sm text-green-700">v1.0.0</p>
          </div>
          
          <div class="bg-yellow-50 p-4 rounded-lg">
            <h3 class="text-lg font-medium text-yellow-900">WebUI 版本</h3>
            <p class="mt-2 text-sm text-yellow-700">v0.1.0</p>
          </div>
        </div>

        <div class="mt-8">
          <h3 class="text-lg font-medium text-gray-900 mb-4">功能模块</h3>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
              <h4 class="font-medium text-gray-900">📦 备份管理</h4>
              <p class="mt-1 text-sm text-gray-500">备份任务配置与管理</p>
            </div>
            
            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
              <h4 class="font-medium text-gray-900">📁 文件管理</h4>
              <p class="mt-1 text-sm text-gray-500">文件浏览与操作</p>
            </div>
            
            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
              <h4 class="font-medium text-gray-900">👥 用户管理</h4>
              <p class="mt-1 text-sm text-gray-500">用户与权限管理</p>
            </div>
            
            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
              <h4 class="font-medium text-gray-900">🔧 系统设置</h4>
              <p class="mt-1 text-sm text-gray-500">系统配置与监控</p>
            </div>
            
            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
              <h4 class="font-medium text-gray-900">📊 媒体管理</h4>
              <p class="mt-1 text-sm text-gray-500">媒体文件管理</p>
            </div>
            
            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
              <h4 class="font-medium text-gray-900">🔗 共享管理</h4>
              <p class="mt-1 text-sm text-gray-500">SMB/NFS/WebDAV 共享</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Home view component
</script>

<style scoped>
/* Home view specific styles */
</style>
