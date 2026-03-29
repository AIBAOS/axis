<template>
  <div class="bg-white rounded-lg shadow-md p-4">
    <div class="flex items-center justify-between mb-3">
      <h3 class="font-semibold text-gray-900">网络流量</h3>
      <div class="flex items-center space-x-4 text-sm">
        <span class="flex items-center">
          <span class="w-3 h-3 rounded-full bg-green-500 mr-1"></span>
          <span class="text-gray-600">下载 {{ formatSpeed(currentRx) }}</span>
        </span>
        <span class="flex items-center">
          <span class="w-3 h-3 rounded-full bg-blue-500 mr-1"></span>
          <span class="text-gray-600">上传 {{ formatSpeed(currentTx) }}</span>
        </span>
      </div>
    </div>
    
    <div class="relative h-24">
      <canvas ref="chartCanvas" class="w-full h-full"></canvas>
    </div>
    
    <div class="grid grid-cols-2 gap-4 mt-3 pt-3 border-t">
      <div class="text-center">
        <p class="text-sm text-gray-500">总下载</p>
        <p class="text-lg font-semibold text-green-600">{{ formatBytes(totalRx) }}</p>
      </div>
      <div class="text-center">
        <p class="text-sm text-gray-500">总上传</p>
        <p class="text-lg font-semibold text-blue-600">{{ formatBytes(totalTx) }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'

const props = defineProps<{
  rxHistory: number[]
  txHistory: number[]
  currentRx: number
  currentTx: number
  totalRx: number
  totalTx: number
}>()

const chartCanvas = ref<HTMLCanvasElement | null>(null)
let animationFrame: number | null = null

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

const formatSpeed = (bytesPerSec: number) => {
  if (!bytesPerSec) return '0 B/s'
  return formatBytes(bytesPerSec) + '/s'
}

const drawChart = () => {
  const canvas = chartCanvas.value
  if (!canvas) return
  
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  // 设置 canvas 实际尺寸
  const rect = canvas.getBoundingClientRect()
  canvas.width = rect.width * 2
  canvas.height = rect.height * 2
  ctx.scale(2, 2)
  
  const width = rect.width
  const height = rect.height
  
  // 清空画布
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
  
  // 计算最大值
  const allValues = [...props.rxHistory, ...props.txHistory]
  const maxValue = Math.max(...allValues, 1)
  
  // 绘制下载线（绿色）
  if (props.rxHistory.length > 1) {
    ctx.strokeStyle = '#22c55e'
    ctx.lineWidth = 2
    ctx.beginPath()
    const stepX = width / (props.rxHistory.length - 1)
    props.rxHistory.forEach((val, i) => {
      const x = stepX * i
      const y = height - (val / maxValue) * (height - 10)
      if (i === 0) ctx.moveTo(x, y)
      else ctx.lineTo(x, y)
    })
    ctx.stroke()
    
    // 绘制填充区域
    ctx.fillStyle = 'rgba(34, 197, 94, 0.1)'
    ctx.beginPath()
    ctx.moveTo(0, height)
    props.rxHistory.forEach((val, i) => {
      const x = stepX * i
      const y = height - (val / maxValue) * (height - 10)
      ctx.lineTo(x, y)
    })
    ctx.lineTo(width, height)
    ctx.closePath()
    ctx.fill()
  }
  
  // 绘制上传线（蓝色）
  if (props.txHistory.length > 1) {
    ctx.strokeStyle = '#3b82f6'
    ctx.lineWidth = 2
    ctx.beginPath()
    const stepX = width / (props.txHistory.length - 1)
    props.txHistory.forEach((val, i) => {
      const x = stepX * i
      const y = height - (val / maxValue) * (height - 10)
      if (i === 0) ctx.moveTo(x, y)
      else ctx.lineTo(x, y)
    })
    ctx.stroke()
    
    // 绘制填充区域
    ctx.fillStyle = 'rgba(59, 130, 246, 0.1)'
    ctx.beginPath()
    ctx.moveTo(0, height)
    props.txHistory.forEach((val, i) => {
      const x = stepX * i
      const y = height - (val / maxValue) * (height - 10)
      ctx.lineTo(x, y)
    })
    ctx.lineTo(width, height)
    ctx.closePath()
    ctx.fill()
  }
}

watch(() => [props.rxHistory, props.txHistory], () => {
  drawChart()
}, { deep: true })

onMounted(() => {
  drawChart()
})

onUnmounted(() => {
  if (animationFrame) cancelAnimationFrame(animationFrame)
})
</script>