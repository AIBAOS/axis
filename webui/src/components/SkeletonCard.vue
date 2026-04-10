<template>
  <div class="bg-white rounded-lg shadow p-4 animate-pulse">
    <!-- 卡片头部骨架 -->
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center space-x-3">
        <div class="w-12 h-12 rounded-lg bg-gray-200"></div>
        <div class="space-y-2">
          <div class="h-4 bg-gray-200 rounded w-24"></div>
          <div class="h-3 bg-gray-200 rounded w-16"></div>
        </div>
      </div>
      <div class="h-6 bg-gray-200 rounded-full w-16"></div>
    </div>

    <!-- 卡片内容骨架 -->
    <div class="space-y-3">
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 bg-gray-200 rounded"></div>
        <div class="h-4 bg-gray-200 rounded w-32"></div>
      </div>
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 bg-gray-200 rounded"></div>
        <div class="h-3 bg-gray-200 rounded w-24"></div>
      </div>
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 bg-gray-200 rounded"></div>
        <div class="h-3 bg-gray-200 rounded w-20"></div>
      </div>
    </div>

    <!-- 操作按钮骨架 -->
    <div class="flex justify-end space-x-2 pt-4 border-t border-gray-100 mt-4">
      <div class="h-8 bg-gray-200 rounded w-16"></div>
      <div class="h-8 bg-gray-200 rounded w-16"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 骨架屏组件 - 用于卡片加载状态展示
</script>