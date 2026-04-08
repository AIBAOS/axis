<template>
  <div class="space-y-6">
    <!-- 页面标题和操作栏 -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        文件管理
      </h1>
      <div class="flex items-center space-x-3">
        <!-- 批量操作按钮（有选中时显示） -->
        <template v-if="selectedFiles.length > 0">
          <button
            @click="batchDelete"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
          >
            <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
            </svg>
            删除 ({{ selectedFiles.length }})
          </button>
          <button
            @click="clearSelection"
            class="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消选择
          </button>
        </template>
        <!-- 常规按钮（无选中时显示） -->
        <template v-else>
          <button
            @click="showUploadModal = true"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          >
            <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path>
            </svg>
            上传文件
          </button>
          <button
            @click="createFolder"
            class="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          >
            <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
            </svg>
            新建文件夹
          </button>
        </template>
      </div>
    </div>

    <!-- 面包屑导航和搜索栏 -->
    <div class="space-y-3">
      <nav class="flex items-center justify-between px-6 py-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
        <div class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
          <!-- 返回上级按钮 -->
          <button
            v-if="currentPath !== '/'"
            @click="navigateToParent()"
            class="hover:text-indigo-600 dark:hover:text-indigo-400 flex items-center"
            title="返回上级"
          >
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
            </svg>
          </button>
          <button
            @click="navigateTo('/')"
            class="hover:text-indigo-600 dark:hover:text-indigo-400"
          >
            📁 根目录
          </button>
          <template v-for="(segment, index) in breadcrumbSegments" :key="index">
            <span class="text-gray-400">/</span>
            <button
              @click="navigateToBreadcrumb(index)"
              class="hover:text-indigo-600 dark:hover:text-indigo-400"
            >
              {{ segment }}
            </button>
          </template>
        </div>
        <div class="flex items-center space-x-4">
          <!-- 全选复选框 -->
          <label class="flex items-center text-sm text-gray-500 dark:text-gray-400 cursor-pointer">
            <input
              type="checkbox"
              :checked="isAllSelected"
              @change="toggleSelectAll"
              class="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
            />
            <span class="ml-2">全选</span>
          </label>
          <!-- 项目数量统计 -->
          <div class="text-xs text-gray-400">
            {{ filteredFiles.length }} 个项目
            <span v-if="selectedFiles.length > 0" class="text-indigo-600 dark:text-indigo-400">
              (已选 {{ selectedFiles.length }})
            </span>
          </div>
        </div>
      </nav>

      <!-- 搜索栏 -->
      <div class="relative">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索文件..."
          class="w-full px-4 py-2 pl-10 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white dark:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500"
        />
        <svg class="absolute left-3 top-2.5 w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
        </svg>
        <button
          v-if="searchQuery"
          @click="searchQuery = ''"
          class="absolute right-3 top-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
    </div>

    <!-- Toast 通知 -->
    <div v-if="toast.show" class="fixed top-4 right-4 z-50 px-6 py-3 rounded-lg shadow-lg text-white transition-opacity duration-300" :class="toast.type === 'error' ? 'bg-red-500' : 'bg-green-500'">
      {{ toast.message }}
    </div>

    <!-- 文件列表 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <!-- 列表头部（可点击排序） -->
      <div class="grid grid-cols-12 gap-4 px-6 py-3 bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600 text-sm font-medium text-gray-500 dark:text-gray-400">
        <div class="col-span-1 flex items-center justify-center">
          <input
            type="checkbox"
            :checked="isAllSelected"
            @change="toggleSelectAll"
            class="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
          />
        </div>
        <div 
          class="col-span-5 cursor-pointer hover:text-indigo-600 dark:hover:text-indigo-400 flex items-center"
          @click="toggleSort('name')"
        >
          <span>名称</span>
          <span v-if="sortField === 'name'" class="ml-1">
            {{ sortOrder === 'asc' ? '↑' : '↓' }}
          </span>
        </div>
        <div 
          class="col-span-2 cursor-pointer hover:text-indigo-600 dark:hover:text-indigo-400 flex items-center"
          @click="toggleSort('size')"
        >
          <span>大小</span>
          <span v-if="sortField === 'size'" class="ml-1">
            {{ sortOrder === 'asc' ? '↑' : '↓' }}
          </span>
        </div>
        <div 
          class="col-span-3 cursor-pointer hover:text-indigo-600 dark:hover:text-indigo-400 flex items-center"
          @click="toggleSort('modified')"
        >
          <span>修改时间</span>
          <span v-if="sortField === 'modified'" class="ml-1">
            {{ sortOrder === 'asc' ? '↑' : '↓' }}
          </span>
        </div>
        <div class="col-span-1 text-right">操作</div>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="px-6 py-12 text-center">
        <svg class="animate-spin h-8 w-8 text-indigo-600 mx-auto" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">加载中...</p>
      </div>

      <!-- 空目录/无搜索结果 -->
      <div v-else-if="filteredFiles.length === 0" class="px-6 py-12 text-center">
        <svg class="w-16 h-16 text-gray-300 dark:text-gray-600 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
        </svg>
        <p class="text-lg font-medium text-gray-500 dark:text-gray-400 mb-2">
          {{ searchQuery ? '未找到匹配的文件' : '此目录为空' }}
        </p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mb-4">
          {{ searchQuery ? '尝试其他搜索关键词' : '上传文件或创建文件夹开始使用' }}
        </p>
        <div v-if="!searchQuery" class="flex justify-center space-x-3">
          <button
            @click="showUploadModal = true"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
          >
            上传文件
          </button>
          <button
            @click="createFolder"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            新建文件夹
          </button>
        </div>
      </div>

      <!-- 文件列表 -->
      <div v-else>
        <div
          v-for="file in filteredFiles"
          :key="file.name"
          class="grid grid-cols-12 gap-4 px-6 py-3 border-b border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 items-center"
          :class="{ 'cursor-pointer': file.type === 'dir' }"
          @dblclick="file.type === 'dir' && navigateTo(file.path)"
        >
          <!-- 复选框 -->
          <div class="col-span-1 flex items-center justify-center" @click.stop>
            <input
              type="checkbox"
              :checked="isSelected(file)"
              @change="toggleFileSelection(file)"
              class="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
            />
          </div>

          <!-- 文件名 -->
          <div class="col-span-5 flex items-center">
            <span class="text-xl mr-3">
              {{ file.type === 'dir' ? '📁' : getFileIcon(file.name) }}
            </span>
            <span class="text-sm font-medium text-gray-900 dark:text-white truncate" :class="{ 'cursor-pointer hover:text-indigo-600 dark:hover:text-indigo-400': file.type === 'dir' }">
              {{ file.name }}
            </span>
          </div>

          <!-- 大小 -->
          <div class="col-span-2 text-sm text-gray-500 dark:text-gray-400">
            {{ file.type === 'dir' ? '-' : formatFileSize(file.size) }}
          </div>

          <!-- 修改时间 -->
          <div class="col-span-3 text-sm text-gray-500 dark:text-gray-400">
            {{ formatDateTime(file.modified) }}
          </div>

          <!-- 操作 -->
          <div class="col-span-1 flex justify-end space-x-2">
            <button
              v-if="file.type === 'file'"
              @click="downloadFile(file)"
              class="text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400"
              title="下载"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
              </svg>
            </button>
            <button
              @click="showRenameModal(file)"
              class="text-gray-400 hover:text-yellow-600 dark:hover:text-yellow-400"
              title="重命名"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
              </svg>
            </button>
            <button
              @click.stop="deleteFile(file)"
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

    <!-- 上传文件模态框 -->
    <div v-if="showUploadModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">上传文件</h3>
          <button @click="showUploadModal = false" class="text-gray-400 hover:text-gray-500">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
          </button>
        </div>
        <div class="px-6 py-4">
          <div class="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-6 text-center">
            <svg class="w-12 h-12 text-gray-400 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
            </svg>
            <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">拖拽文件到此处，或点击选择文件</p>
            <input
              type="file"
              ref="fileInput"
              @change="handleFileSelect"
              class="hidden"
              multiple
            />
            <button
              @click="$refs.fileInput.click()"
              class="mt-4 px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              选择文件
            </button>
          </div>
          <div v-if="selectedUploadFiles.length > 0" class="mt-4">
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">已选择 {{ selectedUploadFiles.length }} 个文件</p>
            <ul class="text-sm text-gray-500 dark:text-gray-400 space-y-1 max-h-32 overflow-y-auto">
              <li v-for="(file, idx) in selectedUploadFiles" :key="idx" class="truncate">{{ file.name }}</li>
            </ul>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="showUploadModal = false"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="uploadFiles"
            :disabled="selectedUploadFiles.length === 0 || uploading"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ uploading ? '上传中...' : '上传' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 重命名模态框 -->
    <div v-if="showRenameModalFlag && fileToRename" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">重命名</h3>
        </div>
        <div class="px-6 py-4">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            新名称
          </label>
          <input
            v-model="newFileName"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-white dark:bg-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            :placeholder="fileToRename.name"
          />
        </div>
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
          <button
            @click="closeRenameModal"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="confirmRename"
            class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
          >
            确认
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'

