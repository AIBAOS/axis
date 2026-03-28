import { BrowserRouter, Routes, Route, useLocation } from 'react-router-dom'
import { Suspense } from 'react'

import { MainLayout, AuthLayout, Dashboard, Login, NotFound } from './routes'

function Router() {
  return (
    <BrowserRouter>
      <AppRoutes />
    </BrowserRouter>
  )
}

function AppRoutes() {
  const location = useLocation()

  // Auth routes
  if (location.pathname.startsWith('/login')) {
    return (
      <AuthLayout>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </AuthLayout>
    )
  }

  // Main app routes
  return (
    <MainLayout>
      <Suspense fallback={<div className="flex items-center justify-center h-screen">Loading...</div>}>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </Suspense>
    </MainLayout>
  )
}

export default Router
