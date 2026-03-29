import axios from 'axios'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080'

// 创建 axios 实例
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json'
  }
})

// 请求拦截器 - 添加 JWT token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('jwt_token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器 - 处理错误
apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Token 过期或未认证，跳转到登录页
      localStorage.removeItem('jwt_token')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

export default apiClient

// API 方法封装
export const api = {
  // 认证
  auth: {
    login: (username: string, password: string) => 
      apiClient.post('/api/v1/auth/login', { username, password }),
    logout: () => apiClient.post('/api/v1/auth/logout')
  },

  // 下载
  downloads: {
    list: (params?: any) => apiClient.get('/api/v1/downloads', { params }),
    get: (id: number) => apiClient.get(`/api/v1/downloads/${id}`),
    create: (data: { url: string; filename?: string; save_path?: string }) => apiClient.post('/api/v1/downloads', data),
    delete: (id: number) => apiClient.delete(`/api/v1/downloads/${id}`),
    cancel: (id: number) => apiClient.post(`/api/v1/downloads/${id}/cancel`),
    start: (id: number) => apiClient.post(`/api/v1/downloads/${id}/start`),
    pause: (id: number) => apiClient.post(`/api/v1/downloads/${id}/pause`),
    retry: (id: number) => apiClient.post(`/api/v1/downloads/${id}/retry`),
    stats: () => apiClient.get('/api/v1/downloads/stats')
  },

  // 用户
  users: {
    list: (params?: any) => apiClient.get('/api/v1/users', { params }),
    get: (id: string) => apiClient.get(`/api/v1/users/${id}`),
    create: (data: any) => apiClient.post('/api/v1/users', data),
    update: (id: string, data: any) => apiClient.put(`/api/v1/users/${id}`, data),
    delete: (id: string) => apiClient.delete(`/api/v1/users/${id}`)
  },

  // 系统
  system: {
    info: () => apiClient.get('/api/v1/system/info'),
    resources: () => apiClient.get('/api/v1/system/resources'),
    logs: (params?: any) => apiClient.get('/api/v1/system/logs', { params }),
    health: () => apiClient.get('/api/v1/system/health'),
    restart: (delaySeconds?: number) => apiClient.post('/api/v1/system/restart', { delay_seconds: delaySeconds || 0 }),
    shutdown: (delaySeconds?: number) => apiClient.post('/api/v1/system/shutdown', { delay_seconds: delaySeconds || 0 }),
    factoryReset: () => apiClient.post('/api/v1/system/factory-reset'),
    checkUpdates: () => apiClient.get('/api/v1/system/updates/check'),
    installUpdate: () => apiClient.post('/api/v1/system/updates/install'),
    cronJobs: {
      list: (params?: any) => apiClient.get('/api/v1/system/cron-jobs', { params }),
      get: (id: number) => apiClient.get(`/api/v1/system/cron-jobs/${id}`),
      create: (data: any) => apiClient.post('/api/v1/system/cron-jobs', data),
      update: (id: number, data: any) => apiClient.put(`/api/v1/system/cron-jobs/${id}`, data),
      delete: (id: number) => apiClient.delete(`/api/v1/system/cron-jobs/${id}`)
    }
  },

  // 文件
  files: {
    browse: (params?: any) => apiClient.get('/api/v1/files/browse', { params }),
    list: (params?: any) => apiClient.get('/api/v1/files', { params }),
    get: (id: string) => apiClient.get(`/api/v1/files/${id}`),
    upload: (file: File, path?: string) => {
      const formData = new FormData()
      formData.append('file', file)
      if (path) formData.append('path', path)
      return apiClient.post('/api/v1/files/upload', formData, {
        headers: { 'Content-Type': 'multipart/form-data' }
      })
    },
    download: (path: string) => apiClient.get(`/api/v1/files/${encodeURIComponent(path)}/download`, {
      responseType: 'blob'
    }),
    delete: (path: string) => apiClient.delete(`/api/v1/files/${encodeURIComponent(path)}`),
    createFolder: (path: string, name: string) => apiClient.post('/api/v1/files/folder', { path, name }),
    rename: (path: string, newName: string) => apiClient.put(`/api/v1/files/${encodeURIComponent(path)}`, { name: newName }),
    move: (path: string, destination: string) => apiClient.post(`/api/v1/files/${encodeURIComponent(path)}/move`, { destination }),
    copy: (path: string, destination: string) => apiClient.post(`/api/v1/files/${encodeURIComponent(path)}/copy`, { destination })
  },

  // 应用管理
  apps: {
    list: (params?: any) => apiClient.get('/api/v1/apps', { params }),
    get: (id: number) => apiClient.get(`/api/v1/apps/${id}`),
    install: (data: { name: string; version: string; category?: string; description?: string; size_bytes?: number }) => 
      apiClient.post('/api/v1/apps', data),
    uninstall: (id: number) => apiClient.delete(`/api/v1/apps/${id}`),
    start: (id: number) => apiClient.post(`/api/v1/apps/${id}/start`),
    stop: (id: number) => apiClient.post(`/api/v1/apps/${id}/stop`),
    restart: (id: number) => apiClient.post(`/api/v1/apps/${id}/restart`),
    update: (id: number) => apiClient.post(`/api/v1/apps/${id}/update`)
  },

  // 备份
  backups: {
    list: (params?: any) => apiClient.get('/api/v1/backups', { params }),
    get: (id: string) => apiClient.get(`/api/v1/backups/${id}`),
    create: (data: any) => apiClient.post('/api/v1/backups', data),
    update: (id: string, data: any) => apiClient.put(`/api/v1/backups/${id}`, data),
    delete: (id: string) => apiClient.delete(`/api/v1/backups/${id}`)
  },

  // 存储
  storage: {
    getVolumes: (params?: any) => apiClient.get('/api/v1/storage/volumes', { params }),
    getVolume: (id: string) => apiClient.get(`/api/v1/storage/volumes/${id}`),
    getPools: (params?: any) => apiClient.get('/api/v1/storage/pools', { params }),
    getPool: (id: string) => apiClient.get(`/api/v1/storage/pools/${id}`),
    getDisks: (params?: any) => apiClient.get('/api/v1/storage/disks', { params }),
    getDisk: (id: string) => apiClient.get(`/api/v1/storage/disks/${id}`),
    getUsage: () => apiClient.get('/api/v1/storage/usage')
  },

  // 设置
  settings: {
    get: () => apiClient.get('/api/v1/settings'),
    update: (data: any) => apiClient.put('/api/v1/settings', data)
  },

  // 共享
  shares: {
    // SMB
    listSmb: (params?: any) => apiClient.get('/api/v1/shares/smb', { params }),
    getSmb: (id: number) => apiClient.get(`/api/v1/shares/smb/${id}`),
    createSmb: (data: any) => apiClient.post('/api/v1/shares/smb', data),
    updateSmb: (id: number, data: any) => apiClient.put(`/api/v1/shares/smb/${id}`, data),
    deleteSmb: (id: number) => apiClient.delete(`/api/v1/shares/smb/${id}`),

    // NFS
    listNfs: (params?: any) => apiClient.get('/api/v1/shares/nfs', { params }),
    getNfs: (id: number) => apiClient.get(`/api/v1/shares/nfs/${id}`),
    createNfs: (data: any) => apiClient.post('/api/v1/shares/nfs', data),
    updateNfs: (id: number, data: any) => apiClient.put(`/api/v1/shares/nfs/${id}`, data),
    deleteNfs: (id: number) => apiClient.delete(`/api/v1/shares/nfs/${id}`),

    // WebDAV
    listWebdav: (params?: any) => apiClient.get('/api/v1/shares/webdav', { params }),
    getWebdav: (id: number) => apiClient.get(`/api/v1/shares/webdav/${id}`),
    createWebdav: (data: any) => apiClient.post('/api/v1/shares/webdav', data),
    updateWebdav: (id: number, data: any) => apiClient.put(`/api/v1/shares/webdav/${id}`, data),
    deleteWebdav: (id: number) => apiClient.delete(`/api/v1/shares/webdav/${id}`),

    // FTP
    listFtp: (params?: any) => apiClient.get('/api/v1/shares/ftp', { params }),
    getFtp: (id: number) => apiClient.get(`/api/v1/shares/ftp/${id}`),
    createFtp: (data: any) => apiClient.post('/api/v1/shares/ftp', data),
    updateFtp: (id: number, data: any) => apiClient.put(`/api/v1/shares/ftp/${id}`, data),
    deleteFtp: (id: number) => apiClient.delete(`/api/v1/shares/ftp/${id}`)
  },

  // 打印机
  printers: {
    list: (params?: any) => apiClient.get('/api/v1/printers', { params }),
    get: (id: number) => apiClient.get(`/api/v1/printers/${id}`),
    create: (data: any) => apiClient.post('/api/v1/printers', data),
    update: (id: number, data: any) => apiClient.put(`/api/v1/printers/${id}`, data),
    delete: (id: number) => apiClient.delete(`/api/v1/printers/${id}`),
    discover: () => apiClient.get('/api/v1/printers/discover'),
    setDefault: (id: number) => apiClient.post(`/api/v1/printers/${id}/set-default`),
    jobs: (id: number, params?: any) => apiClient.get(`/api/v1/printers/${id}/jobs`, { params }),
    createJob: (id: number, data: any) => apiClient.post(`/api/v1/printers/${id}/jobs`, data),
    updateJob: (printerId: number, jobId: number, data: any) => apiClient.put(`/api/v1/printers/${printerId}/jobs/${jobId}`, data),
    cancelJob: (printerId: number, jobId: number) => apiClient.delete(`/api/v1/printers/${printerId}/jobs/${jobId}`)
  },

  // 网络
  network: {
    listInterfaces: (params?: any) => apiClient.get('/api/v1/network/interfaces', { params }),
    getInterface: (id: number) => apiClient.get(`/api/v1/network/interfaces/${id}`),
    createInterface: (data: any) => apiClient.post('/api/v1/network/interfaces', data),
    updateInterface: (id: number, data: any) => apiClient.put(`/api/v1/network/interfaces/${id}`, data),
    deleteInterface: (id: number) => apiClient.delete(`/api/v1/network/interfaces/${id}`),
    listConfig: () => apiClient.get('/api/v1/network/config'),
    getConfig: (id: number) => apiClient.get(`/api/v1/network/config/${id}`),
    updateConfig: (id: number, data: any) => apiClient.put(`/api/v1/network/config/${id}`, data)
  }
}