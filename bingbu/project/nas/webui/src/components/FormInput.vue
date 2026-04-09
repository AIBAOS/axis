<template>
  <div class="space-y-1">
    <!-- 标签 -->
    <label v-if="label" :for="id" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
      {{ label }}
      <span v-if="required" class="text-red-500">*</span>
    </label>

    <!-- 输入框 -->
    <div class="relative">
      <input
        :id="id"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :required="required"
        :disabled="disabled"
        :class="inputClasses"
        @input="$emit('update:modelValue', $event.target.value)"
        @blur="$emit('blur')"
      />

      <!-- 错误图标 -->
      <svg
        v-if="error"
        class="absolute right-3 top-1/2 -translate-y-1/2 w-5 h-5 text-red-500"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
    </div>

    <!-- 错误提示 -->
    <p v-if="error" class="text-sm text-red-600 dark:text-red-400">
      {{ error }}
    </p>

    <!-- 帮助文字 -->
    <p v-if="help && !error" class="text-sm text-gray-500 dark:text-gray-400">
      {{ help }}
    </p>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  // v-model 绑定
  modelValue: {
    type: String,
    default: ''
  },
  // 标签
  label: {
    type: String,
    default: ''
  },
  // 输入框类型
  type: {
    type: String,
    default: 'text'
  },
  // 占位符
  placeholder: {
    type: String,
    default: ''
  },
  // 是否必填
  required: {
    type: Boolean,
    default: false
  },
  // 是否禁用
  disabled: {
    type: Boolean,
    default: false
  },
  // 错误信息
  error: {
    type: String,
    default: ''
  },
  // 帮助文字
  help: {
    type: String,
    default: ''
  },
  // 唯一 ID
  id: {
    type: String,
    default: () => `input-${Math.random().toString(36).substr(2, 9)}`
  }
})

defineEmits(['update:modelValue', 'blur'])

const inputClasses = computed(() => {
  const base = 'w-full px-3 py-2 border rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2'
  
  if (props.error) {
    return `${base} border-red-500 focus:ring-red-500 focus:border-red-500`
  }
  
  if (props.disabled) {
    return `${base} border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-800 cursor-not-allowed`
  }
  
  return `${base} border-gray-300 dark:border-gray-600 focus:ring-indigo-500 focus:border-indigo-500`
})
</script>
