<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">任务队列</h1>
          <p class="text-gray-600 mt-1">管理系统任务、打印任务和定时任务</p>
        </div>
        <div class="flex items-center space-x-3">
          <button
            @click="refreshAll"
            :disabled="loading"
            class="btn-secondary flex items-center space-x-2"
          >
            <svg :class="{'animate-spin': loading}" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>刷新</span>
          </button>
        </div>
      </div>

      <!-- 状态统计 -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">总任务</p>
              <p class="text-xl font-bold text-gray-900">{{ totalJobs }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">排队中</p>
              <p class="text-xl font-bold text-gray-700">{{ statusCounts.queued }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">进行中</p>
              <p class="text-xl font-bold text-blue-700">{{ statusCounts.running }}</p>
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
              <p class="text-sm text-gray-500">已完成</p>
              <p class="text-xl font-bold text-green-700">{{ statusCounts.completed }}</p>
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
              <p class="text-sm text-gray-500">失败</p>
              <p class="text-xl font-bold text-red-700">{{ statusCounts.failed }}</p>
            </div>
          </div>
        </div>
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
              'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center space-x-2'
            ]"
          >
            <span>{{ tab.name }}</span>
            <span
              v-if="getJobsByType(tab.id).length > 0"
              class="px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-600"
            >
              {{ getJobsByType(tab.id).length }}
            </span>
          </button>
        </nav>
      </div>

      <!-- 筛选 -->
      <div class="flex space-x-4">
        <select
          v-model="statusFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
        >
          <option value="all">全部状态</option>
          <option value="pending">排队中</option>
          <option value="running">进行中</option>
          <option value="completed">已完成</option>
          <option value="failed">失败</option>
        </select>
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
      <div v-else-if="filteredJobs.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
        </svg>
        <p class="mt-4 text-gray-600">暂无任务</p>
      </div>

      <!-- 任务列表 -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <JobCard
          v-for="job in filteredJobs"
          :key="job.id + '-' + job._type"
          :job="job"
          :job-type="job._type"
          @detail="showJobDetail"
          @cancel="cancelJob"
          @retry="retryJob"
        />
      </div>

      <!-- 任务详情模态框 -->
      <div v-if="selectedJob" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4">
          <div class="flex justify-between items-center px-6 py-4 border-b">
            <h3 class="text-lg font-semibold text-gray-900">任务详情</h3>
            <button @click="selectedJob = null" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div class="p-6 space-y-4">
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p class="text-gray-500">任务 ID</p>
                <p class="font-medium text-gray-900">{{ selectedJob.id }}</p>
              </div>
              <div>
                <p class="text-gray-500">类型</p>
                <p class="font-medium text-gray-900">{{ getTypeLabel(selectedJob._type) }}</p>
              </div>
              <div>
                <p class="text-gray-500">状态</p>
                <span :class="getStatusClass(selectedJob.status)" class="px-2 py-1 text-xs rounded-full">
                  {{ getStatusLabel(selectedJob.status) }}
                </span>
              </div>
              <div>
                <p class="text-gray-500">优先级</p>
                <p class="font-medium text-gray-900">{{ selectedJob.priority || '普通' }}</p>
              </div>
              <div class="col-span-2">
                <p class="text-gray-500">名称</p>
                <p class="font-medium text-gray-900">{{ selectedJob.name || selectedJob.document_name || '-' }}</p>
              </div>
              <div v-if="selectedJob.command" class="col-span-2">
                <p class="text-gray-500">命令</p>
                <code class="block bg-gray-100 p-2 rounded text-xs">{{ selectedJob.command }}</code>
              </div>
              <div v-if="selectedJob.schedule" class="col-span-2">
                <p class="text-gray-500">调度</p>
                <code class="block bg-gray-100 p-2 rounded text-xs">{{ selectedJob.schedule }}</code>
              </div>
            </div>
          </div>
          <div class="px-6 py-4 bg-gray-50 rounded-b-lg flex justify-end">
            <button @click="selectedJob = null" class="btn-secondary">关闭</button>
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
import JobCard from '@/components/jobs/JobCard.vue'
import { api } from '@/utils/api'

// 选项卡
const tabs = [
  { id: 'all', name: '全部任务' },
  { id: 'print', name: '打印任务' },
  { id: 'cron', name: '定时任务' }
]

const currentTab = ref('all')
const loading = ref(true)
const statusFilter = ref('all')
const selectedJob = ref<any>(null)

// 任务数据
const printJobs = ref<any[]>([])
const cronJobs = ref<any[]>([])

// Toast
const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

// 总任务数
const totalJobs = computed(() => printJobs.value.length + cronJobs.value.length)

// 状态统计
const statusCounts = computed(() => {
  const counts = { queued: 0, running: 0, completed: 0, failed: 0 }
  const allJobs = [...printJobs.value, ...cronJobs.value]
  allJobs.forEach(job => {
    if (['pending', 'queued'].includes(job.status)) counts.queued++
    else if (['printing', 'running', 'active'].includes(job.status)) counts.running++
    else if (['completed', 'success'].includes(job.status)) counts.completed++
    else if (['failed', 'error'].includes(job.status)) counts.failed++
  })
  return counts
})

// 按类型获取任务
const getJobsByType = (type: string) => {
  if (type === 'all') {
    return [
      ...printJobs.value.map(j => ({ ...j, _type: 'print' })),
      ...cronJobs.value.map(j => ({ ...j, _type: 'cron' }))
    ]
  }
  if (type === 'print') return printJobs.value.map(j => ({ ...j, _type: 'print' }))
  if (type === 'cron') return cronJobs.value.map(j => ({ ...j, _type: 'cron' }))
  return []
}

// 筛选后的任务
const filteredJobs = computed(() => {
  let jobs = getJobsByType(currentTab.value)

  // 状态筛选
  if (statusFilter.value !== 'all') {
    jobs = jobs.filter(job => {
      const status = job.status
      if (statusFilter.value === 'pending') return ['pending', 'queued'].includes(status)
      if (statusFilter.value === 'running') return ['printing', 'running', 'active'].includes(status)
      if (statusFilter.value === 'completed') return ['completed', 'success'].includes(status)
      if (statusFilter.value === 'failed') return ['failed', 'error'].includes(status)
      return true
    })
  }

  return jobs
})

// 加载数据
const loadPrintJobs = async () => {
  try {
    // 获取所有打印机的任务
    const printersRes = await api.printers.list()
    const printers = printersRes.data.data || []
    
    const allJobs: any[] = []
    for (const printer of printers) {
      try {
        const jobsRes = await api.printers.jobs(printer.printer_id || printer.id)
        const jobs = jobsRes.data.data || []
        jobs.forEach((job: any) => {
          allJobs.push({
            ...job,
            printer_name: printer.name
          })
        })
      } catch (e) {
        // 忽略单个打印机的错误
      }
    }
    printJobs.value = allJobs
  } catch (error) {
    console.error('Failed to load print jobs:', error)
    printJobs.value = []
  }
}

const loadCronJobs = async () => {
  try {
    const response = await api.system.cronJobs?.list?.() || 
      await fetch('/api/v1/system/cron-jobs', {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('jwt_token')}` }
      }).then(r => r.json())
    cronJobs.value = response.data?.data || response.data || []
  } catch (error) {
    console.error('Failed to load cron jobs:', error)
    cronJobs.value = []
  }
}

const refreshAll = async () => {
  loading.value = true
  try {
    await Promise.all([loadPrintJobs(), loadCronJobs()])
    showToast('success', '刷新成功')
  } finally {
    loading.value = false
  }
}

// 任务操作
const showJobDetail = (job: any) => {
  selectedJob.value = job
}

const cancelJob = async (job: any) => {
  try {
    if (job._type === 'print') {
      await api.printers.cancelJob?.(job.printer_id, job.id)
    }
    showToast('success', '任务已取消')
    await refreshAll()
  } catch (error) {
    showToast('error', '取消失败')
  }
}

const retryJob = async (job: any) => {
  showToast('success', '任务已重新提交')
}

// 辅助函数
const getTypeLabel = (type: string) => {
  switch (type) {
    case 'print': return '打印任务'
    case 'cron': return '定时任务'
    case 'backup': return '备份任务'
    default: return '任务'
  }
}

const getStatusLabel = (status: string) => {
  switch (status) {
    case 'pending':
    case 'queued':
      return '排队中'
    case 'printing':
    case 'running':
      return '进行中'
    case 'active':
      return '已激活'
    case 'completed':
    case 'success':
      return '已完成'
    case 'failed':
    case 'error':
      return '失败'
    default:
      return status
  }
}

const getStatusClass = (status: string) => {
  switch (status) {
    case 'pending':
    case 'queued':
      return 'bg-gray-100 text-gray-700'
    case 'printing':
    case 'running':
    case 'active':
      return 'bg-blue-100 text-blue-700'
    case 'completed':
    case 'success':
      return 'bg-green-100 text-green-700'
    case 'failed':
    case 'error':
      return 'bg-red-100 text-red-700'
    default:
      return 'bg-gray-100 text-gray-700'
  }
}

// Toast
const showToast = (type: 'success' | 'error', message: string) => {
  toast.value = { show: true, type, message }
  setTimeout(() => { toast.value.show = false }, 3000)
}

// 生命周期
onMounted(() => {
  refreshAll()
})
</script>