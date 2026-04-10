<template>
  <div class="px-4 py-6 sm:px-0">
    <!-- 页面标题 -->
    <div class="mb-6">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-2xl font-bold text-gray-900">📦 备份管理</h2>
          <p class="text-gray-600 mt-1">管理备份任务和执行历史</p>
        </div>
        <button
          @click="showCreateModal = true"
          class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
        >
          ➕ 新建备份任务
        </button>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-6">
      <div class="bg-white overflow-hidden shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <dt class="text-sm font-medium text-gray-500 truncate">总任务数</dt>
          <dd class="mt-1 text-3xl font-semibold text-gray-900">{{ totalTasks }}</dd>
        </div>
      </div>
      <div class="bg-white overflow-hidden shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <dt class="text-sm font-medium text-gray-500 truncate">活跃任务</dt>
          <dd class="mt-1 text-3xl font-semibold text-green-600">{{ activeTasks }}</dd>
        </div>
      </div>
      <div class="bg-white overflow-hidden shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <dt class="text-sm font-medium text-gray-500 truncate">最近执行</dt>
          <dd class="mt-1 text-3xl font-semibold text-blue-600">{{ recentExecutions }}</dd>
        </div>
      </div>
      <div class="bg-white overflow-hidden shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <dt class="text-sm font-medium text-gray-500 truncate">总备份大小</dt>
          <dd class="mt-1 text-3xl font-semibold text-purple-600">{{ formatSize(totalBackupSize) }}</dd>
        </div>
      </div>
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

    <!-- 备份任务列表 -->
    <div v-if="currentTab === 'tasks'" class="space-y-6">
      <!-- 搜索和筛选 -->
      <div class="bg-white shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <div class="flex flex-col md:flex-row md:items-center md:space-x-4 space-y-4 md:space-y-0">
            <div class="flex-1">
              <input
                v-model="searchQuery"
                @input="debouncedSearch"
                type="text"
                class="focus:ring-primary-500 focus:border-primary-500 block w-full sm:text-sm border-gray-300 rounded-md"
                placeholder="搜索备份任务..."
              />
            </div>
            <select
              v-model="statusFilter"
              @change="fetchBackupTasks"
              class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm rounded-md"
            >
              <option value="">全部状态</option>
              <option value="active">活跃</option>
              <option value="inactive">非活跃</option>
              <option value="running">运行中</option>
            </select>
            <button
              @click="refreshTasks"
              class="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
            >
              🔄 刷新
            </button>
          </div>
        </div>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="text-center py-12">
        <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        <p class="mt-2 text-gray-600">加载中...</p>
      </div>

      <!-- 错误提示 -->
      <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
        {{ error }}
      </div>

      <!-- 任务列表 -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div
          v-for="task in backupTasks"
          :key="task.id"
          class="bg-white shadow rounded-lg p-6"
        >
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-medium text-gray-900">{{ task.name }}</h3>
            <span
              :class="{
                'bg-green-100 text-green-800': task.status === 'active',
                'bg-gray-100 text-gray-800': task.status === 'inactive',
                'bg-blue-100 text-blue-800': task.status === 'running',
                'bg-yellow-100 text-yellow-800': task.status === 'pending',
              }"
              class="px-2 py-1 text-xs font-medium rounded-full"
            >
              {{ getStatusLabel(task.status) }}
            </span>
          </div>

          <p class="text-sm text-gray-600 mb-4">{{ task.description || '无描述' }}</p>

          <div class="space-y-2 text-sm text-gray-600 mb-4">
            <div class="flex justify-between">
              <span>源路径:</span>
              <span class="font-medium truncate max-w-xs">{{ task.source_path }}</span>
            </div>
            <div class="flex justify-between">
              <span>目标路径:</span>
              <span class="font-medium truncate max-w-xs">{{ task.destination_path }}</span>
            </div>
            <div class="flex justify-between">
              <span>调度:</span>
              <span class="font-medium">{{ task.schedule || '手动' }}</span>
            </div>
            <div class="flex justify-between">
              <span>上次执行:</span>
              <span class="font-medium">{{ formatTime(task.last_run) }}</span>
            </div>
          </div>

          <div class="flex items-center justify-between pt-4 border-t border-gray-200">
            <span class="text-xs text-gray-500">创建于 {{ formatDate(task.created_at) }}</span>
            <div class="flex space-x-2">
              <button
                @click="executeBackup(task)"
                :disabled="task.status === 'running'"
                class="text-green-600 hover:text-green-800 text-sm disabled:opacity-50"
                title="立即执行"
              >
                ▶️ 执行
              </button>
              <button
                @click="editBackup(task)"
                class="text-blue-600 hover:text-blue-800 text-sm"
              >
                ✏️ 编辑
              </button>
              <button
                @click="viewHistory(task)"
                class="text-purple-600 hover:text-purple-800 text-sm"
              >
                📋 历史
              </button>
              <button
                @click="deleteBackup(task)"
                class="text-red-600 hover:text-red-800 text-sm"
              >
                🗑️ 删除
              </button>
            </div>
          </div>
        </div>

        <div v-if="backupTasks.length === 0" class="col-span-full text-center py-12 text-gray-500">
          <p class="text-4xl mb-4">📦</p>
          <p>暂无备份任务</p>
        </div>
      </div>

      <!-- 分页 -->
      <div v-if="!loading && pagination.total_pages > 1" class="mt-4 flex items-center justify-between">
        <span class="text-sm text-gray-600">
          第 {{ pagination.page }} / {{ pagination.total_pages }} 页，共 {{ pagination.total }} 项
        </span>
        <div class="flex space-x-2">
          <button
            @click="changePage(pagination.page - 1)"
            :disabled="pagination.page <= 1"
            class="px-3 py-1 border border-gray-300 rounded text-sm disabled:opacity-50"
          >
            上一页
          </button>
          <button
            @click="changePage(pagination.page + 1)"
            :disabled="pagination.page >= pagination.total_pages"
            class="px-3 py-1 border border-gray-300 rounded text-sm disabled:opacity-50"
          >
            下一页
          </button>
        </div>
      </div>
    </div>

    <!-- 执行历史 -->
    <div v-if="currentTab === 'history'" class="space-y-6">
      <div class="bg-white shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">执行历史</h3>

          <div v-if="historyLoading" class="text-center py-8">
            <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
            <p class="mt-2 text-gray-600">加载中...</p>
          </div>

          <div v-else class="overflow-x-auto">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-gray-50">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">任务名称</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">状态</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">开始时间</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">结束时间</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">耗时</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">数据量</th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr v-for="exec in executions" :key="exec.id" class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {{ exec.task_name }}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      :class="{
                        'bg-green-100 text-green-800': exec.status === 'completed',
                        'bg-red-100 text-red-800': exec.status === 'failed',
                        'bg-blue-100 text-blue-800': exec.status === 'running',
                      }"
                      class="px-2 py-1 text-xs font-medium rounded-full"
                    >
                      {{ exec.status === 'completed' ? '成功' : exec.status === 'failed' ? '失败' : '运行中' }}
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {{ formatDateTime(exec.started_at) }}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {{ formatDateTime(exec.completed_at) }}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {{ formatDuration(exec.duration_seconds) }}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {{ formatSize(exec.bytes_processed) }}
                  </td>
                </tr>
              </tbody>
            </table>

            <div v-if="executions.length === 0" class="text-center py-8 text-gray-500">
              暂无执行记录
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 新建/编辑备份任务模态框 -->
    <div v-if="showCreateModal || showEditModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center">
      <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h3 class="text-lg font-medium text-gray-900 mb-4">
          {{ showEditModal ? '编辑备份任务' : '新建备份任务' }}
        </h3>
        <form @submit.prevent="submitBackupForm" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">任务名称</label>
            <input
              v-model="formData.name"
              type="text"
              required
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">描述</label>
            <textarea
              v-model="formData.description"
              rows="2"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            ></textarea>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">源路径</label>
            <input
              v-model="formData.source_path"
              type="text"
              required
              placeholder="/path/to/source"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">目标路径</label>
            <input
              v-model="formData.destination_path"
              type="text"
              required
              placeholder="/path/to/destination"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">备份类型</label>
            <select
              v-model="formData.backup_type"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="full">全量备份</option>
              <option value="incremental">增量备份</option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">调度 (Cron 表达式，可选)</label>
            <input
              v-model="formData.schedule"
              type="text"
              placeholder="0 2 * * * (每天 2 点)"
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <div class="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              @click="closeModal"
              class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
            >
              取消
            </button>
            <button
              type="submit"
              :disabled="submitting"
              class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
            >
              {{ submitting ? '保存中...' : '保存' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- 确认对话框 -->
    <div v-if="showConfirmDialog" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">{{ confirmDialogTitle }}</h3>
        <p class="text-sm text-gray-600 mb-6">{{ confirmDialogMessage }}</p>
        <div class="flex justify-end space-x-3">
          <button
            @click="handleCancelConfirm"
            class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
          >
            取消
          </button>
          <button
            @click="handleConfirm"
            :class="[
              'px-4 py-2 border border-transparent rounded-md text-sm font-medium text-white',
              confirmDialogDanger ? 'bg-red-600 hover:bg-red-700' : 'bg-primary-600 hover:bg-primary-700'
            ]"
          >
            确认
          </button>
        </div>
      </div>
    </div>

    <!-- 进度条对话框 -->
    <div v-if="executingTask" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-2">备份执行中</h3>
        <p class="text-sm text-gray-600 mb-4">{{ executingTask.name }}</p>
        
        <!-- 进度条 -->
        <div class="mb-4">
          <div class="flex justify-between text-sm text-gray-600 mb-1">
            <span>进度</span>
            <span>{{ Math.round(executionProgress) }}%</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5">
            <div 
              class="bg-primary-600 h-2.5 rounded-full transition-all duration-300"
              :style="{ width: executionProgress + '%' }"
            ></div>
          </div>
        </div>
        
        <!-- 剩余时间 -->
        <div class="text-sm text-gray-500 mb-4">
          剩余时间: {{ executionEta }}
        </div>
        
        <div class="flex justify-end">
          <button
            @click="executingTask = null; executionProgress = 0"
            class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import apiClient from '../api/client';
import { useToast } from '../composables/useToast';

interface BackupTask {
  id: number;
  name: string;
  description: string;
  source_path: string;
  destination_path: string;
  schedule: string;
  backup_type: string;
  status: string;
  last_run?: number;
  next_run?: number;
  created_at: number;
  updated_at: number;
}

interface ExecutionHistory {
  id: number;
  task_id: number;
  task_name: string;
  status: string;
  started_at: number;
  completed_at?: number;
  duration_seconds?: number;
  bytes_processed?: number;
  error_message?: string;
}

interface Pagination {
  page: number;
  per_page: number;
  total: number;
  total_pages: number;
}

const tabs = [
  { id: 'tasks', name: '📦 备份任务' },
  { id: 'history', name: '📋 执行历史' },
];

const currentTab = ref('tasks');
const loading = ref(true);
const error = ref<string | null>(null);
const searchQuery = ref('');
const statusFilter = ref('');
const pagination = ref<Pagination>({
  page: 1,
  per_page: 20,
  total: 0,
  total_pages: 1,
});

const backupTasks = ref<BackupTask[]>([]);
const executions = ref<ExecutionHistory[]>([]);
const historyLoading = ref(false);

// 模态框状态
const showCreateModal = ref(false);
const showEditModal = ref(false);
const submitting = ref(false);
const editingTask = ref<BackupTask | null>(null);

const formData = ref({
  name: '',
  description: '',
  source_path: '',
  destination_path: '',
  backup_type: 'full',
  schedule: '',
});

// Toast 提示
const { showToast } = useToast();

// 确认对话框状态
const showConfirmDialog = ref(false);
const confirmDialogTitle = ref('');
const confirmDialogMessage = ref('');
const confirmDialogAction = ref<(() => void) | null>(null);
const confirmDialogDanger = ref(false);

// 进度条状态
const executingTask = ref<BackupTask | null>(null);
const executionProgress = ref(0);
const executionEta = ref('');

// 计算属性
const totalTasks = computed(() => pagination.value.total);
const activeTasks = computed(() => backupTasks.value.filter(t => t.status === 'active').length);
const recentExecutions = computed(() => executions.value.filter(e => {
  const now = Date.now() / 1000;
  return now - e.started_at < 86400; // 24 小时内
}).length);
const totalBackupSize = computed(() => {
  return executions.value.reduce((sum, e) => sum + (e.bytes_processed || 0), 0);
});

// 获取备份任务列表
const fetchBackupTasks = async () => {
  loading.value = true;
  error.value = null;
  try {
    const response = await apiClient.getBackups({
      page: pagination.value.page,
      per_page: pagination.value.per_page,
      status: statusFilter.value || undefined,
    });
    if (response.success && response.data) {
      backupTasks.value = response.data.items || response.data || [];
      pagination.value = response.pagination || pagination.value;
    }
  } catch (err) {
    error.value = '加载备份任务列表失败';
    console.error('Failed to fetch backup tasks:', err);
  } finally {
    loading.value = false;
  }
};

// 获取执行历史
const fetchExecutionHistory = async () => {
  historyLoading.value = true;
  try {
    const response = await apiClient.getBackupExecutionHistory();
    if (response.success && response.data) {
      executions.value = response.data.items || response.data || [];
    }
  } catch (err) {
    console.error('Failed to fetch execution history:', err);
  } finally {
    historyLoading.value = false;
  }
};

// 刷新任务列表
const refreshTasks = () => {
  fetchBackupTasks();
};

// 搜索（防抖）
let searchTimeout: number | null = null;
const debouncedSearch = () => {
  if (searchTimeout) clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => {
    pagination.value.page = 1;
    fetchBackupTasks();
  }, 300);
};