const router = useRouter()

const loading = ref(false)
const files = ref([])
const currentPath = ref('/')
const showUploadModal = ref(false)
const showRenameModalFlag = ref(false)
const fileToRename = ref(null)
const newFileName = ref('')
const selectedUploadFiles = ref([])
const uploading = ref(false)

// 批量选择
const selectedFiles = ref([])

// 搜索
const searchQuery = ref('')

// Toast 通知
const toast = ref({
  show: false,
  message: '',
  type: 'success',
  timer: null
})

// 显示 Toast
const showToast = (message, type = 'success') => {
  if (toast.value.timer) {
    clearTimeout(toast.value.timer)
  }
  toast.value = {
    show: true,
    message,
    type,
    timer: setTimeout(() => {
      toast.value.show = false
    }, 3000)
  }
}

// 排序状态
const sortField = ref('name')
const sortOrder = ref('asc')

// 面包屑导航片段
const breadcrumbSegments = computed(() => {
  if (currentPath.value === '/') return []
  return currentPath.value.split('/').filter(Boolean)
})

// 排序后的文件列表
const sortedFiles = computed(() => {
  const sorted = [...files.value].sort((a, b) => {
    if (a.type === 'dir' && b.type !== 'dir') return -1
    if (a.type !== 'dir' && b.type === 'dir') return 1
    
    let comparison = 0
    switch (sortField.value) {
      case 'name':
        comparison = a.name.localeCompare(b.name)
        break
      case 'size':
        comparison = (a.size || 0) - (b.size || 0)
        break
      case 'modified':
        comparison = (a.modified || 0) - (b.modified || 0)
        break
    }
    
    return sortOrder.value === 'asc' ? comparison : -comparison
  })
  
  return sorted
})

