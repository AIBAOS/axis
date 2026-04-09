import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Login from '../views/Login.vue'
import Dashboard from '../views/Dashboard.vue'
import Files from '../views/Files.vue'
import Storage from '../views/Storage.vue'
import Printers from '../views/Printers.vue'
import Backups from '../views/backups/BackupList.vue'
import NetworkManagement from '../views/NetworkManagement.vue'
import ShareManagement from '../views/ShareManagement.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: Dashboard,
    meta: { requiresAuth: true },
  },
  {
    path: '/files',
    name: 'Files',
    component: Files,
    meta: { requiresAuth: true },
  },
  {
    path: '/storage',
    name: 'Storage',
    component: Storage,
    meta: { requiresAuth: true },
  },
  {
    path: '/printers',
    name: 'Printers',
    component: PrinterList,
    meta: { requiresAuth: true },
  },
  {
    path: '/backups',
    name: 'Backups',
    component: BackupList,
    meta: { requiresAuth: true },
  },
  {
    path: '/network',
    name: 'NetworkManagement',
    component: NetworkManagement,
    meta: { requiresAuth: true },
  },
  {
    path: '/shares',
    name: 'ShareManagement',
    component: ShareManagement,
    meta: { requiresAuth: true },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// 路由守卫 - 检查登录状态
router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('jwt_token')
  
  if (to.meta.requiresAuth && !token) {
    next('/login')
  } else if (to.name === 'Login' && token) {
    next('/dashboard')
  } else {
    next()
  }
})

export default router