// 分页
const changePage = (page: number) => {
  if (page < 1 || page > pagination.value.total_pages) return;
  pagination.value.page = page;
  fetchBackupTasks();
};

// 执行备份
// 显示确认对话框
const openConfirmDialog = (title: string, message: string, action: () => void, danger = false) => {
  confirmDialogTitle.value = title;
  confirmDialogMessage.value = message;
  confirmDialogAction.value = action;
  confirmDialogDanger.value = danger;
  showConfirmDialog.value = true;
};

const handleConfirm = () => {
  if (confirmDialogAction.value) {
    confirmDialogAction.value();
  }
  showConfirmDialog.value = false;
};

const handleCancelConfirm = () => {
  showConfirmDialog.value = false;
  confirmDialogAction.value = null;
};

// 模拟进度更新
const startProgressSimulation = (task: BackupTask) => {
  executingTask.value = task;
  executionProgress.value = 0;
  executionEta.value = '估算中...';
  
  const interval = setInterval(() => {
    if (executionProgress.value < 100) {
      executionProgress.value += Math.random() * 15;
      if (executionProgress.value > 100) executionProgress.value = 100;
      
      const remaining = Math.ceil((100 - executionProgress.value) / 10);
      executionEta.value = remaining > 0 ? `约 ${remaining} 秒` : '即将完成';
    } else {
      clearInterval(interval);
      showToast('备份任务执行完成', 'success');
      executingTask.value = null;
      executionProgress.value = 0;
      fetchBackupTasks();
    }
  }, 1000);
};

