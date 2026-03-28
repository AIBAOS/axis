<template>
  <div class="px-4 py-6 sm:px-0">
    <!-- 页面标题 -->
    <div class="mb-6">
      <h2 class="text-2xl font-bold text-gray-900">⚙️ 系统设置</h2>
      <p class="text-gray-600 mt-1">配置系统参数和网络设置</p>
    </div>

    <!-- 标签页切换 -->
    <div class="mb-6">
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
    </div>

    <!-- 基本设置 -->
    <div v-if="currentTab === 'basic'" class="space-y-6">
      <div class="bg-white shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">基本设置</h3>
          
          <form @submit.prevent="saveBasicSettings" class="space-y-6">
            <!-- 主机名 -->
            <div>
              <label for="hostname" class="block text-sm font-medium text-gray-700 mb-2">
                主机名
              </label>
              <input
                v-model="basicSettings.hostname"
                type="text"
                id="hostname"
                required
                pattern="^[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?$"
                class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                placeholder="axis-nas"
              />
              <p class="mt-1 text-sm text-gray-500">设备在网络中的名称，仅允许字母、数字和连字符</p>
            </div>

            <!-- 时区 -->
            <div>
              <label for="timezone" class="block text-sm font-medium text-gray-700 mb-2">
                时区
              </label>
              <select
                v-model="basicSettings.timezone"
                id="timezone"
                class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm rounded-md"
              >
                <option value="UTC">UTC (协调世界时)</option>
                <option value="Asia/Shanghai">Asia/Shanghai (北京时间)</option>
                <option value="Asia/Hong_Kong">Asia/Hong_Kong (香港时间)</option>
                <option value="Asia/Tokyo">Asia/Tokyo (日本时间)</option>
                <option value="America/New_York">America/New_York (美东时间)</option>
                <option value="America/Los_Angeles">America/Los_Angeles (美西时间)</option>
                <option value="Europe/London">Europe/London (伦敦时间)</option>
                <option value="Europe/Berlin">Europe/Berlin (柏林时间)</option>
              </select>
            </div>

            <!-- 语言 -->
            <div>
              <label for="language" class="block text-sm font-medium text-gray-700 mb-2">
                系统语言
              </label>
              <select
                v-model="basicSettings.language"
                id="language"
                class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm rounded-md"
              >
                <option value="zh-CN">简体中文</option>
                <option value="zh-TW">繁體中文</option>
                <option value="en-US">English (US)</option>
                <option value="ja-JP">日本語</option>
                <option value="ko-KR">한국어</option>
              </select>
            </div>

            <!-- 保存按钮 -->
            <div class="flex items-center justify-end space-x-3 pt-4 border-t border-gray-200">
              <button
                type="button"
                @click="resetBasicSettings"
                class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                重置
              </button>
              <button
                type="submit"
                :disabled="saving"
                class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
              >
                {{ saving ? '保存中...' : '💾 保存设置' }}
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- 系统信息 -->
      <div class="bg-white shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">系统信息</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="flex justify-between py-2 border-b border-gray-100">
              <span class="text-gray-500">系统版本</span>
              <span class="font-medium">v{{ systemInfo.version }}</span>
            </div>
            <div class="flex justify-between py-2 border-b border-gray-100">
              <span class="text-gray-500">运行时间</span>
              <span class="font-medium">{{ formatUptime(systemInfo.uptime) }}</span>
            </div>
            <div class="flex justify-between py-2 border-b border-gray-100">
              <span class="text-gray-500">设备型号</span>
              <span class="font-medium">{{ systemInfo.model }}</span>
            </div>
            <div class="flex justify-between py-2 border-b border-gray-100">
              <span class="text-gray-500">序列号</span>
              <span class="font-medium">{{ systemInfo.serial }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 网络设置 -->
    <div v-if="currentTab === 'network'" class="space-y-6">
      <div class="bg-white shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">网络配置</h3>
          
          <form @submit.prevent="saveNetworkSettings" class="space-y-6">
            <!-- IP 获取方式 -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                IP 地址获取方式
              </label>
              <div class="flex items-center space-x-4">
                <label class="flex items-center">
                  <input
                    v-model="networkSettings.dhcp_enabled"
                    type="radio"
                    :value="true"
                    name="dhcp"
                    class="focus:ring-primary-500 h-4 w-4 text-primary-600 border-gray-300"
                  />
                  <span class="ml-2 text-sm text-gray-700">DHCP (自动获取)</span>
                </label>
                <label class="flex items-center">
                  <input
                    v-model="networkSettings.dhcp_enabled"
                    type="radio"
                    :value="false"
                    name="dhcp"
                    class="focus:ring-primary-500 h-4 w-4 text-primary-600 border-gray-300"
                  />
                  <span class="ml-2 text-sm text-gray-700">静态 IP (手动配置)</span>
                </label>
              </div>
            </div>

            <!-- 静态 IP 配置 -->
            <div v-if="!networkSettings.dhcp_enabled" class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label for="ip_address" class="block text-sm font-medium text-gray-700 mb-2">
                  IP 地址
                </label>
                <input
                  v-model="networkSettings.ip_address"
                  type="text"
                  id="ip_address"
                  pattern="^(\d{1,3}\.){3}\d{1,3}$"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="192.168.1.100"
                />
              </div>

              <div>
                <label for="netmask" class="block text-sm font-medium text-gray-700 mb-2">
                  子网掩码
                </label>
                <input
                  v-model="networkSettings.netmask"
                  type="text"
                  id="netmask"
                  pattern="^(\d{1,3}\.){3}\d{1,3}$"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="255.255.255.0"
                />
              </div>

              <div>
                <label for="gateway" class="block text-sm font-medium text-gray-700 mb-2">
                  网关
                </label>
                <input
                  v-model="networkSettings.gateway"
                  type="text"
                  id="gateway"
                  pattern="^(\d{1,3}\.){3}\d{1,3}$"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="192.168.1.1"
                />
              </div>

              <div>
                <label for="dns_primary" class="block text-sm font-medium text-gray-700 mb-2">
                  首选 DNS
                </label>
                <input
                  v-model="networkSettings.dns_primary"
                  type="text"
                  id="dns_primary"
                  pattern="^(\d{1,3}\.){3}\d{1,3}$"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="8.8.8.8"
                />
              </div>

              <div>
                <label for="dns_secondary" class="block text-sm font-medium text-gray-700 mb-2">
                  备用 DNS
                </label>
                <input
                  v-model="networkSettings.dns_secondary"
                  type="text"
                  id="dns_secondary"
                  pattern="^(\d{1,3}\.){3}\d{1,3}$"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="8.8.4.4"
                />
              </div>
            </div>

            <!-- 当前网络信息 -->
            <div class="bg-gray-50 rounded-md p-4">
              <h4 class="text-sm font-medium text-gray-700 mb-3">当前网络状态</h4>
              <div class="grid grid-cols-2 gap-3 text-sm">
                <div>
                  <span class="text-gray-500">MAC 地址:</span>
                  <span class="ml-2 font-medium">{{ networkInfo.mac_address }}</span>
                </div>
                <div>
                  <span class="text-gray-500">连接状态:</span>
                  <span :class="networkInfo.connected ? 'text-green-600' : 'text-red-600'" class="ml-2 font-medium">
                    {{ networkInfo.connected ? '已连接' : '未连接' }}
                  </span>
                </div>
                <div>
                  <span class="text-gray-500">当前 IP:</span>
                  <span class="ml-2 font-medium">{{ networkInfo.current_ip || '-' }}</span>
                </div>
                <div>
                  <span class="text-gray-500">网络速度:</span>
                  <span class="ml-2 font-medium">{{ networkInfo.link_speed }} Mbps</span>
                </div>
              </div>
            </div>

            <!-- 保存按钮 -->
            <div class="flex items-center justify-end space-x-3 pt-4 border-t border-gray-200">
              <button
                type="button"
                @click="resetNetworkSettings"
                class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                重置
              </button>
              <button
                type="submit"
                :disabled="saving"
                class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
              >
                {{ saving ? '保存中...' : '💾 保存设置' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- 通知设置 -->
    <div v-if="currentTab === 'notifications'" class="space-y-6">
      <div class="bg-white shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">通知开关</h3>
          
          <div class="space-y-4">
            <!-- 系统通知 -->
            <div class="flex items-center justify-between py-3 border-b border-gray-100">
              <div>
                <h4 class="text-sm font-medium text-gray-900">系统通知</h4>
                <p class="text-sm text-gray-500">系统更新、重启等通知</p>
              </div>
              <button
                @click="notificationSettings.system_enabled = !notificationSettings.system_enabled"
                :class="notificationSettings.system_enabled ? 'bg-primary-600' : 'bg-gray-200'"
                class="relative inline-flex flex-shrink-0 h-6 w-11 border-2 border-transparent rounded-full cursor-pointer transition-colors ease-in-out duration-200"
              >
                <span
                  :class="notificationSettings.system_enabled ? 'translate-x-5' : 'translate-x-0'"
                  class="pointer-events-none inline-block h-5 w-5 rounded-full bg-white shadow transform ring-0 transition ease-in-out duration-200"
                />
              </button>
            </div>

            <!-- 存储警告 -->
            <div class="flex items-center justify-between py-3 border-b border-gray-100">
              <div>
                <h4 class="text-sm font-medium text-gray-900">存储警告</h4>
                <p class="text-sm text-gray-500">磁盘空间不足、SMART 警告等</p>
              </div>
              <button
                @click="notificationSettings.storage_enabled = !notificationSettings.storage_enabled"
                :class="notificationSettings.storage_enabled ? 'bg-primary-600' : 'bg-gray-200'"
                class="relative inline-flex flex-shrink-0 h-6 w-11 border-2 border-transparent rounded-full cursor-pointer transition-colors ease-in-out duration-200"
              >
                <span
                  :class="notificationSettings.storage_enabled ? 'translate-x-5' : 'translate-x-0'"
                  class="pointer-events-none inline-block h-5 w-5 rounded-full bg-white shadow transform ring-0 transition ease-in-out duration-200"
                />
              </button>
            </div>

            <!-- 备份通知 -->
            <div class="flex items-center justify-between py-3 border-b border-gray-100">
              <div>
                <h4 class="text-sm font-medium text-gray-900">备份通知</h4>
                <p class="text-sm text-gray-500">备份任务完成、失败通知</p>
              </div>
              <button
                @click="notificationSettings.backup_enabled = !notificationSettings.backup_enabled"
                :class="notificationSettings.backup_enabled ? 'bg-primary-600' : 'bg-gray-200'"
                class="relative inline-flex flex-shrink-0 h-6 w-11 border-2 border-transparent rounded-full cursor-pointer transition-colors ease-in-out duration-200"
              >
                <span
                  :class="notificationSettings.backup_enabled ? 'translate-x-5' : 'translate-x-0'"
                  class="pointer-events-none inline-block h-5 w-5 rounded-full bg-white shadow transform ring-0 transition ease-in-out duration-200"
                />
              </button>
            </div>

            <!-- 安全通知 -->
            <div class="flex items-center justify-between py-3">
              <div>
                <h4 class="text-sm font-medium text-gray-900">安全通知</h4>
                <p class="text-sm text-gray-500">登录尝试、权限变更等</p>
              </div>
              <button
                @click="notificationSettings.security_enabled = !notificationSettings.security_enabled"
                :class="notificationSettings.security_enabled ? 'bg-primary-600' : 'bg-gray-200'"
                class="relative inline-flex flex-shrink-0 h-6 w-11 border-2 border-transparent rounded-full cursor-pointer transition-colors ease-in-out duration-200"
              >
                <span
                  :class="notificationSettings.security_enabled ? 'translate-x-5' : 'translate-x-0'"
                  class="pointer-events-none inline-block h-5 w-5 rounded-full bg-white shadow transform ring-0 transition ease-in-out duration-200"
                />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 邮件服务器配置 -->
      <div class="bg-white shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">邮件服务器配置</h3>
          
          <form @submit.prevent="saveEmailSettings" class="space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label for="smtp_server" class="block text-sm font-medium text-gray-700 mb-2">
                  SMTP 服务器
                </label>
                <input
                  v-model="emailSettings.smtp_server"
                  type="text"
                  id="smtp_server"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="smtp.example.com"
                />
              </div>

              <div>
                <label for="smtp_port" class="block text-sm font-medium text-gray-700 mb-2">
                  SMTP 端口
                </label>
                <input
                  v-model="emailSettings.smtp_port"
                  type="number"
                  id="smtp_port"
                  min="1"
                  max="65535"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="587"
                />
              </div>

              <div>
                <label for="smtp_username" class="block text-sm font-medium text-gray-700 mb-2">
                  用户名
                </label>
                <input
                  v-model="emailSettings.smtp_username"
                  type="text"
                  id="smtp_username"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="admin@example.com"
                />
              </div>

              <div>
                <label for="smtp_password" class="block text-sm font-medium text-gray-700 mb-2">
                  密码
                </label>
                <input
                  v-model="emailSettings.smtp_password"
                  type="password"
                  id="smtp_password"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="••••••••"
                />
              </div>

              <div>
                <label for="smtp_from" class="block text-sm font-medium text-gray-700 mb-2">
                  发件人邮箱
                </label>
                <input
                  v-model="emailSettings.smtp_from"
                  type="email"
                  id="smtp_from"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="nas@example.com"
                />
              </div>

              <div>
                <label for="smtp_to" class="block text-sm font-medium text-gray-700 mb-2">
                  收件人邮箱
                </label>
                <input
                  v-model="emailSettings.smtp_to"
                  type="email"
                  id="smtp_to"
                  class="shadow-sm focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                  placeholder="admin@example.com"
                />
              </div>
            </div>

            <!-- 使用 SSL/TLS -->
            <div class="flex items-center">
              <input
                v-model="emailSettings.use_tls"
                type="checkbox"
                id="use_tls"
                class="focus:ring-primary-500 h-4 w-4 text-primary-600 border-gray-300 rounded"
              />
              <label for="use_tls" class="ml-2 block text-sm text-gray-700">
                使用 SSL/TLS 加密连接
              </label>
            </div>

            <!-- 保存按钮 -->
            <div class="flex items-center justify-end space-x-3 pt-4 border-t border-gray-200">
              <button
                type="button"
                @click="testEmailSettings"
                class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                📧 发送测试邮件
              </button>
              <button
                type="button"
                @click="resetEmailSettings"
                class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                重置
              </button>
              <button
                type="submit"
                :disabled="saving"
                class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
              >
                {{ saving ? '保存中...' : '💾 保存设置' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- 保存成功提示 -->
    <div v-if="showSuccessToast" class="fixed bottom-4 right-4 bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg">
      ✅ 设置已保存
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import apiClient from '../api/client';

const tabs = [
  { id: 'basic', name: '🏠 基本设置' },
  { id: 'network', name: '🌐 网络设置' },
  { id: 'notifications', name: '🔔 通知设置' },
];

const currentTab = ref('basic');
const saving = ref(false);
const showSuccessToast = ref(false);

// 基本设置
const basicSettings = ref({
  hostname: '',
  timezone: 'Asia/Shanghai',
  language: 'zh-CN',
});

// 系统信息
const systemInfo = ref({
  version: '0.1.0',
  uptime: 0,
  model: 'Axis NAS',
  serial: '',
});

// 网络设置
const networkSettings = ref({
  dhcp_enabled: true,
  ip_address: '',
  netmask: '',
  gateway: '',
  dns_primary: '',
  dns_secondary: '',
});

// 网络信息
const networkInfo = ref({
  mac_address: '',
  connected: true,
  current_ip: '',
  link_speed: 1000,
});

// 通知设置
const notificationSettings = ref({
  system_enabled: true,
  storage_enabled: true,
  backup_enabled: true,
  security_enabled: true,
});

// 邮件设置
const emailSettings = ref({
  smtp_server: '',
  smtp_port: 587,
  smtp_username: '',
  smtp_password: '',
  smtp_from: '',
  smtp_to: '',
  use_tls: true,
});

// 获取设置
const fetchSettings = async () => {
  try {
    // 获取基本设置
    const basicResponse = await apiClient.getSettings('basic');
    if (basicResponse.success && basicResponse.data) {
      basicSettings.value = { ...basicSettings.value, ...basicResponse.data };
    }

    // 获取网络设置
    const networkResponse = await apiClient.getSettings('network');
    if (networkResponse.success && networkResponse.data) {
      networkSettings.value = { ...networkSettings.value, ...networkResponse.data };
    }

    // 获取通知设置
    const notificationResponse = await apiClient.getSettings('notifications');
    if (notificationResponse.success && notificationResponse.data) {
      notificationSettings.value = { ...notificationSettings.value, ...notificationResponse.data };
    }

    // 获取邮件设置
    const emailResponse = await apiClient.getSettings('email');
    if (emailResponse.success && emailResponse.data) {
      emailSettings.value = { ...emailSettings.value, ...emailResponse.data };
    }

    // 获取系统信息
    const systemResponse = await apiClient.getSystemInfo();
    if (systemResponse.success && systemResponse.data) {
      systemInfo.value = { ...systemInfo.value, ...systemResponse.data };
    }

    // 获取网络信息
    const networkInfoResponse = await apiClient.getNetworkInfo();
    if (networkInfoResponse.success && networkInfoResponse.data) {
      networkInfo.value = { ...networkInfo.value, ...networkInfoResponse.data };
    }
  } catch (err) {
    console.error('Failed to fetch settings:', err);
  }
};

// 保存基本设置
const saveBasicSettings = async () => {
  saving.value = true;
  try {
    await apiClient.updateSettings('basic', basicSettings.value);
    showSuccess();
  } catch (err) {
    alert('保存基本设置失败');
    console.error('Save basic settings failed:', err);
  } finally {
    saving.value = false;
  }
};

// 重置基本设置
const resetBasicSettings = () => {
  fetchSettings();
};

// 保存网络设置
const saveNetworkSettings = async () => {
  saving.value = true;
  try {
    await apiClient.updateSettings('network', networkSettings.value);
    showSuccess();
  } catch (err) {
    alert('保存网络设置失败');
    console.error('Save network settings failed:', err);
  } finally {
    saving.value = false;
  }
};

// 重置网络设置
const resetNetworkSettings = () => {
  fetchSettings();
};

// 保存邮件设置
const saveEmailSettings = async () => {
  saving.value = true;
  try {
    await apiClient.updateSettings('email', emailSettings.value);
    showSuccess();
  } catch (err) {
    alert('保存邮件设置失败');
    console.error('Save email settings failed:', err);
  } finally {
    saving.value = false;
  }
};

// 重置邮件设置
const resetEmailSettings = () => {
  fetchSettings();
};

// 测试邮件设置
const testEmailSettings = async () => {
  try {
    await apiClient.testEmailSettings(emailSettings.value);
    alert('✅ 测试邮件已发送，请检查收件箱');
  } catch (err) {
    alert('❌ 发送测试邮件失败');
    console.error('Test email failed:', err);
  }
};

// 显示成功提示
const showSuccess = () => {
  showSuccessToast.value = true;
  setTimeout(() => {
    showSuccessToast.value = false;
  }, 3000);
};

// 工具函数
const formatUptime = (seconds: number): string => {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  
  if (days > 0) return `${days}天 ${hours}小时`;
  if (hours > 0) return `${hours}小时 ${mins}分钟`;
  return `${mins}分钟`;
};

onMounted(() => {
  fetchSettings();
});
</script>

<style scoped>
/* Settings view specific styles */
</style>
