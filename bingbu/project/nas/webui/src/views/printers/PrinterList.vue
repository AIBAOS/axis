<template>
  <div class="space-y-6">
    <!-- 页面标题和操作栏 -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        打印机管理
      </h1>
      <button
        @click="handleAdd"
        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
      >
        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
        </svg>
        添加打印机
      </button>
    </div>

    <!-- 搜索和筛选栏 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <!-- 搜索框 -->
        <div class="md:col-span-2">
          <FormInput
            v-model="searchQuery"
            placeholder="搜索打印机名称..."
            label="搜索"
            :hide-label="true"
          />
        </div>

        <!-- 状态筛选 -->
        <div>
          <select
            v-model="statusFilter"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
          >
            <option value="">全部状态</option>
            <option value="online">在线</option>
            <option value="offline">离线</option>
            <option value="error">错误</option>
          </select>
        </div>
      </div>
    </div>

    <!-- 打印机列表 -->
    <div v-if="loading" class="space-y-4">
      <SkeletonLoader v-for="i in 3" :key="i" :lines="2" has-title />
    </div>

    <div v-else-if="filteredPrinters.length === 0" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-12 text-center">
      <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"></path>
      </svg>
      <p class="text-lg font-medium text-gray-900 dark:text-white mb-2">暂无打印机</p>
      <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">添加打印机开始使用</p>
      <button
        @click="handleAdd"
        class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
      >
        添加打印机
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="printer in filteredPrinters"
        :key="printer.id"
        class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6"
      >
        <!-- 打印机头部 -->
        <div class="flex items-start justify-between mb-4">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <svg class="w-10 h-10 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"></path>
              </svg>
            </div>
            <div class="ml-4">
              <h3 class="text-lg font-medium text-gray-900 dark:text-white">{{ printer.name }}</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">{{ printer.model }}</p>
            </div>
          </div>
          <span
            class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
            :class="statusClasses[printer.status]"
          >
            {{ statusLabels[printer.status] }}
          </span>
        </div>

        <!-- 打印机信息 -->
        <div class="space-y-2 mb-4">
          <div class="flex justify-between text-sm">
            <span class="text-gray-500 dark:text-gray-400">IP 地址</span>
            <span class="text-gray-900 dark:text-white">{{ printer.ip }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500 dark:text-gray-400">连接方式</span>
            <span class="text-gray-900 dark:text-white">{{ printer.connection }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500 dark:text-gray-400">墨量</span>
            <span class="text-gray-900 dark:text-white">{{ printer.ink }}%</span>
          </div>
          <ProgressBar :value="printer.ink" :color="getInkColor(printer.ink)" />
        </div>

        <!-- 操作按钮 -->
        <div class="flex justify-end space-x-2 pt-4 border-t border-gray-200 dark:border-gray-700">
          <button
            @click="handleTestPrint(printer)"
            class="px-3 py-1.5 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            测试打印
          </button>
          <button
            @click="handleEdit(printer)"
            class="px-3 py-1.5 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            编辑
          </button>
          <button
            @click="handleDelete(printer)"
            class="px-3 py-1.5 border border-red-300 dark:border-red-600 text-sm font-medium rounded-md text-red-700 dark:text-red-300 bg-white dark:bg-gray-800 hover:bg-red-50 dark:hover:bg-red-900/20"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      v-if="showDeleteConfirm"
      v-model="showDeleteConfirm"
      title="确认删除"
      :message="deleteMessage"
      type="warning"
      confirm-text="删除"
      confirm-button-color="red"
      @confirm="confirmDelete"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import FormInput from '../../components/FormInput.vue'
import ProgressBar from '../../components/ProgressBar.vue'
import SkeletonLoader from '../../components/SkeletonLoader.vue'
import ConfirmDialog from '../../components/ConfirmDialog.vue'
import { useToast } from '../../composables/useToast'

const toast = useToast()

const loading = ref(true)
const printers = ref([])
const searchQuery = ref('')
const statusFilter = ref('')

const showCreateModal = ref(false)
const showDeleteConfirm = ref(false)
const printerToDelete = ref(null)

const statusClasses = {
  online: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400',
  offline: 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-400',
  error: 'bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-400'
}

const statusLabels = {
  online: '在线',
  offline: '离线',
  error: '错误'
}

const filteredPrinters = computed(() => {
  return printers.value.filter(printer => {
    const matchSearch = printer.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                       printer.model.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchStatus = !statusFilter.value || printer.status === statusFilter.value
    return matchSearch && matchStatus
  })
})

const deleteMessage = computed(() => {
  return printerToDelete.value ? `确定要删除打印机 "${printerToDelete.value.name}" 吗？此操作不可恢复。` : ''
})

const getInkColor = (ink) => {
  if (ink > 50) return 'green'
  if (ink > 20) return 'yellow'
  return 'red'
}

const loadPrinters = async () => {
  loading.value = true
  try {
    // TODO: 调用 API 获取打印机列表
    // const response = await apiClient.get('/api/v1/printers')
    // printers.value = response.data.data
    
    // 模拟数据
    printers.value = [
      {
        id: 1,
        name: '办公室打印机',
        model: 'HP LaserJet Pro M404',
        ip: '192.168.1.100',
        connection: '网络',
        status: 'online',
        ink: 75
      },
      {
        id: 2,
        name: '财务室打印机',
        model: 'Canon imageCLASS',
        ip: '192.168.1.101',
        connection: '网络',
        status: 'online',
        ink: 45
      },
      {
        id: 3,
        name: '会议室打印机',
        model: 'Epson EcoTank',
        ip: '192.168.1.102',
        connection: 'WiFi',
        status: 'offline',
        ink: 15
      }
    ]
  } catch (error) {
    toast.error('加载打印机列表失败')
  } finally {
    loading.value = false
  }
}

const handleAdd = () => {
  toast.info('添加打印机功能待实现')
}

const handleTestPrint = (printer) => {
  toast.success(`已向 "${printer.name}" 发送测试页`)
}

const handleEdit = (printer) => {
  toast.info(`编辑 "${printer.name}" 功能待实现`)
}

const handleDelete = (printer) => {
  printerToDelete.value = printer
  showDeleteConfirm.value = true
}

const confirmDelete = async () => {
  try {
    // TODO: 调用 API 删除打印机
    // await apiClient.delete(`/api/v1/printers/${printerToDelete.value.id}`)
    
    printers.value = printers.value.filter(p => p.id !== printerToDelete.value.id)
    toast.success('打印机已删除')
  } catch (error) {
    toast.error('删除失败')
  }
}

onMounted(() => {
  loadPrinters()
})
</script>
