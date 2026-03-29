<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">存储管理</h1>
          <p class="text-gray-600 mt-1">管理磁盘、存储池和共享文件夹</p>
        </div>
        <div class="flex space-x-2">
          <button @click="refreshAll" :disabled="loading" class="btn-secondary flex items-center space-x-2">
            <svg :class="{'animate-spin': loading}" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" /></svg>
            <span>刷新</span>
          </button>
        </div>
      </div>

      <!-- 存储概览 -->
      <div class="grid grid-cols-1 md:grid-cols-5 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" /></svg></div><div><p class="text-sm text-gray-500">总容量</p><p class="text-xl font-bold">{{ formatBytes(storageUsage.total_bytes) }}</p></div></div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-orange-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2V9a2 2 0 00-2-2h-2a2 2 0 00-2 2v10z" /></svg></div><div><p class="text-sm text-gray-500">已使用</p><p class="text-xl font-bold text-orange-600">{{ formatBytes(storageUsage.used_bytes) }}</p></div></div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-green-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg></div><div><p class="text-sm text-gray-500">可用空间</p><p class="text-xl font-bold text-green-600">{{ formatBytes(storageUsage.available_bytes) }}</p></div></div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-purple-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2" /></svg></div><div><p class="text-sm text-gray-500">磁盘数</p><p class="text-xl font-bold">{{ disks.length }}</p></div></div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center"><div class="w-10 h-10 rounded-lg bg-indigo-100 flex items-center justify-center mr-3"><svg class="w-5 h-5 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg></div><div><p class="text-sm text-gray-500">存储池</p><p class="text-xl font-bold">{{ pools.length }}</p></div></div>
        </div>
      </div>

      <!-- 存储空间使用进度条 -->
      <div class="bg-white rounded-lg shadow p-4">
        <div class="flex justify-between items-center mb-2"><h3 class="font-semibold text-gray-900">存储空间使用</h3><span class="text-sm text-gray-600">{{ usagePercent }}% 已使用</span></div>
        <div class="w-full bg-gray-200 rounded-full h-4"><div :class="usagePercent > 90 ? 'bg-red-500' : usagePercent > 70 ? 'bg-yellow-500' : 'bg-green-500'" class="h-4 rounded-full transition-all" :style="{ width: usagePercent + '%' }"></div></div>
        <div class="flex justify-between mt-2 text-xs text-gray-400"><span>0%</span><span>50%</span><span>100%</span></div>
      </div>

      <!-- 选项卡 -->
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8">
          <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id" :class="[currentTab === tab.id ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700', 'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center space-x-2']">
            <span>{{ tab.name }}</span>
            <span v-if="getTabCount(tab.id) > 0" class="px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-600">{{ getTabCount(tab.id) }}</span>
          </button>
        </nav>
      </div>

      <!-- 磁盘列表 -->
      <template v-if="currentTab === 'disks'">
        <div v-if="loading" class="flex justify-center py-12"><svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" /></svg></div>
        <div v-else-if="disks.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2" /></svg><p class="mt-4 text-gray-600">暂无磁盘</p></div>
        <div v-else class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <DiskCard v-for="disk in disks" :key="disk.id" :disk="disk" @smart="showSmartDetails" @detail="showDiskDetail" />
        </div>
      </template>

      <!-- 存储池 -->
      <template v-else-if="currentTab === 'pools'">
        <div class="flex justify-end mb-4">
          <button @click="showPoolModal = true" class="btn-primary text-sm">新建存储池</button>
        </div>
        <div v-if="pools.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg><p class="mt-4 text-gray-600">暂无存储池</p><p class="mt-2 text-sm text-gray-500">创建存储池以管理磁盘阵列</p></div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <StoragePoolCard v-for="pool in pools" :key="pool.id" :pool="pool" @edit="editPool" @expand="expandPool" @delete="deletePool" />
        </div>
      </template>

      <!-- 卷管理 -->
      <template v-else-if="currentTab === 'volumes'">
        <div class="flex justify-end mb-4">
          <button @click="showVolumeModal = true" class="btn-primary text-sm">新建卷</button>
        </div>
        <div v-if="volumes.length === 0" class="text-center py-12 bg-white rounded-lg shadow">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
          </svg>
          <p class="mt-4 text-gray-600">暂无卷</p>
          <p class="mt-2 text-sm text-gray-500">在存储池上创建卷以存储数据</p>
        </div>
        <div v-else class="bg-white rounded-lg shadow overflow-hidden">
          <table class="w-full">
            <thead class="bg-gray-50 border-b">
              <tr>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">卷名</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">所属池</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">容量</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">文件系统</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">挂载点</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">状态</th>
                <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">操作</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-for="vol in volumes" :key="vol.id" class="hover:bg-gray-50">
                <td class="px-4 py-3 text-sm font-medium text-gray-900">{{ vol.name }}</td>
                <td class="px-4 py-3 text-sm text-gray-600">{{ vol.pool_name }}</td>
                <td class="px-4 py-3 text-sm text-gray-600">{{ formatBytes(vol.size_bytes) }}</td>
                <td class="px-4 py-3"><span class="px-2 py-0.5 text-xs rounded bg-blue-100 text-blue-700">{{ vol.filesystem || 'ext4' }}</span></td>
                <td class="px-4 py-3 text-sm text-gray-600 font-mono">{{ vol.mount_point }}</td>
                <td class="px-4 py-3"><span :class="getVolumeStatusClass(vol.status)" class="px-2 py-1 text-xs rounded-full">{{ getVolumeStatusLabel(vol.status) }}</span></td>
                <td class="px-4 py-3 text-right">
                  <button @click="expandVolume(vol)" class="text-sm text-blue-600 hover:text-blue-700 mr-2">扩展</button>
                  <button @click="deleteVolume(vol)" class="text-sm text-red-600 hover:text-red-700">删除</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>

      <!-- 共享文件夹 -->
      <template v-else-if="currentTab === 'shares'">
        <div class="flex justify-end mb-4">
          <button @click="showShareModal = true" class="btn-primary text-sm">新建共享</button>
        </div>
        <div v-if="shares.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" /></svg><p class="mt-4 text-gray-600">暂无共享文件夹</p></div>
        <div v-else class="bg-white rounded-lg shadow overflow-hidden">
          <table class="w-full"><thead class="bg-gray-50 border-b"><tr><th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">名称</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">路径</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">协议</th><th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">权限</th><th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">操作</th></tr></thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-for="share in shares" :key="share.id" class="hover:bg-gray-50">
                <td class="px-4 py-3 text-sm font-medium text-gray-900">{{ share.name }}</td>
                <td class="px-4 py-3 text-sm text-gray-600 font-mono">{{ share.path }}</td>
                <td class="px-4 py-3"><span :class="getProtocolClass(share.protocol)" class="px-2 py-0.5 text-xs rounded">{{ share.protocol?.toUpperCase() }}</span></td>
                <td class="px-4 py-3 text-sm text-gray-600">{{ share.permissions || 'rw' }}</td>
                <td class="px-4 py-3 text-right">
                  <button @click="editShare(share)" class="text-sm text-primary-600 hover:text-primary-700 mr-2">编辑</button>
                  <button @click="deleteShare(share)" class="text-sm text-red-600 hover:text-red-700">删除</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>

      <!-- S.M.A.R.T. 详情模态框 -->
      <div v-if="selectedDisk" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto">
          <div class="flex justify-between items-center px-6 py-4 border-b sticky top-0 bg-white">
            <h3 class="text-lg font-semibold text-gray-900">磁盘详情</h3>
            <button @click="selectedDisk = null" class="text-gray-400 hover:text-gray-600"><svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button>
          </div>
          <div class="p-6 space-y-4">
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div><p class="text-gray-500">磁盘名称</p><p class="font-medium text-gray-900">{{ selectedDisk.name }}</p></div>
              <div><p class="text-gray-500">型号</p><p class="font-medium text-gray-900">{{ selectedDisk.model || '-' }}</p></div>
              <div><p class="text-gray-500">序列号</p><p class="font-mono text-gray-900">{{ selectedDisk.serial_number || '-' }}</p></div>
              <div><p class="text-gray-500">容量</p><p class="font-medium text-gray-900">{{ formatBytes(selectedDisk.size_bytes) }}</p></div>
              <div><p class="text-gray-500">温度</p><p :class="getTempClass(selectedDisk.temperature)" class="font-medium">{{ selectedDisk.temperature || '-' }}°C</p></div>
              <div><p class="text-gray-500">健康状态</p><span :class="getSmartClass(selectedDisk.smart_status)" class="px-2 py-1 text-xs rounded-full font-medium">{{ getSmartLabel(selectedDisk.smart_status) }}</span></div>
              <div><p class="text-gray-500">通电时间</p><p class="font-medium text-gray-900">{{ formatPowerOnHours(selectedDisk.power_on_hours) }}</p></div>
              <div v-if="selectedDisk.speed_rpm"><p class="text-gray-500">转速</p><p class="font-medium text-gray-900">{{ selectedDisk.speed_rpm }} RPM</p></div>
            </div>
            <div v-if="selectedDisk.smart_attributes" class="border-t pt-4">
              <h4 class="font-medium text-gray-900 mb-3">S.M.A.R.T. 属性</h4>
              <div class="space-y-2 text-sm">
                <div v-for="attr in selectedDisk.smart_attributes" :key="attr.id" class="flex justify-between"><span class="text-gray-600">{{ attr.name }}</span><span :class="attr.value < attr.threshold ? 'text-red-600' : 'text-gray-900'">{{ attr.value }} (阈值: {{ attr.threshold }})</span></div>
              </div>
            </div>
          </div>
          <div class="px-6 py-4 bg-gray-50 rounded-b-lg flex justify-end"><button @click="selectedDisk = null" class="btn-secondary">关闭</button></div>
        </div>
      </div>

      <!-- 存储池模态框 -->
      <div v-if="showPoolModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4">
          <div class="flex justify-between items-center px-6 py-4 border-b"><h3 class="text-lg font-semibold text-gray-900">{{ editingPool ? '编辑存储池' : '新建存储池' }}</h3><button @click="showPoolModal = false; editingPool = null" class="text-gray-400 hover:text-gray-600"><svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button></div>
          <form @submit.prevent="savePool" class="p-6 space-y-4">
            <div><label class="block text-sm font-medium text-gray-700 mb-1">存储池名称</label><input v-model="poolForm.name" type="text" required class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="data-pool" /></div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">RAID 级别</label>
              <select v-model="poolForm.raid_level" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500">
                <option value="single">单盘 (无 RAID)</option>
                <option value="raid0">RAID 0 (条带) - 性能优先</option>
                <option value="raid1">RAID 1 (镜像) - 安全优先</option>
                <option value="raid5">RAID 5 (分布式奇偶校验) - 平衡</option>
                <option value="raid10">RAID 10 (镜像+条带) - 高性能+安全</option>
              </select>
              <div class="mt-2 p-2 bg-gray-50 rounded text-xs text-gray-600">
                <p v-if="poolForm.raid_level === 'single'">无冗余，单盘故障将导致数据丢失</p>
                <p v-else-if="poolForm.raid_level === 'raid0'">最高性能，无冗余，任一盘故障数据全部丢失。最少 2 盘。</p>
                <p v-else-if="poolForm.raid_level === 'raid1'">完整镜像，可承受 1 盘故障。容量利用率 50%。最少 2 盘。</p>
                <p v-else-if="poolForm.raid_level === 'raid5'">可承受 1 盘故障，读写平衡。最少 3 盘。</p>
                <p v-else-if="poolForm.raid_level === 'raid10'">可承受每组 1 盘故障，高性能+安全。最少 4 盘。</p>
              </div>
            </div>
            <div><label class="block text-sm font-medium text-gray-700 mb-1">成员磁盘</label><div class="space-y-2 max-h-40 overflow-y-auto border rounded-lg p-2">
              <label v-for="disk in availableDisks" :key="disk.id" class="flex items-center"><input type="checkbox" :value="disk.id" v-model="poolForm.disk_ids" class="h-4 w-4 text-primary-600 rounded" /><span class="ml-2 text-sm text-gray-700">{{ disk.name }} ({{ formatBytes(disk.size_bytes) }})</span></label>
            </div>
              <p v-if="poolForm.disk_ids.length > 0" class="mt-1 text-xs text-gray-500">已选 {{ poolForm.disk_ids.length }} 盘</p>
            </div>
          </form>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg"><button @click="showPoolModal = false; editingPool = null" class="px-4 py-2 border rounded-lg text-gray-700 hover:bg-gray-50">取消</button><button @click="savePool" class="btn-primary">保存</button></div>
        </div>
      </div>

      <!-- 共享模态框 -->
      <div v-if="showShareModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4">
          <div class="flex justify-between items-center px-6 py-4 border-b"><h3 class="text-lg font-semibold text-gray-900">{{ editingShare ? '编辑共享' : '新建共享' }}</h3><button @click="showShareModal = false; editingShare = null" class="text-gray-400 hover:text-gray-600"><svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button></div>
          <form @submit.prevent="saveShare" class="p-6 space-y-4">
            <div><label class="block text-sm font-medium text-gray-700 mb-1">共享名称</label><input v-model="shareForm.name" type="text" required class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="Public" /></div>
            <div><label class="block text-sm font-medium text-gray-700 mb-1">路径</label><input v-model="shareForm.path" type="text" required class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" placeholder="/srv/shares/public" /></div>
            <div><label class="block text-sm font-medium text-gray-700 mb-1">协议</label><select v-model="shareForm.protocol" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"><option value="smb">SMB/CIFS</option><option value="nfs">NFS</option><option value="webdav">WebDAV</option></select></div>
            <div><label class="block text-sm font-medium text-gray-700 mb-1">权限</label><select v-model="shareForm.permissions" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"><option value="rw">读写</option><option value="ro">只读</option></select></div>
          </form>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg"><button @click="showShareModal = false; editingShare = null" class="px-4 py-2 border rounded-lg text-gray-700 hover:bg-gray-50">取消</button><button @click="saveShare" class="btn-primary">保存</button></div>
        </div>
      </div>

      <!-- 卷模态框 -->
      <div v-if="showVolumeModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4">
          <div class="flex justify-between items-center px-6 py-4 border-b">
            <h3 class="text-lg font-semibold text-gray-900">{{ editingVolume ? '扩展卷' : '新建卷' }}</h3>
            <button @click="showVolumeModal = false; editingVolume = null" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>
          <form @submit.prevent="saveVolume" class="p-6 space-y-4">
            <div><label class="block text-sm font-medium text-gray-700 mb-1">卷名</label><input v-model="volumeForm.name" type="text" required :disabled="!!editingVolume" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-gray-100" placeholder="data" /></div>
            <div v-if="!editingVolume"><label class="block text-sm font-medium text-gray-700 mb-1">所属存储池</label><select v-model="volumeForm.pool_id" required class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"><option v-for="pool in pools" :key="pool.id" :value="pool.id">{{ pool.name }} ({{ formatBytes(pool.size_bytes) }})</option></select></div>
            <div><label class="block text-sm font-medium text-gray-700 mb-1">容量 (GB)</label><input v-model.number="volumeForm.size_gb" type="number" min="1" required class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" /></div>
            <div v-if="!editingVolume"><label class="block text-sm font-medium text-gray-700 mb-1">文件系统</label><select v-model="volumeForm.filesystem" class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"><option value="ext4">ext4</option><option value="xfs">XFS</option><option value="btrfs">Btrfs</option><option value="zfs">ZFS</option></select></div>
          </form>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button @click="showVolumeModal = false; editingVolume = null" class="px-4 py-2 border rounded-lg text-gray-700 hover:bg-gray-50">取消</button>
            <button @click="saveVolume" class="btn-primary">{{ editingVolume ? '扩展' : '创建' }}</button>
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
import DiskCard from '@/components/storage/DiskCard.vue'
import StoragePoolCard from '@/components/storage/StoragePoolCard.vue'
import { api } from '@/utils/api'

const tabs = [{ id: 'disks', name: '物理磁盘' }, { id: 'pools', name: '存储池' }, { id: 'volumes', name: '卷管理' }, { id: 'shares', name: '共享文件夹' }]
const currentTab = ref('disks')
const loading = ref(true)

const disks = ref<any[]>([])
const pools = ref<any[]>([])
const shares = ref<any[]>([])
const volumes = ref<any[]>([])
const storageUsage = ref({ total_bytes: 0, used_bytes: 0, available_bytes: 0 })

const selectedDisk = ref<any>(null)

const showPoolModal = ref(false)
const editingPool = ref<any>(null)
const poolForm = ref({ name: '', raid_level: 'single', disk_ids: [] as number[] })

const showShareModal = ref(false)
const editingShare = ref<any>(null)
const shareForm = ref({ name: '', path: '', protocol: 'smb', permissions: 'rw' })

const showVolumeModal = ref(false)
const editingVolume = ref<any>(null)
const volumeForm = ref({ name: '', pool_id: 0, size_gb: 100, filesystem: 'ext4' })

const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

const usagePercent = computed(() => storageUsage.value.total_bytes > 0 ? Math.round(storageUsage.value.used_bytes / storageUsage.value.total_bytes * 100) : 0)
const availableDisks = computed(() => disks.value.filter(d => !d.pool_id))
const getTabCount = (id: string) => id === 'disks' ? disks.value.length : id === 'pools' ? pools.value.length : id === 'volumes' ? volumes.value.length : shares.value.length

const refreshAll = async () => { loading.value = true; await Promise.all([loadDisks(), loadPools(), loadShares()]); loading.value = false }

const loadDisks = async () => { try { const r = await api.storage.getDisks(); disks.value = r.data.disks || r.data || [] } catch (e) {} }
const loadPools = async () => { try { const r = await api.storage.getPools(); pools.value = r.data.pools || r.data || [] } catch (e) {} }
const loadShares = async () => { try { const r = await api.storage.getUsage(); storageUsage.value = r.data.data || r.data || { total_bytes: 0, used_bytes: 0, available_bytes: 0 } } catch (e) {} }

const showSmartDetails = (d: any) => { selectedDisk.value = d }
const showDiskDetail = (d: any) => { selectedDisk.value = d }

const savePool = async () => { if (!poolForm.value.name) return; if (editingPool.value) { const i = pools.value.findIndex(p => p.id === editingPool.value.id); if (i >= 0) pools.value[i] = { ...editingPool.value, ...poolForm.value }; showToast('success', '存储池已更新') } else { pools.value.push({ id: Date.now(), ...poolForm.value, size_bytes: poolForm.value.disk_ids.reduce((sum, id) => { const d = disks.value.find(x => x.id === id); return sum + (d?.size_bytes || 0) }, 0), used_bytes: 0, status: 'online' }); showToast('success', '存储池已创建') } showPoolModal.value = false; editingPool.value = null }
const editPool = (p: any) => { editingPool.value = p; poolForm.value = { name: p.name, raid_level: p.raid_level || 'single', disk_ids: p.disk_ids || [] }; showPoolModal.value = true }
const expandPool = (p: any) => { editingPool.value = p; poolForm.value = { name: p.name, raid_level: p.raid_level || 'single', disk_ids: p.disk_ids || [] }; showPoolModal.value = true; showToast('info', '选择要添加的磁盘') }
const deletePool = async (p: any) => { if (!confirm(`确定删除存储池 "${p.name}" 吗？`)) return; pools.value = pools.value.filter(x => x.id !== p.id); showToast('success', '存储池已删除') }

const saveShare = async () => { if (!shareForm.value.name || !shareForm.value.path) return; if (editingShare.value) { const i = shares.value.findIndex(s => s.id === editingShare.value.id); if (i >= 0) shares.value[i] = { ...editingShare.value, ...shareForm.value }; showToast('success', '共享已更新') } else { shares.value.push({ id: Date.now(), ...shareForm.value }); showToast('success', '共享已创建') } showShareModal.value = false; editingShare.value = null }
const editShare = (s: any) => { editingShare.value = s; shareForm.value = { name: s.name, path: s.path, protocol: s.protocol || 'smb', permissions: s.permissions || 'rw' }; showShareModal.value = true }
const deleteShare = async (s: any) => { if (!confirm(`确定删除共享 "${s.name}" 吗？`)) return; shares.value = shares.value.filter(x => x.id !== s.id); showToast('success', '共享已删除') }

// 卷管理
const saveVolume = async () => { if (!volumeForm.value.name || !volumeForm.value.pool_id) return; const pool = pools.value.find(p => p.id === volumeForm.value.pool_id); if (editingVolume.value) { const i = volumes.value.findIndex(v => v.id === editingVolume.value.id); if (i >= 0) volumes.value[i] = { ...editingVolume.value, ...volumeForm.value, size_bytes: volumeForm.value.size_gb * 1024 * 1024 * 1024 }; showToast('success', '卷已更新') } else { volumes.value.push({ id: Date.now(), ...volumeForm.value, pool_name: pool?.name, size_bytes: volumeForm.value.size_gb * 1024 * 1024 * 1024, mount_point: `/mnt/${volumeForm.value.name}`, status: 'online' }); showToast('success', '卷已创建') } showVolumeModal.value = false; editingVolume.value = null }
const expandVolume = (v: any) => { editingVolume.value = v; volumeForm.value = { name: v.name, pool_id: v.pool_id, size_gb: Math.ceil(v.size_bytes / 1024 / 1024 / 1024), filesystem: v.filesystem || 'ext4' }; showVolumeModal.value = true }
const deleteVolume = async (v: any) => { if (!confirm(`确定删除卷 "${v.name}" 吗？此操作将删除所有数据！`)) return; volumes.value = volumes.value.filter(x => x.id !== v.id); showToast('success', '卷已删除') }
const getVolumeStatusClass = (s: string) => s === 'online' ? 'bg-green-100 text-green-700' : s === 'offline' ? 'bg-gray-100 text-gray-700' : 'bg-red-100 text-red-700'
const getVolumeStatusLabel = (s: string) => s === 'online' ? '在线' : s === 'offline' ? '离线' : '错误'

const formatBytes = (b: number) => { if (!b) return '0 B'; const k = 1024; const s = ['B', 'KB', 'MB', 'GB', 'TB']; const i = Math.floor(Math.log(b) / Math.log(k)); return (b / Math.pow(k, i)).toFixed(1) + ' ' + s[i] }
const getTempClass = (t: number) => !t ? 'text-gray-900' : t > 50 ? 'text-red-600' : t > 40 ? 'text-yellow-600' : 'text-green-600'
const getSmartClass = (s: string) => ['healthy', 'good'].includes(s?.toLowerCase()) ? 'bg-green-100 text-green-700' : ['warning', 'caution'].includes(s?.toLowerCase()) ? 'bg-yellow-100 text-yellow-700' : ['failed', 'bad'].includes(s?.toLowerCase()) ? 'bg-red-100 text-red-700' : 'bg-gray-100 text-gray-700'
const getSmartLabel = (s: string) => ['healthy', 'good'].includes(s?.toLowerCase()) ? '健康' : ['warning', 'caution'].includes(s?.toLowerCase()) ? '警告' : ['failed', 'bad'].includes(s?.toLowerCase()) ? '故障' : '未知'
const formatPowerOnHours = (h: number) => { if (!h) return '-'; const d = Math.floor(h / 24); return d > 365 ? `${Math.floor(d / 365)} 年 ${d % 365} 天` : d > 0 ? `${d} 天` : `${h} 小时` }
const getProtocolClass = (p: string) => p === 'smb' ? 'bg-blue-100 text-blue-700' : p === 'nfs' ? 'bg-green-100 text-green-700' : p === 'webdav' ? 'bg-purple-100 text-purple-700' : 'bg-gray-100 text-gray-700'

const showToast = (type: 'success' | 'error', msg: string) => { toast.value = { show: true, type, message: msg }; setTimeout(() => toast.value.show = false, 3000) }

onMounted(() => refreshAll())
</script>