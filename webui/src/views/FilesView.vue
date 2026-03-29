<template>
  <DefaultLayout>
    <div class="space-y-4">
      <!-- 工具栏 -->
      <div class="bg-white rounded-lg shadow p-4">
        <div class="flex flex-wrap items-center justify-between gap-4">
          <!-- 左侧：导航 -->
          <div class="flex items-center space-x-2">
            <button
              @click="goBack"
              :disabled="currentPath === '/'"
              class="p-2 rounded-lg hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
              title="返回上级"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <!-- 面包屑 -->
            <nav class="flex items-center text-sm">
              <button @click="navigateTo('/')" class="text-primary-600 hover:text-primary-700">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                </svg>
              </button>
              <template v-for="(segment, index) in pathSegments" :key="index">
                <svg class="w-4 h-4 mx-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
                <button
                  @click="navigateToSegment(index)"
                  class="text-gray-700 hover:text-primary-600 font-medium"
                >
                  {{ segment }}
                </button>
              </template>
            </nav>
          </div>

          <!-- 右侧：操作 -->
          <div class="flex items-center space-x-2">
            <!-- 新建文件夹 -->
            <button
              @click="showNewFolderModal = true"
              class="btn-secondary flex items-center space-x-1 text-sm"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
              </svg>
              <span>新建文件夹</span>
            </button>

            <!-- 上传 -->
            <label class="btn-primary flex items-center space-x-1 text-sm cursor-pointer">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
              </svg>
              <span>上传</span>
              <input type="file" @change="handleUpload" class="hidden" multiple />
            </label>

            <!-- 刷新 -->
            <button @click="loadFiles" class="p-2 rounded-lg hover:bg-gray-100" title="刷新">
              <svg :class="{'animate-spin': loading}" class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </button>

            <!-- 视图切换 -->
            <div class="flex border rounded-lg">
              <button
                @click="viewMode = 'grid'"
                :class="viewMode === 'grid' ? 'bg-gray-100' : ''"
                class="p-2 rounded-l-lg hover:bg-gray-100"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                </svg>
              </button>
              <button
                @click="viewMode = 'list'"
                :class="viewMode === 'list' ? 'bg-gray-100' : ''"
                class="p-2 rounded-r-lg hover:bg-gray-100"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
                </svg>
              </button>
            </div>
          </div>
        </div>

        <!-- 搜索栏 -->
        <div class="mt-3">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索文件和文件夹..."
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 text-sm"
          />
        </div>
      </div>

      <!-- 上传进度 -->
      <div v-if="uploadingFiles.length > 0" class="bg-white rounded-lg shadow p-4">
        <h3 class="text-sm font-medium text-gray-700 mb-3">上传中</h3>
        <div class="space-y-2">
          <div v-for="file in uploadingFiles" :key="file.name" class="flex items-center space-x-3">
            <div class="flex-1">
              <div class="flex justify-between text-sm">
                <span class="truncate">{{ file.name }}</span>
                <span class="text-gray-500">{{ file.progress }}%</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2 mt-1">
                <div
                  class="bg-primary-600 h-2 rounded-full transition-all"
                  :style="{ width: file.progress + '%' }"
                ></div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="flex justify-center items-center py-20">
        <svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
      </div>

      <!-- 空目录 -->
      <div v-else-if="filteredItems.length === 0" class="text-center py-20 bg-white rounded-lg shadow">
        <svg class="mx-auto h-16 w-16 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
        <p class="mt-4 text-gray-500">此目录为空</p>
        <p class="mt-2 text-sm text-gray-400">上传文件或创建文件夹开始使用</p>
      </div>

      <!-- 文件列表 - 网格视图 -->
      <div v-else-if="viewMode === 'grid'" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
        <!-- 文件夹 -->
        <div
          v-for="folder in filteredFolders"
          :key="folder.path"
          @dblclick="navigateTo(folder.path)"
          class="bg-white rounded-lg shadow hover:shadow-md transition-shadow cursor-pointer group"
        >
          <div class="p-4 text-center">
            <div class="w-16 h-16 mx-auto bg-yellow-100 rounded-lg flex items-center justify-center mb-2">
              <svg class="w-10 h-10 text-yellow-500" fill="currentColor" viewBox="0 0 24 24">
                <path d="M10 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-4z" />
              </svg>
            </div>
            <p class="text-sm font-medium text-gray-900 truncate">{{ folder.name }}</p>
            <p class="text-xs text-gray-400 mt-1">{{ formatDate(folder.modified_at) }}</p>
          </div>
          <!-- 操作菜单 -->
          <div class="px-3 pb-3 opacity-0 group-hover:opacity-100 transition-opacity">
            <div class="flex justify-center space-x-1">
              <button @click.stop="confirmDelete(folder)" class="p-1.5 text-gray-500 hover:text-red-600 hover:bg-red-50 rounded" title="删除">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        </div>

        <!-- 文件 -->
        <div
          v-for="file in filteredFiles"
          :key="file.path"
          class="bg-white rounded-lg shadow hover:shadow-md transition-shadow cursor-pointer group"
        >
          <div class="p-4 text-center">
            <div class="w-16 h-16 mx-auto bg-gray-100 rounded-lg flex items-center justify-center mb-2">
              <span class="text-3xl">{{ getFileIcon(file.mime_type) }}</span>
            </div>
            <p class="text-sm font-medium text-gray-900 truncate">{{ file.name }}</p>
            <p class="text-xs text-gray-400 mt-1">{{ formatSize(file.size_bytes) }}</p>
          </div>
          <!-- 操作菜单 -->
          <div class="px-3 pb-3 opacity-0 group-hover:opacity-100 transition-opacity">
            <div class="flex justify-center space-x-1">
              <button @click.stop="downloadFile(file)" class="p-1.5 text-gray-500 hover:text-primary-600 hover:bg-primary-50 rounded" title="下载">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                </svg>
              </button>
              <button @click.stop="confirmDelete(file)" class="p-1.5 text-gray-500 hover:text-red-600 hover:bg-red-50 rounded" title="删除">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 文件列表 - 列表视图 -->
      <div v-else class="bg-white rounded-lg shadow overflow-hidden">
        <table class="w-full">
          <thead class="bg-gray-50 border-b">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">名称</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase w-24">大小</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase w-32">修改时间</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase w-24">操作</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-100">
            <!-- 文件夹 -->
            <tr
              v-for="folder in filteredFolders"
              :key="folder.path"
              @dblclick="navigateTo(folder.path)"
              class="hover:bg-gray-50 cursor-pointer"
            >
              <td class="px-4 py-3">
                <div class="flex items-center space-x-3">
                  <svg class="w-6 h-6 text-yellow-500" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M10 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-4z" />
                  </svg>
                  <span class="text-sm font-medium text-gray-900">{{ folder.name }}</span>
                </div>
              </td>
              <td class="px-4 py-3 text-sm text-gray-500">—</td>
              <td class="px-4 py-3 text-sm text-gray-500">{{ formatDate(folder.modified_at) }}</td>
              <td class="px-4 py-3 text-right">
                <button @click.stop="confirmDelete(folder)" class="text-gray-400 hover:text-red-600">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </td>
            </tr>
            <!-- 文件 -->
            <tr
              v-for="file in filteredFiles"
              :key="file.path"
              class="hover:bg-gray-50"
            >
              <td class="px-4 py-3">
                <div class="flex items-center space-x-3">
                  <span class="text-xl">{{ getFileIcon(file.mime_type) }}</span>
                  <span class="text-sm font-medium text-gray-900">{{ file.name }}</span>
                </div>
              </td>
              <td class="px-4 py-3 text-sm text-gray-500">{{ formatSize(file.size_bytes) }}</td>
              <td class="px-4 py-3 text-sm text-gray-500">{{ formatDate(file.modified_at) }}</td>
              <td class="px-4 py-3 text-right">
                <div class="flex justify-end space-x-2">
                  <button @click="downloadFile(file)" class="text-gray-400 hover:text-primary-600">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                    </svg>
                  </button>
                  <button @click="confirmDelete(file)" class="text-gray-400 hover:text-red-600">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 新建文件夹模态框 -->
    <div v-if="showNewFolderModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
        <div class="px-4 py-3 border-b">
          <h3 class="font-semibold text-gray-900">新建文件夹</h3>
        </div>
        <div class="p-4">
          <input
            v-model="newFolderName"
            type="text"
            placeholder="文件夹名称"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            @keyup.enter="createFolder"
          />
        </div>
        <div class="px-4 py-3 bg-gray-50 rounded-b-lg flex justify-end space-x-2">
          <button @click="showNewFolderModal = false" class="px-4 py-2 text-gray-600 hover:text-gray-800">取消</button>
          <button @click="createFolder" class="btn-primary">创建</button>
        </div>
      </div>
    </div>

    <!-- 删除确认 -->
    <div v-if="deleteTarget" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-sm w-full mx-4">
        <div class="p-4">
          <p class="text-gray-900">确定删除 "{{ deleteTarget.name }}" 吗？</p>
          <p class="text-sm text-gray-500 mt-1">此操作无法撤销</p>
        </div>
        <div class="px-4 py-3 bg-gray-50 rounded-b-lg flex justify-end space-x-2">
          <button @click="deleteTarget = null" class="px-4 py-2 text-gray-600 hover:text-gray-800">取消</button>
          <button @click="executeDelete" class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700">删除</button>
        </div>
      </div>
    </div>

    <!-- Toast -->
    <div v-if="toast.show" class="fixed bottom-4 right-4 z-50">
      <div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">
        {{ toast.message }}
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import { api } from '@/utils/api'