// 过滤后的文件列表（支持搜索）
const filteredFiles = computed(() => {
  if (!searchQuery.value) return sortedFiles.value
  
  const query = searchQuery.value.toLowerCase()
  return sortedFiles.value.filter(file => 
    file.name.toLowerCase().includes(query)
  )
})

// 是否全选
const isAllSelected = computed(() => {
  return filteredFiles.value.length > 0 && selectedFiles.value.length === filteredFiles.value.length
})

// 导航到指定路径
const navigateTo = (path) => {
  currentPath.value = path
  searchQuery.value = ''
  clearSelection()
  loadFiles()
}

// 导航到上级目录
const navigateToParent = () => {
  const parts = currentPath.value.split('/').filter(Boolean)
  if (parts.length > 0) {
    parts.pop()
    currentPath.value = '/' + parts.join('/')
    if (currentPath.value === '/') {
      currentPath.value = '/'
    }
    searchQuery.value = ''
    clearSelection()
    loadFiles()
  }
}

// 导航到面包屑位置
const navigateToBreadcrumb = (index) => {
  const path = '/' + breadcrumbSegments.value.slice(0, index + 1).join('/')
  navigateTo(path)
}

// 切换排序
const toggleSort = (field) => {
  if (sortField.value === field) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortField.value = field
    sortOrder.value = 'asc'
  }
}

// 全选/取消全选
const toggleSelectAll = () => {
  if (isAllSelected.value) {
    clearSelection()
  } else {
    selectedFiles.value = [...filteredFiles.value]
  }
}

// 切换文件选择
const toggleFileSelection = (file) => {
  const index = selectedFiles.value.findIndex(f => f.name === file.name)
  if (index > -1) {
    selectedFiles.value.splice(index, 1)
  } else {
    selectedFiles.value.push(file)
  }
}

// 检查文件是否被选中
const isSelected = (file) => {
  return selectedFiles.value.some(f => f.name === file.name)
}

// 清除选择
const clearSelection = () => {
  selectedFiles.value = []
}

// 批量删除
const batchDelete = async () => {
  if (!confirm(`确定要删除选中的 ${selectedFiles.value.length} 个项目吗？`)) return
  
  try {
    for (const file of selectedFiles.value) {
      await apiClient.delete('/files/delete', {
        params: { path: file.path }
      })
    }
    clearSelection()
    loadFiles()
    showToast(`成功删除 ${selectedFiles.value.length} 个项目`)
  } catch (error) {
    console.error('Batch delete failed:', error)
    showToast('批量删除失败', 'error')
  }
}

