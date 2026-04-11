<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
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

    <!-- 系统信息卡片 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">系统信息</h3>
      <dl class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">系统名称</dt>
          <dd class="mt-1 text-sm text-gray-900 dark:text-white">Axis NAS</dd>
        </div>
        <div>
          <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">版本号</dt>
          <dd class="mt-1 text-sm text-gray-900 dark:text-white">v1.0.0</dd>
        </div>
        <div>
          <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">运行时间</dt>
          <dd class="mt-1 text-sm text-gray-900 dark:text-white">{{ systemInfo.uptime }}</dd>
        </div>
        <div>
          <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">主机名</dt>
          <dd class="mt-1 text-sm text-gray-900 dark:text-white">{{ settings.hostname }}</dd>
        </div>
      </dl>
    </div>

    <!-- 基本设置 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-6">基本设置</h3>
      <div class="space-y-6 max-w-2xl">
        <!-- NAS 名称 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">NAS 名称</label>
          <input
            v-model="settings.nasName"
            type="text"
            placeholder="My NAS"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          />
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">设备显示名称</p>
        </div>

        <!-- 主机名 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">主机名</label>
          <input
            v-model="settings.hostname"
            type="text"
            placeholder="axis-nas"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          />
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">设备在网络中的名称</p>
        </div>

        <!-- 时区 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">时区</label>
          <select
            v-model="settings.timezone"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          >
            <option v-for="tz in timezones" :key="tz.value" :value="tz.value">
              {{ tz.label }}
            </option>
          </select>
        </div>

        <!-- 语言 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">语言</label>
          <select
            v-model="settings.language"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          >
            <option v-for="lang in languages" :key="lang.value" :value="lang.value">
              {{ lang.label }}
            </option>
          </select>
        </div>

        <!-- 自动休眠时间 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">自动休眠时间</label>
          <select
            v-model="settings.autoSleep"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          >
            <option v-for="sleep in autoSleepOptions" :key="sleep.value" :value="sleep.value">
              {{ sleep.label }}
            </option>
          </select>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">无操作后自动进入休眠模式的时间</p>
        </div>

        <!-- 保存按钮 -->
        <div class="pt-4 flex space-x-3">
          <button
            @click="saveSettings"
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

    <!-- 启动项管理 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-6">启动项管理</h3>
      <div class="space-y-4 max-w-2xl">
        <div class="flex items-center justify-between p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <div class="flex-1">
            <h4 class="text-sm font-medium text-gray-900 dark:text-white">SMB 服务</h4>
            <p class="text-xs text-gray-500 dark:text-gray-400">开机自动启动 SMB 文件共享服务</p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              v-model="settings.autoStartSmb"
              type="checkbox"
              class="sr-only peer"
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
          </label>
        </div>

        <div class="flex items-center justify-between p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <div class="flex-1">
            <h4 class="text-sm font-medium text-gray-900 dark:text-white">NFS 服务</h4>
            <p class="text-xs text-gray-500 dark:text-gray-400">开机自动启动 NFS 文件共享服务</p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              v-model="settings.autoStartNfs"
              type="checkbox"
              class="sr-only peer"
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
          </label>
        </div>

        <div class="flex items-center justify-between p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <div class="flex-1">
            <h4 class="text-sm font-medium text-gray-900 dark:text-white">DLNA 服务</h4>
            <p class="text-xs text-gray-500 dark:text-gray-400">开机自动启动 DLNA 媒体服务</p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              v-model="settings.autoStartDlna"
              type="checkbox"
              class="sr-only peer"
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
          </label>
        </div>

        <div class="flex items-center justify-between p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <div class="flex-1">
            <h4 class="text-sm font-medium text-gray-900 dark:text-white">备份服务</h4>
            <p class="text-xs text-gray-500 dark:text-gray-400">开机自动启动定时备份服务</p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              v-model="settings.autoStartBackup"
              type="checkbox"
              class="sr-only peer"
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
          </label>
        </div>
      </div>
    </div>

    <!-- 电源管理 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-6">电源管理</h3>
      <div class="space-y-4">
        <div class="flex items-center justify-between p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <div class="flex items-center justify-center h-12 w-12 rounded-md bg-yellow-500 text-white">
                <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                </svg>
              </div>
            </div>
            <div class="ml-4">
              <h4 class="text-sm font-medium text-gray-900 dark:text-white">重启系统</h4>
              <p class="text-xs text-gray-500 dark:text-gray-400">重启 NAS 系统，可能需要几分钟</p>
            </div>
          </div>
          <button
            @click="showRestartConfirm = true"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-yellow-600 hover:bg-yellow-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-yellow-500"
          >
            重启
          </button>
        </div>

        <div class="flex items-center justify-between p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <div class="flex items-center justify-center h-12 w-12 rounded-md bg-red-500 text-white">
                <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"></path>
                </svg>
              </div>
            </div>
            <div class="ml-4">
              <h4 class="text-sm font-medium text-gray-900 dark:text-white">关闭系统</h4>
              <p class="text-xs text-gray-500 dark:text-gray-400">安全关闭 NAS 系统</p>
            </div>
          </div>
          <button
            @click="showShutdownConfirm = true"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
          >
            关机
          </button>
        </div>
      </div>
    </div>

    <!-- 网络配置入口 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">网络配置</h3>
      <router-link
        to="/settings/network"
        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
      >
        配置网络
        <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path>
        </svg>
      </router-link>
    </div>

    <!-- 重启确认模态框 -->
    <div v-if="showRestartConfirm" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">确认重启</h3>
        </div>
        <div class="px-6 py-4">
          <p class="text-sm text-gray-700 dark:text-gray-300">
            确定要重启系统吗？
          </p>
          <p class="text-xs text-yellow-600 dark:text-yellow-400 mt-2">
            ⚠️ 重启过程中服务将暂时不可用
          </p>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="showRestartConfirm = false"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="restartSystem"
            :disabled="restarting"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-yellow-600 hover:bg-yellow-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ restarting ? '重启中...' : '确认重启' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 关机确认模态框 -->
    <div v-if="showShutdownConfirm" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">确认关机</h3>
        </div>
        <div class="px-6 py-4">
          <p class="text-sm text-gray-700 dark:text-gray-300">
            确定要关闭系统吗？
          </p>
          <p class="text-xs text-red-600 dark:text-red-400 mt-2">
            ⚠️ 关机后需要手动启动电源
          </p>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="showShutdownConfirm = false"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="shutdownSystem"
            :disabled="shuttingDown"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ shuttingDown ? '关机中...' : '确认关机' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'

