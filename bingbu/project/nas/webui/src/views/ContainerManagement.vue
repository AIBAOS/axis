<template>
  <div class="space-y-6">
    <!-- 页面标题和操作栏 -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        容器管理
      </h1>
      <button
        @click="showCreateModal = true"
        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
      >
        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
        </svg>
        创建容器
      </button>
    </div>

    <!-- 容器列表 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <!-- 列表头部 -->
      <div class="grid grid-cols-12 gap-4 px-6 py-3 bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600 text-sm font-medium text-gray-500 dark:text-gray-400">
        <div class="col-span-3">容器名</div>
        <div class="col-span-2">镜像</div>
        <div class="col-span-2">状态</div>
        <div class="col-span-3">网络</div>
        <div class="col-span-2 text-right">操作</div>
      </div>

      <!-- 空状态 -->
      <div v-if="containers.length === 0" class="px-6 py-12 text-center">
        <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"></path>
        </svg>
        <p class="text-lg font-medium text-gray-900 dark:text-white mb-2">暂无容器</p>
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">创建容器开始使用</p>
        <button
          @click="showCreateModal = true"
          class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
        >
          创建容器
        </button>
      </div>

      <!-- 列表 -->
      <div v-else class="divide-y divide-gray-200 dark:divide-gray-700">
        <div
          v-for="container in containers"
          :key="container.id"
          class="grid grid-cols-12 gap-4 px-6 py-4 hover:bg-gray-50 dark:hover:bg-gray-700 items-center"
        >
          <div class="col-span-3">
            <span class="text-sm font-medium text-gray-900 dark:text-white">{{ container.name }}</span>
          </div>
          <div class="col-span-2">
            <span class="text-sm text-gray-500 dark:text-gray-400">{{ container.image }}</span>
          </div>
          <div class="col-span-2">
            <span :class="statusClasses[container.status]" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">
              {{ statusLabels[container.status] }}
            </span>
          </div>
          <div class="col-span-3">
            <span class="text-sm text-gray-500 dark:text-gray-400">{{ container.network }}</span>
          </div>
          <div class="col-span-2 flex justify-end space-x-2">
            <button
              @click="viewConfig(container)"
              class="text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400"
              title="配置"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
              </svg>
            </button>
            <button
              @click="toggleContainer(container)"
              class="text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400"
              :title="container.status === 'running' ? '停止' : '启动'"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path v-if="container.status === 'running'" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m0-6l4 4m-4-4l-4 4m8 0l-4-4 4 4"></path>
                <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path>
              </svg>
            </button>
            <button
              @click="deleteContainer(container)"
              class="text-gray-400 hover:text-red-600 dark:hover:text-red-400"
              title="删除"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 创建容器模态框 -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 overflow-y-auto">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 my-8">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">创建容器</h3>
        </div>
        <div class="px-6 py-4 space-y-4 max-h-[60vh] overflow-y-auto">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              容器名 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formData.name"
              type="text"
              required
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              镜像 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formData.image"
              type="text"
              required
              placeholder="nginx:latest"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              网络
            </label>
            <select
              v-model="formData.network"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="bridge">Bridge</option>
              <option value="host">Host</option>
              <option value="none">None</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              环境变量
            </label>
            <div class="space-y-2">
              <div v-for="(env, index) in formData.env" :key="index" class="flex space-x-2">
                <input
                  v-model="env.key"
                  type="text"
                  placeholder="KEY"
                  class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                />
                <input
                  v-model="env.value"
                  type="text"
                  placeholder="VALUE"
                  class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                />
                <button
                  @click="removeEnv(index)"
                  class="text-red-600 hover:text-red-700"
                >
                  删除
                </button>
              </div>
              <button
                @click="addEnv"
                class="text-sm text-indigo-600 hover:text-indigo-700"
              >
                + 添加环境变量
              </button>
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              卷映射
            </label>
            <div class="space-y-2">
              <div v-for="(volume, index) in formData.volumes" :key="index" class="flex space-x-2">
                <input
                  v-model="volume.host"
                  type="text"
                  placeholder="主机路径"
                  class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                />
                <span class="text-gray-500">:</span>
                <input
                  v-model="volume.container"
                  type="text"
                  placeholder="容器路径"
                  class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                />
                <button
                  @click="removeVolume(index)"
                  class="text-red-600 hover:text-red-700"
                >
                  删除
                </button>
              </div>
              <button
                @click="addVolume"
                class="text-sm text-indigo-600 hover:text-indigo-700"
              >
                + 添加卷映射
              </button>
            </div>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="closeCreateModal"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="createContainer"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
          >
            创建
          </button>
        </div>
      </div>
    </div>

    <!-- 容器配置模态框 -->
    <div v-if="showConfigModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 overflow-y-auto">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 my-8">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">容器配置 - {{ selectedContainer?.name }}</h3>
        </div>
        <div class="px-6 py-4 space-y-4 max-h-[60vh] overflow-y-auto">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              容器名
            </label>
            <input
              v-model="selectedContainer.name"
              type="text"
              disabled
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              镜像
            </label>
            <input
              v-model="selectedContainer.image"
              type="text"
              disabled
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              网络
            </label>
            <input
              v-model="selectedContainer.network"
              type="text"
              disabled
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700"
            />
          </div>
          <div v-if="selectedContainer?.env?.length">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              环境变量
            </label>
            <div class="space-y-2">
              <div v-for="(env, index) in selectedContainer.env" :key="index" class="flex space-x-2">
                <input
                  v-model="env.key"
                  type="text"
                  disabled
                  class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700"
                />
                <input
                  v-model="env.value"
                  type="text"
                  disabled
                  class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700"
                />
              </div>
            </div>
          </div>
          <div v-if="selectedContainer?.volumes?.length">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              卷映射
            </label>
            <div class="space-y-2">
              <div v-for="(volume, index) in selectedContainer.volumes" :key="index" class="flex space-x-2">
                <input
                  v-model="volume.host"
                  type="text"
                  disabled
                  class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700"
                />
                <span class="text-gray-500">:</span>
                <input
                  v-model="volume.container"
                  type="text"
                  disabled
                  class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700"
                />
              </div>
            </div>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end">
          <button
            @click="closeConfigModal"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useToast } from '../composables/useToast'

