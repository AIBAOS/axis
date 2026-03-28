import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Login from '../views/Login.vue'
import Dashboard from '../views/Dashboard.vue'
import Files from '../views/Files.vue'
import Storage from '../views/Storage.vue'
import Users from '../views/Users'
import Backups from '../views/Backups'
import Settings from '../views/Settings'

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
    path: '/users',
    name: 'Users',
    component: Users,
    meta: { requiresAuth: true },
  },
  {
    path: '/backups',
    name: 'Backups',
    component: Backups,
    meta: { requiresAuth: true },
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings,
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