// 状态
const loading = ref(true)
const currentPath = ref('/')
const folders = ref<any[]>([])
const files = ref<any[]>([])
const searchQuery = ref('')
const viewMode = ref<'grid' | 'list'>('grid')

// 上传
const uploadingFiles = ref<{ name: string; progress: number }[]>([])

// 新建文件夹
const showNewFolderModal = ref(false)
const newFolderName = ref('')

// 删除
const deleteTarget = ref<any>(null)

// Toast
const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

// 面包屑
const pathSegments = computed(() => {
  return currentPath.value.split('/').filter(Boolean)
})

// 筛选
const filteredFolders = computed(() => {
  if (!searchQuery.value) return folders.value
  const q = searchQuery.value.toLowerCase()
  return folders.value.filter(f => f.name.toLowerCase().includes(q))
})

const filteredFiles = computed(() => {
  if (!searchQuery.value) return files.value
  const q = searchQuery.value.toLowerCase()
  return files.value.filter(f => f.name.toLowerCase().includes(q))
})

const filteredItems = computed(() => [...filteredFolders.value, ...filteredFiles.value])

// 加载文件
const loadFiles = async () => {
  loading.value = true
  try {
    const response = await api.files.browse({ path: currentPath.value })
    const data = response.data
    folders.value = data.folders || []
    files.value = data.files || []
  } catch (error) {
    console.error('Failed to load files:', error)
    showToast('error', '加载失败')
  } finally {
    loading.value = false
  }
}