const toast = useToast()

const containers = ref([
  { id: 1, name: 'nginx-web', image: 'nginx:latest', status: 'running', network: 'bridge', env: [{ key: 'TZ', value: 'Asia/Shanghai' }], volumes: [{ host: '/data/nginx', container: '/etc/nginx' }] },
  { id: 2, name: 'mysql-db', image: 'mysql:8', status: 'stopped', network: 'bridge', env: [{ key: 'MYSQL_ROOT_PASSWORD', value: 'password' }], volumes: [{ host: '/data/mysql', container: '/var/lib/mysql' }] },
  { id: 3, name: 'redis-cache', image: 'redis:latest', status: 'running', network: 'host', env: [], volumes: [] }
])

const showCreateModal = ref(false)
const showConfigModal = ref(false)
const selectedContainer = ref(null)

const formData = ref({
  name: '',
  image: '',
  network: 'bridge',
  env: [],
  volumes: []
})

const statusClasses = {
  running: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400',
  stopped: 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-400',
  paused: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400'
}

const statusLabels = {
  running: '运行中',
  stopped: '已停止',
  paused: '已暂停'
}

const addEnv = () => {
  formData.value.env.push({ key: '', value: '' })
}

const removeEnv = (index) => {
  formData.value.env.splice(index, 1)
}

const addVolume = () => {
  formData.value.volumes.push({ host: '', container: '' })
}

const removeVolume = (index) => {
  formData.value.volumes.splice(index, 1)
}

const closeCreateModal = () => {
  showCreateModal.value = false
  formData.value = {
    name: '',
    image: '',
    network: 'bridge',
    env: [],
    volumes: []
  }
}

const createContainer = () => {
  if (!formData.value.name || !formData.value.image) {
    toast.error('请填写必填项')
    return
  }

  containers.value.push({
    id: Date.now(),
    ...formData.value,
    status: 'stopped'
  })

  toast.success('容器已创建')
  closeCreateModal()
}

const viewConfig = (container) => {
  selectedContainer.value = { ...container }
  showConfigModal.value = true
}

const closeConfigModal = () => {
  showConfigModal.value = false
  selectedContainer.value = null
}

const toggleContainer = (container) => {
  container.status = container.status === 'running' ? 'stopped' : 'running'
  toast.success(`容器已${container.status === 'running' ? '启动' : '停止'}`)
}

const deleteContainer = (container) => {
  if (!confirm(`确定要删除容器 "${container.name}" 吗？`)) return
  
  containers.value = containers.value.filter(c => c.id !== container.id)
  toast.success('容器已删除')
}

onMounted(() => {
  // 加载容器列表
})
</script>
