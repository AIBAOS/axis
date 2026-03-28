<template>
  <div class="px-4 py-6 sm:px-0">
    <!-- System Status Overview -->
    <div class="mb-6">
      <h2 class="text-2xl font-bold text-gray-900 mb-4">系统概览</h2>
      
      <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="p-5">
            <div class="flex items-center">
              <div class="flex-shrink-0">
                <svg class="h-6 w-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
              </div>
              <div class="ml-5 w-0 flex-1">
                <dl>
                  <dt class="text-sm font-medium text-gray-500 truncate">系统状态</dt>
                  <dd class="text-lg font-medium text-gray-900">运行正常</dd>
                </dl>
              </div>
            </div>
          </div>
        </div>

        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="p-5">
            <div class="flex items-center">
              <div class="flex-shrink-0">
                <svg class="h-6 w-6 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"></path>
                </svg>
              </div>
              <div class="ml-5 w-0 flex-1">
                <dl>
                  <dt class="text-sm font-medium text-gray-500 truncate">API 版本</dt>
                  <dd class="text-lg font-medium text-gray-900">v1.0.0</dd>
                </dl>
              </div>
            </div>
          </div>
        </div>

        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="p-5">
            <div class="flex items-center">
              <div class="flex-shrink-0">
                <svg class="h-6 w-6 text-purple-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2H2.308c1-6 4.325-11.112 9.692-13.368A12.001 12.001 0 0121 17v4a2 2 0 01-2 2H7z"></path>
                </svg>
              </div>
              <div class="ml-5 w-0 flex-1">
                <dl>
                  <dt class="text-sm font-medium text-gray-500 truncate">WebUI 版本</dt>
                  <dd class="text-lg font-medium text-gray-900">v0.1.0</dd>
                </dl>
              </div>
            </div>
          </div>
        </div>

        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="p-5">
            <div class="flex items-center">
              <div class="flex-shrink-0">
                <svg class="h-6 w-6 text-yellow-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
              </div>
              <div class="ml-5 w-0 flex-1">
                <dl>
                  <dt class="text-sm font-medium text-gray-500 truncate">运行时间</dt>
                  <dd class="text-lg font-medium text-gray-900">--:--:--</dd>
                </dl>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="mb-6">
      <h3 class="text-lg font-medium text-gray-900 mb-4">快捷操作</h3>
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <a href="#" class="block border border-gray-200 rounded-lg p-4 hover:bg-gray-50 hover:border-primary-300 transition-colors">
          <div class="flex items-center">
            <div class="text-2xl mr-3">📦</div>
            <div>
              <h4 class="font-medium text-gray-900">备份管理</h4>
              <p class="text-sm text-gray-500">备份任务配置与管理</p>
            </div>
          </div>
        </a>
        
        <a href="#" class="block border border-gray-200 rounded-lg p-4 hover:bg-gray-50 hover:border-primary-300 transition-colors">
          <div class="flex items-center">
            <div class="text-2xl mr-3">📁</div>
            <div>
              <h4 class="font-medium text-gray-900">文件管理</h4>
              <p class="text-sm text-gray-500">文件浏览与操作</p>
            </div>
          </div>
        </a>
        
        <a href="#" class="block border border-gray-200 rounded-lg p-4 hover:bg-gray-50 hover:border-primary-300 transition-colors">
          <div class="flex items-center">
            <div class="text-2xl mr-3">👥</div>
            <div>
              <h4 class="font-medium text-gray-900">用户管理</h4>
              <p class="text-sm text-gray-500">用户与权限管理</p>
            </div>
          </div>
        </a>
        
        <a href="#" class="block border border-gray-200 rounded-lg p-4 hover:bg-gray-50 hover:border-primary-300 transition-colors">
          <div class="flex items-center">
            <div class="text-2xl mr-3">🔧</div>
            <div>
              <h4 class="font-medium text-gray-900">系统设置</h4>
              <p class="text-sm text-gray-500">系统配置与监控</p>
            </div>
          </div>
        </a>
        
        <a href="#" class="block border border-gray-200 rounded-lg p-4 hover:bg-gray-50 hover:border-primary-300 transition-colors">
          <div class="flex items-center">
            <div class="text-2xl mr-3">📊</div>
            <div>
              <h4 class="font-medium text-gray-900">媒体管理</h4>
              <p class="text-sm text-gray-500">媒体文件管理</p>
            </div>
          </div>
        </a>
        
        <a href="#" class="block border border-gray-200 rounded-lg p-4 hover:bg-gray-50 hover:border-primary-300 transition-colors">
          <div class="flex items-center">
            <div class="text-2xl mr-3">🔗</div>
            <div>
              <h4 class="font-medium text-gray-900">共享管理</h4>
              <p class="text-sm text-gray-500">SMB/NFS/WebDAV 共享</p>
            </div>
          </div>
        </a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Home/Dashboard view component
</script>

<style scoped>
/* Dashboard specific styles */
</style>
