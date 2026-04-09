<template>
  <div class="space-y-6">
    <!-- 页面标题和操作栏 -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        打印机管理
      </h1>
      <button
        @click="showAddModal = true"
        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
      >
        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
        </svg>
        添加打印机
      </button>
    </div>

    <!-- 打印机列表 -->
    <div v-if="loading" class="space-y-4">
      <div v-for="i in 3" :key="i" class="animate-pulse">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/4 mb-2"></div>
          <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
        </div>
      </div>
    </div>

    <div v-else-if="printers.length === 0" class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-12 text-center">
      <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"></path>
      </svg>
      <p class="text-lg font-medium text-gray-900 dark:text-white mb-2">暂无打印机</p>
      <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">添加打印机开始使用</p>
      <button
        @click="showAddModal = true"
        class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
      >
        添加打印机
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="printer in printers"
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
            <span class="text-gray-500 dark:text-gray-400">类型</span>
            <span class="text-gray-900 dark:text-white">{{ printer.type }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500 dark:text-gray-400">状态</span>
            <span :class="printer.online ? 'text-green-600 dark:text-green-400' : 'text-gray-600 dark:text-gray-400'">
              {{ printer.online ? '在线' : '离线' }}
            </span>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex justify-end space-x-2">
          <button
            @click="viewPrinter(printer)"
            class="px-3 py-1.5 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            详情
          </button>
          <button
            @click="viewQueue(printer)"
            class="px-3 py-1.5 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            队列
          </button>
          <button
            @click="deletePrinter(printer)"
            class="px-3 py-1.5 border border-red-300 dark:border-red-600 text-sm font-medium rounded-md text-red-700 dark:text-red-300 bg-white dark:bg-gray-800 hover:bg-red-50 dark:hover:bg-red-900/20"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- 添加打印机模态框 -->
    <div v-if="showAddModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">添加打印机</h3>
        </div>
        <div class="px-6 py-4 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              打印机名称 <span class="text-red-500">*</span>
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
              IP 地址 <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formData.ip"
              type="text"
              required
              placeholder="192.168.1.100"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              型号
            </label>
            <input
              v-model="formData.model"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              类型
            </label>
            <select
              v-model="formData.type"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            >
              <option value="激光">激光打印机</option>
              <option value="喷墨">喷墨打印机</option>
              <option value="针式">针式打印机</option>
            </select>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="closeAddModal"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="addPrinter"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
          >
            添加
          </button>
        </div>
      </div>
    </div>

    <!-- 打印机详情模态框 -->
    <div v-if="showDetailModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">打印机详情</h3>
        </div>
        <div class="px-6 py-4 space-y-4">
          <div v-if="selectedPrinter">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">名称</p>
                <p class="text-gray-900 dark:text-white">{{ selectedPrinter.name }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">型号</p>
                <p class="text-gray-900 dark:text-white">{{ selectedPrinter.model }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">IP 地址</p>
                <p class="text-gray-900 dark:text-white">{{ selectedPrinter.ip }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">类型</p>
                <p class="text-gray-900 dark:text-white">{{ selectedPrinter.type }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">状态</p>
                <p :class="selectedPrinter.online ? 'text-green-600 dark:text-green-400' : 'text-gray-600 dark:text-gray-400'">
                  {{ selectedPrinter.online ? '在线' : '离线' }}
                </p>
              </div>
            </div>

            <!-- 耗材状态 -->
            <div class="mt-6">
              <h4 class="text-sm font-medium text-gray-900 dark:text-white mb-3">耗材状态</h4>
              <div class="space-y-3">
                <div>
                  <div class="flex justify-between text-sm mb-1">
                    <span class="text-gray-500 dark:text-gray-400">黑色墨盒</span>
                    <span class="text-gray-900 dark:text-white">75%</span>
                  </div>
                  <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div class="bg-black h-2 rounded-full" style="width: 75%"></div>
                  </div>
                </div>
                <div>
                  <div class="flex justify-between text-sm mb-1">
                    <span class="text-gray-500 dark:text-gray-400">青色墨盒</span>
                    <span class="text-gray-900 dark:text-white">60%</span>
                  </div>
                  <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div class="bg-cyan-500 h-2 rounded-full" style="width: 60%"></div>
                  </div>
                </div>
                <div>
                  <div class="flex justify-between text-sm mb-1">
                    <span class="text-gray-500 dark:text-gray-400">品红墨盒</span>
                    <span class="text-gray-900 dark:text-white">45%</span>
                  </div>
                  <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div class="bg-pink-500 h-2 rounded-full" style="width: 45%"></div>
                  </div>
                </div>
                <div>
                  <div class="flex justify-between text-sm mb-1">
                    <span class="text-gray-500 dark:text-gray-400">黄色墨盒</span>
                    <span class="text-gray-900 dark:text-white">80%</span>
                  </div>
                  <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div class="bg-yellow-500 h-2 rounded-full" style="width: 80%"></div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 计数器 -->
            <div class="mt-6">
              <h4 class="text-sm font-medium text-gray-900 dark:text-white mb-3">计数器</h4>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <p class="text-sm text-gray-500 dark:text-gray-400">总打印页数</p>
                  <p class="text-lg font-medium text-gray-900 dark:text-white">12,345</p>
                </div>
                <div>
                  <p class="text-sm text-gray-500 dark:text-gray-400">本月打印</p>
                  <p class="text-lg font-medium text-gray-900 dark:text-white">1,234</p>
                </div>
                <div>
                  <p class="text-sm text-gray-500 dark:text-gray-400">黑白打印</p>
                  <p class="text-lg font-medium text-gray-900 dark:text-white">10,000</p>
                </div>
                <div>
                  <p class="text-sm text-gray-500 dark:text-gray-400">彩色打印</p>
                  <p class="text-lg font-medium text-gray-900 dark:text-white">2,345</p>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end">
          <button
            @click="closeDetailModal"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            关闭
          </button>
        </div>
      </div>
    </div>

    <!-- 打印队列模态框 -->
    <div v-if="showQueueModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">打印队列</h3>
        </div>
        <div class="px-6 py-4">
          <div v-if="queue.length === 0" class="text-center py-8">
            <p class="text-gray-500 dark:text-gray-400">队列为空</p>
          </div>
          <div v-else class="space-y-3">
            <div
              v-for="task in queue"
              :key="task.id"
              class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium text-gray-900 dark:text-white">{{ task.document }}</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">{{ task.user }} • {{ task.size }}</p>
              </div>
              <div class="flex items-center space-x-2">
                <span
                  class="px-2 py-1 rounded-full text-xs font-medium"
                  :class="statusClasses[task.status]"
                >
                  {{ statusLabels[task.status] }}
                </span>
                <button
                  v-if="task.status === 'waiting'"
                  @click="cancelTask(task)"
                  class="text-red-600 hover:text-red-700"
                >
                  取消
                </button>
              </div>
            </div>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end">
          <button
            @click="closeQueueModal"
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
import { ref, computed, onMounted } from 'vue'
import { useToast } from '../composables/useToast'

const toast = useToast()

const loading = ref(false)
const printers = ref([])
const showAddModal = ref(false)
const showDetailModal = ref(false)
const showQueueModal = ref(false)
const selectedPrinter = ref(null)
const queue = ref([])

const formData = ref({
  name: '',
  ip: '',
  model: '',
  type: '激光'
})

const statusClasses = {
  online: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400',
  offline: 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-400',
  waiting: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400',
  printing: 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400',
  completed: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400'
}

const statusLabels = {
  online: '在线',
  offline: '离线',
  waiting: '等待中',
  printing: '打印中',
  completed: '已完成'
}

const loadPrinters = async () => {
  loading.value = true
  try {
    // 模拟数据
    printers.value = [
      { id: 1, name: '办公室打印机', model: 'HP LaserJet Pro', ip: '192.168.1.100', type: '激光', status: 'online', online: true },
      { id: 2, name: '财务室打印机', model: 'Canon imageCLASS', ip: '192.168.1.101', type: '激光', status: 'online', online: true },
      { id: 3, name: '会议室打印机', model: 'Epson EcoTank', ip: '192.168.1.102', type: '喷墨', status: 'offline', online: false }
    ]
  } catch (error) {
    toast.error('加载打印机列表失败')
  } finally {
    loading.value = false
  }
}

const closeAddModal = () => {
  showAddModal.value = false
  formData.value = { name: '', ip: '', model: '', type: '激光' }
}

const addPrinter = () => {
  // 验证表单
  if (!formData.value.name || !formData.value.ip) {
    toast.error('请填写必填项')
    return
  }

  // 模拟添加
  printers.value.push({
    id: Date.now(),
    name: formData.value.name,
    model: formData.value.model,
    ip: formData.value.ip,
    type: formData.value.type,
    status: 'online',
    online: true
  })

  toast.success('打印机添加成功')
  closeAddModal()
}

const viewPrinter = (printer) => {
  selectedPrinter.value = printer
  showDetailModal.value = true
}

const closeDetailModal = () => {
  showDetailModal.value = false
  selectedPrinter.value = null
}

const viewQueue = (printer) => {
  // 模拟队列数据
  queue.value = [
    { id: 1, document: '报告.pdf', user: 'admin', size: '2.5 MB', status: 'printing' },
    { id: 2, document: '合同.docx', user: 'user1', size: '1.2 MB', status: 'waiting' },
    { id: 3, document: '表格.xlsx', user: 'user2', size: '800 KB', status: 'waiting' }
  ]
  showQueueModal.value = true
}

const closeQueueModal = () => {
  showQueueModal.value = false
}

const cancelTask = (task) => {
  queue.value = queue.value.filter(t => t.id !== task.id)
  toast.success('任务已取消')
}

const deletePrinter = (printer) => {
  if (!confirm(`确定要删除打印机 "${printer.name}" 吗？`)) return
  
  printers.value = printers.value.filter(p => p.id !== printer.id)
  toast.success('打印机已删除')
}

onMounted(() => {
  loadPrinters()
})
</script>
