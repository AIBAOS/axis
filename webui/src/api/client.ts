/**
 * Axis NAS API Client
 * 
 * Base URL is configured via environment variable:
 * - VITE_API_BASE_URL (default: http://localhost:8080)
 */

import axios, { AxiosInstance, AxiosError, AxiosResponse } from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';
const API_VERSION = '/api/v1';

export interface ApiResponse<T = unknown> {
  success: boolean;
  message?: string;
  data?: T;
  error?: string;
  code?: string;
}

export interface ApiError {
  message: string;
  code: string;
  status: number;
}

class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string = API_BASE_URL) {
    this.client = axios.create({
      baseURL: baseURL + API_VERSION,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Request interceptor - add JWT token
    this.client.interceptors.request.use(
      (config) => {
        const token = localStorage.getItem('jwt_token');
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
      },
      (error) => Promise.reject(error)
    );

    // Response interceptor - handle errors
    this.client.interceptors.response.use(
      (response: AxiosResponse<ApiResponse>) => response,
      (error: AxiosError<ApiResponse>) => {
        const apiError: ApiError = {
          message: error.response?.data?.error || error.response?.data?.message || 'Network error',
          code: error.response?.data?.code || 'UNKNOWN_ERROR',
          status: error.response?.status || 0,
        };
        return Promise.reject(apiError);
      }
    );
  }

  async get<T>(endpoint: string): Promise<ApiResponse<T>> {
    const response = await this.client.get<ApiResponse<T>>(endpoint);
    return response.data;
  }

  async post<T>(endpoint: string, data?: unknown): Promise<ApiResponse<T>> {
    const response = await this.client.post<ApiResponse<T>>(endpoint, data);
    return response.data;
  }

  async put<T>(endpoint: string, data?: unknown): Promise<ApiResponse<T>> {
    const response = await this.client.put<ApiResponse<T>>(endpoint, data);
    return response.data;
  }

  async delete<T>(endpoint: string): Promise<ApiResponse<T>> {
    const response = await this.client.delete<ApiResponse<T>>(endpoint);
    return response.data;
  }

  // Auth methods
  async login(username: string, password: string): Promise<ApiResponse<{ token: string; user: unknown }>> {
    return this.post('/auth/login', { username, password });
  }

  async logout(): Promise<ApiResponse<void>> {
    return this.post('/auth/logout');
  }

  // System methods
  async getSystemHealth() {
    return this.get('/system/health');
  }

  async getSystemInfo() {
    return this.get('/system/info');
  }

  // Backup methods
  async getBackups(params?: { page?: number; page_size?: number; status?: string }) {
    return this.get('/backups', { params });
  }

  async createBackup(data: {
    name: string;
    description?: string;
    source_path: string;
    destination: string;
    backup_type: string;
    schedule?: string;
  }) {
    return this.post('/backups', data);
  }

  // File methods
  async getFilesBrowse(path?: string, page?: number, limit?: number) {
    return this.get('/files/browse', { 
      params: { 
        path: path || '/',
        page: page || 1,
        limit: limit || 20,
      } 
    });
  }

  async uploadFile(formData: FormData) {
    return this.client.post<ApiResponse>('/files/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    }).then(r => r.data);
  }

  async deleteFile(path: string) {
    return this.delete('/files/delete', { params: { path } });
  }

  async renameFile(oldPath: string, newName: string) {
    return this.put('/files/rename', { old_path: oldPath, new_name: newName });
  }

  async createFolder(path: string, name: string) {
    return this.post('/files/mkdir', { path, name });
  }

  async downloadFile(path: string) {
    const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';
    return `${API_BASE_URL}${API_VERSION}/files/download?path=${encodeURIComponent(path)}`;
  }

  // Storage methods
  async getStorageDisks() {
    return this.get('/storage/disks');
  }

  async getStorageDiskDetail(id: number) {
    return this.get(`/storage/disks/${id}`);
  }

  async getStoragePools() {
    return this.get('/storage/pools');
  }

  async getStoragePoolDetail(id: number) {
    return this.get(`/storage/pools/${id}`);
  }

  async createStoragePool(data: {
    name: string;
    raid_type: string;
    disk_ids: number[];
  }) {
    return this.post('/storage/pools', data);
  }

  async getStorageVolumes() {
    return this.get('/storage/volumes');
  }

  async getStorageVolumeDetail(id: number) {
    return this.get(`/storage/volumes/${id}`);
  }

  async createStorageVolume(data: {
    pool_id: number;
    name: string;
    size: number;
  }) {
    return this.post('/storage/volumes', data);
  }

  async getStorageUsage() {
    return this.get('/storage/usage');
  }

  // User methods
  async getUsers(params?: { page?: number; per_page?: number; role?: string; status?: string }) {
    return this.get('/users', { params });
  }

  async getUserDetail(id: number) {
    return this.get(`/users/${id}`);
  }

  async createUser(data: {
    username: string;
    password: string;
    email: string;
    role: string;
  }) {
    return this.post('/users', data);
  }

  async updateUser(id: number, data: {
    email?: string;
    role?: string;
    is_active?: boolean;
  }) {
    return this.put(`/users/${id}`, data);
  }

  async deleteUser(id: number) {
    return this.delete(`/users/${id}`);
  }
}

export const apiClient = new ApiClient();
export default apiClient;
