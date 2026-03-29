import { createI18n } from 'vue-i18n'
import enUS from '../locales/en-US.json'
import zhCN from '../locales/zh-CN.json'

// 从 localStorage 获取保存的语言设置，默认中文
const savedLocale = localStorage.getItem('locale') || 'zh-CN'

export const i18n = createI18n({
  legacy: false, // 使用 Composition API 模式
  locale: savedLocale,
  fallbackLocale: 'en-US',
  messages: {
    'en-US': enUS,
    'zh-CN': zhCN
  }
})

export const setLocale = (locale: string) => {
  i18n.global.locale.value = locale as any
  localStorage.setItem('locale', locale)
  document.documentElement.setAttribute('lang', locale)
}

export const getLocale = () => {
  return i18n.global.locale.value
}

export const availableLocales = [
  { code: 'zh-CN', name: '简体中文', flag: '🇨🇳' },
  { code: 'en-US', name: 'English', flag: '🇺🇸' }
]