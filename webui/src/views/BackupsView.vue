<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">备份管理</h1>
          <p class="text-gray-600 mt-1">管理备份任务和恢复数据</p>
        </div>
        <button @click="showCreateModal = true" class="btn-primary flex items-center space-x-2">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span>新建备份</span>
        </button>
      </div>

      <!-- 筛选栏 -->
      <div class="flex space-x-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索备份任务名称..."
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            @input="handleSearch"
          />
        </div>
        <select
          v-model="statusFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          @change="handleFilter"
        >
          <option value="all">全部状态</option>
          <option value="active">活跃</option>
          <option value="inactive">非活跃</option>
          <option value="running">运行中</option>
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

      <!-- 备份列表 -->
      <div v-else-if="backups.length === 0" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H2a2 2 0 01-2-2V5a2 2 0 012-2h6" />
        </svg>
        <p class="mt-4 text-gray-600">暂无备份任务</p>
        <p class="mt-2 text-sm text-gray-500">创建第一个备份任务开始使用</p>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <BackupCard
          v-for="backup in filteredBackups"
          :key="backup.id"
          :backup="backup"
          @execute="handleExecuteBackup"
          @restore="handleRestoreBackup"
          @edit="handleEditBackup"
          @delete="handleDeleteBackup"
        />
      </div>

      <!-- 分页 -->
      <div v-if="totalPages > 1" class="flex justify-center items-center space-x-2">
        <button
          @click="currentPage--"
          :disabled="currentPage === 1"
          class="px-4 py-2 border rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
        >
          上一页
        </button>
        <span class="text-gray-600">
          第 {{ currentPage }} / {{ totalPages }} 页
        </span>
        <button
          @click="currentPage++"
          :disabled="currentPage === totalPages"
          class="px-4 py-2 border rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
        >
          下一页
        </button>
      </div>
    </div>

    <!-- 新建/编辑模态框 -->
    <BackupModal
      v-if="showCreateModal || showEditModal"
      :backup="editingBackup"
      :mode="editingBackup ? 'edit' : 'create'"
      @close="closeModal"
      @save="handleSaveBackup"
    />
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import BackupCard from '@/components/backups/BackupCard.vue'
import BackupModal from '@/components/backups/BackupModal.vue'
import { api } from '@/utils/api'

// 状态
const loading = ref(true)
const backups = ref<any[]>([])
const searchQuery = ref('')
const statusFilter = ref('all')
const currentPage = ref(1)
const totalPages = ref(1)

// 模态框
const showCreateModal = ref(false)
const showEditModal = ref(false)
const editingBackup = ref<any>(null)

// 计算筛选后的备份
const filteredBackups = computed(() => {
  let result = backups.value

  // 状态筛选
  if (statusFilter.value !== 'all') {
    result = result.filter(b => b.status === statusFilter.value)
  }

  // 搜索筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(b => b.name.toLowerCase().includes(query))
  }

  return result
})

// 加载备份列表
const loadBackups = async () => {
  loading.value = true
  try {
    const response = await api.backups.list({
      page: currentPage.value,
      page_size: 20
    })
    backups.value = response.data.backups || []
    totalPages.value = response.data.total_pages || 1
  } catch (error) {
    console.error('Failed to load backups:', error)
    alert('加载备份列表失败')
  } finally {
    loading.value = false
  }
}

// 搜索
const handleSearch = () => {
  currentPage.value = 1
}

// 筛选
const handleFilter = () => {
  currentPage.value = 1
}

// 执行备份
const handleExecuteBackup = async (backup: any) => {
  if (!confirm(`确定要执行备份任务 "${backup.name}" 吗？`)) return

  try {
    await api.backups.execute(backup.id)
    alert('备份任务已启动')
    await loadBackups()
  } catch (error) {
    console.error('Failed to execute backup:', error)
    alert('执行失败')
  }
}

// 恢复备份
const handleRestoreBackup = async (backup: any) => {
  if (!confirm(`确定要从备份 "${backup.name}" 恢复数据吗？此操作不可逆！`)) return

  try {
    await api.backups.restore(backup.id)
    alert('恢复任务已启动')
    await loadBackups()
  } catch (error) {
    console.error('Failed to restore backup:', error)
    alert('恢复失败')
  }
}

// 编辑备份
const handleEditBackup = (backup: any) => {
  editingBackup.value = backup
  showEditModal.value = true
}

// 删除备份
const handleDeleteBackup = async (backup: any) => {
  if (!confirm(`确定要删除备份任务 "${backup.name}" 吗？`)) return

  try {
    await api.backups.delete(backup.id)
    await loadBackups()
  } catch (error) {
    console.error('Failed to delete backup:', error)
    alert('删除失败')
  }
}

// 保存备份（新建/编辑）
const handleSaveBackup = async (backupData: any) => {
  try {
    if (editingBackup.value) {
      await api.backups.update(editingBackup.value.id, backupData)
    } else {
      await api.backups.create(backupData)
    }
    closeModal()
    await loadBackups()
  } catch (error) {
    console.error('Failed to save backup:', error)
    alert('保存失败')
  }
}

// 关闭模态框
const closeModal = () => {
  showCreateModal.value = false
  showEditModal.value = false
  editingBackup.value = null
}

// 生命周期
onMounted(() => {
  loadBackups()
})
</script>
