<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题和操作栏 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">文件管理</h1>
          <p class="text-gray-600 mt-1">浏览和管理您的文件</p>
        </div>
        <div class="flex space-x-3">
          <button
            @click="handleRefresh"
            class="btn-secondary flex items-center space-x-2"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>刷新</span>
          </button>
          <label class="btn-primary flex items-center space-x-2 cursor-pointer">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
            </svg>
            <span>上传文件</span>
            <input
              type="file"
              @change="handleFileUpload"
              class="hidden"
              multiple
            />
          </label>
        </div>
      </div>

      <!-- 面包屑导航 -->
      <div class="flex items-center space-x-2 text-sm text-gray-600">
        <button
          @click="navigateTo('/')"
          class="hover:text-primary-600 transition-colors"
        >
          📁 根目录
        </button>
        <span v-for="(segment, index) in pathSegments" :key="index" class="flex items-center space-x-2">
          <span>/</span>
          <button
            @click="navigateToSegment(index)"
            class="hover:text-primary-600 transition-colors font-medium"
          >
            {{ segment }}
          </button>
        </span>
      </div>

      <!-- 搜索和筛选栏 -->
      <div class="flex space-x-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索文件..."
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            @input="handleSearch"
          />
        </div>
        <select
          v-model="fileTypeFilter"
          class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
          @change="handleFilter"
        >
          <option value="all">全部类型</option>
          <option value="folder">📁 文件夹</option>
          <option value="image">🖼️ 图片</option>
          <option value="document">📄 文档</option>
          <option value="video">🎬 视频</option>
          <option value="audio">🎵 音频</option>
          <option value="archive">📦 压缩包</option>
          <option value="other">📎 其他</option>
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

      <!-- 文件列表 -->
      <div v-else-if="files.length === 0" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
        <p class="mt-4 text-gray-600">当前目录为空</p>
        <p class="mt-2 text-sm text-gray-500">上传文件或文件夹开始使用</p>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        <FileCard
          v-for="file in filteredFiles"
          :key="file.id"
          :file="file"
          @open="handleOpenFile"
          @download="handleDownloadFile"
          @delete="handleDeleteFile"
          @rename="handleRenameFile"
        />
      </div>

      <!-- 分页 -->
      <div v-if="totalPages > 1" class="flex justify-center items-center space-x-2 mt-8">
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
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import FileCard from '@/components/files/FileCard.vue'
import { api } from '@/utils/api'

// 状态
const loading = ref(true)
const files = ref<any[]>([])
const searchQuery = ref('')
const fileTypeFilter = ref('all')
const currentPage = ref(1)
const totalPages = ref(1)
const currentPath = ref('/')

// 计算面包屑路径段
const pathSegments = computed(() => {
  if (currentPath.value === '/') return []
  return currentPath.value.split('/').filter(Boolean)
})

// 计算筛选后的文件
const filteredFiles = computed(() => {
  let result = files.value

  // 类型筛选
  if (fileTypeFilter.value !== 'all') {
    if (fileTypeFilter.value === 'folder') {
      result = result.filter(f => f.is_folder)
    } else {
      result = result.filter(f => !f.is_folder && f.file_type === fileTypeFilter.value)
    }
  }

  // 搜索筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(f => f.name.toLowerCase().includes(query))
  }

  return result
})

// 加载文件列表
const loadFiles = async () => {
  loading.value = true
  try {
    const response = await api.files.list({
      path: currentPath.value,
      page: currentPage.value,
      page_size: 20
    })
    files.value = response.data.files || []
    totalPages.value = response.data.total_pages || 1
  } catch (error) {
    console.error('Failed to load files:', error)
    alert('加载文件列表失败')
  } finally {
    loading.value = false
  }
}

// 导航到指定路径
const navigateTo = (path: string) => {
  currentPath.value = path
  currentPage.value = 1
  loadFiles()
}

// 导航到面包屑段
const navigateToSegment = (index: number) => {
  const path = '/' + pathSegments.value.slice(0, index + 1).join('/')
  navigateTo(path)
}

// 刷新
const handleRefresh = () => {
  loadFiles()
}

// 搜索
const handleSearch = () => {
  currentPage.value = 1
}

// 筛选
const handleFilter = () => {
  currentPage.value = 1
}

// 打开文件/文件夹
const handleOpenFile = (file: any) => {
  if (file.is_folder) {
    navigateTo(file.path)
  } else {
    // 打开文件详情或预览
    console.log('Open file:', file)
  }
}

// 下载文件
const handleDownloadFile = async (file: any) => {
  try {
    const response = await api.files.download(file.id)
    const url = window.URL.createObjectURL(new Blob([response.data]))
    const link = document.createElement('a')
    link.href = url
    link.setAttribute('download', file.name)
    document.body.appendChild(link)
    link.click()
    link.remove()
  } catch (error) {
    console.error('Failed to download file:', error)
    alert('下载失败')
  }
}

// 删除文件
const handleDeleteFile = async (file: any) => {
  if (!confirm(`确定要删除 "${file.name}" 吗？`)) return

  try {
    await api.files.delete(file.id)
    await loadFiles()
  } catch (error) {
    console.error('Failed to delete file:', error)
    alert('删除失败')
  }
}

// 重命名文件
const handleRenameFile = async (file: any, newName: string) => {
  try {
    // TODO: 实现重命名 API
    console.log('Rename file:', file.id, 'to', newName)
    alert('重命名功能开发中')
  } catch (error) {
    console.error('Failed to rename file:', error)
    alert('重命名失败')
  }
}

// 上传文件
const handleFileUpload = async (event: Event) => {
  const target = event.target as HTMLInputElement
  const fileList = target.files
  if (!fileList || fileList.length === 0) return

  for (const file of fileList) {
    try {
      await api.files.upload(file)
    } catch (error) {
      console.error('Failed to upload file:', file.name, error)
      alert(`上传失败：${file.name}`)
    }
  }

  await loadFiles()
  target.value = '' // 重置 input
}

// 生命周期
onMounted(() => {
  loadFiles()
})
</script>
