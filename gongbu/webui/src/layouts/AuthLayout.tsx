import { ReactNode } from 'react'

export default function AuthLayout({ children }: { children: ReactNode }) {
  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div>
          <h1 className="text-center text-3xl font-bold text-gray-900 dark:text-white">
            Axis NAS
          </h1>
          <p className="mt-2 text-center text-sm text-gray-600 dark:text-gray-400">
            WebUI v1.0.0
          </p>
        </div>
        {children}
      </div>
    </div>
  )
}
