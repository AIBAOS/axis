<template>
  <div class="bg-white rounded-lg shadow-md p-4">
    <div class="flex items-center justify-between mb-3">
      <h3 class="font-semibold text-gray-900">CPU & 内存趋势</h3>
      <div class="flex items-center space-x-4 text-sm">
        <span class="flex items-center">
          <span class="w-3 h-3 rounded-full bg-blue-500 mr-1"></span>
          <span class="text-gray-600">CPU {{ cpuPercent.toFixed(1) }}%</span>
        </span>
        <span class="flex items-center">
          <span class="w-3 h-3 rounded-full bg-purple-500 mr-1"></span>
          <span class="text-gray-600">内存 {{ memoryPercent.toFixed(1) }}%</span>
        </span>
      </div>
    </div>
    
    <div class="relative h-32">
      <canvas ref="chartCanvas" class="w-full h-full"></canvas>
    </div>
    
    <div class="grid grid-cols-2 gap-4 mt-3 pt-3 border-t">
      <div>
        <p class="text-sm text-gray-500">CPU 负载</p>
        <p class="text-lg font-semibold text-blue-600">{{ cpuLoad }}</p>
      </div>
      <div>
        <p class="text-sm text-gray-500">内存使用</p>
        <p class="text-lg font-semibold text-purple-600">{{ memoryUsed }} / {{ memoryTotal }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'

const props = defineProps<{
  cpuHistory: number[]
  memoryHistory: number[]
  cpuPercent: number
  memoryPercent: number
  cpuLoad: string
  memoryUsed: string
  memoryTotal: string
}>()

const chartCanvas = ref<HTMLCanvasElement | null>(null)

const drawChart = () => {
  const canvas = chartCanvas.value
  if (!canvas) return
  
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  const rect = canvas.getBoundingClientRect()
  canvas.width = rect.width * 2
  canvas.height = rect.height * 2
  ctx.scale(2, 2)
  
  const width = rect.width
  const height = rect.height
  
  ctx.clearRect(0, 0, width, height)
  
  // 绘制背景网格
  ctx.strokeStyle = '#e5e7eb'
  ctx.lineWidth = 0.5
  for (let i = 0; i < 5; i++) {
    const y = (height / 5) * i
    ctx.beginPath()
    ctx.moveTo(0, y)
    ctx.lineTo(width, y)
    ctx.stroke()
  }
  
  // 绘制 CPU 线（蓝色）
  if (props.cpuHistory.length > 1) {
    ctx.strokeStyle = '#3b82f6'
    ctx.lineWidth = 2
    ctx.beginPath()
    const stepX = width / (props.cpuHistory.length - 1)
    props.cpuHistory.forEach((val, i) => {
      const x = stepX * i
      const y = height - (val / 100) * (height - 10)
      if (i === 0) ctx.moveTo(x, y)
      else ctx.lineTo(x, y)
    })
    ctx.stroke()
    
    ctx.fillStyle = 'rgba(59, 130, 246, 0.1)'
    ctx.beginPath()
    ctx.moveTo(0, height)
    props.cpuHistory.forEach((val, i) => {
      const x = stepX * i
      const y = height - (val / 100) * (height - 10)
      ctx.lineTo(x, y)
    })
    ctx.lineTo(width, height)
    ctx.closePath()
    ctx.fill()
  }
  
  // 绘制内存线（紫色）
  if (props.memoryHistory.length > 1) {
    ctx.strokeStyle = '#a855f7'
    ctx.lineWidth = 2
    ctx.beginPath()
    const stepX = width / (props.memoryHistory.length - 1)
    props.memoryHistory.forEach((val, i) => {
      const x = stepX * i
      const y = height - (val / 100) * (height - 10)
      if (i === 0) ctx.moveTo(x, y)
      else ctx.lineTo(x, y)
    })
    ctx.stroke()
    
    ctx.fillStyle = 'rgba(168, 85, 247, 0.1)'
    ctx.beginPath()
    ctx.moveTo(0, height)
    props.memoryHistory.forEach((val, i) => {
      const x = stepX * i
      const y = height - (val / 100) * (height - 10)
      ctx.lineTo(x, y)
    })
    ctx.lineTo(width, height)
    ctx.closePath()
    ctx.fill()
  }
}

watch(() => [props.cpuHistory, props.memoryHistory], () => {
  drawChart()
}, { deep: true })

onMounted(() => {
  drawChart()
})
</script>