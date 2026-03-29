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
          <!-- 主机名 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">主机名</label>
            <input
              v-model="basicSettings.hostname"
              type="text"
              required
              pattern="[a-zA-Z0-9][a-zA-Z0-9\-]*"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="例如：axis-nas"
            />
            <p class="text-xs text-gray-500 mt-1">只允许字母、数字和连字符，不能以连字符开头</p>
          </div>

          <!-- 时区 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">时区</label>
            <select
              v-model="basicSettings.timezone"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="UTC">UTC</option>
              <option value="Asia/Shanghai">Asia/Shanghai (北京时间)</option>
              <option value="Asia/Tokyo">Asia/Tokyo (东京)</option>
              <option value="Asia/Hong_Kong">Asia/Hong_Kong (香港)</option>
              <option value="Asia/Singapore">Asia/Singapore (新加坡)</option>
              <option value="America/New_York">America/New_York (纽约)</option>
              <option value="America/Los_Angeles">America/Los_Angeles (洛杉矶)</option>
              <option value="Europe/London">Europe/London (伦敦)</option>
              <option value="Europe/Paris">Europe/Paris (巴黎)</option>
            </select>
          </div>

          <!-- 语言 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">系统语言</label>
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

          <!-- 更新通道 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">更新通道</label>
            <select
              v-model="basicSettings.update_channel"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="stable">稳定版 (推荐)</option>
              <option value="beta">测试版</option>
              <option value="nightly">开发版</option>
            </select>
            <p class="text-xs text-gray-500 mt-1">稳定版经过充分测试，测试版包含新功能，开发版可能有不稳定特性</p>
          </div>

          <!-- 自动更新 -->
          <div class="flex items-center justify-between">
            <div>
              <label class="block text-sm font-medium text-gray-700">自动更新</label>
              <p class="text-sm text-gray-500">系统自动安装安全更新</p>
            </div>
            <input
              v-model="basicSettings.auto_update_enabled"
              type="checkbox"
              class="h-5 w-5 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
          </div>

          <div class="flex justify-end">
            <button type="submit" :disabled="saving" class="btn-primary">
              {{ saving ? '保存中...' : '保存设置' }}
            </button>
          </div>
        </form>
      </div>

      <!-- 网络设置 -->
      <div v-else-if="currentTab === 'network'" class="max-w-3xl space-y-6">
        <!-- 网络接口列表 -->
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex justify-between items-center mb-4">
            <h3 class="font-semibold text-gray-900">网络接口</h3>
            <button @click="loadNetworkConfig" class="text-sm text-primary-600 hover:text-primary-700">
              刷新
            </button>
          </div>
          
          <div v-if="networkLoading" class="text-center py-4 text-gray-500">加载中...</div>
          <div v-else-if="networkInterfaces.length === 0" class="text-center py-4 text-gray-500">
            暂无网络接口
          </div>
          <div v-else class="space-y-3">
            <div
              v-for="iface in networkInterfaces"
              :key="iface.id"
              class="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
            >
              <div class="flex items-center space-x-3">
                <div :class="iface.enabled ? 'bg-green-100 text-green-600' : 'bg-gray-100 text-gray-400'" class="w-10 h-10 rounded-lg flex items-center justify-center">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.156-3.905 14.06 0M1.394 9.393c5.583-5.587 14.629-5.587 20.212 0" />
                  </svg>
                </div>
                <div>
                  <p class="font-medium text-gray-900">{{ iface.interface }}</p>
                  <p class="text-sm text-gray-500">{{ iface.ip_address }} {{ iface.dhcp_enabled ? '(DHCP)' : '(静态)' }}</p>
                </div>
              </div>
              <span :class="iface.enabled ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'" class="px-2 py-1 text-xs rounded-full">
                {{ iface.enabled ? '已启用' : '已禁用' }}
              </span>
            </div>
          </div>
        </div>

        <!-- 代理设置 -->
        <div class="bg-white rounded-lg shadow p-4">
          <h3 class="font-semibold text-gray-900 mb-4">代理设置</h3>
          <form @submit.prevent="handleSaveProxy" class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <label class="block text-sm font-medium text-gray-700">启用代理</label>
                <p class="text-sm text-gray-500">通过代理服务器访问外网</p>
              </div>
              <input
                v-model="proxySettings.enabled"
                type="checkbox"
                class="h-5 w-5 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">代理服务器</label>
                <input
                  v-model="proxySettings.server"
                  type="text"
                  :disabled="!proxySettings.enabled"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                  placeholder="例如：proxy.example.com"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">端口</label>
                <input
                  v-model.number="proxySettings.port"
                  type="number"
                  :disabled="!proxySettings.enabled"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                  placeholder="例如：8080"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">不使用代理的地址</label>
              <input
                v-model="proxySettings.no_proxy"
                type="text"
                :disabled="!proxySettings.enabled"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                placeholder="例如：localhost, 192.168.0.0/16"
              />
              <p class="text-xs text-gray-500 mt-1">用逗号分隔多个地址</p>
            </div>

            <div class="flex justify-end">
              <button type="submit" :disabled="saving" class="btn-primary">
                {{ saving ? '保存中...' : '保存代理设置' }}
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- 用户管理 -->
      <div v-else-if="currentTab === 'users'" class="max-w-2xl">
        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex justify-between items-center mb-4">
            <h3 class="font-semibold text-gray-900">用户管理</h3>
            <router-link to="/users" class="btn-primary text-sm">
              进入用户管理
            </router-link>
          </div>
          <p class="text-gray-600 mb-4">
            在用户管理页面可以：
          </p>
          <ul class="list-disc list-inside text-gray-600 space-y-2">
            <li>查看所有用户列表</li>
            <li>添加新用户</li>
            <li>编辑用户信息</li>
            <li>删除用户</li>
            <li>管理用户权限</li>
          </ul>
          <div class="mt-6 p-4 bg-blue-50 rounded-lg">
            <div class="flex items-center space-x-2 text-blue-700">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span class="font-medium">提示</span>
            </div>
            <p class="text-sm text-blue-600 mt-2">用户管理功能已整合到独立的用户管理页面，点击上方按钮跳转。</p>
          </div>
        </div>
      </div>

      <!-- 通知设置 -->
      <div v-else-if="currentTab === 'notification'" class="max-w-2xl space-y-6">
        <!-- 邮件通知 -->
        <div class="bg-white rounded-lg shadow p-4">
          <h3 class="font-semibold text-gray-900 mb-4">邮件通知</h3>
          <form @submit.prevent="handleSaveEmailNotification" class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <label class="block text-sm font-medium text-gray-700">启用邮件通知</label>
                <p class="text-sm text-gray-500">系统事件通过邮件通知管理员</p>
              </div>
              <input
                v-model="emailSettings.enabled"
                type="checkbox"
                class="h-5 w-5 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">SMTP 服务器</label>
              <input
                v-model="emailSettings.smtp_server"
                type="text"
                :disabled="!emailSettings.enabled"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                placeholder="例如：smtp.gmail.com"
              />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">SMTP 端口</label>
                <input
                  v-model.number="emailSettings.smtp_port"
                  type="number"
                  :disabled="!emailSettings.enabled"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                  placeholder="例如：587"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">加密方式</label>
                <select
                  v-model="emailSettings.encryption"
                  :disabled="!emailSettings.enabled"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                >
                  <option value="none">无加密</option>
                  <option value="starttls">STARTTLS</option>
                  <option value="ssl">SSL/TLS</option>
                </select>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">发件人邮箱</label>
                <input
                  v-model="emailSettings.from_email"
                  type="email"
                  :disabled="!emailSettings.enabled"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                  placeholder="例如：nas@example.com"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">管理员邮箱</label>
                <input
                  v-model="emailSettings.admin_email"
                  type="email"
                  :disabled="!emailSettings.enabled"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                  placeholder="例如：admin@example.com"
                />
              </div>
            </div>

            <div class="flex justify-end">
              <button type="submit" :disabled="saving" class="btn-primary">
                {{ saving ? '保存中...' : '保存邮件设置' }}
              </button>
            </div>
          </form>
        </div>

        <!-- Webhook 通知 -->
        <div class="bg-white rounded-lg shadow p-4">
          <h3 class="font-semibold text-gray-900 mb-4">Webhook 通知</h3>
          <form @submit.prevent="handleSaveWebhookNotification" class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <label class="block text-sm font-medium text-gray-700">启用 Webhook</label>
                <p class="text-sm text-gray-500">系统事件通过 Webhook 推送到外部服务</p>
              </div>
              <input
                v-model="webhookSettings.enabled"
                type="checkbox"
                class="h-5 w-5 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Webhook URL</label>
              <input
                v-model="webhookSettings.url"
                type="url"
                :disabled="!webhookSettings.enabled"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                placeholder="https://example.com/webhook"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">密钥 (可选)</label>
              <input
                v-model="webhookSettings.secret"
                type="password"
                :disabled="!webhookSettings.enabled"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100"
                placeholder="用于签名验证"
              />
              <p class="text-xs text-gray-500 mt-1">密钥用于生成请求签名，确保请求来自可信来源</p>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">触发事件</label>
              <div class="space-y-2">
                <label class="flex items-center">
                  <input v-model="webhookSettings.events" type="checkbox" value="system" :disabled="!webhookSettings.enabled" class="h-4 w-4 text-primary-600 rounded" />
                  <span class="ml-2 text-sm text-gray-600">系统事件（启动、关机、更新）</span>
                </label>
                <label class="flex items-center">
                  <input v-model="webhookSettings.events" type="checkbox" value="storage" :disabled="!webhookSettings.enabled" class="h-4 w-4 text-primary-600 rounded" />
                  <span class="ml-2 text-sm text-gray-600">存储事件（磁盘错误、空间不足）</span>
                </label>
                <label class="flex items-center">
                  <input v-model="webhookSettings.events" type="checkbox" value="backup" :disabled="!webhookSettings.enabled" class="h-4 w-4 text-primary-600 rounded" />
                  <span class="ml-2 text-sm text-gray-600">备份事件（完成、失败）</span>
                </label>
                <label class="flex items-center">
                  <input v-model="webhookSettings.events" type="checkbox" value="security" :disabled="!webhookSettings.enabled" class="h-4 w-4 text-primary-600 rounded" />
                  <span class="ml-2 text-sm text-gray-600">安全事件（登录失败、权限变更）</span>
                </label>
              </div>
            </div>

            <div class="flex justify-end">
              <button type="submit" :disabled="saving" class="btn-primary">
                {{ saving ? '保存中...' : '保存 Webhook 设置' }}
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Toast -->
      <div v-if="toast.show" class="fixed bottom-4 right-4 z-50">
        <div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">
          {{ toast.message }}
        </div>
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
  { id: 'users', name: '用户管理' },
  { id: 'notification', name: '通知设置' }
]

