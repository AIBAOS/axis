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
        <nav class="-mb-px flex space-x-8 overflow-x-auto">
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

      <!-- 系统信息 -->
      <div v-else-if="currentTab === 'system'" class="space-y-6">
        <!-- 系统操作 -->
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="font-semibold text-gray-900 mb-4">系统操作</h3>
          <div class="flex flex-wrap gap-3">
            <button @click="checkUpdates" :disabled="checkingUpdates" class="btn-secondary flex items-center space-x-2">
              <svg :class="{'animate-spin': checkingUpdates}" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              <span>{{ checkingUpdates ? '检查中...' : '检查更新' }}</span>
            </button>
            <button @click="confirmRestart" class="px-4 py-2 border border-orange-300 text-orange-600 rounded-lg hover:bg-orange-50">重启系统</button>
            <button @click="confirmShutdown" class="px-4 py-2 border border-red-300 text-red-600 rounded-lg hover:bg-red-50">关机</button>
            <button @click="confirmFactoryReset" class="px-4 py-2 border border-gray-300 text-gray-600 rounded-lg hover:bg-gray-50">恢复出厂</button>
          </div>
          
          <!-- 更新状态 -->
          <div v-if="updateStatus" class="mt-4 p-3 rounded-lg bg-gray-50">
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600">当前版本</span>
              <span class="text-sm font-medium">{{ updateStatus.current_version || systemInfo.version }}</span>
            </div>
            <div v-if="updateStatus.has_update" class="mt-2 p-2 bg-green-50 rounded flex items-center justify-between">
              <span class="text-sm text-green-700">发现新版本: {{ updateStatus.latest_version }}</span>
              <button @click="installUpdate" :disabled="installing" class="text-sm text-green-600 hover:text-green-700 font-medium">
                {{ installing ? '安装中...' : '立即更新' }}
              </button>
            </div>
          </div>
        </div>

        <!-- 硬件信息 -->
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="font-semibold text-gray-900 mb-4">硬件信息</h3>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
            <div>
              <p class="text-gray-500">CPU 型号</p>
              <p class="font-medium text-gray-900 truncate">{{ systemInfo.cpu_model || '-' }}</p>
            </div>
            <div>
              <p class="text-gray-500">CPU 核心数</p>
              <p class="font-medium text-gray-900">{{ resources.cpu.core_count || systemInfo.cpu_cores || '-' }} 核</p>
            </div>
            <div>
              <p class="text-gray-500">总内存</p>
              <p class="font-medium text-gray-900">{{ formatBytes(resources.memory.total_bytes) || systemInfo.total_memory_gb + ' GB' || '-' }}</p>
            </div>
            <div>
              <p class="text-gray-500">磁盘温度</p>
              <p class="font-medium" :class="avgTemperature > 60 ? 'text-red-600' : avgTemperature > 40 ? 'text-yellow-600' : 'text-green-600'">
                {{ avgTemperature > 0 ? avgTemperature + '°C' : '-' }}
              </p>
            </div>
          </div>
        </div>

        <!-- 版本信息 -->
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="font-semibold text-gray-900 mb-4">系统版本</h3>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
            <div>
              <p class="text-gray-500">系统版本</p>
              <p class="font-medium text-gray-900">{{ systemInfo.version || 'v0.1.0' }}</p>
            </div>
            <div>
              <p class="text-gray-500">内核版本</p>
              <p class="font-medium text-gray-900 truncate">{{ systemInfo.kernel_version || '-' }}</p>
            </div>
            <div>
              <p class="text-gray-500">操作系统</p>
              <p class="font-medium text-gray-900">{{ systemInfo.os_version || '-' }}</p>
            </div>
            <div>
              <p class="text-gray-500">架构</p>
              <p class="font-medium text-gray-900">{{ systemInfo.arch || 'x86_64' }}</p>
            </div>
          </div>
        </div>

        <!-- 运行状态 -->
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="font-semibold text-gray-900 mb-4">运行状态</h3>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
            <div>
              <p class="text-gray-500">运行时间</p>
              <p class="font-medium text-gray-900">{{ formatUptime(systemInfo.uptime_seconds) }}</p>
            </div>
            <div>
              <p class="text-gray-500">启动时间</p>
              <p class="font-medium text-gray-900">{{ formatBootTime(systemInfo.boot_time) }}</p>
            </div>
            <div>
              <p class="text-gray-500">主机名</p>
              <p class="font-medium text-gray-900">{{ systemInfo.hostname || '-' }}</p>
            </div>
            <div>
              <p class="text-gray-500">时区</p>
              <p class="font-medium text-gray-900">{{ systemInfo.timezone || 'UTC' }}</p>
            </div>
          </div>
        </div>

        <!-- 资源使用 -->
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="font-semibold text-gray-900 mb-4">资源使用</h3>
          <div class="space-y-4">
            <!-- CPU -->
            <div>
              <div class="flex justify-between text-sm mb-1">
                <span class="text-gray-600">CPU 使用率</span>
                <span class="font-medium">{{ resources.cpu.usage_percent?.toFixed(1) || 0 }}%</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div
                  class="bg-blue-500 h-2 rounded-full transition-all"
                  :style="{ width: (resources.cpu.usage_percent || 0) + '%' }"
                ></div>
              </div>
              <p class="text-xs text-gray-500 mt-1">
                负载: {{ resources.cpu.load_1m?.toFixed(2) || '-' }} / {{ resources.cpu.load_5m?.toFixed(2) || '-' }} / {{ resources.cpu.load_15m?.toFixed(2) || '-' }}
              </p>
            </div>

            <!-- 内存 -->
            <div>
              <div class="flex justify-between text-sm mb-1">
                <span class="text-gray-600">内存使用率</span>
                <span class="font-medium">{{ resources.memory.usage_percent?.toFixed(1) || 0 }}%</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div
                  :class="[(resources.memory.usage_percent || 0) > 80 ? 'bg-red-500' : (resources.memory.usage_percent || 0) > 60 ? 'bg-yellow-500' : 'bg-green-500']"
                  class="h-2 rounded-full transition-all"
                  :style="{ width: (resources.memory.usage_percent || 0) + '%' }"
                ></div>
              </div>
              <p class="text-xs text-gray-500 mt-1">
                {{ formatBytes(resources.memory.used_bytes) }} / {{ formatBytes(resources.memory.total_bytes) }}
              </p>
            </div>

            <!-- 磁盘 -->
            <div>
              <div class="flex justify-between text-sm mb-1">
                <span class="text-gray-600">磁盘使用率</span>
                <span class="font-medium">{{ diskUsagePercent }}%</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div
                  :class="[(diskUsagePercent) > 90 ? 'bg-red-500' : (diskUsagePercent) > 70 ? 'bg-yellow-500' : 'bg-green-500']"
                  class="h-2 rounded-full transition-all"
                  :style="{ width: diskUsagePercent + '%' }"
                ></div>
              </div>
              <p class="text-xs text-gray-500 mt-1">
                {{ disks.length }} 个磁盘在线
              </p>
            </div>
          </div>
        </div>

        <!-- 快速操作 -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <router-link to="/logs" class="bg-white rounded-lg shadow p-4 hover:shadow-lg transition-shadow">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 rounded-lg bg-orange-100 flex items-center justify-center">
                <svg class="w-5 h-5 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                </svg>
              </div>
              <div>
                <p class="font-medium text-gray-900">系统日志</p>
                <p class="text-sm text-gray-500">查看系统运行日志</p>
              </div>
            </div>
          </router-link>

          <router-link to="/users" class="bg-white rounded-lg shadow p-4 hover:shadow-lg transition-shadow">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 rounded-lg bg-indigo-100 flex items-center justify-center">
                <svg class="w-5 h-5 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                </svg>
              </div>
              <div>
                <p class="font-medium text-gray-900">用户管理</p>
                <p class="text-sm text-gray-500">添加、编辑、删除用户</p>
              </div>
            </div>
          </router-link>

          <router-link to="/storage" class="bg-white rounded-lg shadow p-4 hover:shadow-lg transition-shadow">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 rounded-lg bg-green-100 flex items-center justify-center">
                <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
                </svg>
              </div>
              <div>
                <p class="font-medium text-gray-900">存储管理</p>
                <p class="text-sm text-gray-500">磁盘和存储池管理</p>
              </div>
            </div>
          </router-link>
        </div>
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
              pattern="[a-zA-Z0-9][a-zA-Z0-9\-]*"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="例如：axis-nas"
            />
            <p class="text-xs text-gray-500 mt-1">只允许字母、数字和连字符</p>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">时区</label>
            <select v-model="basicSettings.timezone" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
              <option value="UTC">UTC</option>
              <option value="Asia/Shanghai">Asia/Shanghai (北京时间)</option>
              <option value="Asia/Tokyo">Asia/Tokyo (东京)</option>
              <option value="America/New_York">America/New_York (纽约)</option>
              <option value="Europe/London">Europe/London (伦敦)</option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">系统语言</label>
            <select v-model="basicSettings.language" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
              <option value="zh-CN">简体中文</option>
              <option value="zh-TW">繁體中文</option>
              <option value="en-US">English (US)</option>
              <option value="ja-JP">日本語</option>
            </select>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <label class="block text-sm font-medium text-gray-700">自动更新</label>
              <p class="text-sm text-gray-500">系统自动安装安全更新</p>
            </div>
            <input v-model="basicSettings.auto_update_enabled" type="checkbox" class="h-5 w-5 text-primary-600 rounded" />
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
        <!-- HTTPS 配置 -->
        <div class="bg-white rounded-lg shadow p-4">
          <h3 class="font-semibold text-gray-900 mb-4">HTTPS 配置</h3>
          <form class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <label class="block text-sm font-medium text-gray-700">启用 HTTPS</label>
                <p class="text-sm text-gray-500">使用 HTTPS 加密连接</p>
              </div>
              <input v-model="httpsSettings.enabled" type="checkbox" class="h-5 w-5 text-primary-600 rounded" />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">HTTPS 端口</label>
                <input
                  v-model.number="httpsSettings.port"
                  type="number"
                  :disabled="!httpsSettings.enabled"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100"
                  placeholder="443"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">HTTP 端口</label>
                <input
                  v-model.number="httpsSettings.http_port"
                  type="number"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg"
                  placeholder="80"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">SSL 证书</label>
              <select v-model="httpsSettings.cert_type" :disabled="!httpsSettings.enabled" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100">
                <option value="self">自签名证书</option>
                <option value="letsencrypt">Let's Encrypt</option>
                <option value="custom">自定义证书</option>
              </select>
            </div>

            <div v-if="httpsSettings.cert_type === 'letsencrypt'" class="p-3 bg-blue-50 rounded-lg">
              <p class="text-sm text-blue-700">Let's Encrypt 将自动获取和续期证书</p>
            </div>
          </form>
        </div>

        <!-- 网络接口 -->
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex justify-between items-center mb-4">
            <h3 class="font-semibold text-gray-900">网络接口</h3>
            <button @click="loadNetworkConfig" class="text-sm text-primary-600 hover:text-primary-700">刷新</button>
          </div>
          
          <div v-if="networkLoading" class="text-center py-4 text-gray-500">加载中...</div>
          <div v-else-if="networkInterfaces.length === 0" class="text-center py-4 text-gray-500">暂无网络接口</div>
          <div v-else class="space-y-3">
            <div v-for="iface in networkInterfaces" :key="iface.id" class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
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
              <input v-model="proxySettings.enabled" type="checkbox" class="h-5 w-5 text-primary-600 rounded" />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">代理服务器</label>
                <input v-model="proxySettings.server" type="text" :disabled="!proxySettings.enabled" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100" placeholder="proxy.example.com" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">端口</label>
                <input v-model.number="proxySettings.port" type="number" :disabled="!proxySettings.enabled" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100" placeholder="8080" />
              </div>
            </div>

            <div class="flex justify-end">
              <button type="submit" :disabled="saving" class="btn-primary">{{ saving ? '保存中...' : '保存' }}</button>
            </div>
          </form>
        </div>
      </div>

      <!-- 用户管理 -->
      <div v-else-if="currentTab === 'users'" class="max-w-2xl">
        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex justify-between items-center mb-4">
            <h3 class="font-semibold text-gray-900">用户管理</h3>
            <router-link to="/users" class="btn-primary text-sm">进入用户管理</router-link>
          </div>
          <p class="text-gray-600 mb-4">在用户管理页面可以：</p>
          <ul class="list-disc list-inside text-gray-600 space-y-2">
            <li>查看所有用户列表</li>
            <li>添加新用户</li>
            <li>编辑用户信息</li>
            <li>删除用户</li>
            <li>管理用户权限</li>
          </ul>
        </div>
      </div>

      <!-- 通知设置 -->
      <div v-else-if="currentTab === 'notification'" class="max-w-2xl space-y-6">
        <div class="bg-white rounded-lg shadow p-4">
          <h3 class="font-semibold text-gray-900 mb-4">邮件通知</h3>
          <form @submit.prevent="handleSaveEmailNotification" class="space-y-4">
            <div class="flex items-center justify-between">
              <label class="block text-sm font-medium text-gray-700">启用邮件通知</label>
              <input v-model="emailSettings.enabled" type="checkbox" class="h-5 w-5 text-primary-600 rounded" />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">SMTP 服务器</label>
                <input v-model="emailSettings.smtp_server" type="text" :disabled="!emailSettings.enabled" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100" placeholder="smtp.gmail.com" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">端口</label>
                <input v-model.number="emailSettings.smtp_port" type="number" :disabled="!emailSettings.enabled" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100" placeholder="587" />
              </div>
            </div>

            <div class="flex justify-end">
              <button type="submit" :disabled="saving" class="btn-primary">{{ saving ? '保存中...' : '保存' }}</button>
            </div>
          </form>
        </div>

        <div class="bg-white rounded-lg shadow p-4">
          <h3 class="font-semibold text-gray-900 mb-4">Webhook 通知</h3>
          <form @submit.prevent="handleSaveWebhookNotification" class="space-y-4">
            <div class="flex items-center justify-between">
              <label class="block text-sm font-medium text-gray-700">启用 Webhook</label>
              <input v-model="webhookSettings.enabled" type="checkbox" class="h-5 w-5 text-primary-600 rounded" />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Webhook URL</label>
              <input v-model="webhookSettings.url" type="url" :disabled="!webhookSettings.enabled" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100" placeholder="https://example.com/webhook" />
            </div>

            <div class="flex justify-end">
              <button type="submit" :disabled="saving" class="btn-primary">{{ saving ? '保存中...' : '保存' }}</button>
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

      <!-- 确认对话框 -->
      <div v-if="confirmAction" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
          <div class="p-6">
            <h3 class="text-lg font-semibold text-gray-900 mb-2">{{ confirmAction.title }}</h3>
            <p class="text-gray-600">{{ confirmAction.message }}</p>
            <p v-if="confirmAction.warning" class="mt-2 text-sm text-red-600">{{ confirmAction.warning }}</p>
          </div>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button @click="confirmAction = null" class="px-4 py-2 text-gray-600 hover:text-gray-800">取消</button>
            <button @click="executeConfirmAction" :disabled="executing" :class="confirmAction.danger ? 'bg-red-600 hover:bg-red-700' : 'btn-primary'" class="px-4 py-2 text-white rounded-lg disabled:opacity-50">
              {{ executing ? '处理中...' : confirmAction.confirmText || '确认' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import { api } from '@/utils/api'

// 选项卡
const tabs = [
  { id: 'system', name: '系统信息' },
  { id: 'basic', name: '基本设置' },
  { id: 'network', name: '网络设置' },
  { id: 'users', name: '用户管理' },
  { id: 'notification', name: '通知设置' }
]

const currentTab = ref('system')
const loading = ref(true)
const saving = ref(false)
const networkLoading = ref(false)

// Toast
const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

// 系统操作
const checkingUpdates = ref(false)
const installing = ref(false)
const updateStatus = ref<any>(null)
const confirmAction = ref<{title: string; message: string; warning?: string; action: string; danger?: boolean; confirmText?: string} | null>(null)
const executing = ref(false)

// 系统信息
const systemInfo = ref({
  version: 'v0.1.0',
  kernel_version: '',
  os_version: '',
  arch: 'x86_64',
  uptime_seconds: 0,
  boot_time: 0,
  hostname: '',
  timezone: 'UTC',
  cpu_model: '',
  cpu_cores: 0,
  total_memory_gb: 0
})

// 资源
const resources = ref({
  cpu: { usage_percent: 0, load_1m: 0, load_5m: 0, load_15m: 0, core_count: 0 },
  memory: { total_bytes: 0, used_bytes: 0, usage_percent: 0 }
})

// 磁盘
const disks = ref<any[]>([])

// 基本设置
const basicSettings = ref({
  hostname: '',
  timezone: 'Asia/Shanghai',
  language: 'zh-CN',
  auto_update_enabled: true
})

// HTTPS 设置
const httpsSettings = ref({
  enabled: false,
  port: 443,
  http_port: 80,
  cert_type: 'self'
})

// 网络接口
const networkInterfaces = ref<any[]>([])

// 代理设置
const proxySettings = ref({
  enabled: false,
  server: '',
  port: 8080
})

// 邮件设置
const emailSettings = ref({
  enabled: false,
  smtp_server: '',
  smtp_port: 587
})

// Webhook 设置
const webhookSettings = ref({
  enabled: false,
  url: ''
})

// 磁盘使用率
const diskUsagePercent = computed(() => {
  if (disks.value.length === 0) return 0
  const total = disks.value.reduce((sum, d) => sum + (d.size_bytes || 0), 0)
  const used = disks.value.reduce((sum, d) => sum + (d.used_bytes || 0), 0)
  return total > 0 ? Math.round(used / total * 100) : 0
})

// 平均温度
const avgTemperature = computed(() => {
  const temps = disks.value.filter(d => d.temperature && d.temperature > 0).map(d => d.temperature)
  if (temps.length === 0) return 0
  return Math.round(temps.reduce((a, b) => a + b, 0) / temps.length)
})

// 加载系统信息
const loadSystemInfo = async () => {
  try {
    const response = await api.system.info()
    systemInfo.value = { ...systemInfo.value, ...(response.data.data || response.data) }
  } catch (error) {
    console.error('Failed to load system info:', error)
  }
}

// 加载资源
const loadResources = async () => {
  try {
    const response = await api.system.resources()
    resources.value = { ...resources.value, ...(response.data.data || response.data) }
  } catch (error) {
    console.error('Failed to load resources:', error)
  }
}

// 加载磁盘
const loadDisks = async () => {
  try {
    const response = await api.storage.getDisks()
    disks.value = response.data.disks || response.data || []
  } catch (error) {
    console.error('Failed to load disks:', error)
  }
}

// 加载设置
const loadSettings = async () => {
  try {
    const response = await api.settings.get()
    const settings = response.data.data || response.data || {}
    basicSettings.value = {
      hostname: settings.hostname || '',
      timezone: settings.timezone || 'Asia/Shanghai',
      language: settings.language || 'zh-CN',
      auto_update_enabled: settings.auto_update_enabled ?? true
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
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
  saving.value = true
  try {
    await api.settings.update(basicSettings.value)
    showToast('success', '设置已保存')
  } catch (error) {
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
      proxy_port: proxySettings.value.port
    })
    showToast('success', '代理设置已保存')
  } catch (error) {
    showToast('error', '保存失败')
  } finally {
    saving.value = false
  }
}

// 保存邮件通知
const handleSaveEmailNotification = async () => {
  saving.value = true
  try {
    await api.settings.update({
      notification_enabled: emailSettings.value.enabled,
      smtp_server: emailSettings.value.smtp_server,
      smtp_port: emailSettings.value.smtp_port
    })
    showToast('success', '邮件设置已保存')
  } catch (error) {
    showToast('error', '保存失败')
  } finally {
    saving.value = false
  }
}

// 保存 Webhook
const handleSaveWebhookNotification = async () => {
  if (webhookSettings.value.enabled && !webhookSettings.value.url) {
    showToast('error', '请填写 Webhook URL')
    return
  }
  saving.value = true
  try {
    await api.settings.update({
      webhook_enabled: webhookSettings.value.enabled,
      webhook_url: webhookSettings.value.url
    })
    showToast('success', 'Webhook 设置已保存')
  } catch (error) {
    showToast('error', '保存失败')
  } finally {
    saving.value = false
  }
}

// 格式化
const formatUptime = (seconds: number) => {
  if (!seconds) return '-'
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  if (days > 0) return `${days} 天 ${hours} 小时`
  if (hours > 0) return `${hours} 小时`
  return `${Math.floor(seconds / 60)} 分钟`
}

const formatBootTime = (timestamp: number) => {
  if (!timestamp) return '-'
  return new Date(timestamp * 1000).toLocaleDateString('zh-CN')
}

const formatBytes = (bytes: number) => {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

// Toast
const showToast = (type: 'success' | 'error', message: string) => {
  toast.value = { show: true, type, message }
  setTimeout(() => { toast.value.show = false }, 3000)
}

// 检查更新
const checkUpdates = async () => {
  checkingUpdates.value = true
  try {
    const response = await api.system.checkUpdates()
    updateStatus.value = response.data.data || response.data
    if (updateStatus.value?.has_update) {
      showToast('success', `发现新版本 ${updateStatus.value.latest_version}`)
    } else {
      showToast('success', '已是最新版本')
    }
  } catch (error) {
    showToast('error', '检查更新失败')
  } finally {
    checkingUpdates.value = false
  }
}

// 安装更新
const installUpdate = async () => {
  installing.value = true
  try {
    await api.system.installUpdate?.()
    showToast('success', '系统更新已开始，请稍候...')
  } catch (error) {
    showToast('error', '更新失败')
  } finally {
    installing.value = false
  }
}

// 确认重启
const confirmRestart = () => {
  confirmAction.value = {
    title: '确认重启',
    message: '系统将立即重启，所有服务将暂时中断。',
    action: 'restart',
    confirmText: '重启'
  }
}

// 确认关机
const confirmShutdown = () => {
  confirmAction.value = {
    title: '确认关机',
    message: '系统将立即关机，请确保所有重要数据已保存。',
    action: 'shutdown',
    danger: true,
    confirmText: '关机'
  }
}

// 确认恢复出厂
const confirmFactoryReset = () => {
  confirmAction.value = {
    title: '恢复出厂设置',
    message: '此操作将删除所有用户数据和配置，系统将恢复到初始状态。',
    warning: '此操作不可撤销！',
    action: 'factory_reset',
    danger: true,
    confirmText: '恢复出厂'
  }
}

// 执行确认操作
const executeConfirmAction = async () => {
  if (!confirmAction.value) return
  executing.value = true
  
  try {
    switch (confirmAction.value.action) {
      case 'restart':
        await api.system.restart()
        showToast('success', '重启命令已发送')
        break
      case 'shutdown':
        await api.system.shutdown()
        showToast('success', '关机命令已发送')
        break
      case 'factory_reset':
        await api.system.factoryReset?.()
        showToast('success', '恢复出厂设置已开始')
        break
    }
    confirmAction.value = null
  } catch (error) {
    showToast('error', '操作失败')
  } finally {
    executing.value = false
  }
}

// 生命周期
onMounted(async () => {
  loading.value = true
  await Promise.all([
    loadSystemInfo(),
    loadResources(),
    loadDisks(),
    loadSettings(),
    loadNetworkConfig()
  ])
  loading.value = false
})
</script>