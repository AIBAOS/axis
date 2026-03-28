import { ReactNode } from 'react'
import { Link, useLocation } from 'react-router-dom'
import { BiHome, BiComputer, BiFolder, BiNetworkChart, BiSettings } from 'react-icons/bi'

interface NavItem {
  path: string
  label: string
  icon: ReactNode
}

const navItems: NavItem[] = [
  { path: '/', label: '仪表盘', icon: <BiHome className="w-5 h-5" /> },
  { path: '/storage', label: '存储', icon: <BiComputer className="w-5 h-5" /> },
  { path: '/shares', label: '共享', icon: <BiFolder className="w-5 h-5" /> },
  { path: '/network', label: '网络', icon: <BiNetworkChart className="w-5 h-5" /> },
  { path: '/settings', label: '设置', icon: <BiSettings className="w-5 h-5" /> },
]

export default function MainLayout({ children }: { children: ReactNode }) {
  const location = useLocation()

  return (
    <div className="flex min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Sidebar */}
      <aside className="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 fixed h-full">
        <div className="p-6">
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Axis NAS</h1>
          <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
            WebUI v1.0.0
          </p>
        </div>
        <nav className="mt-6 px-4 space-y-2">
          {navItems.map((item) => (
            <Link
              key={item.path}
              to={item.path}
              className={`flex items-center px-4 py-3 rounded-lg transition-colors ${
                location.pathname === item.path
                  ? 'bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400'
                  : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'
              }`}
            >
              {item.icon}
              <span className="ml-3">{item.label}</span>
            </Link>
          ))}
        </nav>
      </aside>

      {/* Main content */}
      <div className="flex-1 ml-64">
        <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-8 py-4">
          <div className="flex justify-between items-center">
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
              {navItems.find((i) => i.path === location.pathname)?.label || '仪表盘'}
            </h2>
            <div className="flex items-center space-x-4">
              <button className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">
                <span className="sr-only">Notifications</span>
              </button>
              <div className="flex items-center space-x-2">
                <span className="text-sm text-gray-700 dark:text-gray-300">Admin</span>
              </div>
            </div>
          </div>
        </header>
        <main className="p-8">{children}</main>
      </div>
    </div>
  )
}