const router = useRouter()
const loading = ref(false)
const saving = ref(false)
const restarting = ref(false)
const shuttingDown = ref(false)
const saveMessage = ref('')
const saveMessageType = ref<'success' | 'error'>('success')
const showRestartConfirm = ref(false)
const showShutdownConfirm = ref(false)

const systemInfo = ref({
  uptime: '--',
  hostname: '--'
})

const settings = ref({
  nasName: 'My NAS',
  hostname: '',
  timezone: 'Asia/Shanghai',
  language: 'zh-CN',
  autoSleep: 'never',
  autoStartSmb: true,
  autoStartNfs: false,
  autoStartDlna: false,
  autoStartBackup: true
})

const autoSleepOptions = [
  { value: '10min', label: '10 分钟' },
  { value: '30min', label: '30 分钟' },
  { value: '1h', label: '1 小时' },
  { value: 'never', label: '从不' }
]

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

const languages = [
  { value: 'zh-CN', label: '简体中文' },
  { value: 'zh-TW', label: '繁體中文' },
  { value: 'en-US', label: 'English' },
  { value: 'ja-JP', label: '日本語' }
]

const showSaveMessage = (message: string, type: 'success' | 'error') => {
  saveMessage.value = message
  saveMessageType.value = type
  setTimeout(() => {
    saveMessage.value = ''
  }, 3000)
}

const loadSettings = async () => {
  loading.value = true
  try {
    const response = await apiClient.get('/system/settings')
    if (response.data.success) {
      settings.value = {
        nasName: response.data.data.nasName || 'My NAS',
        hostname: response.data.data.hostname || '',
        timezone: response.data.data.timezone || 'Asia/Shanghai',
        language: response.data.data.language || 'zh-CN',
        autoSleep: response.data.data.autoSleep || 'never',
        autoStartSmb: response.data.data.autoStartSmb ?? true,
        autoStartNfs: response.data.data.autoStartNfs ?? false,
        autoStartDlna: response.data.data.autoStartDlna ?? false,
        autoStartBackup: response.data.data.autoStartBackup ?? true
      }
      systemInfo.value = {
        uptime: response.data.data.uptime || '--',
        hostname: response.data.data.hostname || '--'
      }
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
    // 使用默认值
    settings.value = {
      nasName: 'My NAS',
      hostname: 'axis-nas',
      timezone: 'Asia/Shanghai',
      language: 'zh-CN',
      autoSleep: 'never',
      autoStartSmb: true,
      autoStartNfs: false,
      autoStartDlna: false,
      autoStartBackup: true
    }
    systemInfo.value = {
      uptime: '--',
      hostname: 'axis-nas'
    }
  } finally {
    loading.value = false
  }
}

const saveSettings = async () => {
  saving.value = true
  try {
    await apiClient.put('/system/settings', {
      nasName: settings.value.nasName,
      hostname: settings.value.hostname,
      timezone: settings.value.timezone,
      language: settings.value.language,
      autoSleep: settings.value.autoSleep,
      autoStartSmb: settings.value.autoStartSmb,
      autoStartNfs: settings.value.autoStartNfs,
      autoStartDlna: settings.value.autoStartDlna,
      autoStartBackup: settings.value.autoStartBackup
    })
    showSaveMessage('系统设置已保存', 'success')
    loadSettings()
  } catch (error) {
    console.error('Failed to save settings:', error)
    showSaveMessage('保存系统设置失败', 'error')
  } finally {
    saving.value = false
  }
}

const restartSystem = async () => {
  restarting.value = true
  try {
    await apiClient.post('/system/restart')
    showSaveMessage('系统正在重启...', 'success')
    showRestartConfirm.value = false
    setTimeout(() => {
      router.push('/login')
    }, 3000)
  } catch (error) {
    console.error('Failed to restart system:', error)
    showSaveMessage('重启系统失败', 'error')
  } finally {
    restarting.value = false
  }
}

const shutdownSystem = async () => {
  shuttingDown.value = true
  try {
    await apiClient.post('/system/shutdown')
    showSaveMessage('系统正在关闭...', 'success')
    showShutdownConfirm.value = false
    setTimeout(() => {
      router.push('/login')
    }, 3000)
  } catch (error) {
    console.error('Failed to shutdown system:', error)
    showSaveMessage('关闭系统失败', 'error')
  } finally {
    shuttingDown.value = false
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
</script>
