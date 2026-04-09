<template>
  <div class="animate-pulse space-y-3" :class="containerClass">
    <!-- 标题骨架 -->
    <div v-if="hasTitle" class="h-4 bg-gray-200 dark:bg-gray-700 rounded" :class="titleClass"></div>
    
    <!-- 内容骨架 -->
    <div v-for="i in lines" :key="i" class="h-3 bg-gray-200 dark:bg-gray-700 rounded" :class="lineClass"></div>
    
    <!-- 自定义骨架槽位 -->
    <slot></slot>
  </div>
</template>

<script setup>
defineProps({
  // 是否显示标题
  hasTitle: {
    type: Boolean,
    default: false
  },
  // 行数
  lines: {
    type: Number,
    default: 3
  },
  // 标题宽度
  titleWidth: {
    type: String,
    default: 'w-1/3'
  },
  // 行宽度
  lineWidth: {
    type: String,
    default: 'w-full'
  },
  // 容器类
  containerClass: {
    type: String,
    default: 'p-4'
  }
})

const titleClass = computed(() => props.titleWidth)
const lineClass = computed(() => props.lineWidth)
</script>
