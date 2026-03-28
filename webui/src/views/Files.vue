<template>
  <div class="px-4 py-6 sm:px-0">
    <div class="bg-white overflow-hidden shadow rounded-lg">
      <div class="px-4 py-5 sm:p-6">
        <!-- 标题和操作栏 -->
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900">📁 文件管理</h2>
          <div class="flex space-x-3">
            <button
              @click="showUploadModal = true"
              class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
            >
              ⬆️ 上传文件
            </button>
            <button
              @click="createFolder"
              class="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
            >
              📁 新建文件夹
            </button>
          </div>
        </div>

        <!-- 面包屑导航 -->
        <div class="flex items-center space-x-2 mb-4 text-sm">
          <button
            @click="navigateTo('/')"
            class="text-primary-600 hover:text-primary-800"
          >
            🏠 根目录
          </button>
          <span v-for="(segment, index) in pathSegments" :key="index" class="flex items-center">
            <span class="text-gray-400 mx-2">/</span>
            <button
              @click="navigateTo(segment.path)"
              class="text-primary-600 hover:text-primary-800"
            >
              {{ segment.name }}
            </button>
          </span>
        </div>

        <!-- 搜索栏 -->
        <div class="mb-4">
          <div class="flex space-x-3">
            <input
              v-model="searchQuery"
              @input="debouncedSearch"
              type="text"
              placeholder="搜索文件或文件夹..."
              class="flex-1 rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
            <button
              @click="refreshFiles"
              class="px-4 py-2 border border-gray-300 rounded-md text-gray-700 hover:bg-gray-50"
            >
              🔄 刷新
            </button>
          </div>
        </div>

        <!-- 加载状态 -->
        <div v-if="loading" class="text-center py-12">
          <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
          <p class="mt-2 text-gray-600">加载中...</p>
        </div>

        <!-- 错误提示 -->
        <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
          {{ error }}
        </div>

        <!-- 文件列表 -->
        <div v-else class="space-y-2">
          <!-- 表头 -->
          <div class="grid grid-cols-12 gap-4 px-4 py-2 bg-gray-50 rounded-t-lg text-sm font-medium text-gray-600">
            <div class="col-span-6">名称</div>
            <div class="col-span-2">大小</div>
            <div class="col-span-2">修改时间</div>
            <div class="col-span-2 text-right">操作</div>
          </div>

          <!-- 父目录 -->
          <div
            v-if="currentPath !== '/'"
            @click="navigateTo(parentPath)"
            class="grid grid-cols-12 gap-4 px-4 py-3 hover:bg-gray-50 cursor-pointer border-b border-gray-100"
          >
            <div class="col-span-6 flex items-center">
              <span class="text-gray-500 mr-2">📁</span>
              <span class="text-gray-600">..</span>
            </div>
            <div class="col-span-2"></div>
            <div class="col-span-2"></div>
            <div class="col-span-2"></div>
          </div>

          <!-- 文件夹列表 -->
          <div
            v-for="folder in folders"
            :key="folder.path"
            @click="navigateTo(folder.path)"
            class="grid grid-cols-12 gap-4 px-4 py-3 hover:bg-blue-50 cursor-pointer border-b border-gray-100"
          >
            <div class="col-span-6 flex items-center">
              <span class="text-yellow-500 mr-2">📁</span>
              <span class="text-gray-900 font-medium">{{ folder.name }}</span>
            </div>
            <div class="col-span-2 text-gray-500 text-sm">{{ formatSize(folder.size_bytes) }}</div>
            <div class="col-span-2 text-gray-500 text-sm">{{ formatTime(folder.modified_at) }}</div>
            <div class="col-span-2 flex justify-end space-x-2">
              <button
                @click.stop="renameItem(folder.name, 'folder')"
                class="text-blue-600 hover:text-blue-800 text-sm"
              >
                ✏️
              </button>
              <button
                @click.stop="deleteItem(folder.path, 'folder')"
                class="text-red-600 hover:text-red-800 text-sm"
              >
                🗑️
              </button>
            </div>
          </div>

          <!-- 文件列表 -->
          <div
            v-for="file in files"
            :key="file.path"
            class="grid grid-cols-12 gap-4 px-4 py-3 hover:bg-gray-50 border-b border-gray-100"
          >
            <div class="col-span-6 flex items-center">
              <span class="mr-2">{{ getFileIcon(file.name) }}</span>
              <span class="text-gray-900">{{ file.name }}</span>
            </div>
            <div class="col-span-2 text-gray-500 text-sm">{{ formatSize(file.size_bytes) }}</div>
            <div class="col-span-2 text-gray-500 text-sm">{{ formatTime(file.modified_at) }}</div>
            <div class="col-span-2 flex justify-end space-x-2">
              <a
                :href="getDownloadUrl(file.path)"
                class="text-green-600 hover:text-green-800 text-sm"
                download
              >
                ⬇️
              </a>
              <button
                @click="renameItem(file.name, 'file')"
                class="text-blue-600 hover:text-blue-800 text-sm"
              >
                ✏️
              </button>
              <button
                @click="deleteItem(file.path, 'file')"
                class="text-red-600 hover:text-red-800 text-sm"
              >
                🗑️
              </button>
            </div>
          </div>

          <!-- 空状态 -->
          <div v-if="folders.length === 0 && files.length === 0" class="text-center py-12 text-gray-500">
            <p class="text-4xl mb-4">📂</p>
            <p>此文件夹为空</p>
          </div>
        </div>

        <!-- 分页信息 -->
        <div v-if="!loading && pagination.total_pages > 1" class="mt-4 flex items-center justify-between">
          <span class="text-sm text-gray-600">
            第 {{ pagination.page }} / {{ pagination.total_pages }} 页，共 {{ pagination.total_items }} 项
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
    </div>

    <!-- 上传文件模态框 -->
    <div v-if="showUploadModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center">
      <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h3 class="text-lg font-medium text-gray-900 mb-4">上传文件</h3>
        <form @submit.prevent="uploadFiles" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">选择文件</label>
            <input
              ref="fileInput"
              type="file"
              multiple
              @change="handleFileSelect"
              class="w-full"
            />
          </div>
          <div v-if="selectedFiles.length > 0" class="text-sm text-gray-600">
            已选择 {{ selectedFiles.length }} 个文件
          </div>
          <div class="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              @click="showUploadModal = false"
              class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
            >
              取消
            </button>
            <button
              type="submit"
              :disabled="uploading || selectedFiles.length === 0"
              class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
            >
              {{ uploading ? '上传中...' : '上传' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- 重命名模态框 -->
    <div v-if="showRenameModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center">
      <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h3 class="text-lg font-medium text-gray-900 mb-4">重命名</h3>
        <form @submit.prevent="confirmRename" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">新名称</label>
            <input
              v-model="newName"
              type="text"
              required
              class="w-full rounded-md border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>
          <div class="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              @click="showRenameModal = false"
              class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
            >
              取消
            </button>
            <button
              type="submit"
              :disabled="renaming"
              class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
            >
              {{ renaming ? '保存中...' : '保存' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import apiClient from '../api/client';

interface FolderInfo {
  name: string;
  path: string;
  size_bytes: number;
  modified_at: number;
}

interface FileInfo {
  name: string;
  path: string;
  size_bytes: number;
  mime_type: string;
  modified_at: number;
}

interface PaginationInfo {
  page: number;
  limit: number;
  total_items: number;
  total_pages: number;
}

const currentPath = ref('/');
const folders = ref<FolderInfo[]>([]);
const files = ref<FileInfo[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const searchQuery = ref('');
const pagination = ref<PaginationInfo>({
  page: 1,
  limit: 20,
  total_items: 0,
  total_pages: 1,
});

// 模态框状态
const showUploadModal = ref(false);
const showRenameModal = ref(false);
const selectedFiles = ref<File[]>([]);
const uploading = ref(false);
const renaming = ref(false);
const renameItemData = ref<{ name: string; type: string } | null>(null);
const newName = ref('');

// 面包屑路径
const pathSegments = computed(() => {
  if (currentPath.value === '/') return [];
  const parts = currentPath.value.split('/').filter(Boolean);
  return parts.map((part, index) => ({
    name: part,
    path: '/' + parts.slice(0, index + 1).join('/'),
  }));
});

const parentPath = computed(() => {
  if (currentPath.value === '/') return '/';
  const parts = currentPath.value.split('/').filter(Boolean);
  parts.pop();
  return '/' + parts.join('/') || '/';
});

// 获取文件列表
const fetchFiles = async (path: string = currentPath.value, page: number = 1) => {
  loading.value = true;
  error.value = null;
  try {
    const response = await apiClient.getFilesBrowse(path, page, 20);
    if (response.success && response.data) {
      folders.value = response.data.folders || [];
      files.value = response.data.files || [];
      pagination.value = response.data.pagination || pagination.value;
    }
  } catch (err) {
    error.value = '加载文件列表失败';
    console.error('Failed to fetch files:', err);
  } finally {
    loading.value = false;
  }
};

// 导航到指定路径
const navigateTo = (path: string) => {
  currentPath.value = path;
  fetchFiles(path);
};

// 刷新文件列表
const refreshFiles = () => {
  fetchFiles(currentPath.value, pagination.value.page);
};

// 分页
const changePage = (page: number) => {
  if (page < 1 || page > pagination.value.total_pages) return;
  fetchFiles(currentPath.value, page);
};

// 搜索（防抖）
let searchTimeout: number | null = null;
const debouncedSearch = () => {
  if (searchTimeout) clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => {
    if (searchQuery.value.trim()) {
      // TODO: 实现搜索功能
      console.log('Search:', searchQuery.value);
    } else {
      refreshFiles();
    }
  }, 300);
};

// 文件选择
const handleFileSelect = (event: Event) => {
  const input = event.target as HTMLInputElement;
  if (input.files) {
    selectedFiles.value = Array.from(input.files);
  }
};

// 上传文件
const uploadFiles = async () => {
  uploading.value = true;
  try {
    for (const file of selectedFiles.value) {
      const formData = new FormData();
      formData.append('file', file);
      formData.append('path', currentPath.value);
      await apiClient.uploadFile(formData);
    }
    showUploadModal.value = false;
    selectedFiles.value = [];
    await refreshFiles();
  } catch (err) {
    alert('上传失败');
    console.error('Upload failed:', err);
  } finally {
    uploading.value = false;
  }
};

// 新建文件夹
const createFolder = async () => {
  const name = prompt('请输入文件夹名称:');
  if (!name) return;
  try {
    await apiClient.createFolder(currentPath.value, name);
    await refreshFiles();
  } catch (err) {
    alert('创建文件夹失败');
    console.error('Create folder failed:', err);
  }
};

// 重命名
const renameItem = (name: string, type: string) => {
  renameItemData.value = { name, type };
  newName.value = name;
  showRenameModal.value = true;
};

const confirmRename = async () => {
  if (!renameItemData.value) return;
  renaming.value = true;
  try {
    const oldPath = currentPath.value + '/' + renameItemData.value.name;
    await apiClient.renameFile(oldPath, newName.value);
    await refreshFiles();
    showRenameModal.value = false;
    renameItemData.value = null;
  } catch (err) {
    alert('重命名失败');
    console.error('Rename failed:', err);
  } finally {
    renaming.value = false;
  }
};

// 删除
const deleteItem = async (path: string, type: string) => {
  if (!confirm(`确定要删除这个${type === 'folder' ? '文件夹' : '文件'}吗？`)) return;
  try {
    await apiClient.deleteFile(path);
    await refreshFiles();
  } catch (err) {
    alert('删除失败');
    console.error('Delete failed:', err);
  }
};

// 获取下载 URL
const getDownloadUrl = (path: string) => {
  return apiClient.downloadFile(path);
};

// 工具函数
const formatSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};

const formatTime = (timestamp: number): string => {
  const date = new Date(timestamp * 1000);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
};

const getFileIcon = (filename: string): string => {
  const ext = filename.split('.').pop()?.toLowerCase();
  const icons: Record<string, string> = {
    jpg: '🖼️', jpeg: '🖼️', png: '🖼️', gif: '🖼️', webp: '🖼️', bmp: '🖼️',
    mp4: '🎬', avi: '🎬', mov: '🎬', wmv: '🎬', mkv: '🎬',
    mp3: '🎵', wav: '🎵', flac: '🎵', aac: '🎵',
    pdf: '📄', doc: '📝', docx: '📝', xls: '📊', xlsx: '📊', ppt: '📊', pptx: '📊',
    txt: '📄', md: '📄', json: '📄', xml: '📄',
    zip: '📦', rar: '📦', tar: '📦', gz: '📦',
    exe: '⚙️', sh: '⚙️', bat: '⚙️',
  };
  return icons[ext || ''] || '📄';
};

onMounted(() => {
  fetchFiles();
});
</script>

<style scoped>
/* Files view specific styles */
</style>
