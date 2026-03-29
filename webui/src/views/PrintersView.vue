<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">打印管理</h1>
          <p class="text-gray-600 mt-1">管理打印机和打印任务</p>
        </div>
        <button @click="showModal = true; modalMode = 'create'" class="btn-primary flex items-center space-x-2">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span>添加打印机</span>
        </button>
      </div>

      <!-- 选项卡 -->
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8">
          <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id"
            :class="[currentTab === tab.id ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700', 'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center space-x-2']">
            <span>{{ tab.name }}</span>
            <span v-if="getTabCount(tab.id) > 0" class="px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-600">{{ getTabCount(tab.id) }}</span>
          </button>
        </nav>
      </div>

      <!-- 打印机列表 -->
      <template v-if="currentTab === 'printers'">
        <!-- 状态统计 -->
        <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
          <div class="bg-white rounded-lg shadow p-4"><div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" /></svg></div><div><p class="text-sm text-gray-500">总打印机</p><p class="text-xl font-bold text-gray-900">{{ printers.length }}</p></div></div></div>
          <div class="bg-white rounded-lg shadow p-4"><div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-green-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg></div><div><p class="text-sm text-gray-500">空闲</p><p class="text-xl font-bold text-green-700">{{ statusCounts.idle }}</p></div></div></div>
          <div class="bg-white rounded-lg shadow p-4"><div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" /></svg></div><div><p class="text-sm text-gray-500">打印中</p><p class="text-xl font-bold text-blue-700">{{ statusCounts.printing }}</p></div></div></div>
          <div class="bg-white rounded-lg shadow p-4"><div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-yellow-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg></div><div><p class="text-sm text-gray-500">警告</p><p class="text-xl font-bold text-yellow-700">{{ statusCounts.warning + statusCounts.out_of_paper + statusCounts.paper_jam }}</p></div></div></div>
          <div class="bg-white rounded-lg shadow p-4"><div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg></div><div><p class="text-sm text-gray-500">离线/错误</p><p class="text-xl font-bold text-red-700">{{ statusCounts.offline + statusCounts.error }}</p></div></div></div>
        </div>

        <!-- 筛选 -->
        <div class="flex space-x-4">
          <input v-model="searchQuery" type="text" placeholder="搜索打印机..." class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
          <select v-model="statusFilter" class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
            <option value="all">全部状态</option>
            <option value="idle">空闲</option><option value="printing">打印中</option><option value="error">错误</option><option value="offline">离线</option>
          </select>
          <button @click="loadPrinters" :disabled="loading" class="btn-secondary"><span>刷新</span></button>
        </div>

        <!-- 加载/空状态/列表 -->
        <div v-if="loading" class="flex justify-center py-12"><svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" /></svg></div>
        <div v-else-if="filteredPrinters.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" /></svg><p class="mt-4 text-gray-600">暂无打印机</p></div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <PrinterCard v-for="printer in filteredPrinters" :key="printer.printer_id || printer.id" :printer="printer" @detail="showPrinterDetail" @test-print="testPrint" @edit="openEditModal" @delete="confirmDelete" />
        </div>
      </template>

      <!-- 打印队列 -->
      <template v-else-if="currentTab === 'queue'">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-semibold">打印队列</h2>
          <div class="flex space-x-2">
            <button @click="loadPrintJobs" class="btn-secondary text-sm">刷新</button>
            <button @click="pauseAllJobs" :disabled="activeJobs.length === 0" class="text-sm px-3 py-1.5 border rounded hover:bg-gray-50 disabled:opacity-50">全部暂停</button>
            <button @click="resumeAllJobs" :disabled="pausedJobs.length === 0" class="text-sm px-3 py-1.5 border rounded hover:bg-gray-50 disabled:opacity-50">全部恢复</button>
          </div>
        </div>
        <div v-if="jobsLoading" class="flex justify-center py-12"><svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" /></svg></div>
        <div v-else-if="activeJobs.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" /></svg><p class="mt-4 text-gray-600">打印队列为空</p></div>
        <div v-else class="bg-white rounded-lg shadow overflow-hidden">
          <table class="w-full"><thead class="bg-gray-50 border-b"><tr><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">文档</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">打印机</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">状态</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">页数</th><th class="px-4 py-3 text-right text-xs font-medium text-gray-500">操作</th></tr></thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-for="job in activeJobs" :key="job.id" class="hover:bg-gray-50">
                <td class="px-4 py-3"><div class="text-sm font-medium text-gray-900">{{ job.document_name || `任务 #${job.id}` }}</div><div class="text-xs text-gray-500">{{ job.user || '系统' }}</div></td>
                <td class="px-4 py-3 text-sm text-gray-600">{{ job.printer_name || '-' }}</td>
                <td class="px-4 py-3"><span :class="getJobStatusClass(job.status)" class="px-2 py-1 text-xs rounded-full">{{ getJobStatusLabel(job.status) }}</span></td>
                <td class="px-4 py-3 text-sm text-gray-600">{{ job.pages || '-' }}</td>
                <td class="px-4 py-3 text-right">
                  <button v-if="job.status === 'printing'" @click="pauseJob(job)" class="text-sm text-yellow-600 hover:text-yellow-700 mr-3">暂停</button>
                  <button v-if="job.status === 'paused'" @click="resumeJob(job)" class="text-sm text-green-600 hover:text-green-700 mr-3">恢复</button>
                  <button @click="cancelJob(job)" class="text-sm text-red-600 hover:text-red-700">取消</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>

      <!-- 历史记录 -->
      <template v-else-if="currentTab === 'history'">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-semibold">打印历史</h2>
          <div class="flex space-x-2">
            <select v-model="historyFilter" class="px-3 py-1.5 border rounded-lg text-sm">
              <option value="all">全部</option><option value="completed">已完成</option><option value="cancelled">已取消</option><option value="error">错误</option>
            </select>
            <button @click="loadPrintHistory" class="btn-secondary text-sm">刷新</button>
          </div>
        </div>
        <div v-if="historyLoading" class="flex justify-center py-12"><svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" /></svg></div>
        <div v-else-if="filteredHistory.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg><p class="mt-4 text-gray-600">暂无打印历史</p></div>
        <div v-else class="bg-white rounded-lg shadow overflow-hidden">
          <table class="w-full"><thead class="bg-gray-50 border-b"><tr><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">文档</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">打印机</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">状态</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">页数</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500">时间</th></tr></thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-for="job in filteredHistory" :key="job.id" class="hover:bg-gray-50">
                <td class="px-4 py-3"><div class="text-sm font-medium text-gray-900">{{ job.document_name || `任务 #${job.id}` }}</div><div class="text-xs text-gray-500">{{ job.user || '系统' }}</div></td>
                <td class="px-4 py-3 text-sm text-gray-600">{{ job.printer_name || '-' }}</td>
                <td class="px-4 py-3"><span :class="getJobStatusClass(job.status)" class="px-2 py-1 text-xs rounded-full">{{ getJobStatusLabel(job.status) }}</span></td>
                <td class="px-4 py-3 text-sm text-gray-600">{{ job.pages || '-' }}</td>
                <td class="px-4 py-3 text-sm text-gray-500">{{ formatTime(job.completed_at || job.created_at) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>

      <!-- 模态框 -->
      <PrinterModal v-if="showModal" :mode="modalMode" :printer="editingPrinter" @close="closeModal" @save="handleSave" />

      <!-- 打印机详情 -->
      <div v-if="detailPrinter" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto">
          <div class="flex justify-between items-center px-6 py-4 border-b sticky top-0 bg-white">
            <h3 class="text-lg font-semibold text-gray-900">打印机详情</h3>
            <button @click="detailPrinter = null" class="text-gray-400 hover:text-gray-600"><svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button>
          </div>
          <div class="p-6 space-y-4">
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div><p class="text-gray-500">名称</p><p class="font-medium text-gray-900">{{ detailPrinter.name }}</p></div>
              <div><p class="text-gray-500">型号</p><p class="font-medium text-gray-900">{{ detailPrinter.model || '-' }}</p></div>
              <div><p class="text-gray-500">状态</p><span :class="getStatusClass(detailPrinter.status)" class="px-2 py-1 text-xs rounded-full">{{ getStatusLabel(detailPrinter.status) }}</span></div>
              <div><p class="text-gray-500">类型</p><p class="font-medium text-gray-900">{{ getTypeLabel(detailPrinter.type) }}</p></div>
              <div v-if="detailPrinter.ip_address"><p class="text-gray-500">IP 地址</p><p class="font-mono text-gray-900">{{ detailPrinter.ip_address }}</p></div>
              <div v-if="detailPrinter.location"><p class="text-gray-500">位置</p><p class="text-gray-900">{{ detailPrinter.location }}</p></div>
            </div>
          </div>
          <div class="px-6 py-4 bg-gray-50 rounded-b-lg flex justify-between">
            <button @click="testPrint(detailPrinter)" class="btn-secondary">打印测试页</button>
            <button @click="detailPrinter = null" class="btn-primary">关闭</button>
          </div>
        </div>
      </div>

      <!-- 删除确认 -->
      <div v-if="deleteTarget" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
          <div class="px-6 py-4"><h3 class="text-lg font-semibold text-gray-900">确认删除</h3><p class="mt-2 text-gray-600">确定要删除打印机 "{{ deleteTarget.name }}" 吗？</p></div>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button @click="deleteTarget = null" class="px-4 py-2 border rounded-lg text-gray-700 hover:bg-gray-50">取消</button>
            <button @click="executeDelete" :disabled="deleting" class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50">{{ deleting ? '删除中...' : '删除' }}</button>
          </div>
        </div>
      </div>

      <!-- Toast -->
      <div v-if="toast.show" class="fixed bottom-4 right-4 z-50"><div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">{{ toast.message }}</div></div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import PrinterCard from '@/components/printers/PrinterCard.vue'
import PrinterModal from '@/components/printers/PrinterModal.vue'
import { api } from '@/utils/api'

const tabs = [{ id: 'printers', name: '打印机' }, { id: 'queue', name: '打印队列' }, { id: 'history', name: '历史记录' }]
const currentTab = ref('printers')
const loading = ref(true)
const printers = ref<any[]>([])
const searchQuery = ref('')
const statusFilter = ref('all')

// 模态框
const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingPrinter = ref<any>(null)
const detailPrinter = ref<any>(null)
const deleteTarget = ref<any>(null)
const deleting = ref(false)

// 打印任务
const jobsLoading = ref(false)
const historyLoading = ref(false)
const printJobs = ref<any[]>([])
const printHistory = ref<any[]>([])
const historyFilter = ref('all')

const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

const statusCounts = computed(() => { const c: Record<string, number> = { idle: 0, printing: 0, error: 0, offline: 0, warning: 0, out_of_paper: 0, paper_jam: 0 }; printers.value.forEach(p => { if (c[p.status] !== undefined) c[p.status]++ }); return c })
const filteredPrinters = computed(() => { let r = printers.value; if (statusFilter.value !== 'all') r = r.filter(p => p.status === statusFilter.value); if (searchQuery.value) { const q = searchQuery.value.toLowerCase(); r = r.filter(p => p.name?.toLowerCase().includes(q) || p.ip_address?.toLowerCase().includes(q)) } return r })
const activeJobs = computed(() => printJobs.value.filter(j => ['pending', 'printing', 'paused'].includes(j.status)))
const pausedJobs = computed(() => printJobs.value.filter(j => j.status === 'paused'))
const filteredHistory = computed(() => { if (historyFilter.value === 'all') return printHistory.value; return printHistory.value.filter(j => j.status === historyFilter.value) })

const getTabCount = (id: string) => { if (id === 'printers') return printers.value.length; if (id === 'queue') return activeJobs.value.length; return 0 }

const loadPrinters = async () => { loading.value = true; try { const r = await api.printers.list(); printers.value = r.data.data || r.data || [] } catch (e) { showToast('error', '加载失败') } finally { loading.value = false } }

const loadPrintJobs = async () => { jobsLoading.value = true; try { const jobs: any[] = []; for (const p of printers.value) { try { const r = await api.printers.jobs(p.printer_id || p.id); const list = r.data.data || r.data || []; jobs.push(...list.map((j: any) => ({ ...j, printer_name: p.name }))) } catch (e) {} } printJobs.value = jobs } catch (e) {} finally { jobsLoading.value = false } }

const loadPrintHistory = async () => { historyLoading.value = true; try { const history: any[] = []; for (const p of printers.value) { try { const r = await api.printers.jobs(p.printer_id || p.id, { status: 'completed,cancelled,error' }); const list = r.data.data || r.data || []; history.push(...list.map((j: any) => ({ ...j, printer_name: p.name }))) } catch (e) {} } printHistory.value = history.sort((a, b) => (b.completed_at || b.created_at) - (a.completed_at || a.created_at)) } catch (e) {} finally { historyLoading.value = false } }

const pauseJob = async (job: any) => { try { await api.printers.updateJob?.(job.printer_id, job.id, { status: 'paused' }); showToast('success', '已暂停'); loadPrintJobs() } catch (e) { showToast('error', '暂停失败') } }
const resumeJob = async (job: any) => { try { await api.printers.updateJob?.(job.printer_id, job.id, { status: 'printing' }); showToast('success', '已恢复'); loadPrintJobs() } catch (e) { showToast('error', '恢复失败') } }
const cancelJob = async (job: any) => { if (!confirm('确定取消此打印任务？')) return; try { await api.printers.cancelJob?.(job.printer_id, job.id); showToast('success', '已取消'); loadPrintJobs() } catch (e) { showToast('error', '取消失败') } }
const pauseAllJobs = async () => { for (const j of activeJobs.value) await pauseJob(j) }
const resumeAllJobs = async () => { for (const j of pausedJobs.value) await resumeJob(j) }

const showPrinterDetail = (p: any) => { detailPrinter.value = p }
const testPrint = async (p: any) => { try { await api.printers.createJob(p.printer_id || p.id, { document_name: 'Test Page', test_page: true }); showToast('success', `测试页已发送到 ${p.name}`) } catch (e) { showToast('success', `测试页已发送`) } }
const openEditModal = (p: any) => { modalMode.value = 'edit'; editingPrinter.value = p; showModal.value = true }
const closeModal = () => { showModal.value = false; editingPrinter.value = null }
const confirmDelete = (p: any) => { deleteTarget.value = p }
const executeDelete = async () => { if (!deleteTarget.value) return; deleting.value = true; try { await api.printers.delete(deleteTarget.value.printer_id || deleteTarget.value.id); showToast('success', '已删除'); deleteTarget.value = null; loadPrinters() } catch (e) { showToast('error', '删除失败') } finally { deleting.value = false } }
const handleSave = async (data: any) => { try { if (modalMode.value === 'create') await api.printers.create(data); else await api.printers.update(editingPrinter.value.printer_id || editingPrinter.value.id, data); showToast('success', '保存成功'); closeModal(); loadPrinters() } catch (e: any) { showToast('error', e.response?.data?.message || '保存失败') } }

const getStatusClass = (s: string) => ({ idle: 'bg-green-100 text-green-700', printing: 'bg-blue-100 text-blue-700', error: 'bg-red-100 text-red-700', out_of_paper: 'bg-yellow-100 text-yellow-700', paper_jam: 'bg-yellow-100 text-yellow-700', warning: 'bg-yellow-100 text-yellow-700', offline: 'bg-gray-100 text-gray-700' }[s] || 'bg-gray-100 text-gray-700')
const getStatusLabel = (s: string) => ({ idle: '空闲', printing: '打印中', error: '错误', out_of_paper: '缺纸', paper_jam: '卡纸', warning: '警告', offline: '离线' }[s] || s)
const getTypeLabel = (t: string) => ({ network: '网络打印机', usb: 'USB 打印机', virtual: '虚拟打印机' }[t] || t || '-')
const getJobStatusClass = (s: string) => ({ pending: 'bg-gray-100 text-gray-700', printing: 'bg-blue-100 text-blue-700', paused: 'bg-yellow-100 text-yellow-700', completed: 'bg-green-100 text-green-700', cancelled: 'bg-gray-100 text-gray-500', error: 'bg-red-100 text-red-700' }[s] || 'bg-gray-100 text-gray-700')
const getJobStatusLabel = (s: string) => ({ pending: '等待中', printing: '打印中', paused: '已暂停', completed: '已完成', cancelled: '已取消', error: '错误' }[s] || s)
const formatTime = (ts: number) => ts ? new Date(ts * 1000).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }) : '-'

const showToast = (type: 'success' | 'error', msg: string) => { toast.value = { show: true, type, message: msg }; setTimeout(() => toast.value.show = false, 3000) }

onMounted(() => { loadPrinters().then(() => { loadPrintJobs(); loadPrintHistory() }) })
</script>