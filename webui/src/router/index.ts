import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '../views/DashboardView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: DashboardView
    },
    {
      path: '/home',
      name: 'home',
      component: () => import('../views/HomeView.vue')
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue')
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/LoginView.vue')
    },
    {
      path: '/files',
      name: 'files',
      component: () => import('../views/FilesView.vue')
    },
    {
      path: '/storage',
      name: 'storage',
      component: () => import('../views/StorageView.vue')
    },
    {
      path: '/users',
      name: 'users',
      component: () => import('../views/UsersView.vue')
    },
    {
      path: '/backups',
      name: 'backups',
      component: () => import('../views/BackupsView.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/SettingsView.vue')
    },
    {
      path: '/shares',
      name: 'shares',
      component: () => import('../views/SharesView.vue')
    },
    {
      path: '/logs',
      name: 'logs',
      component: () => import('../views/LogsView.vue')
    },
    {
      path: '/printers',
      name: 'printers',
      component: () => import('../views/PrintersView.vue')
    }
  ]
})

export default router
