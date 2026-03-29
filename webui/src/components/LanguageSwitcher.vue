<template>
  <div class="relative" ref="dropdownRef">
    <button
      @click="isOpen = !isOpen"
      class="flex items-center space-x-2 px-3 py-2 rounded-lg hover:bg-gray-100 transition-colors"
    >
      <span class="text-lg">{{ currentLocale.flag }}</span>
      <span class="text-sm text-gray-700">{{ currentLocale.name }}</span>
      <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>
    
    <div
      v-if="isOpen"
      class="absolute right-0 mt-2 w-40 bg-white rounded-lg shadow-lg border border-gray-200 py-1 z-50"
    >
      <button
        v-for="locale in availableLocales"
        :key="locale.code"
        @click="changeLocale(locale.code)"
        :class="[
          'w-full px-4 py-2 text-left hover:bg-gray-50 flex items-center space-x-3',
          locale.code === currentLocaleCode ? 'bg-primary-50 text-primary-700' : 'text-gray-700'
        ]"
      >
        <span class="text-lg">{{ locale.flag }}</span>
        <span class="text-sm">{{ locale.name }}</span>
        <svg v-if="locale.code === currentLocaleCode" class="w-4 h-4 ml-auto text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { setLocale, getLocale, availableLocales } from '@/i18n'

const isOpen = ref(false)
const dropdownRef = ref<HTMLElement | null>(null)

const currentLocaleCode = computed(() => getLocale())

const currentLocale = computed(() => {
  return availableLocales.find(l => l.code === currentLocaleCode.value) || availableLocales[0]
})

const changeLocale = (code: string) => {
  setLocale(code)
  isOpen.value = false
  window.location.reload() // 刷新页面以应用新语言
}

// 点击外部关闭下拉菜单
const handleClickOutside = (event: MouseEvent) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>