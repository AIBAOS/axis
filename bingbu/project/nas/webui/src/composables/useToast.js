import { ref } from 'vue'

const toasts = ref([])
let toastId = 0

export function useToast() {
  // 添加 Toast
  const addToast = (message, type = 'info', duration = 3000) => {
    const id = toastId++
    const toast = { id, message, type }
    toasts.value.push(toast)

    // 自动消失
    if (duration > 0) {
      setTimeout(() => {
        removeToast(id)
      }, duration)
    }

    return id
  }

  // 移除 Toast
  const removeToast = (id) => {
    const index = toasts.value.findIndex(t => t.id === id)
    if (index > -1) {
      toasts.value.splice(index, 1)
    }
  }

  // 快捷方法
  const success = (message, duration) => addToast(message, 'success', duration)
  const error = (message, duration) => addToast(message, 'error', duration)
  const warning = (message, duration) => addToast(message, 'warning', duration)
  const info = (message, duration) => addToast(message, 'info', duration)

  return {
    toasts,
    addToast,
    success,
    error,
    warning,
    info,
    removeToast
  }
}
