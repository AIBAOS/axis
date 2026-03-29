<template>
  <div class="bg-white rounded-lg shadow-md p-4">
    <h3 class="font-semibold text-gray-900 mb-4">快速操作</h3>
    
    <div class="grid grid-cols-3 gap-3">
      <!-- 重启 -->
      <button
        @click="handleRestart"
        :disabled="operating"
        class="flex flex-col items-center justify-center p-3 rounded-lg border border-gray-200 hover:bg-orange-50 hover:border-orange-300 transition-colors disabled:opacity-50"
      >
        <svg class="w-6 h-6 text-orange-500 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        <span class="text-sm font-medium text-gray-700">重启</span>
      </button>
      
      <!-- 关机 -->
      <button
        @click="handleShutdown"
        :disabled="operating"
        class="flex flex-col items-center justify-center p-3 rounded-lg border border-gray-200 hover:bg-red-50 hover:border-red-300 transition-colors disabled:opacity-50"
      >
        <svg class="w-6 h-6 text-red-500 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 5.636a9 9 0 11-12.728 0M12 3v9" />
        </svg>
        <span class="text-sm font-medium text-gray-700">关机</span>
      </button>
      
      <!-- 更新检查 -->
      <button
        @click="handleCheckUpdates"
        :disabled="checkingUpdates"
        class="flex flex-col items-center justify-center p-3 rounded-lg border border-gray-200 hover:bg-blue-50 hover:border-blue-300 transition-colors disabled:opacity-50"
      >
        <svg :class="{'animate-spin': checkingUpdates}" class="w-6 h-6 text-blue-500 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        <span class="text-sm font-medium text-gray-700">更新检查</span>
      </button>
    </div>
    
    <!-- 更新状态 -->
    <div v-if="updateStatus" class="mt-4 p-3 rounded-lg bg-gray-50">
      <div class="flex items-center justify-between">
        <span class="text-sm text-gray-600">系统版本</span>
        <span class="text-sm font-medium text-gray-900">{{ updateStatus.current_version || '-' }}</span>
      </div>
      <div v-if="updateStatus.has_update" class="mt-2 flex items-center justify-between">
        <span class="text-sm text-green-600">有新版本可用</span>
        <span class="text-sm font-medium text-green-600">{{ updateStatus.latest_version }}</span>
      </div>
      <div v-else class="mt-2 text-sm text-gray-500">
        已是最新版本
      </div>
    </div>
    
    <!-- 确认对话框 -->
    <div v-if="showConfirm" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 max-w-sm mx-4">
        <h4 class="text-lg font-semibold text-gray-900 mb-2">{{ confirmTitle }}</h4>
        <p class="text-sm text-gray-600 mb-4">{{ confirmMessage }}</p>
        <div class="flex justify-end space-x-3">
          <button @click="showConfirm = false" class="btn-secondary">取消</button>
          <button @click="executeOperation" class="btn-danger">确认</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@/utils/api'
import { useToast } from '@/composables/useToast'

const { showToast } = useToast()

const operating = ref(false)
const checkingUpdates = ref(false)
const updateStatus = ref<any>(null)
const showConfirm = ref(false)
const confirmTitle = ref('')
const confirmMessage = ref('')
const pendingOperation = ref<'restart' | 'shutdown' | null>(null)

const handleRestart = () => {
  confirmTitle.value = '确认重启'
  confirmMessage.value = '系统将在确认后立即重启，所有服务将暂时中断。'
  pendingOperation.value = 'restart'
  showConfirm.value = true
}

const handleShutdown = () => {
  confirmTitle.value = '确认关机'
  confirmMessage.value = '系统将在确认后立即关机，请确保所有重要操作已完成。'
  pendingOperation.value = 'shutdown'
  showConfirm.value = true
}

const executeOperation = async () => {
  showConfirm.value = false
  operating.value = true
  
  try {
    if (pendingOperation.value === 'restart') {
      await api.system.restart()
      showToast('重启命令已发送', 'success')
    } else if (pendingOperation.value === 'shutdown') {
      await api.system.shutdown()
      showToast('关机命令已发送', 'success')
    }
  } catch (error: any) {
    showToast(error.message || '操作失败', 'error')
  } finally {
    operating.value = false
    pendingOperation.value = null
  }
}

const handleCheckUpdates = async () => {
  checkingUpdates.value = true
  
  try {
    const response = await api.system.checkUpdates()
    updateStatus.value = response.data.data || response.data
    
    if (updateStatus.value?.has_update) {
      showToast(`发现新版本 ${updateStatus.value.latest_version}`, 'info')
    } else {
      showToast('已是最新版本', 'success')
    }
  } catch (error: any) {
    showToast(error.message || '检查更新失败', 'error')
  } finally {
    checkingUpdates.value = false
  }
}
</script>