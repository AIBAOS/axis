import { lazy } from 'react'

// Layouts
export const MainLayout = lazy(() => import('../layouts/MainLayout'))
export const AuthLayout = lazy(() => import('../layouts/AuthLayout'))

// Pages
export const Dashboard = lazy(() => import('../pages/Dashboard'))
export const Login = lazy(() => import('../pages/Login'))
export const NotFound = lazy(() => import('../pages/NotFound'))
