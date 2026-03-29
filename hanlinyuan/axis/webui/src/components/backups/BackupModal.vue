<template>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4">
      <!-- 标题栏 -->
      <div class="flex justify-between items-center px-6 py-4 border-b">
        <h3 class="text-lg font-semibold text-gray-900">
          {{ mode === 'create' ? '新建备份任务' : '编辑备份任务' }}
        </h3>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 表单 -->
      <form @submit.prevent="handleSubmit" class="px-6 py-4 space-y-4">
        <!-- 任务名称 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">任务名称</label>
          <input
            v-model="formData.name"
            type="text"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="请输入任务名称"
          />
        </div>

        <!-- 备份类型 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">备份类型</label>
          <select
            v-model="formData.type"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          >
            <option value="full">全量备份</option>
            <option value="incremental">增量备份</option>
          </select>
        </div>

        <!-- 源路径 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">源路径</label>
          <input
            v-model="formData.source_path"
            type="text"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="例如：/data/source"
          />
        </div>

        <!-- 目标路径 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">目标路径</label>
          <input
            v-model="formData.destination_path"
            type="text"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="例如：/backup/destination"
          />
        </div>

        <!-- 计划任务 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">计划任务</label>
          <select
            v-model="formData.schedule"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          >
            <option value="manual">手动执行</option>
            <option value="hourly">每小时</option>
            <option value="daily">每天</option>
            <option value="weekly">每周</option>
            <option value="monthly">每月</option>
          </select>
        </div>

        <!-- 描述 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">描述</label>
          <textarea
            v-model="formData.description"
            rows="3"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            placeholder="可选，描述备份任务的用途"
          />
        </div>
      </form>

      <!-- 按钮栏 -->
      <div class="flex justify-end space-x-3 px-6 py-4 border-t bg-gray-50 rounded-b-lg">
        <button
          type="button"
          @click="$emit('close')"
          class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50"
        >
          取消
        </button>
        <button
          type="submit"
          @click="handleSubmit"
          class="btn-primary"
        >
          {{ mode === 'create' ? '创建' : '保存' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'

const props = defineProps<{
  mode: 'create' | 'edit'
  backup?: any
}>()

const emit = defineEmits<{
  close: []
  save: [data: any]
}>()

// 表单数据
const formData = ref({
  name: '',
  type: 'full',
  source_path: '',
  destination_path: '',
  schedule: 'manual',
  description: ''
})

// 监听备份数据变化（编辑模式）
watch(() => props.backup, (newBackup) => {
  if (newBackup && props.mode === 'edit') {
    formData.value = {
      name: newBackup.name || '',
      type: newBackup.type || 'full',
      source_path: newBackup.source_path || '',
      destination_path: newBackup.destination_path || '',
      schedule: newBackup.schedule || 'manual',
      description: newBackup.description || ''
    }
  }
}, { immediate: true })

// 提交表单
const handleSubmit = () => {
  emit('save', {
    name: formData.value.name,
    type: formData.value.type,
    source_path: formData.value.source_path,
    destination_path: formData.value.destination_path,
    schedule: formData.value.schedule === 'manual' ? null : formData.value.schedule,
    description: formData.value.description
  })
}
</script>
