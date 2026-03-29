<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div>
        <h1 class="text-2xl font-bold text-gray-900">系统设置</h1>
        <p class="text-gray-600 mt-1">配置系统参数和首选项</p>
      </div>

      <!-- 选项卡 -->
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="currentTab = tab.id"
            :class="[
              currentTab === tab.id
                ? 'border-primary-500 text-primary-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300',
              'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm'
            ]"
          >
            {{ tab.name }}
          </button>
        </nav>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="flex justify-center items-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
        <span class="ml-3 text-gray-600">加载中...</span>
      </div>

      <!-- 基本设置 -->
      <div v-else-if="currentTab === 'basic'" class="max-w-2xl">
        <form @submit.prevent="handleSaveBasic" class="space-y-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">主机名</label>
            <input
              v-model="basicSettings.hostname"
              type="text"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="例如：axis-nas"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">时区</label>
            <select
              v-model="basicSettings.timezone"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="UTC">UTC</option>
              <option value="Asia/Shanghai">Asia/Shanghai (北京时间)</option>
              <option value="Asia/Tokyo">Asia/Tokyo</option>
              <option value="America/New_York">America/New_York</option>
              <option value="Europe/London">Europe/London</option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">语言</label>
            <select
              v-model="basicSettings.language"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="zh-CN">简体中文</option>
              <option value="zh-TW">繁體中文</option>
              <option value="en-US">English (US)</option>
              <option value="ja-JP">日本語</option>
            </select>
          </div>

          <div class="flex justify-end">
            <button type="submit" :disabled="saving" class="btn-primary">
              {{ saving ? '保存中...' : '保存设置' }}
            </button>
          </div>
        </form>
      </div>

      <!-- 网络设置 -->
      <div v-else-if="currentTab === 'network'" class="max-w-2xl">
        <form @submit.prevent="handleSaveNetwork" class="space-y-6">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">IP 地址</label>
              <input
                v-model="networkSettings.ip_address"
                type="text"
                required
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                placeholder="例如：192.168.1.100"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">子网掩码</label>
              <input
                v-model="networkSettings.netmask"
                type="text"
                required
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                placeholder="例如：255.255.255.0"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">网关</label>
            <input
              v-model="networkSettings.gateway"
              type="text"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="例如：192.168.1.1"
            />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">首选 DNS</label>
              <input
                v-model="networkSettings.dns_primary"
                type="text"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                placeholder="例如：8.8.8.8"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">备用 DNS</label>
              <input
                v-model="networkSettings.dns_secondary"
                type="text"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                placeholder="例如：8.8.4.4"
              />
            </div>
          </div>

          <div class="flex justify-end">
            <button type="submit" :disabled="saving" class="btn-primary">
              {{ saving ? '保存中...' : '保存设置' }}
            </button>
          </div>
        </form>
      </div>

      <!-- 通知设置 -->
      <div v-else-if="currentTab === 'notification'" class="max-w-2xl">
        <form @submit.prevent="handleSaveNotification" class="space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <label class="block text-sm font-medium text-gray-700">启用邮件通知</label>
              <p class="text-sm text-gray-500">系统事件通过邮件通知管理员</p>
            </div>
            <input
              v-model="notificationSettings.enabled"
              type="checkbox"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">SMTP 服务器</label>
            <input
              v-model="notificationSettings.smtp_server"
              type="text"
              :disabled="!notificationSettings.enabled"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
              placeholder="例如：smtp.gmail.com"
            />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">SMTP 端口</label>
              <input
                v-model="notificationSettings.smtp_port"
                type="number"
                :disabled="!notificationSettings.enabled"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                placeholder="例如：587"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">发件人邮箱</label>
              <input
                v-model="notificationSettings.from_email"
                type="email"
                :disabled="!notificationSettings.enabled"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                placeholder="例如：nas@example.com"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">管理员邮箱</label>
            <input
              v-model="notificationSettings.admin_email"
              type="email"
              :disabled="!notificationSettings.enabled"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
              placeholder="例如：admin@example.com"
            />
          </div>

          <div class="flex justify-end">
            <button type="submit" :disabled="saving" class="btn-primary">
              {{ saving ? '保存中...' : '保存设置' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import { api } from '@/utils/api'

// 选项卡
const tabs = [
  { id: 'basic', name: '基本设置' },
  { id: 'network', name: '网络设置' },
  { id: 'notification', name: '通知设置' }
]

const currentTab = ref('basic')
const loading = ref(true)
const saving = ref(false)

// 设置数据
const basicSettings = ref({
  hostname: '',
  timezone: 'Asia/Shanghai',
  language: 'zh-CN'
})

const networkSettings = ref({
  ip_address: '',
  netmask: '',
  gateway: '',
  dns_primary: '',
  dns_secondary: ''
})

const notificationSettings = ref({
  enabled: false,
  smtp_server: '',
  smtp_port: 587,
  from_email: '',
  admin_email: ''
})

// 加载设置
const loadSettings = async () => {
  loading.value = true
  try {
    const response = await api.settings.get()
    const settings = response.data.settings || {}
    
    // 基本设置
    basicSettings.value = {
      hostname: settings.hostname || '',
      timezone: settings.timezone || 'Asia/Shanghai',
      language: settings.language || 'zh-CN'
    }
    
    // 网络设置
    networkSettings.value = {
      ip_address: settings.ip_address || '',
      netmask: settings.netmask || '',
      gateway: settings.gateway || '',
      dns_primary: settings.dns_primary || '',
      dns_secondary: settings.dns_secondary || ''
    }
    
    // 通知设置
    notificationSettings.value = {
      enabled: settings.notification_enabled || false,
      smtp_server: settings.smtp_server || '',
      smtp_port: settings.smtp_port || 587,
      from_email: settings.from_email || '',
      admin_email: settings.admin_email || ''
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
    alert('加载设置失败')
  } finally {
    loading.value = false
  }
}

// 保存基本设置
const handleSaveBasic = async () => {
  saving.value = true
  try {
    await api.settings.update({
      hostname: basicSettings.value.hostname,
      timezone: basicSettings.value.timezone,
      language: basicSettings.value.language
    })
    alert('基本设置已保存')
  } catch (error) {
    console.error('Failed to save basic settings:', error)
    alert('保存失败')
  } finally {
    saving.value = false
  }
}

// 保存网络设置
const handleSaveNetwork = async () => {
  saving.value = true
  try {
    await api.settings.update({
      ip_address: networkSettings.value.ip_address,
      netmask: networkSettings.value.netmask,
      gateway: networkSettings.value.gateway,
      dns_primary: networkSettings.value.dns_primary,
      dns_secondary: networkSettings.value.dns_secondary
    })
    alert('网络设置已保存')
  } catch (error) {
    console.error('Failed to save network settings:', error)
    alert('保存失败')
  } finally {
    saving.value = false
  }
}

// 保存通知设置
const handleSaveNotification = async () => {
  saving.value = true
  try {
    await api.settings.update({
      notification_enabled: notificationSettings.value.enabled,
      smtp_server: notificationSettings.value.smtp_server,
      smtp_port: notificationSettings.value.smtp_port,
      from_email: notificationSettings.value.from_email,
      admin_email: notificationSettings.value.admin_email
    })
    alert('通知设置已保存')
  } catch (error) {
    console.error('Failed to save notification settings:', error)
    alert('保存失败')
  } finally {
    saving.value = false
  }
}

// 生命周期
onMounted(() => {
  loadSettings()
})
</script>