// 加载文件列表
const loadFiles = async () => {
  loading.value = true
  try {
    const response = await apiClient.get('/files/list', {
      params: { path: currentPath.value === '/' ? '' : currentPath.value }
    })
    
    if (response.data.success) {
      files.value = response.data.data.map(file => ({
        ...file,
        path: currentPath.value === '/' ? `/${file.name}` : `${currentPath.value}/${file.name}`
      }))
    }
  } catch (error) {
    console.error('Failed to load files:', error)
    showToast('加载文件失败', 'error')
  } finally {
    loading.value = false
  }
}

// 获取文件图标
const getFileIcon = (filename) => {
  const ext = filename.split('.').pop().toLowerCase()
  const icons = {
    pdf: '📄',
    doc: '📄',
    docx: '📄',
    xls: '📊',
    xlsx: '📊',
    jpg: '🖼️',
    jpeg: '🖼️',
    png: '🖼️',
    gif: '🖼️',
    mp3: '🎵',
    mp4: '🎬',
    zip: '📦',
    rar: '📦',
    txt: '📝'
  }
  return icons[ext] || '📄'
}

// 格式化文件大小
const formatFileSize = (bytes) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 格式化日期时间
const formatDateTime = (timestamp) => {
  if (!timestamp) return '--'
  const date = new Date(timestamp * 1000)
  return date.toLocaleString('zh-CN')
}

// 下载文件
const downloadFile = async (file) => {
  try {
    const response = await apiClient.get('/files/download', {
      params: { path: file.path },
      responseType: 'blob'
    })
    
    const url = window.URL.createObjectURL(new Blob([response.data]))
    const link = document.createElement('a')
    link.href = url
    link.setAttribute('download', file.name)
    document.body.appendChild(link)
    link.click()
    link.remove()
    showToast('下载成功')
  } catch (error) {
    console.error('Download failed:', error)
    showToast('下载失败', 'error')
  }
}

// 删除文件
const deleteFile = async (file) => {
  if (!confirm(`确定要删除 "${file.name}" 吗？`)) return
  
  try {
    await apiClient.delete('/files/delete', {
      params: { path: file.path }
    })
    loadFiles()
    showToast('删除成功')
  } catch (error) {
    console.error('Delete failed:', error)
    showToast('删除失败', 'error')
  }
}

// 显示重命名模态框
const showRenameModal = (file) => {
  fileToRename.value = file
  newFileName.value = file.name
  showRenameModalFlag.value = true
}

// 关闭重命名模态框
const closeRenameModal = () => {
  showRenameModalFlag.value = false
  fileToRename.value = null
  newFileName.value = ''
}

// 确认重命名
const confirmRename = async () => {
  if (!newFileName.value || newFileName.value === fileToRename.value.name) {
    closeRenameModal()
    return
  }
  
  try {
    await apiClient.put('/files/rename', {
      path: fileToRename.value.path,
      new_name: newFileName.value
    })
    loadFiles()
    closeRenameModal()
    showToast('重命名成功')
  } catch (error) {
    console.error('Rename failed:', error)
    showToast('重命名失败', 'error')
  }
}

// 创建文件夹
const createFolder = async () => {
  const name = prompt('请输入文件夹名称：')
  if (!name) return
  
  try {
    await apiClient.post('/files/create_folder', {
      path: currentPath.value === '/' ? `/${name}` : `${currentPath.value}/${name}`
    })
    loadFiles()
    showToast('创建成功')
  } catch (error) {
    console.error('Create folder failed:', error)
    showToast('创建文件夹失败', 'error')
  }
}

// 处理文件选择
const handleFileSelect = (event) => {
  selectedUploadFiles.value = Array.from(event.target.files)
}

// 上传文件
const uploadFiles = async () => {
  if (selectedUploadFiles.value.length === 0) return
  
  uploading.value = true
  try {
    for (const file of selectedUploadFiles.value) {
      const formData = new FormData()
      formData.append('file', file)
      formData.append('path', currentPath.value)
      
      await apiClient.post('/files/upload', formData, {
        headers: { 'Content-Type': 'multipart/form-data' }
      })
    }
    
    showUploadModal.value = false
    selectedUploadFiles.value = []
    loadFiles()
    showToast('上传成功')
  } catch (error) {
    console.error('Upload failed:', error)
    showToast('上传失败', 'error')
  } finally {
    uploading.value = false
  }
}

// 页面加载时检查登录状态并加载文件
onMounted(() => {
  const token = localStorage.getItem('jwt_token')
  if (!token) {
    router.push('/login')
    return
  }
  loadFiles()
})
</script>
