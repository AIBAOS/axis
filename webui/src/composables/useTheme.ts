import { ref, watch, onMounted } from 'vue'

export type Theme = 'light' | 'dark' | 'system'

const STORAGE_KEY = 'axis-theme'

// 全局状态
const currentTheme = ref<Theme>('system')
const isDark = ref(false)

// 获取系统偏好
const getSystemPreference = (): 'light' | 'dark' => {
  if (typeof window !== 'undefined' && window.matchMedia) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }
  return 'light'
}

// 应用主题到 DOM
const applyTheme = (theme: Theme) => {
  const html = document.documentElement
  const effectiveTheme = theme === 'system' ? getSystemPreference() : theme
  
  if (effectiveTheme === 'dark') {
    html.classList.add('dark')
    isDark.value = true
  } else {
    html.classList.remove('dark')
    isDark.value = false
  }
}

// 切换主题
const setTheme = (theme: Theme) => {
  currentTheme.value = theme
  localStorage.setItem(STORAGE_KEY, theme)
  applyTheme(theme)
}

// 切换深色/浅色
const toggleTheme = () => {
  const newTheme = isDark.value ? 'light' : 'dark'
  setTheme(newTheme)
}

// 初始化主题
const initTheme = () => {
  const saved = localStorage.getItem(STORAGE_KEY) as Theme | null
  const theme = saved || 'system'
  currentTheme.value = theme
  applyTheme(theme)
  
  // 监听系统偏好变化
  if (typeof window !== 'undefined' && window.matchMedia) {
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      if (currentTheme.value === 'system') {
        applyTheme('system')
      }
    })
  }
}

export function useTheme() {
  onMounted(() => {
    initTheme()
  })
  
  return {
    currentTheme,
    isDark,
    setTheme,
    toggleTheme,
    initTheme
  }
}