import { defineComponent, ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'

interface SystemSettings {
  hostname: string
  timezone: string
  language: string
}

interface NetworkSettings {
  ip: string
  netmask: string
  gateway: string
  dns1: string
  dns2: string
  dhcp: boolean
}

interface NotificationSettings {
  emailEnabled: boolean
  emailServer: string
  emailPort: number
  emailUser: string
  emailPassword: string
  emailFrom: string
  notifyOnBackup: boolean
  notifyOnStorage: boolean
  notifyOnSystem: boolean
}

export default defineComponent({
  name: 'Settings',
  setup() {
    const router = useRouter()
    const loading = ref(false)
    const saving = ref(false)
    const activeTab = ref<'basic' | 'network' | 'notification'>('basic')
    const saveMessage = ref('')
    const saveMessageType = ref<'success' | 'error'>('success')

    // 系统设置
    const systemSettings = ref<SystemSettings>({
      hostname: '',
      timezone: 'Asia/Shanghai',
      language: 'zh-CN'
    })

    // 网络设置
    const networkSettings = ref<NetworkSettings>({
      ip: '192.168.1.100',
      netmask: '255.255.255.0',
      gateway: '192.168.1.1',
      dns1: '8.8.8.8',
      dns2: '8.8.4.4',
      dhcp: false
    })

    // 通知设置
    const notificationSettings = ref<NotificationSettings>({
      emailEnabled: false,
      emailServer: 'smtp.gmail.com',
      emailPort: 587,
      emailUser: '',
      emailPassword: '',
      emailFrom: '',
      notifyOnBackup: true,
      notifyOnStorage: true,
      notifyOnSystem: true
    })

    // 时区列表
    const timezones = [
      { value: 'Asia/Shanghai', label: '上海 (UTC+8)' },
      { value: 'Asia/Hong_Kong', label: '香港 (UTC+8)' },
      { value: 'Asia/Tokyo', label: '东京 (UTC+9)' },
      { value: 'America/New_York', label: '纽约 (UTC-5)' },
      { value: 'America/Los_Angeles', label: '洛杉矶 (UTC-8)' },
      { value: 'Europe/London', label: '伦敦 (UTC+0)' },
      { value: 'Europe/Paris', label: '巴黎 (UTC+1)' },
      { value: 'UTC', label: 'UTC' }
    ]

    // 语言列表
    const languages = [
      { value: 'zh-CN', label: '简体中文' },
      { value: 'zh-TW', label: '繁體中文' },
      { value: 'en-US', label: 'English' },
      { value: 'ja-JP', label: '日本語' }
    ]

    // 显示保存消息
    const showSaveMessage = (message: string, type: 'success' | 'error') => {
      saveMessage.value = message
      saveMessageType.value = type
      setTimeout(() => {
        saveMessage.value = ''
      }, 3000)
    }

    // 加载设置
    const loadSettings = async () => {
      loading.value = true
      try {
        // 加载系统设置
        const systemResponse = await apiClient.get('/settings/system')
        if (systemResponse.data.success) {
          systemSettings.value = {
            hostname: systemResponse.data.data.hostname || '',
            timezone: systemResponse.data.data.timezone || 'Asia/Shanghai',
            language: systemResponse.data.data.language || 'zh-CN'
          }
        }

        // 加载网络设置
        const networkResponse = await apiClient.get('/settings/network')
        if (networkResponse.data.success) {
          networkSettings.value = {
            ip: networkResponse.data.data.ip || '192.168.1.100',
            netmask: networkResponse.data.data.netmask || '255.255.255.0',
            gateway: networkResponse.data.data.gateway || '192.168.1.1',
            dns1: networkResponse.data.data.dns1 || '8.8.8.8',
            dns2: networkResponse.data.data.dns2 || '8.8.4.4',
            dhcp: networkResponse.data.data.dhcp || false
          }
        }

        // 加载通知设置
        const notificationResponse = await apiClient.get('/settings/notification')
        if (notificationResponse.data.success) {
          notificationSettings.value = {
            emailEnabled: notificationResponse.data.data.email_enabled || false,
            emailServer: notificationResponse.data.data.email_server || 'smtp.gmail.com',
            emailPort: notificationResponse.data.data.email_port || 587,
            emailUser: notificationResponse.data.data.email_user || '',
            emailPassword: '',
            emailFrom: notificationResponse.data.data.email_from || '',
            notifyOnBackup: notificationResponse.data.data.notify_on_backup ?? true,
            notifyOnStorage: notificationResponse.data.data.notify_on_storage ?? true,
            notifyOnSystem: notificationResponse.data.data.notify_on_system ?? true
          }
        }
      } catch (error) {
        console.error('Failed to load settings:', error)
        // 使用默认值
      } finally {
        loading.value = false
      }
    }

    // 保存系统设置
    const saveSystemSettings = async () => {
      saving.value = true
      try {
        await apiClient.put('/settings/system', {
          hostname: systemSettings.value.hostname,
          timezone: systemSettings.value.timezone,
          language: systemSettings.value.language
        })
        showSaveMessage('系统设置已保存', 'success')
      } catch (error) {
        console.error('Failed to save system settings:', error)
        showSaveMessage('保存系统设置失败', 'error')
      } finally {
        saving.value = false
      }
    }

    // 保存网络设置
    const saveNetworkSettings = async () => {
      saving.value = true
      try {
        await apiClient.put('/settings/network', {
          ip: networkSettings.value.ip,
          netmask: networkSettings.value.netmask,
          gateway: networkSettings.value.gateway,
          dns1: networkSettings.value.dns1,
          dns2: networkSettings.value.dns2,
          dhcp: networkSettings.value.dhcp
        })
        showSaveMessage('网络设置已保存', 'success')
      } catch (error) {
        console.error('Failed to save network settings:', error)
        showSaveMessage('保存网络设置失败', 'error')
      } finally {
        saving.value = false
      }
    }

    // 保存通知设置
    const saveNotificationSettings = async () => {
      saving.value = true
      try {
        await apiClient.put('/settings/notification', {
          email_enabled: notificationSettings.value.emailEnabled,
          email_server: notificationSettings.value.emailServer,
          email_port: notificationSettings.value.emailPort,
          email_user: notificationSettings.value.emailUser,
          email_password: notificationSettings.value.emailPassword || undefined,
          email_from: notificationSettings.value.emailFrom,
          notify_on_backup: notificationSettings.value.notifyOnBackup,
          notify_on_storage: notificationSettings.value.notifyOnStorage,
          notify_on_system: notificationSettings.value.notifyOnSystem
        })
        showSaveMessage('通知设置已保存', 'success')
      } catch (error) {
        console.error('Failed to save notification settings:', error)
        showSaveMessage('保存通知设置失败', 'error')
      } finally {
        saving.value = false
      }
    }

    // 测试邮件设置
    const testEmailSettings = async () => {
      try {
        await apiClient.post('/settings/notification/test', {
          email_server: notificationSettings.value.emailServer,
          email_port: notificationSettings.value.emailPort,
          email_user: notificationSettings.value.emailUser,
          email_password: notificationSettings.value.emailPassword,
          email_from: notificationSettings.value.emailFrom
        })
        showSaveMessage('邮件发送成功', 'success')
      } catch (error) {
        console.error('Failed to test email settings:', error)
        showSaveMessage('邮件发送失败，请检查设置', 'error')
      }
    }

    onMounted(() => {
      const token = localStorage.getItem('jwt_token')
      if (!token) {
        router.push('/login')
        return
      }
      loadSettings()
    })

    return {
      loading,
      saving,
      activeTab,
      saveMessage,
      saveMessageType,
      systemSettings,
      networkSettings,
      notificationSettings,
      timezones,
      languages,
      showSaveMessage,
      loadSettings,
      saveSystemSettings,
      saveNetworkSettings,
      saveNotificationSettings,
      testEmailSettings
    }
  },
  template: `
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex items-center justify-between">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          系统设置
        </h1>
        <button
          @click="loadSettings"
          class="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
        >
          <svg class="w-5 h-5 mr-2" :class="{ 'animate-spin': loading }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
          </svg>
          刷新
        </button>
      </div>

      <!-- 保存提示 -->
      <div v-if="saveMessage" class="rounded-md p-4" :class="saveMessageType === 'success' ? 'bg-green-50 dark:bg-green-900/20' : 'bg-red-50 dark:bg-red-900/20'">
        <div class="flex">
          <div class="ml-3">
            <h3 class="text-sm font-medium" :class="saveMessageType === 'success' ? 'text-green-800 dark:text-green-200' : 'text-red-800 dark:text-red-200'">
              {{ saveMessage }}
            </h3>
          </div>
        </div>
      </div>

      <!-- 选项卡导航 -->
      <div class="border-b border-gray-200 dark:border-gray-700">
        <nav class="-mb-px flex space-x-8">
          <button
            @click="activeTab = 'basic'"
            class="py-4 px-1 border-b-2 font-medium text-sm"
            :class="activeTab === 'basic' ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400' : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300'"
          >
            <svg class="w-5 h-5 inline-block mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
            </svg>
            基本设置
          </button>
          <button
            @click="activeTab = 'network'"
            class="py-4 px-1 border-b-2 font-medium text-sm"
            :class="activeTab === 'network' ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400' : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300'"
          >
            <svg class="w-5 h-5 inline-block mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"></path>
            </svg>
            网络设置
          </button>
          <button
            @click="activeTab = 'notification'"
            class="py-4 px-1 border-b-2 font-medium text-sm"
            :class="activeTab === 'notification' ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400' : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300'"
          >
            <svg class="w-5 h-5 inline-block mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
            </svg>
            通知设置
          </button>
        </nav>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="px-6 py-12 text-center">
        <svg class="animate-spin h-8 w-8 text-indigo-600 mx-auto" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">加载中...</p>
      </div>

      <!-- 基本设置 -->
      <div v-if="!loading && activeTab === 'basic'" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-6">基本系统设置</h3>
        <div class="space-y-6 max-w-2xl">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">主机名</label>
            <input
              v-model="systemSettings.hostname"
              type="text"
              placeholder="axis-nas"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">设备在网络中的名称</p>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">时区</label>
            <select
              v-model="systemSettings.timezone"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option v-for="tz in timezones" :key="tz.value" :value="tz.value">
                {{ tz.label }}
              </option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">语言</label>
            <select
              v-model="systemSettings.language"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option v-for="lang in languages" :key="lang.value" :value="lang.value">
                {{ lang.label }}
              </option>
            </select>
          </div>

          <div class="pt-4">
            <button
              @click="saveSystemSettings"
              :disabled="saving"
              class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <svg v-if="saving" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              {{ saving ? '保存中...' : '保存设置' }}
            </button>
          </div>
        </div>
      </div>

      <!-- 网络设置 -->
      <div v-if="!loading && activeTab === 'network'" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-6">网络配置</h3>
        <div class="space-y-6 max-w-2xl">
          <div>
            <label class="flex items-center">
              <input
                v-model="networkSettings.dhcp"
                type="checkbox"
                class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">使用 DHCP 自动获取 IP</span>
            </label>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">IP 地址</label>
              <input
                v-model="networkSettings.ip"
                type="text"
                :disabled="networkSettings.dhcp"
                placeholder="192.168.1.100"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">子网掩码</label>
              <input
                v-model="networkSettings.netmask"
                type="text"
                :disabled="networkSettings.dhcp"
                placeholder="255.255.255.0"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">默认网关</label>
            <input
              v-model="networkSettings.gateway"
              type="text"
              :disabled="networkSettings.dhcp"
              placeholder="192.168.1.1"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">首选 DNS</label>
              <input
                v-model="networkSettings.dns1"
                type="text"
                :disabled="networkSettings.dhcp"
                placeholder="8.8.8.8"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">备用 DNS</label>
              <input
                v-model="networkSettings.dns2"
                type="text"
                :disabled="networkSettings.dhcp"
                placeholder="8.8.4.4"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              />
            </div>
          </div>

          <div class="pt-4">
            <button
              @click="saveNetworkSettings"
              :disabled="saving"
              class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <svg v-if="saving" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              {{ saving ? '保存中...' : '保存设置' }}
            </button>
          </div>
        </div>
      </div>

      <!-- 通知设置 -->
      <div v-if="!loading && activeTab === 'notification'" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-6">通知配置</h3>
        <div class="space-y-6 max-w-2xl">
          <div>
            <label class="flex items-center">
              <input
                v-model="notificationSettings.emailEnabled"
                type="checkbox"
                class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
              />
              <span class="ml-2 text-sm font-medium text-gray-700 dark:text-gray-300">启用邮件通知</span>
            </label>
          </div>

          <div class="grid grid-cols-3 gap-4">
            <div class="col-span-2">
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">SMTP 服务器</label>
              <input
                v-model="notificationSettings.emailServer"
                type="text"
                :disabled="!notificationSettings.emailEnabled"
                placeholder="smtp.gmail.com"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">端口</label>
              <input
                v-model="notificationSettings.emailPort"
                type="number"
                :disabled="!notificationSettings.emailEnabled"
                placeholder="587"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">用户名</label>
              <input
                v-model="notificationSettings.emailUser"
                type="text"
                :disabled="!notificationSettings.emailEnabled"
                placeholder="your@email.com"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">密码</label>
              <input
                v-model="notificationSettings.emailPassword"
                type="password"
                :disabled="!notificationSettings.emailEnabled"
                placeholder="••••••••"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">发件人邮箱</label>
            <input
              v-model="notificationSettings.emailFrom"
              type="email"
              :disabled="!notificationSettings.emailEnabled"
              placeholder="noreply@axis.com"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            />
          </div>

          <div class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <h4 class="text-sm font-medium text-gray-900 dark:text-white mb-3">通知类型</h4>
            <div class="space-y-2">
              <label class="flex items-center">
                <input
                  v-model="notificationSettings.notifyOnBackup"
                  type="checkbox"
                  :disabled="!notificationSettings.emailEnabled"
                  class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded disabled:opacity-50"
                />
                <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">备份完成/失败通知</span>
              </label>
              <label class="flex items-center">
                <input
                  v-model="notificationSettings.notifyOnStorage"
                  type="checkbox"
                  :disabled="!notificationSettings.emailEnabled"
                  class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded disabled:opacity-50"
                />
                <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">存储空间不足通知</span>
              </label>
              <label class="flex items-center">
                <input
                  v-model="notificationSettings.notifyOnSystem"
                  type="checkbox"
                  :disabled="!notificationSettings.emailEnabled"
                  class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded disabled:opacity-50"
                />
                <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">系统事件通知</span>
              </label>
            </div>
          </div>

          <div class="pt-4 flex space-x-3">
            <button
              @click="saveNotificationSettings"
              :disabled="saving"
              class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <svg v-if="saving" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              {{ saving ? '保存中...' : '保存设置' }}
            </button>
            <button
              @click="testEmailSettings"
              :disabled="!notificationSettings.emailEnabled || saving"
              class="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              发送测试邮件
            </button>
          </div>
        </div>
      </div>
    </div>
  `
})
