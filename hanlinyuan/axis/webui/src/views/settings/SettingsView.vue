<template>
  <div class="px-4 py-6 sm:px-0">
    <!-- Page Header -->
    <div class="mb-6">
      <h2 class="text-2xl font-bold text-gray-900">系统设置</h2>
      <p class="mt-1 text-sm text-gray-500">配置系统参数和首选项</p>
    </div>

    <!-- Settings Tabs -->
    <div class="bg-white shadow rounded-lg">
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8 px-6" aria-label="Tabs">
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

      <!-- Tab Content -->
      <div class="p-6">
        <!-- Basic Settings -->
        <div v-show="currentTab === 'basic'" class="space-y-6">
          <div>
            <label for="hostname" class="block text-sm font-medium text-gray-700">主机名</label>
            <input
              v-model="settings.hostname"
              type="text"
              id="hostname"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              placeholder="nas-server"
            />
          </div>

          <div>
            <label for="timezone" class="block text-sm font-medium text-gray-700">时区</label>
            <select
              v-model="settings.timezone"
              id="timezone"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
            >
              <option value="Asia/Shanghai">Asia/Shanghai (UTC+8)</option>
              <option value="UTC">UTC</option>
              <option value="America/New_York">America/New_York</option>
              <option value="Europe/London">Europe/London</option>
            </select>
          </div>

          <div>
            <label for="language" class="block text-sm font-medium text-gray-700">语言</label>
            <select
              v-model="settings.language"
              id="language"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
            >
              <option value="zh-CN">简体中文</option>
              <option value="en-US">English (US)</option>
              <option value="zh-TW">繁體中文</option>
            </select>
          </div>
        </div>

        <!-- Network Settings -->
        <div v-show="currentTab === 'network'" class="space-y-6">
          <div>
            <label for="ipAddress" class="block text-sm font-medium text-gray-700">IP 地址</label>
            <input
              v-model="network.ipAddress"
              type="text"
              id="ipAddress"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              placeholder="192.168.1.100"
            />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label for="subnetMask" class="block text-sm font-medium text-gray-700">子网掩码</label>
              <input
                v-model="network.subnetMask"
                type="text"
                id="subnetMask"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
                placeholder="255.255.255.0"
              />
            </div>

            <div>
              <label for="gateway" class="block text-sm font-medium text-gray-700">网关</label>
              <input
                v-model="network.gateway"
                type="text"
                id="gateway"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
                placeholder="192.168.1.1"
              />
            </div>
          </div>

          <div>
            <label for="dns" class="block text-sm font-medium text-gray-700">DNS 服务器</label>
            <input
              v-model="network.dns"
              type="text"
              id="dns"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              placeholder="8.8.8.8, 8.8.4.4"
            />
          </div>
        </div>

        <!-- Notification Settings -->
        <div v-show="currentTab === 'notifications'" class="space-y-6">
          <div>
            <label for="smtpServer" class="block text-sm font-medium text-gray-700">SMTP 服务器</label>
            <input
              v-model="notifications.smtpServer"
              type="text"
              id="smtpServer"
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              placeholder="smtp.example.com"
            />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label for="smtpPort" class="block text-sm font-medium text-gray-700">SMTP 端口</label>
              <input
                v-model="notifications.smtpPort"
                type="number"
                id="smtpPort"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
                placeholder="587"
              />
            </div>

            <div>
              <label for="smtpUser" class="block text-sm font-medium text-gray-700">SMTP 用户名</label>
              <input
                v-model="notifications.smtpUser"
                type="text"
                id="smtpUser"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
          </div>

          <div class="space-y-4">
            <div class="flex items-center">
              <input
                v-model="notifications.emailNotifications"
                type="checkbox"
                id="emailNotifications"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="emailNotifications" class="ml-2 block text-sm text-gray-900">
                启用邮件通知
              </label>
            </div>

            <div class="flex items-center">
              <input
                v-model="notifications.systemAlerts"
                type="checkbox"
                id="systemAlerts"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="systemAlerts" class="ml-2 block text-sm text-gray-900">
                系统告警通知
              </label>
            </div>

            <div class="flex items-center">
              <input
                v-model="notifications.backupReminders"
                type="checkbox"
                id="backupReminders"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="backupReminders" class="ml-2 block text-sm text-gray-900">
                备份提醒
              </label>
            </div>
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="px-6 py-4 bg-gray-50 border-t border-gray-200 flex justify-end space-x-3">
        <button
          @click="handleReset"
          type="button"
          class="px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
        >
          重置
        </button>
        <button
          @click="handleSave"
          type="button"
          class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
        >
          保存设置
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'

const currentTab = ref('basic')

const tabs = [
  { id: 'basic', name: '基本设置' },
  { id: 'network', name: '网络设置' },
  { id: 'notifications', name: '通知设置' }
]

const settings = reactive({
  hostname: 'nas-server',
  timezone: 'Asia/Shanghai',
  language: 'zh-CN'
})

const network = reactive({
  ipAddress: '192.168.1.100',
  subnetMask: '255.255.255.0',
  gateway: '192.168.1.1',
  dns: '8.8.8.8, 8.8.4.4'
})

const notifications = reactive({
  smtpServer: 'smtp.example.com',
  smtpPort: 587,
  smtpUser: '',
  emailNotifications: true,
  systemAlerts: true,
  backupReminders: true
})

const handleSave = () => {
  // TODO: Call API to save settings
  alert('设置已保存')
}

const handleReset = () => {
  // TODO: Reset to default values
  if (confirm('确定重置为默认设置？')) {
    settings.hostname = 'nas-server'
    settings.timezone = 'Asia/Shanghai'
    settings.language = 'zh-CN'
    alert('设置已重置')
  }
}
</script>

<style scoped>
/* Settings view specific styles */
</style>
