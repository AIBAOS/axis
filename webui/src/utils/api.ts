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
    logs: (params?: any) => apiClient.get('/api/v1/system/logs', { params })
  },

  // 文件
  files: {
    list: (params?: any) => apiClient.get('/api/v1/files', { params }),
    upload: (file: File) => {
      const formData = new FormData()
      formData.append('file', file)
      return apiClient.post('/api/v1/files/upload', formData, {
        headers: { 'Content-Type': 'multipart/form-data' }
      })
    },
    delete: (id: string) => apiClient.delete(`/api/v1/files/${id}`)
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
  }
}