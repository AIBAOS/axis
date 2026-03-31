import axios, { AxiosError, AxiosRequestConfig } from 'axios'

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080'

// OPT-4: API 请求自动重试配置
const RETRY_CONFIG = {
  maxRetries: 3,           // 最大重试次数
  baseDelay: 1000,         // 基础延迟 1 秒
  maxDelay: 4000,          // 最大延迟 4 秒
  retryableStatusCodes: [  // 可重试的 HTTP 状态码
    408, // Request Timeout
    429, // Too Many Requests
    500, // Internal Server Error
    502, // Bad Gateway
    503, // Service Unavailable
    504, // Gateway Timeout
  ],
}

// 重试状态码映射（用于日志）
const STATUS_CODE_NAMES: Record<number, string> = {
  408: 'Request Timeout',
  429: 'Too Many Requests',
  500: 'Internal Server Error',
  502: 'Bad Gateway',
  503: 'Service Unavailable',
  504: 'Gateway Timeout',
}

// 延迟函数
function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

// 计算指数退避延迟
function getRetryDelay(attempt: number): number {
  // 指数退避：1s, 2s, 4s
  const delayMs = RETRY_CONFIG.baseDelay * Math.pow(2, attempt - 1)
  return Math.min(delayMs, RETRY_CONFIG.maxDelay)
}

// 判断是否应该重试
function shouldRetry(error: AxiosError): boolean {
  // 网络错误（无响应）- 重试
  if (!error.response) {
    return true
  }
  
  const status = error.response.status
  
  // 4xx 客户端错误 - 不重试（特例除外）
  if (status >= 400 && status < 500) {
    // Bug #66 修复：408 Request Timeout 和 429 Too Many Requests 可以重试
    if (status === 408 || status === 429) {
      return true
    }
    return false
  }
  
  // 5xx 服务器错误 - 检查是否在可重试列表中
  return RETRY_CONFIG.retryableStatusCodes.includes(status)
}

// 记录重试日志
function logRetry(attempt: number, maxRetries: number, error: AxiosError, delayMs: number): void {
  const status = error.response?.status
  const statusName = status ? STATUS_CODE_NAMES[status] || `HTTP ${status}` : 'Network Error'
  const url = error.config?.url || 'unknown'
  
  console.warn(
    `[API Retry] Attempt ${attempt}/${maxRetries} | ` +
    `${statusName} | ` +
    `URL: ${url} | ` +
    `Retry in ${delayMs}ms`
  )
}

// Create axios instance
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json'
  }
})

// Request interceptor
apiClient.interceptors.request.use(
  config => {
    // Add JWT token if available
    const token = localStorage.getItem('jwt_token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    
    // 初始化重试计数器（存储在 config 中）
    if (!config.metadata) {
      config.metadata = { retryCount: 0 }
    }
    
    return config
  },
  error => {
    return Promise.reject(error)
  }
)

// Response interceptor with retry logic
apiClient.interceptors.response.use(
  response => response,
  async (error: AxiosError) => {
    const config = error.config as (AxiosRequestConfig & { metadata?: { retryCount: number } }) | undefined
    
    // Bug #68 修复：处理 config 为 undefined 的情况
    if (!config) {
      return Promise.reject(error)
    }
    
    // 401 未授权 - 不重试，清除 token 并跳转登录
    if (error.response?.status === 401) {
      localStorage.removeItem('jwt_token')
      window.location.href = '/login'
      return Promise.reject(error)
    }
    
    // 检查是否应该重试
    if (!shouldRetry(error)) {
      return Promise.reject(error)
    }
    
    // Bug #67 修复：确保 metadata 存在并正确初始化
    if (!config.metadata) {
      config.metadata = { retryCount: 0 }
    }
    
    const retryCount = config.metadata.retryCount
    
    // 检查是否达到最大重试次数
    if (retryCount >= RETRY_CONFIG.maxRetries) {
      console.warn(`[API Retry] Max retries (${RETRY_CONFIG.maxRetries}) reached for ${config.url}`)
      return Promise.reject(error)
    }
    
    // 增加重试计数
    config.metadata.retryCount++
    
    // 计算延迟时间（指数退避）
    const delayMs = getRetryDelay(retryCount)
    
    // 记录重试日志
    logRetry(retryCount, RETRY_CONFIG.maxRetries, error, delayMs)
    
    // 等待后重试
    await delay(delayMs)
    
    // 重新发送请求（确保 metadata 传递）
    return apiClient.request({
      ...config,
      metadata: config.metadata  // Bug #67 修复：显式传递 metadata
    })
  }
)

// 扩展 AxiosRequestConfig 类型以支持 metadata
declare module 'axios' {
  interface AxiosRequestConfig {
    metadata?: {
      retryCount: number
    }
  }
}

export default apiClient

// 导出重试配置（供测试使用）
export { RETRY_CONFIG, shouldRetry, getRetryDelay }