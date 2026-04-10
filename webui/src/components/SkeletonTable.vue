<template>
  <div class="bg-white rounded-lg shadow overflow-hidden">
    <!-- 表格骨架屏 -->
    <table class="w-full">
      <thead class="bg-gray-50 border-b">
        <tr>
          <th v-for="i in columns" :key="i" class="px-4 py-3 text-left">
            <div class="h-3 bg-gray-200 rounded w-16 animate-pulse"></div>
          </th>
        </tr>
      </thead>
      <tbody class="divide-y divide-gray-100">
        <tr v-for="row in rows" :key="row" class="hover:bg-gray-50">
          <td v-for="col in columns" :key="col" class="px-4 py-3">
            <div class="h-4 bg-gray-200 rounded animate-pulse" :class="getWidth(col)"></div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  columns?: number
  rows?: number
}>()

const columns = computed(() => props.columns || 5)
const rows = computed(() => props.rows || 5)

const getWidth = (col: number) => {
  const widths = ['w-24', 'w-16', 'w-20', 'w-32', 'w-12']
  return widths[col % widths.length]
}
</script>