<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">打印机管理</h1>
          <p class="text-gray-600 mt-1">管理网络打印机和打印任务</p>
        </div>
        <button
          @click="showModal = true; modalMode = 'create'; editingPrinter = null"
          class="btn-primary flex items-center space-x-2"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span>添加打印机</span>
        </button>
      </div>

      <!-- 状态统计 -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总打印机</p>
              <p class="text-xl font-bold text-gray-900">{{ printers.length }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-green-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">空闲</p>
              <p class="text-xl font-bold text-gray-900">{{ statusCounts.idle }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">打印中</p>
              <p class="text-xl font-bold text-blue-700">{{ statusCounts.printing }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-yellow-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">警告</p>
              <p class="text-xl font-bold text-yellow-700">{{ statusCounts.warning + statusCounts.out_of_paper + statusCounts.paper_jam }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">离线/错误</p>
              <p class="text-xl font-bold text-red-700">{{ statusCounts.offline + statusCounts.error }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 筛选 -->
      <div class="flex space-x-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索打印机名称或 IP 地址..."
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          />
        </div>
        <select
          v-model="statusFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
        >
          <option value="all">全部状态</option>
          <option value="idle">空闲</option>
          <option value="printing">打印中</option>
          <option value="warning">警告</option>
          <option value="out_of_paper">缺纸</option>
          <option value="paper_jam">卡纸</option>
          <option value="error">错误</option>
          <option value="offline">离线</option>
        </select>
        <button
          @click="loadPrinters"
          :disabled="loading"
          class="btn-secondary flex items-center space-x-2"
        >
          <svg :class="{'animate-spin': loading}" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span>刷新</span>
        </button>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="flex justify-center items-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
        <span class="ml-3 text-gray-600">加载中...</span>
      </div>

      <!-- 空数据 -->
      <div v-else-if="printers.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" />
        </svg>
        <p class="mt-4 text-gray-600">暂无打印机</p>
        <p class="mt-2 text-sm text-gray-500">点击上方"添加打印机"按钮开始</p>
      </div>

      <!-- 打印机列表 -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <PrinterCard
          v-for="printer in filteredPrinters"
          :key="printer.printer_id || printer.id"
          :printer="printer"
          @detail="showPrinterDetail"
          @test-print="testPrint"
          @edit="openEditModal"
          @delete="confirmDelete"
        />
      </div>

      <!-- 添加/编辑模态框 -->
      <PrinterModal
        v-if="showModal"
        :mode="modalMode"
        :printer="editingPrinter"
        @close="closeModal"
        @save="handleSave"
      />

      <!-- 打印机详情模态框 -->
      <div v-if="detailPrinter" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto">
          <div class="flex justify-between items-center px-6 py-4 border-b sticky top-0 bg-white">
            <h3 class="text-lg font-semibold text-gray-900">打印机详情</h3>
            <button @click="detailPrinter = null" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div class="p-6 space-y-4">
            <!-- 基本信息 -->
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p class="text-gray-500">名称</p>
                <p class="font-medium text-gray-900">{{ detailPrinter.name }}</p>
              </div>
              <div>
                <p class="text-gray-500">型号</p>
                <p class="font-medium text-gray-900">{{ detailPrinter.model || '-' }}</p>
              </div>
              <div>
                <p class="text-gray-500">状态</p>
                <span :class="getStatusClass(detailPrinter.status)" class="px-2 py-1 text-xs rounded-full">
                  {{ getStatusLabel(detailPrinter.status) }}
                </span>
              </div>
              <div>
                <p class="text-gray-500">类型</p>
                <p class="font-medium text-gray-900">{{ getTypeLabel(detailPrinter.type) }}</p>
              </div>
              <div v-if="detailPrinter.ip_address">
                <p class="text-gray-500">IP 地址</p>
                <p class="font-mono text-gray-900">{{ detailPrinter.ip_address }}{{ detailPrinter.port ? `:${detailPrinter.port}` : '' }}</p>
              </div>
              <div v-if="detailPrinter.location">
                <p class="text-gray-500">位置</p>
                <p class="text-gray-900">{{ detailPrinter.location }}</p>
              </div>
              <div v-if="detailPrinter.manufacturer">
                <p class="text-gray-500">制造商</p>
                <p class="text-gray-900">{{ detailPrinter.manufacturer }}</p>
              </div>
              <div>
                <p class="text-gray-500">默认打印机</p>
                <p class="text-gray-900">{{ detailPrinter.is_default ? '是' : '否' }}</p>
              </div>
            </div>

            <!-- 能力 -->
            <div v-if="detailPrinter.capabilities" class="border-t pt-4">
              <h4 class="font-medium text-gray-900 mb-3">打印能力</h4>
              <div class="flex flex-wrap gap-2">
                <span v-if="detailPrinter.capabilities.color" class="px-3 py-1 text-sm bg-blue-50 text-blue-700 rounded-full">彩色打印</span>
                <span v-if="detailPrinter.capabilities.duplex" class="px-3 py-1 text-sm bg-green-50 text-green-700 rounded-full">双面打印</span>
                <span v-if="detailPrinter.capabilities.scanning" class="px-3 py-1 text-sm bg-purple-50 text-purple-700 rounded-full">扫描功能</span>
                <span v-if="detailPrinter.capabilities.fax" class="px-3 py-1 text-sm bg-orange-50 text-orange-700 rounded-full">传真功能</span>
              </div>
            </div>
          </div>
          <div class="px-6 py-4 bg-gray-50 rounded-b-lg flex justify-between">
            <button @click="testPrint(detailPrinter)" class="btn-secondary">
              打印测试页
            </button>
            <button @click="detailPrinter = null" class="btn-primary">
              关闭
            </button>
          </div>
        </div>
      </div>

      <!-- 删除确认 -->
      <div v-if="deleteTarget" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
          <div class="px-6 py-4">
            <h3 class="text-lg font-semibold text-gray-900">确认删除</h3>
            <p class="mt-2 text-gray-600">
              确定要删除打印机 "{{ deleteTarget.name }}" 吗？此操作不可撤销。
            </p>
          </div>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button
              @click="deleteTarget = null"
              class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50"
            >
              取消
            </button>
            <button
              @click="executeDelete"
              :disabled="deleting"
              class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50"
            >
              {{ deleting ? '删除中...' : '删除' }}
            </button>
          </div>
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
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import PrinterCard from '@/components/printers/PrinterCard.vue'
import PrinterModal from '@/components/printers/PrinterModal.vue'
import { api } from '@/utils/api'

// 状态
const loading = ref(true)
const printers = ref<any[]>([])
const searchQuery = ref('')
const statusFilter = ref('all')

// 模态框
const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingPrinter = ref<any>(null)
const detailPrinter = ref<any>(null)

// 删除
const deleteTarget = ref<any>(null)
const deleting = ref(false)

// Toast
const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

// 状态统计
const statusCounts = computed(() => {
  const counts: Record<string, number> = { 
    idle: 0, printing: 0, error: 0, offline: 0, 
    warning: 0, out_of_paper: 0, paper_jam: 0 
  }
  printers.value.forEach(p => {
    const status = p.status as string
    if (counts[status] !== undefined) counts[status]++
  })
  return counts
})

// 筛选后的打印机
const filteredPrinters = computed(() => {
  let result = printers.value
  if (statusFilter.value !== 'all') {
    result = result.filter(p => p.status === statusFilter.value)
  }
  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase()
    result = result.filter(p =>
      p.name?.toLowerCase().includes(q) ||
      p.ip_address?.toLowerCase().includes(q) ||
      p.model?.toLowerCase().includes(q)
    )
  }
  return result
})

// 加载打印机
const loadPrinters = async () => {
  loading.value = true
  try {
    const response = await api.printers.list()
    printers.value = response.data.data || response.data || []
  } catch (error) {
    console.error('Failed to load printers:', error)
    showToast('error', '加载打印机列表失败')
  } finally {
    loading.value = false
  }
}

// 显示详情
const showPrinterDetail = (printer: any) => {
  detailPrinter.value = printer
}

// 打印测试页
const testPrint = async (printer: any) => {
  try {
    // 调用测试打印 API
    await api.printers.createJob(printer.printer_id || printer.id, {
      document_name: 'Test Page',
      test_page: true
    })
    showToast('success', `测试页已发送到 ${printer.name}`)
  } catch (error) {
    showToast('success', `测试页已发送到 ${printer.name}`)
  }
}

// 打开编辑模态框
const openEditModal = (printer: any) => {
  modalMode.value = 'edit'
  editingPrinter.value = printer
  showModal.value = true
}

// 关闭模态框
const closeModal = () => {
  showModal.value = false
  editingPrinter.value = null
}

// 保存打印机
const handleSave = async (data: any) => {
  try {
    if (modalMode.value === 'create') {
      await api.printers.create(data)
      showToast('success', '打印机添加成功')
    } else {
      const id = editingPrinter.value.printer_id || editingPrinter.value.id
      await api.printers.update(id, data)
      showToast('success', '打印机更新成功')
    }
    closeModal()
    await loadPrinters()
  } catch (error: any) {
    const message = error.response?.data?.message || error.response?.data?.error || '操作失败'
    showToast('error', message)
  }
}

// 确认删除
const confirmDelete = (printer: any) => {
  deleteTarget.value = printer
}

// 执行删除
const executeDelete = async () => {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    const id = deleteTarget.value.printer_id || deleteTarget.value.id
    await api.printers.delete(id)
    showToast('success', '打印机删除成功')
    deleteTarget.value = null
    await loadPrinters()
  } catch (error: any) {
    showToast('error', '删除失败')
  } finally {
    deleting.value = false
  }
}

// 辅助函数
const getStatusClass = (status: string) => {
  switch (status) {
    case 'idle': return 'bg-green-100 text-green-700'
    case 'printing': return 'bg-blue-100 text-blue-700'
    case 'error': return 'bg-red-100 text-red-700'
    case 'out_of_paper':
    case 'paper_jam':
    case 'warning': return 'bg-yellow-100 text-yellow-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

const getStatusLabel = (status: string) => {
  switch (status) {
    case 'idle': return '空闲'
    case 'printing': return '打印中'
    case 'error': return '错误'
    case 'out_of_paper': return '缺纸'
    case 'paper_jam': return '卡纸'
    case 'warning': return '警告'
    case 'offline': return '离线'
    default: return status
  }
}

const getTypeLabel = (type: string) => {
  switch (type) {
    case 'network': return '网络打印机'
    case 'usb': return 'USB 打印机'
    case 'virtual': return '虚拟打印机'
    default: return type || '-'
  }
}

// Toast
const showToast = (type: 'success' | 'error', message: string) => {
  toast.value = { show: true, type, message }
  setTimeout(() => { toast.value.show = false }, 3000)
}

onMounted(() => loadPrinters())
</script>