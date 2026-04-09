<template>
  <DefaultLayout>
    <router-view />
    <!-- 全局 Toast 容器 -->
    <div class="fixed top-4 right-4 z-50 space-y-2">
      <TransitionGroup name="toast">
        <div
          v-for="toast in toasts"
          :key="toast.id"
          class="flex items-center w-full max-w-xs p-4 rounded-lg shadow-lg"
          :class="toastClasses[toast.type]"
          role="alert"
        >
          <!-- 图标 -->
          <div class="flex-shrink-0">
            <svg v-if="toast.type === 'success'" class="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <svg v-else-if="toast.type === 'error'" class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <svg v-else-if="toast.type === 'warning'" class="w-5 h-5 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
            </svg>
            <svg v-else class="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
          </div>

          <!-- 消息内容 -->
          <div class="ml-3 text-sm font-medium text-gray-900 dark:text-white">
            {{ toast.message }}
          </div>

          <!-- 关闭按钮 -->
          <button
            @click="removeToast(toast.id)"
            class="ml-auto -mx-1.5 -my-1.5 rounded-lg p-1.5 inline-flex h-8 w-8 hover:bg-gray-100 dark:hover:bg-gray-700"
            :class="buttonClasses[toast.type]"
            aria-label="关闭"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
            </svg>
          </button>
        </div>
      </TransitionGroup>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { provide } from 'vue'
import DefaultLayout from './layouts/DefaultLayout.vue'
import { useToast } from './composables/useToast'

const { toasts, removeToast } = useToast()

// 提供 toast 方法给子组件
provide('toast', {
  toasts,
  removeToast
})

const toastClasses = {
  success: 'bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800',
  error: 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800',
  warning: 'bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800',
  info: 'bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800'
}

const buttonClasses = {
  success: 'text-green-500 hover:bg-green-100 dark:text-green-400 dark:hover:bg-green-800',
  error: 'text-red-500 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-800',
  warning: 'text-yellow-500 hover:bg-yellow-100 dark:text-yellow-400 dark:hover:bg-yellow-800',
  info: 'text-blue-500 hover:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-800'
}
</script>

<style>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
