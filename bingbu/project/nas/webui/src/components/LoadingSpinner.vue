<template>
  <div class="flex flex-col items-center justify-center" :class="containerClass">
    <!-- 旋转动画 -->
    <svg
      class="animate-spin"
      :class="spinnerClass"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>

    <!-- 加载文字 -->
    <p v-if="text" class="mt-3 text-sm" :class="textClass">
      {{ text }}
    </p>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  // 尺寸：sm, md, lg
  size: {
    type: String,
    default: 'md',
    validator: (value) => ['sm', 'md', 'lg'].includes(value)
  },
  // 加载文字
  text: {
    type: String,
    default: ''
  },
  // 颜色
  color: {
    type: String,
    default: 'indigo'
  },
  // 容器类
  containerClass: {
    type: String,
    default: 'p-4'
  }
})

const sizeClasses = {
  sm: 'w-4 h-4',
  md: 'w-8 h-8',
  lg: 'w-12 h-12'
}

const colorClasses = {
  indigo: 'text-indigo-600 dark:text-indigo-400',
  white: 'text-white',
  gray: 'text-gray-600 dark:text-gray-400'
}

const spinnerClass = computed(() => {
  return `${sizeClasses[props.size]} ${colorClasses[props.color]}`
})

const textClass = computed(() => {
  return `text-${props.color}-600 dark:text-${props.color}-400`
})
</script>