// 导航
const navigateTo = (path: string) => {
  currentPath.value = path
  loadFiles()
}

const navigateToSegment = (index: number) => {
  const path = '/' + pathSegments.value.slice(0, index + 1).join('/')
  navigateTo(path)
}

const goBack = () => {
  const segments = pathSegments.value
  if (segments.length > 0) {
    segments.pop()
    navigateTo('/' + segments.join('/'))
  }
}

// 上传
const handleUpload = async (event: Event) => {
  const input = event.target as HTMLInputElement
  if (!input.files?.length) return

  for (const file of Array.from(input.files)) {
    uploadingFiles.value.push({ name: file.name, progress: 0 })

    try {
      await api.files.upload(file, currentPath.value)
      uploadingFiles.value = uploadingFiles.value.filter(f => f.name !== file.name)
      showToast('success', `${file.name} 上传成功`)
    } catch (error) {
      uploadingFiles.value = uploadingFiles.value.filter(f => f.name !== file.name)
      showToast('error', `${file.name} 上传失败`)
    }
  }

  input.value = ''
  loadFiles()
}

// 下载
const downloadFile = async (file: any) => {
  try {
    const response = await api.files.download(file.path)
    const url = window.URL.createObjectURL(new Blob([response.data]))
    const link = document.createElement('a')
    link.href = url
    link.download = file.name
    link.click()
    link.remove()
    window.URL.revokeObjectURL(url)
  } catch (error) {
    showToast('error', '下载失败')
  }
}

// 创建文件夹
const createFolder = async () => {
  if (!newFolderName.value.trim()) return

  try {
    await api.files.createFolder(currentPath.value, newFolderName.value.trim())
    showNewFolderModal.value = false
    newFolderName.value = ''
    loadFiles()
    showToast('success', '文件夹创建成功')
  } catch (error) {
    showToast('error', '创建失败')
  }
}

// 删除
const confirmDelete = (item: any) => {
  deleteTarget.value = item
}

const executeDelete = async () => {
  if (!deleteTarget.value) return

  try {
    await api.files.delete(deleteTarget.value.path)
    deleteTarget.value = null
    loadFiles()
    showToast('success', '删除成功')
  } catch (error) {
    showToast('error', '删除失败')
  }
}

// Toast
const showToast = (type: 'success' | 'error', message: string) => {
  toast.value = { show: true, type, message }
  setTimeout(() => { toast.value.show = false }, 3000)
}

// 格式化
const formatSize = (bytes: number) => {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return (bytes / Math.pow(k, i)).toFixed(1) + ' ' + sizes[i]
}

const formatDate = (ts: number) => {
  if (!ts) return '-'
  return new Date(ts * 1000).toLocaleDateString('zh-CN')
}

const getFileIcon = (mime: string) => {
  if (!mime) return '📄'
  if (mime.startsWith('image/')) return '🖼️'
  if (mime.startsWith('video/')) return '🎬'
  if (mime.startsWith('audio/')) return '🎵'
  if (mime.includes('pdf')) return '📕'
  if (mime.includes('zip') || mime.includes('rar')) return '📦'
  if (mime.includes('word') || mime.includes('document')) return '📘'
  if (mime.includes('sheet') || mime.includes('excel')) return '📗'
  return '📄'
}

onMounted(() => loadFiles())
</script>