const currentTab = ref('basic')
const loading = ref(true)
const saving = ref(false)
const networkLoading = ref(false)

// Toast
const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

// 基本设置
const basicSettings = ref({
  hostname: '',
  timezone: 'Asia/Shanghai',
  language: 'zh-CN',
  update_channel: 'stable',
  auto_update_enabled: true
})

// 网络接口
const networkInterfaces = ref<any[]>([])

// 代理设置
const proxySettings = ref({
  enabled: false,
  server: '',
  port: 8080,
  no_proxy: ''
})

// 邮件设置
const emailSettings = ref({
  enabled: false,
  smtp_server: '',
  smtp_port: 587,
  encryption: 'starttls',
  from_email: '',
  admin_email: ''
})

// Webhook 设置
const webhookSettings = ref({
  enabled: false,
  url: '',
  secret: '',
  events: ['system', 'storage', 'backup'] as string[]
})

// 加载设置
const loadSettings = async () => {
  loading.value = true
  try {
    const response = await api.settings.get()
    const settings = response.data.data || response.data.settings || response.data || {}
    
    // 基本设置
    basicSettings.value = {
      hostname: settings.hostname || '',
      timezone: settings.timezone || 'Asia/Shanghai',
      language: settings.language || 'zh-CN',
      update_channel: settings.update_channel || 'stable',
      auto_update_enabled: settings.auto_update_enabled ?? true
    }
    
    // 代理设置
    proxySettings.value = {
      enabled: settings.proxy_enabled || false,
      server: settings.proxy_server || '',
      port: settings.proxy_port || 8080,
      no_proxy: settings.no_proxy || ''
    }
    
    // 邮件设置
    emailSettings.value = {
      enabled: settings.notification_enabled || false,
      smtp_server: settings.smtp_server || '',
      smtp_port: settings.smtp_port || 587,
      encryption: settings.smtp_encryption || 'starttls',
      from_email: settings.from_email || '',
      admin_email: settings.admin_email || ''
    }
    
    // Webhook 设置
    webhookSettings.value = {
      enabled: settings.webhook_enabled || false,
      url: settings.webhook_url || '',
      secret: settings.webhook_secret || '',
      events: settings.webhook_events || ['system', 'storage', 'backup']
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
    showToast('error', '加载设置失败')
  } finally {
    loading.value = false
  }
}

// 加载网络配置
const loadNetworkConfig = async () => {
  networkLoading.value = true
  try {
    const response = await api.network.listConfig()
    networkInterfaces.value = response.data.data || response.data || []
  } catch (error) {
    console.error('Failed to load network config:', error)
    networkInterfaces.value = []
  } finally {
    networkLoading.value = false
  }
}

// 保存基本设置
const handleSaveBasic = async () => {
  // 验证主机名
  if (basicSettings.value.hostname && !/^[a-zA-Z0-9][a-zA-Z0-9\-]*$/.test(basicSettings.value.hostname)) {
    showToast('error', '主机名格式不正确')
    return
  }

  saving.value = true
  try {
    await api.settings.update({
      hostname: basicSettings.value.hostname,
      timezone: basicSettings.value.timezone,
      language: basicSettings.value.language,
      update_channel: basicSettings.value.update_channel,
      auto_update_enabled: basicSettings.value.auto_update_enabled
    })
    showToast('success', '基本设置已保存')
  } catch (error) {
    console.error('Failed to save basic settings:', error)
    showToast('error', '保存失败')
  } finally {
    saving.value = false
  }
}

// 保存代理设置
const handleSaveProxy = async () => {
  saving.value = true
  try {
    await api.settings.update({
      proxy_enabled: proxySettings.value.enabled,
      proxy_server: proxySettings.value.server,
      proxy_port: proxySettings.value.port,
      no_proxy: proxySettings.value.no_proxy
    })
    showToast('success', '代理设置已保存')
  } catch (error) {
    console.error('Failed to save proxy settings:', error)
    showToast('error', '保存失败')
  } finally {
    saving.value = false
  }
}

// 保存邮件通知设置
const handleSaveEmailNotification = async () => {
  saving.value = true
  try {
    await api.settings.update({
      notification_enabled: emailSettings.value.enabled,
      smtp_server: emailSettings.value.smtp_server,
      smtp_port: emailSettings.value.smtp_port,
      smtp_encryption: emailSettings.value.encryption,
      from_email: emailSettings.value.from_email,
      admin_email: emailSettings.value.admin_email
    })
    showToast('success', '邮件设置已保存')
  } catch (error) {
    console.error('Failed to save email settings:', error)
    showToast('error', '保存失败')
  } finally {
    saving.value = false
  }
}

// 保存 Webhook 设置
const handleSaveWebhookNotification = async () => {
  if (webhookSettings.value.enabled && !webhookSettings.value.url) {
    showToast('error', '请填写 Webhook URL')
    return
  }

  saving.value = true
  try {
    await api.settings.update({
      webhook_enabled: webhookSettings.value.enabled,
      webhook_url: webhookSettings.value.url,
      webhook_secret: webhookSettings.value.secret,
      webhook_events: webhookSettings.value.events
    })
    showToast('success', 'Webhook 设置已保存')
  } catch (error) {
    console.error('Failed to save webhook settings:', error)
    showToast('error', '保存失败')
  } finally {
    saving.value = false
  }
}

// Toast
const showToast = (type: 'success' | 'error', message: string) => {
  toast.value = { show: true, type, message }
  setTimeout(() => { toast.value.show = false }, 3000)
}

// 生命周期
onMounted(() => {
  loadSettings()
  loadNetworkConfig()
})
</script>