const executeBackup = async (task: BackupTask) => {
  openConfirmDialog(
    '确认执行备份',
    `确定要立即执行备份任务 "${task.name}" 吗？`,
    async () => {
      try {
        await apiClient.executeBackup(task.id);
        showToast('备份任务已开始执行', 'info');
        startProgressSimulation(task);
      } catch (err) {
        showToast('执行备份失败', 'error');
        console.error('Execute backup failed:', err);
      }
    }
  );
};

// 编辑备份
const editBackup = (task: BackupTask) => {
  editingTask.value = task;
  formData.value = {
    name: task.name,
    description: task.description,
    source_path: task.source_path,
    destination_path: task.destination_path,
    backup_type: task.backup_type || 'full',
    schedule: task.schedule,
  };
  showEditModal.value = true;
};

// 查看历史
const viewHistory = (task: BackupTask) => {
  currentTab.value = 'history';
  fetchExecutionHistory();
};

// 删除备份
const deleteBackup = async (task: BackupTask) => {
  openConfirmDialog(
    '确认删除备份',
    `确定要删除备份任务 "${task.name}" 吗？此操作不可恢复！`,
    async () => {
      try {
        await apiClient.deleteBackup(task.id);
        showToast('备份任务已删除', 'success');
        await fetchBackupTasks();
      } catch (err) {
        showToast('删除备份任务失败', 'error');
        console.error('Delete backup failed:', err);
      }
    },
    true // danger = true
  );
};

