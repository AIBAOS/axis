// useNetwork.ts - 网络状态管理和自动重连
import { ref, computed, onMounted, onUnmounted } from 'vue'
import apiClient from '../api'

export type NetworkStatus = 'online' | 'offline' | 'reconnecting'

// 全局网络状态
const isOnline = ref(navigator.onLine)
const status = ref<NetworkStatus>(navigator.onLine ? 'online' : 'offline')
const lastHeartbeat = ref<Date | null>(null)
const reconnectAttempts = ref(0)
const maxReconnectAttempts = 5

// 心跳检测配置
const HEARTBEAT_INTERVAL = 30000 // 30 秒
const RECONNECT_BASE_DELAY = 1000 // 1 秒基础延迟
const RECONNECT_MAX_DELAY = 30000 // 最大 30 秒

let heartbeatTimer: ReturnType<typeof setInterval> | null = null
let reconnectTimer: ReturnType<typeof setTimeout> | null = null

// 心跳检测
async function sendHeartbeat(): Promise<boolean> {
  try {
    await apiClient.get('/api/v1/health', { timeout: 5000 })
    lastHeartbeat.value = new Date()
    return true
  } catch {
    return false
  }
}

// 开始心跳检测
function startHeartbeat() {
  if (heartbeatTimer) clearInterval(heartbeatTimer)
  
  heartbeatTimer = setInterval(async () => {
    const success = await sendHeartbeat()
    if (!success && status.value === 'online') {
      handleDisconnect()
    }
  }, HEARTBEAT_INTERVAL)
}

// 停止心跳检测
function stopHeartbeat() {
  if (heartbeatTimer) {
    clearInterval(heartbeatTimer)
    heartbeatTimer = null
  }
}

// 处理断网
function handleDisconnect() {
  status.value = 'offline'
  isOnline.value = false
  reconnectAttempts.value = 0
  attemptReconnect()
}

// 尝试重连（指数退避）
function attemptReconnect() {
  if (reconnectAttempts.value >= maxReconnectAttempts) {
    console.warn('Max reconnect attempts reached')
    return
  }

  if (reconnectTimer) clearTimeout(reconnectTimer)
  
  status.value = 'reconnecting'
  reconnectAttempts.value++
  
  // 指数退避：1s, 2s, 4s, 8s, 16s
  const delay = Math.min(
    RECONNECT_BASE_DELAY * Math.pow(2, reconnectAttempts.value - 1),
    RECONNECT_MAX_DELAY
  )
  
  console.log(`Reconnecting in ${delay}ms (attempt ${reconnectAttempts.value}/${maxReconnectAttempts})`)
  
  reconnectTimer = setTimeout(async () => {
    const success = await sendHeartbeat()
    if (success) {
      handleReconnectSuccess()
    } else {
      attemptReconnect()
    }
  }, delay)
}

// 重连成功
function handleReconnectSuccess() {
  status.value = 'online'
  isOnline.value = true
  reconnectAttempts.value = 0
  lastHeartbeat.value = new Date()
  
  // 触发全局事件，通知其他组件刷新数据
  window.dispatchEvent(new CustomEvent('network-reconnected'))
}

// 手动重连
async function manualReconnect(): Promise<boolean> {
  reconnectAttempts.value = 0
  status.value = 'reconnecting'
  
  const success = await sendHeartbeat()
  if (success) {
    handleReconnectSuccess()
    return true
  } else {
    status.value = 'offline'
    return false
  }
}

// 浏览器网络事件监听
function handleOnline() {
  sendHeartbeat().then(success => {
    if (success) {
      handleReconnectSuccess()
    }
  })
}

function handleOffline() {
  status.value = 'offline'
  isOnline.value = false
}

export function useNetwork() {
  const isOffline = computed(() => !isOnline.value)
  const isReconnecting = computed(() => status.value === 'reconnecting')
  
  onMounted(() => {
    // 监听浏览器网络事件
    window.addEventListener('online', handleOnline)
    window.addEventListener('offline', handleOffline)
    
    // 启动心跳检测
    startHeartbeat()
    
    // 首次心跳
    sendHeartbeat().then(success => {
      if (!success) {
        handleDisconnect()
      }
    })
  })
  
  onUnmounted(() => {
    window.removeEventListener('online', handleOnline)
    window.removeEventListener('offline', handleOffline)
    stopHeartbeat()
    if (reconnectTimer) clearTimeout(reconnectTimer)
  })
  
  return {
    isOnline,
    isOffline,
    status,
    isReconnecting,
    reconnectAttempts,
    maxReconnectAttempts,
    lastHeartbeat,
    manualReconnect,
    sendHeartbeat,
  }
}

// 全局方法（不依赖组件生命周期）
export const networkService = {
  getStatus: () => status.value,
  isOnline: () => isOnline.value,
  manualReconnect,
  sendHeartbeat,
}