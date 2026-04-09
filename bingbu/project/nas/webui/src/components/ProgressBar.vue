<template>
  <div class="w-full">
    <!-- 进度条 -->
    <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5">
      <div
        class="h-2.5 rounded-full transition-all duration-300 ease-out"
        :class="colorClass"
        :style="{ width: `${normalizedValue}%` }"
      ></div>
    </div>

    <!-- 文字显示 -->
    <div v-if="showLabel" class="flex justify-between mt-1">
      <span class="text-xs text-gray-500 dark:text-gray-400">{{ label }}</span>
      <span class="text-xs text-gray-500 dark:text-gray-400">{{ normalizedValue.toFixed(1) }}%</span>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  // 进度值 (0-100)
  value: {
    type: Number,
    default: 0
  },
  // 标签文字
  label: {
    type: String,
    default: ''
  },
  // 是否显示标签
  showLabel: {
    type: Boolean,
    default: false
  },
  // 颜色：blue/green/yellow/red/indigo
  color: {
    type: String,
    default: 'blue',
    validator: (value) => ['blue', 'green', 'yellow', 'red', 'indigo'].includes(value)
  }
})

const normalizedValue = computed(() => {
  return Math.max(0, Math.min(100, props.value))
})

const colorClass = computed(() => {
  const colors = {
    blue: 'bg-blue-600',
    green: 'bg-green-600',
    yellow: 'bg-yellow-600',
    red: 'bg-red-600',
    indigo: 'bg-indigo-600'
  }
  return colors[props.color]
})
</script>