// 提交表单
const submitBackupForm = async () => {
  submitting.value = true;
  try {
    if (showEditModal.value && editingTask.value) {
      await apiClient.updateBackup(editingTask.value.id, formData.value);
      showToast('备份任务已更新', 'success');
    } else {
      await apiClient.createBackup(formData.value);
      showToast('备份任务已创建', 'success');
    }
    closeModal();
    await fetchBackupTasks();
  } catch (err) {
    showToast(showEditModal.value ? '更新备份任务失败' : '创建备份任务失败', 'error');
    console.error('Submit backup form failed:', err);
  } finally {
    submitting.value = false;
  }
};

// 关闭模态框
const closeModal = () => {
  showCreateModal.value = false;
  showEditModal.value = false;
  editingTask.value = null;
  formData.value = {
    name: '',
    description: '',
    source_path: '',
    destination_path: '',
    backup_type: 'full',
    schedule: '',
  };
};

// 工具函数
const getStatusLabel = (status: string): string => {
  const labels: Record<string, string> = {
    active: '活跃',
    inactive: '非活跃',
    running: '运行中',
    pending: '等待中',
    completed: '已完成',
    failed: '失败',
  };
  return labels[status] || status;
};

const formatTime = (timestamp?: number): string => {
  if (!timestamp) return '从未';
  return new Date(timestamp * 1000).toLocaleString('zh-CN');
};

const formatDate = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleDateString('zh-CN');
};

const formatDateTime = (timestamp: number): string => {
  if (!timestamp) return '-';
  return new Date(timestamp * 1000).toLocaleString('zh-CN');
};

const formatDuration = (seconds?: number): string => {
  if (!seconds) return '-';
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  if (mins > 0) return `${mins}分${secs}秒`;
  return `${secs}秒`;
};

const formatSize = (bytes?: number): string => {
  if (!bytes || bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

onMounted(() => {
  fetchBackupTasks();
  fetchExecutionHistory();
});
</script>

<style scoped>
/* Backups view specific styles */
</style>
