<template>
  <div class="px-4 py-6 sm:px-0">
    <div class="bg-white overflow-hidden shadow rounded-lg">
      <!-- Header -->
      <div class="px-4 py-5 sm:px-6 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h2 class="text-lg font-medium text-gray-900">文件管理</h2>
          <div class="flex space-x-2">
            <button
              @click="handleUpload"
              class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
            >
              <svg class="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path>
              </svg>
              上传
            </button>
            <button
              @click="handleNewFolder"
              class="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
            >
              <svg class="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
              </svg>
              新建文件夹
            </button>
          </div>
        </div>
      </div>

      <!-- Breadcrumb -->
      <div class="px-4 py-3 bg-gray-50 border-b border-gray-200">
        <nav class="flex items-center space-x-2 text-sm text-gray-500">
          <button @click="navigateTo('/')" class="hover:text-gray-700">📁</button>
          <template v-for="(segment, index) in pathSegments" :key="index">
            <span class="text-gray-400">/</span>
            <button
              @click="navigateToSegment(index)"
              class="hover:text-gray-700 truncate max-w-xs"
            >
              {{ segment }}
            </button>
          </template>
        </nav>
      </div>

      <!-- File List -->
      <div class="divide-y divide-gray-200">
        <!-- Parent Directory -->
        <div
          v-if="currentPath !== '/'"
          @click="navigateUp"
          class="px-4 py-3 hover:bg-gray-50 cursor-pointer flex items-center"
        >
          <svg class="h-5 w-5 text-gray-400 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
          </svg>
          <span class="text-sm font-medium text-gray-900">..</span>
        </div>

        <!-- Files and Folders -->
        <div
          v-for="item in files"
          :key="item.id"
          @click="handleItemClick(item)"
          class="px-4 py-3 hover:bg-gray-50 cursor-pointer flex items-center justify-between"
        >
          <div class="flex items-center flex-1">
            <!-- Icon -->
            <div class="flex-shrink-0">
              <svg
                v-if="item.type === 'folder'"
                class="h-6 w-6 text-yellow-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
              </svg>
              <svg
                v-else
                class="h-6 w-6 text-gray-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
              </svg>
            </div>

            <!-- Name -->
            <div class="ml-3 flex-1">
              <p class="text-sm font-medium text-gray-900">{{ item.name }}</p>
              <p class="text-xs text-gray-500">{{ formatSize(item.size) }} · {{ formatDate(item.modified_at) }}</p>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex items-center space-x-2">
            <button
              @click.stop="handleDownload(item)"
              class="text-gray-400 hover:text-gray-600"
              title="下载"
            >
              <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
              </svg>
            </button>
            <button
              @click.stop="handleRename(item)"
              class="text-gray-400 hover:text-gray-600"
              title="重命名"
            >
              <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
              </svg>
            </button>
            <button
              @click.stop="handleDelete(item)"
              class="text-gray-400 hover:text-red-600"
              title="删除"
            >
              <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
              </svg>
            </button>
          </div>
        </div>

        <!-- Empty State -->
        <div v-if="files.length === 0" class="px-4 py-12 text-center">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
          </svg>
          <h3 class="mt-2 text-sm font-medium text-gray-900">此文件夹为空</h3>
          <p class="mt-1 text-sm text-gray-500">上传文件或创建文件夹以开始</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

// Mock data - replace with API calls
const files = ref([
  { id: '1', name: 'Documents', type: 'folder', size: 0, modified_at: '2026-03-28T10:00:00Z' },
  { id: '2', name: 'Photos', type: 'folder', size: 0, modified_at: '2026-03-28T09:30:00Z' },
  { id: '3', name: 'backup.zip', type: 'file', size: 1073741824, modified_at: '2026-03-28T08:00:00Z' },
  { id: '4', name: 'report.pdf', type: 'file', size: 2097152, modified_at: '2026-03-27T16:00:00Z' },
])

const currentPath = ref('/')

const pathSegments = computed(() => {
  if (currentPath.value === '/') return []
  return currentPath.value.split('/').filter(Boolean)
})

const navigateTo = (path: string) => {
  currentPath.value = path
  // TODO: Fetch files from API
}

const navigateToSegment = (index: number) => {
  const newPath = '/' + pathSegments.value.slice(0, index + 1).join('/')
  navigateTo(newPath)
}

const navigateUp = () => {
  const segments = currentPath.value.split('/').filter(Boolean)
  segments.pop()
  navigateTo('/' + segments.join('/') || '/')
}

const handleItemClick = (item: any) => {
  if (item.type === 'folder') {
    navigateTo(currentPath.value === '/' ? `/${item.name}` : `${currentPath.value}/${item.name}`)
  }
}

const handleUpload = () => {
  // TODO: Implement file upload
  alert('上传功能开发中')
}

const handleNewFolder = () => {
  // TODO: Implement new folder creation
  alert('新建文件夹功能开发中')
}

const handleDownload = (item: any) => {
  // TODO: Implement file download
  alert(`下载 ${item.name}`)
}

const handleRename = (item: any) => {
  // TODO: Implement file rename
  alert(`重命名 ${item.name}`)
}

const handleDelete = (item: any) => {
  // TODO: Implement file delete
  if (confirm(`确定删除 ${item.name}？`)) {
    alert(`删除 ${item.name}`)
  }
}

const formatSize = (bytes: number) => {
  if (bytes === 0) return '-'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString('zh-CN')
}
</script>

<style scoped>
/* Files view specific styles */
</style>
