import { useState, useEffect } from 'react'
import apiClient from '@/libs/api'

export default function Dashboard() {
  const [status, setStatus] = useState(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    apiClient
      .get('/api/v1/system/info')
      .then((data) => setStatus(data))
      .finally(() => setLoading(false))
  }, [])

  if (loading) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
      </div>
    )
  }

  if (!status) {
    return <div className="text-center text-gray-500">无法获取系统状态</div>
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
          <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">CPU使用率</h3>
          <p className="mt-2 text-3xl font-semibold text-gray-900 dark:text-white">24%</p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
          <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">内存使用</h3>
          <p className="mt-2 text-3xl font-semibold text-gray-900 dark:text-white">4.2 / 16 GB</p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
          <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">存储容量</h3>
          <p className="mt-2 text-3xl font-semibold text-gray-900 dark:text-white">2.4 / 10 TB</p>
        </div>
      </div>

      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">系统信息</h2>
        <div className="space-y-2">
          <div className="flex justify-between py-2 border-b border-gray-100 dark:border-gray-700">
            <span className="text-gray-600 dark:text-gray-400">主机名</span>
            <span className="font-medium text-gray-900 dark:text-white">{status.hostName}</span>
          </div>
          <div className="flex justify-between py-2 border-b border-gray-100 dark:border-gray-700">
            <span className="text-gray-600 dark:text-gray-400">版本</span>
            <span className="font-medium text-gray-900 dark:text-white">{status.version}</span>
          </div>
          <div className="flex justify-between py-2 border-b border-gray-100 dark:border-gray-700">
            <span className="text-gray-600 dark:text-gray-400">内核</span>
            <span className="font-medium text-gray-900 dark:text-white">{status.kernel}</span>
          </div>
          <div className="flex justify-between py-2">
            <span className="text-gray-600 dark:text-gray-400">运行时间</span>
            <span className="font-medium text-gray-900 dark:text-white">{status.uptime}</span>
          </div>
        </div>
      </div>
    </div>
  )
}
