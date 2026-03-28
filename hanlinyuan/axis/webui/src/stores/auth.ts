import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/utils/api'

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('jwt_token'))
  const user = ref<any>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const isAuthenticated = computed(() => !!token.value)

  async function login(username: string, password: string) {
    loading.value = true
    error.value = null
    try {
      const response = await api.auth.login(username, password)
      token.value = response.data.token
      localStorage.setItem('jwt_token', token.value)
      await fetchUser()
      return true
    } catch (e: any) {
      error.value = e.response?.data?.message || '登录失败'
      return false
    } finally {
      loading.value = false
    }
  }

  async function fetchUser() {
    try {
      // TODO: 实现获取当前用户信息的 API
      // const response = await api.auth.me()
      // user.value = response.data
    } catch (e) {
      console.error('Failed to fetch user:', e)
    }
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('jwt_token')
  }

  return {
    token,
    user,
    loading,
    error,
    isAuthenticated,
    login,
    logout,
    fetchUser
  }
})
